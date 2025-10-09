use eyre::Result;
use reqwest::Client;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

use common::config::InclusionPreconfConfig;
use common::constants::routes;
use common::types::{DatabaseContext, ProcessConstraintsRequest, ProcessConstraintsResponse};
use common::SlotTimer;

/// Constraints task that monitors delegated slots and triggers constraint processing
/// 2 seconds before each delegated slot begins
pub struct ConstraintsTask {
	config: InclusionPreconfConfig,
	slot_timer: SlotTimer,
	http_client: Client,
	database: DatabaseContext,
}

impl ConstraintsTask {
	/// Create a new constraints task
	pub fn new(config: InclusionPreconfConfig, slot_timer: SlotTimer, database: DatabaseContext) -> Self {
		Self { config, slot_timer, http_client: Client::new(), database }
	}

	/// Run the constraints task continuously
	pub async fn run(&self) -> Result<()> {
		info!("Starting constraints task - monitoring delegated slots");

		loop {
			if let Err(e) = self.check_and_process_constraints().await {
				error!("Error in constraints check: {}", e);
			}

			// Sleep for a short interval before checking again
			tokio::time::sleep(Duration::from_millis(100)).await;
		}
	}

	/// Check for delegated slots and process constraints if needed
	async fn check_and_process_constraints(&self) -> Result<()> {
		let current_slot = self.slot_timer.get_current_slot();
		let target_slot = current_slot + 1;

		// Check if target slot is delegated
		match self.database.is_delegated(target_slot) {
			Ok(true) => {
				// Calculate time until 2 seconds before target slot starts
				let target_slot_start = self.config.eth_genesis_timestamp + (target_slot * 12);
				let trigger_time = target_slot_start - 2; // 2 seconds before slot starts
				let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

				if now >= trigger_time {
					// Time to process constraints for this slot
					info!("Triggering constraints processing for slot {} (2 seconds before slot start)", target_slot);
					if let Err(e) = self.post_constraints(target_slot).await {
						warn!("Failed to process constraints for slot {}: {}", target_slot, e);
					}
				} else {
					// Wait until it's time to trigger
					let wait_duration = trigger_time - now;
					info!("Slot {} is delegated, waiting {} seconds until trigger time", target_slot, wait_duration);
					tokio::time::sleep(Duration::from_secs(wait_duration)).await;

					// Now process constraints
					info!("Triggering constraints processing for slot {}", target_slot);
					if let Err(e) = self.post_constraints(target_slot).await {
						warn!("Failed to process constraints for slot {}: {}", target_slot, e);
					}
				}
			}
			Ok(false) => {
				// Target slot is not delegated, nothing to do
				// Sleep for a longer interval to avoid busy waiting
				tokio::time::sleep(Duration::from_secs(1)).await;
			}
			Err(e) => {
				error!("Failed to check delegation status for slot {}: {}", target_slot, e);
				// Sleep briefly before retrying
				tokio::time::sleep(Duration::from_millis(500)).await;
			}
		}

		Ok(())
	}

	/// Post constraints for a specific slot
	async fn post_constraints(&self, slot: u64) -> Result<()> {
		// Get delegation from database
		let delegation = match self.database.get_delegation_for_slot(slot) {
			Ok(Some(delegation)) => delegation,
			Ok(None) => {
				warn!("No delegation found for slot {} despite is_delegated() returning true", slot);
				return Ok(());
			}
			Err(e) => {
				error!("Failed to get delegation for slot {}: {}", slot, e);
				return Err(eyre::eyre!("Failed to get delegation for slot {}: {}", slot, e));
			}
		};

		// Extract BLS keys from delegation
		let bls_public_key = format!("0x{}", hex::encode(delegation.message.delegate.serialize()));
		let proposer_public_key = format!("0x{}", hex::encode(delegation.message.proposer.serialize()));

		// Build request
		let request = ProcessConstraintsRequest {
			slot,
			bls_public_key,
			proposer_public_key,
			receivers: self.config.constraints_receivers.clone(),
		};

		// Build URL
		let url = format!(
			"http://{}:{}{}",
			self.config.constraints_server_host,
			self.config.constraints_server_port,
			routes::constraints::PROCESS
		);

		info!("Calling process_constraints for slot {} at {}", slot, url);

		// Send request
		let response = self.http_client.post(&url).json(&request).send().await?;

		if !response.status().is_success() {
			return Err(eyre::eyre!("HTTP error: {}", response.status()));
		}

		let process_response: ProcessConstraintsResponse = response.json().await?;

		if process_response.success {
			info!("Successfully processed {} constraints for slot {}", process_response.processed_count, slot);
		} else {
			warn!("Process constraints failed for slot {}: {}", slot, process_response.message);
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_constraints_task_creation() {
		let config = InclusionPreconfConfig {
			commitments_server_host: "127.0.0.1".to_string(),
			commitments_server_port: 8080,
			commitments_database_url: "test.db".to_string(),
			constraints_database_url: "constraints.db".to_string(),
			delegations_database_url: "delegations.db".to_string(),
			pricing_database_url: "pricing.db".to_string(),
			log_level: "info".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],
			constraints_server_host: "127.0.0.1".to_string(),
			constraints_server_port: 8081,
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: None,
			constraints_bls_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			constraints_delegate_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			eth_genesis_timestamp: 1606824023,
			constraints_receivers: vec![
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			],
		};
		let slot_timer = SlotTimer::new(1606824023);

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

		let _task = ConstraintsTask::new(config, slot_timer, database);
		// Test passes if creation doesn't panic
	}
}
