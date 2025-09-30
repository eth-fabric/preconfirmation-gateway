use alloy::{
	primitives::{Address, B256, Bytes, Signature},
	sol_types::SolValue,
};
use commit_boost::prelude::*;
use eyre::{Result, WrapErr};
use tracing::{debug, info, warn};

use super::super::types::rpc::InclusionPayload;
use super::super::types::{Commitment, CommitmentRequest, FeeInfo, SignedCommitment};

/// Helper functions for RPC business logic
/// This module contains utility functions that can be shared across multiple RPC handlers

/// Validates a commitment request
pub fn validate_commitment_request(request: &CommitmentRequest) -> Result<()> {
	if request.commitment_type == 0 {
		return Err(eyre::eyre!("Invalid commitment type: 0"));
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
		}
		Err(e) => {
			return Err(eyre::eyre!("Invalid payload format: {}", e));
		}
	}

	debug!("Commitment request validation passed");
	Ok(())
}

/// Creates a mock commitment for testing/development
pub fn create_mock_commitment(request: &CommitmentRequest) -> Commitment {
	debug!("Creating mock commitment for type: {}", request.commitment_type);

	Commitment {
		commitment_type: request.commitment_type,
		payload: request.payload.clone(),
		request_hash: B256::ZERO, // TODO: Calculate actual hash
		slasher: request.slasher,
	}
}

/// Creates a mock signature for testing/development
pub fn create_mock_signature() -> Signature {
	debug!("Creating mock signature");
	"0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        .parse()
        .unwrap()
}

/// Creates a signed commitment with mock data
pub fn create_mock_signed_commitment(request: &CommitmentRequest) -> SignedCommitment {
	let commitment = create_mock_commitment(request);
	let signature = create_mock_signature();

	SignedCommitment { commitment, signature }
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
pub async fn create_signed_commitment(request: &CommitmentRequest, private_key: &str) -> Result<SignedCommitment> {
	debug!("Creating signed commitment with commit-boost");

	// Create the commitment
	let commitment = create_mock_commitment(request);

	// Sign the commitment using commit-boost
	let signature = sign_commitment(&commitment, private_key).await?;

	Ok(SignedCommitment { commitment, signature })
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
