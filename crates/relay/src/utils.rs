use alloy::primitives::Address;
use commit_boost::prelude::{verify_proposer_commitment_signature_bls_for_message, BlsPublicKey, Chain};
use common::slot_timer::SlotTimer;
use common::types::{ConstraintsMessage, Delegation, SignedConstraints, SignedDelegation};
use eyre::Result;
use tracing::{debug, error, info};

/// Verify BLS signature on a SignedConstraints message using the delegate public key from the message
pub fn verify_constraints_signature(signed_constraints: &SignedConstraints, chain: Chain) -> Result<bool> {
	debug!("Verifying constraints signature");

	// Get the message hash for signature verification
	let message_hash = signed_constraints.message.to_message_hash()?;
	debug!("DEBUG: Message hash: {:?}", message_hash);

	// Use the delegate public key from the message for verification
	let delegate_public_key = &signed_constraints.message.delegate;
	debug!("DEBUG: Delegate public key: {:?}", hex::encode(delegate_public_key.serialize()));

	// Use the existing commit-boost BLS verification function
	let is_valid = verify_proposer_commitment_signature_bls_for_message(
		chain,
		delegate_public_key,
		&message_hash,
		&signed_constraints.signature,
		&signed_constraints.signing_id,
		signed_constraints.nonce,
	);

	debug!("DEBUG: Signature verification result: {}", is_valid);

	if is_valid {
		info!("Constraints signature verification successful");
	} else {
		error!("Constraints signature verification failed");
	}

	Ok(is_valid)
}

/// Verify BLS signature on a SignedDelegation message using the proposer public key from the message
pub fn verify_delegation_signature(signed_delegation: &SignedDelegation, chain: Chain) -> Result<bool> {
	debug!("Verifying delegation signature");

	// Get the message hash for signature verification
	let message_hash = signed_delegation.message.to_message_hash()?;

	// Use the proposer public key from the message for verification
	let proposer_public_key = &signed_delegation.message.proposer;

	// Use the existing commit-boost BLS verification function
	let is_valid = verify_proposer_commitment_signature_bls_for_message(
		chain,
		proposer_public_key,
		&message_hash,
		&signed_delegation.signature,
		&signed_delegation.signing_id,
		signed_delegation.nonce,
	);

	if is_valid {
		info!("Delegation signature verification successful");
	} else {
		error!("Delegation signature verification failed");
	}

	Ok(is_valid)
}

/// Validate delegation message structure
pub fn validate_delegation_message(delegation: &Delegation, slot_timer: &common::slot_timer::SlotTimer) -> Result<()> {
	// Check that committer address is not zero
	if delegation.committer == Address::ZERO {
		error!("Invalid committer address");
		return Err(eyre::eyre!("Invalid committer address"));
	}

	// Check that the delegation slot has not already elapsed
	if delegation.slot <= slot_timer.get_current_slot() {
		error!("Delegation slot {} has already elapsed", delegation.slot);
		return Err(eyre::eyre!("Delegation slot has already elapsed"));
	}

	Ok(())
}

/// Validate a constraints message
/// Checks that the constraints slot has not already elapsed
pub fn validate_constraints_message(message: &ConstraintsMessage, slot_timer: &SlotTimer) -> Result<()> {
	// Check that the constraints slot has not already elapsed
	if message.slot <= slot_timer.get_current_slot() {
		error!("Constraints slot {} has already elapsed", message.slot);
		return Err(eyre::eyre!("Constraints slot has already elapsed"));
	}

	Ok(())
}

/// Validate that the given public key is the scheduled proposer for the given slot
/// Reads from the proposer lookahead stored in the database
pub fn validate_is_proposer(
	pubkey: &BlsPublicKey,
	slot: u64,
	database: &common::types::database::DatabaseContext,
) -> Result<bool> {
	debug!("Validating proposer for slot {} with pubkey: {:?}", slot, pubkey.serialize());

	// Look up the expected proposer from the lookahead database
	match database.get_proposer_lookahead(slot)? {
		Some(expected_proposer) => {
			// Compare the provided pubkey with the expected proposer
			if pubkey == &expected_proposer {
				info!("Proposer validation successful for slot {}", slot);
				Ok(true)
			} else {
				info!("Proposer validation failed for slot {}: provided pubkey does not match expected proposer", slot);
				Ok(false)
			}
		}
		None => {
			info!("No proposer lookahead found for slot {}, rejecting validation", slot);
			Ok(false)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::Bytes;
	use commit_boost::prelude::BlsPublicKey;
	use hex;

	#[test]
	fn test_validate_delegation_message_zero_committer() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = common::slot_timer::SlotTimer::new(1742213400);

		let delegation = Delegation {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			committer: Address::ZERO,
			slot: 12345,
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		assert!(validate_delegation_message(&delegation, &slot_timer).is_err());
	}

	#[test]
	fn test_validate_delegation_message_slot_elapsed() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = common::slot_timer::SlotTimer::new(1742213400);

		// Get current slot and try to delegate a slot that has already elapsed
		let current_slot = slot_timer.get_current_slot();

		let delegation = Delegation {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			committer: "0x1234567890123456789012345678901234567890".parse().unwrap(),
			slot: current_slot - 1, // Slot in the past
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		let result = validate_delegation_message(&delegation, &slot_timer);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("already elapsed"));
	}

	#[test]
	fn test_validate_delegation_message_future_slot() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = common::slot_timer::SlotTimer::new(1742213400);

		// Get current slot and try to delegate a future slot
		let current_slot = slot_timer.get_current_slot();

		let delegation = Delegation {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			committer: "0x1234567890123456789012345678901234567890".parse().unwrap(),
			slot: current_slot + 10, // Future slot
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		let result = validate_delegation_message(&delegation, &slot_timer);
		assert!(result.is_ok());
	}

	#[test]
	fn test_validate_constraints_message_slot_elapsed() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		// Create slot timer with a genesis timestamp
		let slot_timer = common::slot_timer::SlotTimer::new(1742213400);

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
		let slot_timer = common::slot_timer::SlotTimer::new(1742213400);

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
		let slot_timer = common::slot_timer::SlotTimer::new(1742213400);

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
