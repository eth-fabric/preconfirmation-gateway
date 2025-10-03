use alloy::primitives::{Address, Bytes};
use jsonrpsee::Extensions;
use jsonrpsee::types::Params;
use preconfirmation_gateway::rpc::handlers::commitment_request_handler;
use preconfirmation_gateway::types::CommitmentRequest;
use preconfirmation_gateway::types::commitments::InclusionPayload;
use std::sync::Arc;

mod common;
use common::{PUBKEY, create_test_context};

/// Test harness for commitment request testing
/// This provides a clean interface for testing different commitment request scenarios
struct CommitmentRequestTestHarness {
	context: preconfirmation_gateway::RpcContext,
}

impl CommitmentRequestTestHarness {
	/// Creates a new test harness with a properly configured context
	async fn new() -> eyre::Result<Self> {
		let mut context = create_test_context().await?;

		// Generate a proxy key for the committer using the signer from the context
		let test_bls_pubkey = cb_common::types::BlsPublicKey::deserialize(&PUBKEY).unwrap();
		let mut commit_config = context.commit_config.lock().await;
		let proxy_address = commit_config
			.signer_client
			.generate_proxy_key_ecdsa(test_bls_pubkey)
			.await
			.map_err(|e| eyre::eyre!("Failed to generate ECDSA proxy key: {}", e))?;
		let committer_address = proxy_address.message.proxy;
		drop(commit_config); // Release the lock

		// Update the context with the committer address
		context.committer_address = committer_address;

		Ok(Self { context })
	}

	/// Tests a commitment request and returns the result
	async fn test_commitment_request(
		&self,
		request: CommitmentRequest,
	) -> eyre::Result<preconfirmation_gateway::types::SignedCommitment> {
		// Serialize the request to JSON for the params
		let request_json = serde_json::to_string(&request)?;
		let params = Params::new(Some(&request_json));

		// Call the handler
		let result =
			commitment_request_handler::<()>(params, Arc::new(self.context.clone()), Extensions::default()).await;

		match result {
			Ok(signed_commitment) => Ok(signed_commitment),
			Err(e) => Err(eyre::eyre!("Handler failed: {}", e)),
		}
	}

	/// Tests a commitment request and expects it to fail with a specific error
	async fn test_commitment_request_should_fail(&self, request: CommitmentRequest) -> eyre::Result<()> {
		// Serialize the request to JSON for the params
		let request_json = serde_json::to_string(&request)?;
		let params = Params::new(Some(&request_json));

		// Call the handler
		let result =
			commitment_request_handler::<()>(params, Arc::new(self.context.clone()), Extensions::default()).await;

		match result {
			Ok(_) => Err(eyre::eyre!("Expected handler to fail but it succeeded")),
			Err(_) => Ok(()), // Expected failure
		}
	}
}

/// Helper functions for creating test data
mod test_helpers {
	use preconfirmation_gateway::constants::COMMITMENT_TYPE;

	use super::*;

	/// Creates a valid inclusion payload
	pub fn create_valid_inclusion_payload(slot: u64, signed_tx: Vec<u8>) -> eyre::Result<Bytes> {
		let inclusion_payload = InclusionPayload { slot, signed_tx: signed_tx.into() };
		inclusion_payload.abi_encode()
	}

	/// Creates a valid commitment request
	pub fn create_valid_commitment_request(
		commitment_type: u64,
		payload: Bytes,
		slasher: Address,
	) -> CommitmentRequest {
		CommitmentRequest { commitment_type, payload, slasher }
	}

	/// Creates a valid slasher address
	pub fn create_valid_slasher() -> Address {
		Address::from([0x2; 20])
	}

	pub fn create_valid_commitment_type() -> u64 {
		COMMITMENT_TYPE
	}

	/// Creates an invalid slasher address (zero address)
	pub fn create_invalid_slasher() -> Address {
		Address::ZERO
	}

	/// Creates an invalid commitment type
	pub fn create_invalid_commitment_type() -> u64 {
		COMMITMENT_TYPE + 1 // Invalid commitment type
	}

	/// Creates an empty payload
	pub fn create_empty_payload() -> Bytes {
		Bytes::new()
	}

	/// Creates an invalid payload (not ABI-encoded InclusionPayload)
	pub fn create_invalid_payload() -> Bytes {
		Bytes::from(vec![0x01, 0x02, 0x03]) // Not ABI-encoded
	}
}

/// Test cases for different scenarios
mod test_cases {
	use super::*;

	#[tokio::test]
	async fn test_valid_commitment_request() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a valid test request
		let payload = test_helpers::create_valid_inclusion_payload(12345, vec![0x01, 0x02, 0x03, 0x04])?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		// Test the request
		let result = harness.test_commitment_request(request).await?;

		// Verify the result
		println!("Successfully processed commitment request");
		println!("   Commitment type: {}", result.commitment.commitment_type);
		println!("   Nonce: {}", result.nonce);
		println!("   Signature: {}", result.signature);

		// Basic assertions
		assert_eq!(result.commitment.commitment_type, commitment_type);
		assert!(result.nonce > 0, "Nonce should be greater than 0");
		assert!(!result.signature.to_string().is_empty(), "Signature should not be empty");

		Ok(())
	}

	#[tokio::test]
	async fn test_invalid_commitment_type() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a request with invalid commitment type
		let payload = test_helpers::create_valid_inclusion_payload(12345, vec![0x01, 0x02, 0x03, 0x04])?;
		let slasher = test_helpers::create_valid_slasher();
		let request = test_helpers::create_valid_commitment_request(
			test_helpers::create_invalid_commitment_type(),
			payload,
			slasher,
		);

		// Test that the request fails
		harness.test_commitment_request_should_fail(request).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_empty_payload() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a request with empty payload
		let request = test_helpers::create_valid_commitment_request(
			test_helpers::create_valid_commitment_type(),
			test_helpers::create_empty_payload(),
			test_helpers::create_valid_slasher(),
		);

		// Test that the request fails
		harness.test_commitment_request_should_fail(request).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_zero_address_slasher() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a request with zero address slasher
		let payload = test_helpers::create_valid_inclusion_payload(12345, vec![0x01, 0x02, 0x03, 0x04])?;
		let request = test_helpers::create_valid_commitment_request(
			test_helpers::create_valid_commitment_type(),
			payload,
			test_helpers::create_invalid_slasher(),
		);

		// Test that the request fails
		harness.test_commitment_request_should_fail(request).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_invalid_payload_format() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a request with invalid payload format
		let request = test_helpers::create_valid_commitment_request(
			test_helpers::create_valid_commitment_type(),
			test_helpers::create_invalid_payload(),
			test_helpers::create_valid_slasher(),
		);

		// Test that the request fails
		harness.test_commitment_request_should_fail(request).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_invalid_slot_zero() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a request with slot 0 (invalid)
		let payload = test_helpers::create_valid_inclusion_payload(0, vec![0x01, 0x02, 0x03, 0x04])?;
		let request = test_helpers::create_valid_commitment_request(
			test_helpers::create_valid_commitment_type(),
			payload,
			test_helpers::create_valid_slasher(),
		);

		// Test that the request fails
		harness.test_commitment_request_should_fail(request).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_empty_signed_tx() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a request with empty signed transaction
		let payload = test_helpers::create_valid_inclusion_payload(12345, vec![])?;
		let request = test_helpers::create_valid_commitment_request(
			test_helpers::create_valid_commitment_type(),
			payload,
			test_helpers::create_valid_slasher(),
		);

		// Test that the request fails
		harness.test_commitment_request_should_fail(request).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_multiple_valid_requests() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Test multiple valid requests with different parameters
		let test_cases = vec![
			(12345, vec![0x01, 0x02, 0x03, 0x04]),
			(67890, vec![0x05, 0x06, 0x07, 0x08]),
			(11111, vec![0x09, 0x0a, 0x0b, 0x0c]),
		];

		for (slot, signed_tx) in test_cases {
			let payload = test_helpers::create_valid_inclusion_payload(slot, signed_tx)?;
			let request = test_helpers::create_valid_commitment_request(
				test_helpers::create_valid_commitment_type(),
				payload,
				test_helpers::create_valid_slasher(),
			);

			let result = harness.test_commitment_request(request).await?;

			// Verify the result
			assert_eq!(result.commitment.commitment_type, 1);
			assert!(result.nonce > 0, "Nonce should be greater than 0");
			assert!(!result.signature.to_string().is_empty(), "Signature should not be empty");
		}

		Ok(())
	}
}
