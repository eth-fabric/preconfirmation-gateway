use alloy::primitives::B256;
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use common::slot_timer::SlotTimer;
use eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use common::signer;
use common::types::constraints::{Constraint, ConstraintsMessage, SignedConstraints};

/// Creates a properly signed constraints message using BLS
pub async fn create_signed_constraints<T>(
	message: &ConstraintsMessage,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	bls_public_key: BlsPublicKey,
) -> Result<SignedConstraints> {
	debug!("Creating signed constraints with proper BLS signing");

	// 1. Get the message hash
	let message_hash = message.to_message_hash()?;

	// 2. Call the proxy_bls signer
	let response = {
		let mut commit_config = commit_config.lock().await;
		signer::call_proxy_bls_signer(&mut *commit_config, message_hash, bls_public_key).await?
	};
	debug!("Received response from proxy_bls: {:?}", response);

	let signed_constraints = SignedConstraints {
		message: message.clone(),
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	debug!("Signed constraints created successfully");
	Ok(signed_constraints)
}

/// Validates a constraints message
/// In this PoC we assume that constraints must be submitted before the target slot
pub fn validate_constraints_message(message: &ConstraintsMessage, slot_timer: &SlotTimer) -> Result<()> {
	// Check that the constraints slot has not already elapsed
	if message.slot <= slot_timer.get_current_slot() {
		error!("Constraints slot {} has already elapsed", message.slot);
		return Err(eyre::eyre!("Constraints slot has already elapsed"));
	}

	Ok(())
}

/// Creates a ConstraintsMessage from pending constraints
pub fn create_constraints_message(
	pending_constraints: Vec<(B256, Constraint)>,
	proposer: BlsPublicKey,
	delegate: BlsPublicKey,
	slot: u64,
	receivers: Vec<BlsPublicKey>,
) -> Result<ConstraintsMessage> {
	info!("Creating constraints message from {} pending constraints", pending_constraints.len());

	if pending_constraints.is_empty() {
		return Err(eyre::eyre!("No pending constraints to process"));
	}

	let constraints: Vec<Constraint> = pending_constraints.into_iter().map(|(_, constraint)| constraint).collect();

	let message = ConstraintsMessage { proposer, delegate, slot, constraints, receivers };

	info!("Created constraints message with {} constraints", message.constraints.len());
	Ok(message)
}

/// Parse a BLS public key from hex string with error handling
pub fn parse_bls_public_key(hex_string: &str, field_name: &str) -> Result<BlsPublicKey> {
	cb_common::utils::bls_pubkey_from_hex(hex_string)
		.map_err(|e| eyre::eyre!("Invalid {} BLS public key: {}", field_name, e))
}

/// Parse multiple BLS public keys from hex strings with error handling
pub fn parse_bls_public_keys(hex_strings: &[String], field_name: &str) -> Result<Vec<BlsPublicKey>> {
	let mut keys = Vec::new();
	for (index, hex_string) in hex_strings.iter().enumerate() {
		let key = cb_common::utils::bls_pubkey_from_hex(hex_string)
			.map_err(|e| eyre::eyre!("Invalid {} BLS public key at index {}: {}", field_name, index, e))?;
		keys.push(key);
	}
	Ok(keys)
}

#[cfg(test)]
mod tests {
	use super::*;
	use commit_boost::prelude::BlsPublicKey;

	#[test]
	fn test_validate_constraints_message_slot_elapsed() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = SlotTimer::new(1742213400);

		// Get current slot and try to create constraints for a slot that has already elapsed
		let current_slot = slot_timer.get_current_slot();

		let constraints_message = ConstraintsMessage {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			slot: current_slot - 1, // Slot in the past
			constraints: vec![],
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message, &slot_timer);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("already elapsed"));
	}

	#[test]
	fn test_validate_constraints_message_current_slot() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = SlotTimer::new(1742213400);

		// Get current slot and try to create constraints for the current slot
		let current_slot = slot_timer.get_current_slot();

		let constraints_message = ConstraintsMessage {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			slot: current_slot, // Current slot
			constraints: vec![],
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message, &slot_timer);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("already elapsed"));
	}

	#[test]
	fn test_validate_constraints_message_future_slot() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = SlotTimer::new(1742213400);

		// Get current slot and try to create constraints for a future slot
		let current_slot = slot_timer.get_current_slot();

		let constraints_message = ConstraintsMessage {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			slot: current_slot + 10, // Future slot
			constraints: vec![],
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message, &slot_timer);
		assert!(result.is_ok());
	}
}
