use commit_boost::prelude::BlsPublicKey;
use eyre::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use common::beacon::{BeaconApiClient, HttpClient};
use common::slot_timer::SlotTimer;
use common::types::beacon::BeaconTiming;
use common::types::DatabaseContext;

/// Configuration for the proposer lookahead task
#[derive(Debug, Clone)]
pub struct ProposerLookaheadConfig {
	/// How often to update the lookahead (in seconds)
	pub update_interval_seconds: u64,
	/// Number of slots to look ahead
	pub lookahead_window: u64,
}

/// Proposer lookahead task that periodically updates the proposer schedule
pub struct ProposerLookaheadTask<H: HttpClient> {
	config: ProposerLookaheadConfig,
	slot_timer: SlotTimer,
	database: DatabaseContext,
	beacon_client: BeaconApiClient<H>,
}

impl<H: HttpClient> ProposerLookaheadTask<H> {
	/// Create a new proposer lookahead task
	pub fn new(
		config: ProposerLookaheadConfig,
		slot_timer: SlotTimer,
		database: DatabaseContext,
		beacon_client: BeaconApiClient<H>,
	) -> Self {
		Self { config, slot_timer, database, beacon_client }
	}

	/// Run the proposer lookahead task continuously
	pub async fn run(&self) -> Result<()> {
		info!(
			"Starting proposer lookahead task with {}s update interval and {} slot lookahead",
			self.config.update_interval_seconds, self.config.lookahead_window
		);

		loop {
			if let Err(e) = self.update_lookahead().await {
				error!("Error updating proposer lookahead: {}", e);
			}

			sleep(Duration::from_secs(self.config.update_interval_seconds)).await;
		}
	}

	/// Update the proposer lookahead for upcoming slots
	async fn update_lookahead(&self) -> Result<()> {
		let current_slot = self.slot_timer.get_current_slot();
		let lookahead_end_slot = current_slot + self.config.lookahead_window;

		// Calculate which epochs we need to populate
		let current_epoch = BeaconTiming::slot_to_epoch(current_slot);
		let end_epoch = BeaconTiming::slot_to_epoch(lookahead_end_slot);

		info!(
			"Updating proposer lookahead for epochs {} to {} (slots {} to {})",
			current_epoch, end_epoch, current_slot, lookahead_end_slot
		);

		// Populate each epoch in the range
		for epoch in current_epoch..=end_epoch {
			if let Err(e) = self.populate_lookahead(epoch, None).await {
				error!("Failed to populate lookahead for epoch {}: {}", epoch, e);
				// Continue to next epoch instead of failing completely
			}
		}

		Ok(())
	}

	/// Populate the proposer lookahead for a specific epoch
	/// This is a public method that can be called from tests or for manual population
	/// If proposer_key is provided, all slots in the epoch will use that key (useful for testing)
	/// Otherwise, fetch proposer duties from the beacon node
	pub async fn populate_lookahead(&self, epoch: u64, proposer_key: Option<BlsPublicKey>) -> Result<()> {
		// Calculate the slot range for this epoch
		let start_slot = BeaconTiming::epoch_to_first_slot(epoch);
		let end_slot = BeaconTiming::epoch_to_last_slot(epoch);

		// If a test proposer key is provided, use it for all slots in the epoch
		if let Some(key) = proposer_key {
			for slot in start_slot..=end_slot {
				self.database.store_proposer_lookahead(slot, &key)?;
			}
			info!(
				"Populated proposer lookahead for epoch {} (slots {} to {}) with test key",
				epoch, start_slot, end_slot
			);
			return Ok(());
		}

		info!("Fetching proposer duties from beacon node for epoch {} (slots {} to {})", epoch, start_slot, end_slot);

		// Fetch proposer duties for this epoch
		match self.beacon_client.get_proposer_duties(epoch).await {
			Ok(duties) => {
				// Process each duty and store the slot's proposer
				for duty in duties.data {
					match duty.parse_slot() {
						Ok(slot) => match duty.parse_pubkey() {
							Ok(pubkey) => {
								self.database.store_proposer_lookahead(slot, &pubkey)?;
							}
							Err(e) => {
								warn!("Failed to parse pubkey for slot {} in epoch {}: {}", slot, epoch, e);
							}
						},
						Err(e) => {
							warn!("Failed to parse slot in epoch {}: {}", epoch, e);
						}
					}
				}
				info!("Successfully processed proposer duties for epoch {}", epoch);
				Ok(())
			}
			Err(e) => {
				error!("Failed to fetch proposer duties for epoch {}: {}", epoch, e);
				Err(e)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use cb_common::types::BlsSecretKey;
	use common::beacon::{BeaconApiClient, HttpResponse, MockHttpClient};
	use common::config::BeaconApiConfig;
	use common::types::beacon::{ProposerDutiesResponse, ValidatorDuty};
	use common::types::DatabaseContext;
	use hex;
	use rocksdb::{Options, DB};
	use std::sync::Arc;
	use tempfile::TempDir;

	fn create_test_beacon_config() -> BeaconApiConfig {
		BeaconApiConfig {
			primary_endpoint: "https://test-beacon.example.com".to_string(),
			fallback_endpoints: vec![],
			request_timeout_secs: 30,
			genesis_time: 1606824023,
		}
	}

	#[test]
	fn test_proposer_lookahead_config() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };
		assert_eq!(config.update_interval_seconds, 60);
		assert_eq!(config.lookahead_window, 64);
	}

	#[test]
	fn test_proposer_lookahead_task_creation() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

		// Create a mock database context for testing
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let slot_timer = SlotTimer::new(1606824023); // Mainnet genesis

		// Create mock beacon client
		let mock_http = MockHttpClient::new();
		let beacon_config = create_test_beacon_config();
		let beacon_client = BeaconApiClient::new(beacon_config, mock_http).unwrap();

		let _task = ProposerLookaheadTask::new(config, slot_timer, database, beacon_client);
		// Test passes if creation doesn't panic
	}

	#[tokio::test]
	async fn test_populate_lookahead_with_test_key() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

		// Create a mock database context for testing
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let slot_timer = SlotTimer::new(1606824023);

		// Create mock beacon client (won't be called since we provide a test key)
		let mock_http = MockHttpClient::new();
		let beacon_config = create_test_beacon_config();
		let beacon_client = BeaconApiClient::new(beacon_config, mock_http).unwrap();

		let task = ProposerLookaheadTask::new(config, slot_timer, database.clone(), beacon_client);

		// Create a test BLS key
		let test_key = BlsSecretKey::random().public_key();

		// Populate lookahead for epoch 3 (slots 96-127) with test key
		task.populate_lookahead(3, Some(test_key.clone())).await.unwrap();

		// Verify that the test key was stored for all slots in epoch 3
		for slot in 96..=127 {
			let proposer = database.get_proposer_lookahead(slot).unwrap();
			assert!(proposer.is_some(), "Proposer key should be stored for slot {}", slot);
			assert_eq!(proposer.unwrap(), test_key, "Stored key should match test key for slot {}", slot);
		}
	}

	#[tokio::test]
	async fn test_populate_lookahead_from_beacon() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

		// Create a mock database context for testing
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let slot_timer = SlotTimer::new(1606824023);

		// Create mock beacon client with expected response
		let mut mock_http = MockHttpClient::new();

		// Create a valid BLS public key for testing
		let test_bls_key = BlsSecretKey::random().public_key();
		let test_pubkey_bytes = test_bls_key.serialize();
		let test_pubkey_hex = format!("0x{}", hex::encode(test_pubkey_bytes));

		// Clone the response for use in the closure
		let duties_data = vec![
			ValidatorDuty {
				validator_index: "100".to_string(),
				pubkey: test_pubkey_hex.clone(),
				slot: "100".to_string(),
			},
			ValidatorDuty {
				validator_index: "101".to_string(),
				pubkey: test_pubkey_hex.clone(),
				slot: "105".to_string(),
			},
		];

		mock_http.expect_get().times(1).returning(move |url| {
			assert!(url.contains("eth/v1/validator/duties/proposer/3")); // Epoch 3
			let response =
				ProposerDutiesResponse { execution_optimistic: false, finalized: true, data: duties_data.clone() };
			Ok(HttpResponse { status: 200, body: serde_json::to_vec(&response).unwrap() })
		});

		let beacon_config = create_test_beacon_config();
		let beacon_client = BeaconApiClient::new(beacon_config, mock_http).unwrap();

		let task = ProposerLookaheadTask::new(config, slot_timer, database.clone(), beacon_client);

		// Populate lookahead for epoch 3 from beacon
		task.populate_lookahead(3, None).await.unwrap();

		// Verify that keys were stored for the slots that were in the beacon response
		let proposer_100 = database.get_proposer_lookahead(100).unwrap();
		assert!(proposer_100.is_some(), "Proposer key should be stored for slot 100");
		assert_eq!(proposer_100.unwrap(), test_bls_key, "Stored key should match test key");

		let proposer_105 = database.get_proposer_lookahead(105).unwrap();
		assert!(proposer_105.is_some(), "Proposer key should be stored for slot 105");
		assert_eq!(proposer_105.unwrap(), test_bls_key, "Stored key should match test key");

		// Slots not in the beacon response should not have keys
		let proposer_102 = database.get_proposer_lookahead(102).unwrap();
		assert!(proposer_102.is_none(), "Proposer key should not be stored for slot 102");
	}
}
