use eyre::Result;
use tracing::{error, info, warn};

use super::client::ConstraintsClient;
use crate::utils::{create_constraints_message, create_signed_constraints};
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use common::types::{DatabaseContext, ProcessConstraintsResponse, ProcessDelegationsResponse, SignedDelegation};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Process constraints for a specific slot
pub async fn process_constraints(
	slot: u64,
	gateway_public_key: BlsPublicKey,
	proposer_public_key: BlsPublicKey,
	receivers: Vec<BlsPublicKey>,
	database: &DatabaseContext,
	commit_config: Arc<Mutex<StartCommitModuleConfig<common::config::InclusionPreconfConfig>>>,
	relay_url: String,
	api_key: Option<String>,
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
	let signed_constraints = create_signed_constraints(&constraints_message, commit_config, gateway_public_key).await?;

	// 4. Send to relay using the client
	let client = ConstraintsClient::new(relay_url, api_key);

	match client.post_constraints(&signed_constraints).await {
		Ok(_) => {
			info!("Successfully sent constraints for slot {} to relay", slot);

			// 5. Mark constraints as sent (atomic operation)
			let mut all_marked = true;
			for (constraint_id, _) in &slot_constraints {
				if let Err(e) = database.mark_constraint_sent(constraint_id) {
					error!("Failed to mark constraint {} as sent: {}", constraint_id, e);
					all_marked = false;
					// Continue processing other constraints
				}
			}

			if !all_marked {
				warn!("Some constraints could not be marked as sent for slot {}", slot);
			}

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
			Err(eyre::eyre!("Failed to send constraints to relay: {}", e))
		}
	}
}

/// Process delegations for a specific slot and delegate
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process_constraints_response() {
		let response = ProcessConstraintsResponse {
			success: true,
			slot: 12345,
			processed_count: 5,
			signed_constraints: None,
			message: "Test message".to_string(),
		};

		assert!(response.success);
		assert_eq!(response.slot, 12345);
		assert_eq!(response.processed_count, 5);
		assert_eq!(response.message, "Test message");
	}

	#[test]
	fn test_process_delegations_response() {
		let response = ProcessDelegationsResponse {
			success: true,
			slot: 12345,
			total_delegations: 10,
			matching_delegations: vec![],
			message: "Found 0 matching delegations out of 10 total".to_string(),
		};

		assert!(response.success);
		assert_eq!(response.slot, 12345);
		assert_eq!(response.total_delegations, 10);
		assert_eq!(response.matching_delegations.len(), 0);
		assert_eq!(response.message, "Found 0 matching delegations out of 10 total");
	}
}
