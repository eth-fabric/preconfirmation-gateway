use eyre::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use commit_boost::prelude::BlsPublicKey;
use common::config::InclusionGatewayConfig;
use common::slot_timer::SlotTimer;
use common::types::{DatabaseContext, ProcessDelegationsResponse, SignedDelegation};
use constraints::client::ConstraintsClient;
use constraints::parse_bls_public_key;

/// Configuration for the delegation task
#[derive(Debug, Clone)]
pub struct DelegationTaskConfig {
	/// How often to run the delegation check (in seconds)
	pub check_interval_seconds: u64,
	/// Number of slots to look ahead
	pub lookahead_window: u64,
}

/// Delegation task that periodically checks for delegations in upcoming slots
pub struct DelegationTask {
	config: DelegationTaskConfig,
	app_config: InclusionGatewayConfig,
	slot_timer: SlotTimer,
	database: DatabaseContext,
	relay_url: String,
	api_key: Option<String>,
}

impl DelegationTask {
	/// Create a new delegation task
	pub fn new(
		config: DelegationTaskConfig,
		app_config: InclusionGatewayConfig,
		slot_timer: SlotTimer,
		database: DatabaseContext,
		relay_url: String,
		api_key: Option<String>,
	) -> Self {
		Self { config, app_config, slot_timer, database, relay_url, api_key }
	}

	/// Run the delegation task continuously
	pub async fn run(&self) -> Result<()> {
		info!(
			"Starting delegation task with {}s interval and {} slot lookahead",
			self.config.check_interval_seconds, self.config.lookahead_window
		);

		loop {
			if let Err(e) = self.check_delegations().await {
				error!("Error in delegation check: {}", e);
			}

			sleep(Duration::from_secs(self.config.check_interval_seconds)).await;
		}
	}

	/// Check delegations for upcoming slots
	async fn check_delegations(&self) -> Result<()> {
		let current_slot = self.slot_timer.get_current_slot();
		let lookahead_end = current_slot + self.config.lookahead_window;

		info!("Checking delegations for slots {} to {}", current_slot, lookahead_end);

		// Check each slot in the lookahead window
		for slot in current_slot..=lookahead_end {
			// Check if slot is already delegated in database
			match self.database.is_delegated(slot) {
				Ok(true) => {
					info!("Slot {} already has delegations, skipping", slot);
					continue;
				}
				Ok(false) => {
					// Slot is not delegated, call process_delegations endpoint
					if let Err(e) = self.process_delegations_for_slot(slot).await {
						warn!("Failed to process delegations for slot {}: {}", slot, e);
					}
				}
				Err(e) => {
					error!("Failed to check delegation status for slot {}: {}", slot, e);
				}
			}
		}

		Ok(())
	}

	/// Process delegations for a specific slot
	async fn process_delegations_for_slot(&self, slot: u64) -> Result<()> {
		// Parse the delegate BLS public key
		let delegate_bls_public_key = parse_bls_public_key(&self.app_config.delegate_public_key, "delegate")?;

		info!("Processing delegations for slot {}", slot);

		// Call delegations processing function
		let response = process_delegations(
			slot,
			delegate_bls_public_key,
			&self.database,
			self.relay_url.clone(),
			self.api_key.clone(),
		)
		.await?;

		if response.success {
			info!("Successfully processed {} delegations for slot {}", response.matching_delegations.len(), slot);
		} else {
			warn!("Process delegations failed for slot {}: {}", slot, response.message);
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_delegation_task_config_default() {
		let config = DelegationTaskConfig { check_interval_seconds: 1, lookahead_window: 64 };
		assert_eq!(config.check_interval_seconds, 1);
		assert_eq!(config.lookahead_window, 64);
	}

	#[test]
	fn test_delegation_task_creation() {
		let config = DelegationTaskConfig { check_interval_seconds: 1, lookahead_window: 64 };

		let app_config = InclusionGatewayConfig {
			// Commitments server configuration
			commitments_server_host: "127.0.0.1".to_string(),
			commitments_server_port: 8080,
			commitments_database_path: "test.db".to_string(),
			commitments_bls_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			log_level: "info".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],

			// Gateway orchestration configuration
			relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: None,
			genesis_timestamp: 1606824023,
			delegation_database_path: "delegations.db".to_string(),
			execution_endpoint_url: "http://localhost:8545".to_string(),
			execution_request_timeout_secs: 10,
			execution_max_retries: 3,
			constraints_receivers: vec![
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			],
			delegate_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			module_signing_id: "0x1111111111111111111111111111111111111111111111111111111111111111".to_string(),
		};
		let slot_timer = SlotTimer::new(1606824023); // Mainnet genesis
											   // Create a mock database context for testing
		use common::types::DatabaseContext;
		use rocksdb::{Options, DB};
		use std::sync::Arc;
		use tempfile::TempDir;

		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let _task = DelegationTask::new(
			config,
			app_config,
			slot_timer,
			database,
			"https://relay.example.com".to_string(),
			None,
		);
		// Test passes if creation doesn't panic
	}
}

/// Process delegations for a specific slot and delegate
/// This function can be used by any gateway implementation to process delegations
pub async fn process_delegations(
	slot: u64,
	delegate_bls_public_key: BlsPublicKey,
	database: &DatabaseContext,
	relay_url: String,
	api_key: Option<String>,
) -> Result<ProcessDelegationsResponse> {
	info!("Processing delegations request for slot {} and delegate", slot);

	// Create client and fetch delegations for the slot
	let client = ConstraintsClient::new(relay_url, api_key);

	let delegations = client.get_delegations_for_slot(slot).await?;

	info!("Retrieved {} delegations for slot {}", delegations.len(), slot);

	// Store total count before filtering
	let total_delegations = delegations.len();

	// Filter delegations that match the delegate BLS public key
	let matching_delegations: Vec<SignedDelegation> = delegations
		.into_iter()
		.filter(|signed_delegation| {
			// Compare delegate BLS public keys
			signed_delegation.message.delegate == delegate_bls_public_key
		})
		.collect();

	let matching_count = matching_delegations.len();
	info!("Found {} matching delegations for delegate", matching_count);

	// Store matching delegations in the database
	for delegation in &matching_delegations {
		if let Err(e) = database.store_delegation(slot, delegation) {
			error!("Failed to store delegation for slot {}: {}", slot, e);
			// Continue processing other delegations even if one fails
		} else {
			info!("Stored delegation for slot {} with committer {}", slot, delegation.message.committer);
		}
	}

	Ok(ProcessDelegationsResponse {
		success: true,
		slot,
		total_delegations,
		matching_delegations,
		message: format!("Found {} matching delegations out of {} total", matching_count, total_delegations),
	})
}
