use alloy::primitives::{Address, B256, Bytes, Signature, keccak256};
use commit_boost::prelude::StartCommitModuleConfig;
use eyre::{Result, WrapErr};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use crate::constants::COMMITMENT_TYPE;
use crate::signing;
use crate::types::commitments::InclusionPayload;
use crate::types::{Commitment, CommitmentRequest, FeeInfo, SignedCommitment};

/// Helper functions for RPC business logic
/// This module contains utility functions that can be shared across multiple RPC handlers

/// Validates a commitment request
pub fn validate_commitment_request(request: &CommitmentRequest) -> Result<()> {
	if request.commitment_type != COMMITMENT_TYPE {
		return Err(eyre::eyre!(
			"Invalid commitment type: expected {}, got {}",
			COMMITMENT_TYPE,
			request.commitment_type
		));
	}

	if request.payload.is_empty() {
		return Err(eyre::eyre!("Payload cannot be empty"));
	}

	if request.slasher == Address::ZERO {
		return Err(eyre::eyre!("Invalid slasher address"));
	}

	// Validate that the payload is a valid InclusionPayload
	match InclusionPayload::abi_decode(&request.payload) {
		Ok(inclusion_payload) => {
			debug!(
				"Decoded InclusionPayload: slot={}, signed_tx_len={}",
				inclusion_payload.slot,
				inclusion_payload.signed_tx.len()
			);

			// Additional validation for InclusionPayload
			if inclusion_payload.slot == 0 {
				return Err(eyre::eyre!("Invalid slot: 0"));
			}

			if inclusion_payload.signed_tx.is_empty() {
				return Err(eyre::eyre!("Signed transaction cannot be empty"));
			}

			// TODO: Validate signed_tx format and signature
			// Should verify:
			// - Transaction is properly formatted
			// - Signature is valid
			// - Transaction is signed by expected signer
			// - Transaction is for a valid slot
			// - Enough gas for transaction to be executed successfully
		}
		Err(e) => {
			return Err(eyre::eyre!("Invalid payload format: {}", e));
		}
	}

	debug!("Commitment request validation passed");
	Ok(())
}

/// Calculates fee information for a commitment request
pub fn calculate_fee_info(request: &CommitmentRequest) -> FeeInfo {
	debug!("Calculating fee for commitment type: {}", request.commitment_type);

	// TODO: Implement actual fee calculation logic
	// This is a placeholder implementation
	FeeInfo {
		fee_payload: Bytes::new(), // TODO: Calculate actual fee payload
		commitment_type: request.commitment_type,
	}
}

/// Validates a request hash format
pub fn validate_request_hash(hash: &str) -> Result<B256> {
	if hash.len() != 66 || !hash.starts_with("0x") {
		return Err(eyre::eyre!("Invalid hash format. Expected 0x followed by 64 hex characters"));
	}

	hash.parse::<B256>().wrap_err("Failed to parse hash")
}

/// Logs commitment processing information
pub fn log_commitment_processing(commitment_type: u64, slasher: &Address) {
	info!("Processing commitment - Type: {}, Slasher: {}", commitment_type, slasher);
}

/// Helper to convert string to Address with validation
pub fn parse_address(address_str: &str) -> Result<Address> {
	if address_str.len() != 42 || !address_str.starts_with("0x") {
		return Err(eyre::eyre!("Invalid address format"));
	}

	address_str.parse::<Address>().wrap_err("Failed to parse address")
}

/// Helper to create empty bytes of specified length
pub fn create_empty_bytes(length: usize) -> Bytes {
	Bytes::from(vec![0u8; length])
}

/// Validates commitment type against allowed types
pub fn is_valid_commitment_type(commitment_type: u64) -> bool {
	// TODO: Define valid commitment types based on your business logic
	const VALID_TYPES: &[u64] = &[1, 2, 3, 4, 5]; // Example valid types
	VALID_TYPES.contains(&commitment_type)
}

/// Helper to format error messages consistently
pub fn format_error(context: &str, error: &str) -> String {
	format!("{}: {}", context, error)
}

/// Signs a commitment using ECDSA with the provided private key
pub async fn sign_commitment(_commitment: &Commitment, _private_key: &str) -> Result<Signature> {
	debug!("Signing commitment with ECDSA using commit-boost");

	// TODO: Implement proper commit-boost integration
	// The commit-boost client requires a signer service setup
	// For now, using mock signature until we set up the signer service

	warn!("Using mock signature - commit-boost signer service integration pending");

	// Create a mock signature for now
	let signature = "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        .parse()
        .wrap_err("Failed to parse mock signature")?;

	debug!("Commitment signed successfully (mock)");
	Ok(signature)
}

/// Creates a properly signed commitment using ECDSA
pub async fn create_signed_commitment<T>(
	request: &CommitmentRequest,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	committer_address: Address,
) -> Result<SignedCommitment> {
	debug!("Creating signed commitment with proper ECDSA signing");

	// 1. ABI-encode the CommitmentRequest
	let encoded_request = request.abi_encode()?;
	debug!("ABI-encoded CommitmentRequest: {:?}", encoded_request);

	// 2. Keccak256 hash the encoded request
	let request_hash = keccak256(&encoded_request);
	debug!("Keccak256 hash: {:?}", request_hash);

	// 3. Create the commitment
	let commitment = Commitment {
		commitment_type: request.commitment_type,
		payload: request.payload.clone(),
		request_hash: request_hash,
		slasher: request.slasher,
	};

	// 4. ABI-encode the commitment
	let encoded_commitment = commitment.abi_encode()?;
	debug!("ABI-encoded Commitment: {:?}", encoded_commitment);

	// 5. Keccak256 hash the encoded commitment
	let commitment_hash = keccak256(&encoded_commitment);
	debug!("Keccak256 hash: {:?}", commitment_hash);

	// 6. Call the proxy_ecdsa signer
	let response = {
		let mut commit_config = commit_config.lock().await;
		signing::call_proxy_ecdsa_signer(&mut *commit_config, commitment_hash, committer_address).await?
	};
	debug!("Received response from proxy_ecdsa: {:?}", response);

	// 7. Construct the SignedCommitment
	let signed_commitment = SignedCommitment {
		commitment,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature.normalized_s(),
	};

	debug!("Signed commitment created successfully");
	Ok(signed_commitment)
}

/// Calculate the request hash for a CommitmentRequest
pub fn calculate_request_hash(request: &CommitmentRequest) -> Result<B256> {
	let encoded_request = request.abi_encode()?;
	Ok(keccak256(&encoded_request))
}

/// Validates a signature against a commitment
pub fn verify_commitment_signature(
	commitment: &Commitment,
	signature: &Signature,
	expected_signer: &Address,
) -> Result<bool> {
	debug!("Verifying commitment signature");

	// ABI encode the commitment
	let encoded_data = commitment.abi_encode()?;

	// Recover the signer from the signature
	let message_hash = alloy::primitives::keccak256(&encoded_data);
	let recovered_address =
		signature.recover_address_from_prehash(&message_hash).wrap_err("Failed to recover address from signature")?;

	Ok(recovered_address == *expected_signer)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_validate_commitment_request() -> Result<()> {
		// Test valid request with proper InclusionPayload
		let inclusion_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01, 0x02]) };
		let encoded_payload = inclusion_payload.abi_encode()?;

		let valid_request = CommitmentRequest {
			commitment_type: 1,
			payload: encoded_payload,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		assert!(validate_commitment_request(&valid_request).is_ok());

		// Test invalid commitment type
		let invalid_type_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01]) };
		let invalid_type_encoded = invalid_type_payload.abi_encode()?;
		let invalid_type_request = CommitmentRequest {
			commitment_type: 0,
			payload: invalid_type_encoded,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		assert!(validate_commitment_request(&invalid_type_request).is_err());

		// Test empty payload
		let empty_payload_request = CommitmentRequest {
			commitment_type: 1,
			payload: Bytes::new(),
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		assert!(validate_commitment_request(&empty_payload_request).is_err());

		// Test zero address
		let zero_address_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01]) };
		let zero_address_encoded = zero_address_payload.abi_encode()?;
		let zero_address_request =
			CommitmentRequest { commitment_type: 1, payload: zero_address_encoded, slasher: Address::ZERO };

		assert!(validate_commitment_request(&zero_address_request).is_err());

		Ok(())
	}

	#[test]
	fn test_validate_commitment_request_with_inclusion_payload() -> Result<()> {
		// Create a valid InclusionPayload
		let inclusion_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]) };

		// Encode it
		let encoded_payload = inclusion_payload.abi_encode()?;

		// Create a valid commitment request
		let valid_request = CommitmentRequest {
			commitment_type: 1,
			payload: encoded_payload,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		// Should pass validation
		assert!(validate_commitment_request(&valid_request).is_ok());

		// Test invalid slot (0)
		let invalid_slot_payload = InclusionPayload {
			slot: 0, // Invalid
			signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]),
		};
		let invalid_slot_encoded = invalid_slot_payload.abi_encode()?;
		let invalid_slot_request = CommitmentRequest {
			commitment_type: 1,
			payload: invalid_slot_encoded,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};
		assert!(validate_commitment_request(&invalid_slot_request).is_err());

		// Test empty signed_tx
		let empty_tx_payload = InclusionPayload {
			slot: 100,
			signed_tx: Bytes::new(), // Invalid
		};
		let empty_tx_encoded = empty_tx_payload.abi_encode()?;
		let empty_tx_request = CommitmentRequest {
			commitment_type: 1,
			payload: empty_tx_encoded,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};
		assert!(validate_commitment_request(&empty_tx_request).is_err());

		println!("InclusionPayload validation tests passed");
		Ok(())
	}

	#[test]
	fn test_validate_commitment_request_invalid_payload_format() -> Result<()> {
		// Create a request with invalid payload (not ABI-encoded InclusionPayload)
		let invalid_request = CommitmentRequest {
			commitment_type: 1,
			payload: Bytes::from(vec![0x01, 0x02, 0x03]), // Not ABI-encoded
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		// Should fail validation due to invalid payload format
		assert!(validate_commitment_request(&invalid_request).is_err());

		println!("Invalid payload format test passed");
		Ok(())
	}

	#[tokio::test]
	async fn test_calculate_fee_info() -> Result<()> {
		let request = CommitmentRequest {
			commitment_type: 3,
			payload: Bytes::from(vec![0x07, 0x08, 0x09]),
			slasher: "0x9876543210987654321098765432109876543210".parse()?,
		};

		let fee_info = calculate_fee_info(&request);

		assert_eq!(fee_info.commitment_type, request.commitment_type);
		assert_eq!(fee_info.fee_payload, Bytes::new());

		println!("Fee info: {:?}", fee_info);
		Ok(())
	}
}
