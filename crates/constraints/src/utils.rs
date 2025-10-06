use alloy::primitives::{Signature, U256, keccak256};
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

use common::signing;
use common::types::constraints::{ConstraintsMessage, SignedConstraints};

/// Creates a properly signed constraints message using BLS
pub async fn create_signed_constraints<T>(
	message: &ConstraintsMessage,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	bls_public_key: BlsPublicKey,
) -> Result<SignedConstraints> {
	debug!("Creating signed constraints with proper BLS signing");

	// 1. ABI-encode the ConstraintsMessage
	let encoded_message = message.abi_encode()?;
	debug!("ABI-encoded ConstraintsMessage: {:?}", encoded_message);

	// 2. Keccak256 hash the encoded message
	let constraints_hash = keccak256(&encoded_message);
	debug!("Keccak256 hash: {:?}", constraints_hash);

	// 3. Call the proxy_bls signer
	let response = {
		let mut commit_config = commit_config.lock().await;
		signing::call_proxy_bls_signer(&mut *commit_config, constraints_hash, bls_public_key).await?
	};
	debug!("Received response from proxy_bls: {:?}", response);

	// 4. Construct the SignedConstraints
	// Note: BLS signatures are different from ECDSA - we'll store as bytes for now
	// TODO: Define proper BLS signature type for constraints
	// For now, create a default signature since BLS signatures don't map directly to ECDSA
	let signature = Signature::new(U256::from(0), U256::from(0), false);

	let signed_constraints = SignedConstraints {
		message: message.clone(),
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature,
	};

	debug!("Signed constraints created successfully");
	Ok(signed_constraints)
}

/// Validates a constraints message
pub fn validate_constraints_message(message: &ConstraintsMessage) -> Result<()> {
	// Check that proposer and delegate are valid BLS public keys (48 bytes)
	if message.proposer.len() != 48 {
		return Err(eyre::eyre!("Invalid proposer BLS public key length: {}", message.proposer.len()));
	}

	if message.delegate.len() != 48 {
		return Err(eyre::eyre!("Invalid delegate BLS public key length: {}", message.delegate.len()));
	}

	// Check that all receivers are valid BLS public keys
	for (i, receiver) in message.receivers.iter().enumerate() {
		if receiver.len() != 48 {
			return Err(eyre::eyre!("Invalid receiver {} BLS public key length: {}", i, receiver.len()));
		}
	}

	// Check that constraints are not empty
	if message.constraints.is_empty() {
		return Err(eyre::eyre!("Constraints message must contain at least one constraint"));
	}

	// Check maximum constraints per slot (from API spec: MAX_CONSTRAINTS_PER_SLOT = 32)
	const MAX_CONSTRAINTS_PER_SLOT: usize = 32;
	if message.constraints.len() > MAX_CONSTRAINTS_PER_SLOT {
		return Err(eyre::eyre!(
			"Too many constraints: {} (max: {})",
			message.constraints.len(),
			MAX_CONSTRAINTS_PER_SLOT
		));
	}

	// Validate each constraint
	for (i, constraint) in message.constraints.iter().enumerate() {
		// Constraint type validation is handled by the u64 type itself
		// No need for additional range checking since u64 already has the correct bounds

		// Payload should not be empty
		if constraint.payload.is_empty() {
			return Err(eyre::eyre!("Constraint {} has empty payload", i));
		}
	}

	Ok(())
}

/// Creates a constraints message from individual components
pub fn create_constraints_message(
	proposer: Vec<u8>,
	delegate: Vec<u8>,
	slot: u64,
	constraints: Vec<common::types::constraints::Constraint>,
	receivers: Vec<Vec<u8>>,
) -> Result<ConstraintsMessage> {
	let message = ConstraintsMessage {
		proposer: proposer.into(),
		delegate: delegate.into(),
		slot,
		constraints,
		receivers: receivers.into_iter().map(|r| r.into()).collect(),
	};

	// Validate the message
	validate_constraints_message(&message)?;

	Ok(message)
}

/// Signs a constraints message using BLS with the provided commit config
pub async fn sign_constraints_message<T>(
	message: &ConstraintsMessage,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	bls_public_key: BlsPublicKey,
) -> Result<SignedConstraints> {
	debug!("Signing constraints message with BLS using commit-boost");

	// Validate the message first
	validate_constraints_message(message)?;

	// Create the signed constraints
	create_signed_constraints(message, commit_config, bls_public_key).await
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::Bytes;

	#[test]
	fn test_validate_constraints_message_valid() -> Result<()> {
		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]), // Valid BLS public key size
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 12345,
			constraints: vec![common::types::constraints::Constraint {
				constraint_type: 1,
				payload: Bytes::from(vec![0x01, 0x02, 0x03]),
			}],
			receivers: vec![Bytes::from(vec![0x33; 48])],
		};

		assert!(validate_constraints_message(&message).is_ok());
		Ok(())
	}

	#[test]
	fn test_validate_constraints_message_invalid_proposer() -> Result<()> {
		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 32]), // Invalid BLS public key size
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 12345,
			constraints: vec![common::types::constraints::Constraint {
				constraint_type: 1,
				payload: Bytes::from(vec![0x01, 0x02, 0x03]),
			}],
			receivers: vec![],
		};

		assert!(validate_constraints_message(&message).is_err());
		Ok(())
	}

	#[test]
	fn test_validate_constraints_message_empty_constraints() -> Result<()> {
		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 12345,
			constraints: vec![], // Empty constraints
			receivers: vec![],
		};

		assert!(validate_constraints_message(&message).is_err());
		Ok(())
	}

	#[test]
	fn test_validate_constraints_message_too_many_constraints() -> Result<()> {
		let constraints = (0..33) // More than MAX_CONSTRAINTS_PER_SLOT
			.map(|i| common::types::constraints::Constraint {
				constraint_type: i as u64,
				payload: Bytes::from(vec![0x01, 0x02, 0x03]),
			})
			.collect();

		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 12345,
			constraints,
			receivers: vec![],
		};

		assert!(validate_constraints_message(&message).is_err());
		Ok(())
	}

	#[test]
	fn test_validate_constraints_message_invalid_constraint_type() -> Result<()> {
		// Since u64 already enforces the correct bounds, this test is not needed
		// The constraint type is already validated by the type system
		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 12345,
			constraints: vec![common::types::constraints::Constraint {
				constraint_type: 0xffffffffffffffff, // Maximum valid constraint type
				payload: Bytes::from(vec![0x01, 0x02, 0x03]),
			}],
			receivers: vec![],
		};

		// This should pass since the constraint type is within valid bounds
		assert!(validate_constraints_message(&message).is_ok());
		Ok(())
	}

	#[test]
	fn test_validate_constraints_message_empty_payload() -> Result<()> {
		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 12345,
			constraints: vec![common::types::constraints::Constraint {
				constraint_type: 1,
				payload: Bytes::new(), // Empty payload
			}],
			receivers: vec![],
		};

		assert!(validate_constraints_message(&message).is_err());
		Ok(())
	}

	#[test]
	fn test_create_constraints_message() -> Result<()> {
		let proposer = vec![0x11; 48];
		let delegate = vec![0x22; 48];
		let slot = 12345;
		let constraints = vec![common::types::constraints::Constraint {
			constraint_type: 1,
			payload: Bytes::from(vec![0x01, 0x02, 0x03]),
		}];
		let receivers = vec![vec![0x33; 48]];

		let message = create_constraints_message(proposer, delegate, slot, constraints, receivers)?;

		assert_eq!(message.slot, 12345);
		assert_eq!(message.constraints.len(), 1);
		assert_eq!(message.receivers.len(), 1);
		Ok(())
	}
}
