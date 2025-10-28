use eyre::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

use alloy::primitives::B256;
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use common::config::{GatewayConfig, InclusionGatewayConfig};
use common::slot_timer::{SlotTimer, CONSTRAINT_TRIGGER_OFFSET, SLOT_TIME_SECONDS};
use common::types::{DatabaseContext, ProcessConstraintsResponse};
use constraints::client::ConstraintsClient;
use constraints::utils::{create_constraints_message, create_signed_constraints};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Constraints task that monitors delegated slots and triggers constraint processing
/// 2 seconds before each delegated slot begins
pub struct ConstraintsTask {
	config: InclusionGatewayConfig,
	slot_timer: SlotTimer,
	database: DatabaseContext,
	commit_config: Arc<Mutex<StartCommitModuleConfig<InclusionGatewayConfig>>>,
	relay_url: String,
	api_key: Option<String>,
}

impl ConstraintsTask {
	/// Create a new constraints task
	pub fn new(
		config: InclusionGatewayConfig,
		slot_timer: SlotTimer,
		database: DatabaseContext,
		commit_config: Arc<Mutex<StartCommitModuleConfig<InclusionGatewayConfig>>>,
		relay_url: String,
		api_key: Option<String>,
	) -> Self {
		Self { config, slot_timer, database, commit_config, relay_url, api_key }
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
				// Check if constraints have already been posted for this slot to prevent reprocessing
				match self.database.are_constraints_posted_for_slot(target_slot) {
					Ok(true) => {
						info!("Constraints already posted for slot {}, skipping", target_slot);
						// Sleep for a longer interval since we don't need to process this slot
						tokio::time::sleep(Duration::from_secs(1)).await;
						return Ok(());
					}
					Ok(false) => {
						// Calculate time until trigger offset before target slot starts
						let target_slot_start = self.config.genesis_timestamp() + (target_slot * SLOT_TIME_SECONDS);
						let trigger_time = target_slot_start - CONSTRAINT_TRIGGER_OFFSET; // trigger offset before slot starts
						let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

						if now >= trigger_time {
							// Time to process constraints for this slot
							info!(
								"Triggering constraints processing for slot {} ({} seconds before slot start)",
								target_slot, CONSTRAINT_TRIGGER_OFFSET
							);
							if let Err(e) = self.post_constraints(target_slot).await {
								warn!("Failed to process constraints for slot {}: {}", target_slot, e);
							}
						} else {
							// Wait until it's time to trigger
							let wait_duration = trigger_time - now;
							info!(
								"Slot {} is delegated, waiting {} seconds until trigger time",
								target_slot, wait_duration
							);
							tokio::time::sleep(Duration::from_secs(wait_duration)).await;

							// Now process constraints
							info!("Triggering constraints processing for slot {}", target_slot);
							if let Err(e) = self.post_constraints(target_slot).await {
								warn!("Failed to process constraints for slot {}: {}", target_slot, e);
							}
						}
					}
					Err(e) => {
						error!("Failed to check constraint posted status for slot {}: {}", target_slot, e);
						// Continue with processing despite the error
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

	/// Process constraints for a specific slot
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
		let gateway_public_key = delegation.message.delegate;
		let proposer_public_key = delegation.message.proposer;

		// Parse receiver BLS public keys from config
		let receivers = constraints::parse_bls_public_keys(self.config.constraints_receivers(), "receiver")?;

		// Parse module_signing_id from config
		let module_signing_id = self
			.config
			.module_signing_id()
			.parse::<B256>()
			.map_err(|e| eyre::eyre!("Failed to parse module_signing_id from config: {}", e))?;

		info!("Processing constraints for slot {}", slot);

		// Call constraints processing function
		let response = process_constraints::<InclusionGatewayConfig>(
			slot,
			gateway_public_key,
			proposer_public_key,
			receivers, // receivers
			&self.database,
			self.commit_config.clone(),
			self.relay_url.clone(),
			self.api_key.clone(),
			&module_signing_id,
		)
		.await?;

		if response.success {
			info!("Successfully processed {} constraints for slot {}", response.processed_count, slot);

			// Mark constraints as posted for this slot to prevent reprocessing
			if let Err(e) = self.database.mark_constraints_posted_for_slot(slot) {
				error!("Failed to mark constraints as posted for slot {}: {}", slot, e);
			} else {
				info!("Marked constraints as posted for slot {}", slot);
			}
		} else {
			warn!("Process constraints failed for slot {}: {}", slot, response.message);
		}

		Ok(())
	}
}

/// Process constraints for a specific slot
/// This function can be used by any gateway implementation to process constraints
pub async fn process_constraints<T>(
	slot: u64,
	gateway_public_key: BlsPublicKey,
	proposer_public_key: BlsPublicKey,
	receivers: Vec<BlsPublicKey>,
	database: &DatabaseContext,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	relay_url: String,
	api_key: Option<String>,
	module_signing_id: &B256,
) -> Result<ProcessConstraintsResponse> {
	info!("Processing constraints for slot {} with gateway public key", slot);

	// 1. Get constraints for the specific slot
	let slot_constraints = database
		.get_constraints_for_slot(slot)
		.map_err(|e| eyre::eyre!("Failed to get constraints for slot {}: {}", slot, e))?;

	if slot_constraints.is_empty() {
		info!("No constraints found for slot {}", slot);
		return Ok(ProcessConstraintsResponse {
			success: true,
			slot,
			processed_count: 0,
			signed_constraints: None,
			message: format!("No constraints found for slot {}", slot),
		});
	}

	// 2. Create constraints message from slot constraints
	let constraints_message = create_constraints_message(
		slot_constraints.clone(),
		proposer_public_key,
		gateway_public_key.clone(), // gateway acts as the delegate for signing
		slot,
		receivers,
	)?;

	// 3. Sign the constraints message with the provided gateway public key
	let signed_constraints =
		create_signed_constraints(&constraints_message, commit_config, gateway_public_key, module_signing_id).await?;

	// 4. Send to relay using the client
	let client = ConstraintsClient::new(relay_url, api_key);

	match client.post_constraints(&signed_constraints).await {
		Ok(_) => {
			info!("Successfully sent constraints for slot {} to relay", slot);

			Ok(ProcessConstraintsResponse {
				success: true,
				slot,
				processed_count: constraints_message.constraints.len(),
				signed_constraints: Some(signed_constraints),
				message: format!(
					"Successfully processed {} constraints for slot {}",
					constraints_message.constraints.len(),
					slot
				),
			})
		}
		Err(e) => {
			error!("Failed to send constraints for slot {} to relay: {}", slot, e);
			return Ok(ProcessConstraintsResponse {
				success: false,
				slot,
				processed_count: 0,
				signed_constraints: None,
				message: format!("Failed to send constraints for slot {} to relay: {}", slot, e),
			});
		}
	}
}
