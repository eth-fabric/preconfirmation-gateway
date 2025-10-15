use commit_boost::prelude::{verify_proposer_commitment_signature_bls_for_message, BlsPublicKey, Chain};
use common::types::{ConstraintsMessage, Delegation, SignedConstraints, SignedDelegation};
use eyre::Result;
use tracing::{debug, error, info};

/// Verify BLS signature on a SignedConstraints message using the delegate public key from the message
pub fn verify_constraints_signature(signed_constraints: &SignedConstraints, chain: Chain) -> Result<bool> {
	debug!("Verifying constraints signature");

	// Get the object root for signature verification
	let object_root = signed_constraints.message.to_object_root()?;
	debug!("DEBUG: Object root: {:?}", object_root);

	// Use the delegate public key from the message for verification
	let delegate_public_key = &signed_constraints.message.delegate;
	debug!("DEBUG: Delegate public key: {:?}", hex::encode(delegate_public_key.serialize()));

	// Use the existing commit-boost BLS verification function
	let is_valid = verify_proposer_commitment_signature_bls_for_message(
		chain,
		delegate_public_key,
		&object_root,
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

	// Get the object root for signature verification
	let _object_root = signed_delegation.message.to_object_root()?;

	// Use the proposer public key from the message for verification
	let proposer_public_key = &signed_delegation.message.proposer;

	// Use the existing commit-boost BLS verification function
	let is_valid = verify_proposer_commitment_signature_bls_for_message(
		chain,
		proposer_public_key,
		&signed_delegation.message.to_object_root()?,
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

/// Validate constraints message structure
pub fn validate_constraints_message(constraints_message: &ConstraintsMessage) -> Result<bool> {
	// Check that constraints list is not empty
	if constraints_message.constraints.is_empty() {
		return Ok(false);
	}

	// Check that all constraints have valid structure
	for constraint in &constraints_message.constraints {
		if constraint.payload.is_empty() {
			return Ok(false);
		}
	}

	// TODO: Add more validation logic

	Ok(true)
}

/// Validate delegation message structure
pub fn validate_delegation_message(delegation: &Delegation) -> Result<bool> {
	// Check that committer address is not zero
	if delegation.committer
		== "0x0000000000000000000000000000000000000000".parse::<alloy::primitives::Address>().unwrap()
	{
		return Ok(false);
	}

	// Check that BLS public keys are not zero
	let zero_key = BlsPublicKey::deserialize(&[0u8; 48]).unwrap_or_else(|_| {
		// Create a zero key for comparison
		BlsPublicKey::deserialize(&[0u8; 48]).unwrap()
	});
	if delegation.proposer == zero_key || delegation.delegate == zero_key {
		return Ok(false);
	}

	Ok(true)
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
	fn test_validate_constraints_message_empty() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let constraints_message = ConstraintsMessage {
			constraints: vec![],
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			slot: 12345,
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message);
		assert_eq!(result.unwrap(), false);
	}

	#[test]
	fn test_validate_delegation_message_zero_committer() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let delegation = Delegation {
			proposer: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			delegate: BlsPublicKey::deserialize(&valid_bls_key).unwrap(),
			committer: "0x0000000000000000000000000000000000000000".parse::<alloy::primitives::Address>().unwrap(),
			slot: 12345,
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		let result = validate_delegation_message(&delegation);
		assert_eq!(result.unwrap(), false);
	}
}
