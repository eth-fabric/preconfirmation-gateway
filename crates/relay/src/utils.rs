use alloy::primitives::Address;
use commit_boost::prelude::{verify_proposer_commitment_signature_bls_for_message, BlsPublicKey, Chain};
use common::types::{Delegation, SignedConstraints, SignedDelegation};
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

/// Validate that the given public key is the scheduled proposer for the given slot
/// TODO: Call beacon node API to verify this pubkey is the scheduled proposer for this slot
pub fn validate_is_proposer(pubkey: &BlsPublicKey, slot: u64) -> Result<bool> {
	debug!("Validating proposer for slot {} with pubkey: {:?}", slot, pubkey.serialize());

	// TODO: Call beacon node API to verify this pubkey is the scheduled proposer for this slot
	// For now, always return true to allow all valid signatures through
	// This should be replaced with actual beacon node validation
	info!("Proposer validation for slot {} - currently allowing all (TODO: implement beacon node check)", slot);

	Ok(true)
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
}
