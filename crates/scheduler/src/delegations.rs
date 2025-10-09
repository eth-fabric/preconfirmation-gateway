use eyre::Result;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use common::constants::routes;
use common::types::{DatabaseContext, ProcessDelegationsRequest, ProcessDelegationsResponse};
use common::SlotTimer;

/// Configuration for the delegation task
#[derive(Debug, Clone)]
pub struct DelegationTaskConfig {
	/// How often to run the delegation check (in seconds)
	pub check_interval_seconds: u64,
	/// Number of slots to look ahead
	pub lookahead_window: u64,
	/// URL of the constraints service
	pub constraints_service_url: String,
	/// BLS public key for the delegate
	pub delegate_bls_public_key: String,
}

impl Default for DelegationTaskConfig {
	fn default() -> Self {
		Self {
			check_interval_seconds: 1,
			lookahead_window: 64,
			constraints_service_url: "http://localhost:8081".to_string(),
			delegate_bls_public_key: String::new(),
		}
	}
}

/// Delegation task that periodically checks for delegations in upcoming slots
pub struct DelegationTask {
	config: DelegationTaskConfig,
	slot_timer: SlotTimer,
	http_client: Client,
	database: DatabaseContext,
}

impl DelegationTask {
	/// Create a new delegation task
	pub fn new(config: DelegationTaskConfig, slot_timer: SlotTimer, database: DatabaseContext) -> Self {
		Self { config, slot_timer, http_client: Client::new(), database }
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
		let request =
			ProcessDelegationsRequest { slot, delegate_bls_public_key: self.config.delegate_bls_public_key.clone() };

		let url = format!("{}{}", self.config.constraints_service_url, routes::constraints::PROCESS_DELEGATIONS);

		info!("Calling process_delegations for slot {} at {}", slot, url);

		let response = self.http_client.post(&url).json(&request).send().await?;

		if !response.status().is_success() {
			return Err(eyre::eyre!("HTTP error: {}", response.status()));
		}

		let process_response: ProcessDelegationsResponse = response.json().await?;

		if process_response.success {
			info!(
				"Successfully processed {} delegations for slot {}",
				process_response.matching_delegations.len(),
				slot
			);
			// Note: Delegations are already stored by the process_delegations_handler
		} else {
			warn!("Process delegations failed for slot {}: {}", slot, process_response.message);
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_delegation_task_config_default() {
		let config = DelegationTaskConfig::default();
		assert_eq!(config.check_interval_seconds, 1);
		assert_eq!(config.lookahead_window, 64);
		assert_eq!(config.constraints_service_url, "http://localhost:8081");
	}

	#[test]
	fn test_delegation_task_creation() {
		let config = DelegationTaskConfig::default();
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

		let _task = DelegationTask::new(config, slot_timer, database);
		// Test passes if creation doesn't panic
	}
}
