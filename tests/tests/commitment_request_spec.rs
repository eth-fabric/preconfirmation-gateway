use commitments::handlers::commitment_request_handler;
use common::types::CommitmentRequest;
use jsonrpsee::Extensions;
use jsonrpsee::types::Params;
use std::sync::Arc;

use integration_tests::test_common::{PUBKEY, test_helpers};

/// Test harness for commitment request testing
/// This provides a clean interface for testing different commitment request scenarios
struct CommitmentRequestTestHarness {
	context: common::types::RpcContext,
}

impl CommitmentRequestTestHarness {
	/// Creates a new test harness with a properly configured context
	async fn new() -> eyre::Result<Self> {
		let mut context = test_helpers::create_test_context().await?;

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
	) -> eyre::Result<common::types::SignedCommitment> {
		// Serialize the request to JSON and wrap in array for JSON-RPC params
		let request_json = serde_json::to_string(&request)?;
		let params_json = format!("[{}]", request_json);
		let params = Params::new(Some(&params_json));

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
		// Serialize the request to JSON and wrap in array for JSON-RPC params
		let request_json = serde_json::to_string(&request)?;
		let params_json = format!("[{}]", request_json);
		let params = Params::new(Some(&params_json));

		// Call the handler
		let result =
			commitment_request_handler::<()>(params, Arc::new(self.context.clone()), Extensions::default()).await;

		match result {
			Ok(_) => Err(eyre::eyre!("Expected handler to fail but it succeeded")),
			Err(_) => Ok(()), // Expected failure
		}
	}
}

/// Test cases for different scenarios
mod test_cases {
	use super::*;

	#[tokio::test]
	async fn test_valid_commitment_request() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a valid test request
		let payload = test_helpers::create_valid_inclusion_payload(12345, test_helpers::create_valid_signed_tx())?;
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
		let payload = test_helpers::create_valid_inclusion_payload(12345, test_helpers::create_valid_signed_tx())?;
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
		let payload = test_helpers::create_valid_inclusion_payload(12345, test_helpers::create_valid_signed_tx())?;
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
		let payload = test_helpers::create_valid_inclusion_payload(0, test_helpers::create_valid_signed_tx())?;
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
			(12345, test_helpers::create_valid_signed_tx()),
			(67890, test_helpers::create_valid_signed_tx()),
			(11111, test_helpers::create_valid_signed_tx()),
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

	#[tokio::test]
	async fn test_database_storage_after_successful_request() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a valid test request
		let payload = test_helpers::create_valid_inclusion_payload(12345, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		// Calculate the expected request hash
		let expected_request_hash = request.request_hash()?;

		// Test the request
		let result = harness.test_commitment_request(request).await?;

		// Verify the result was stored in the database
		let stored_commitment = harness.context.database().get_commitment(&expected_request_hash).unwrap();
		assert!(stored_commitment.is_some(), "SignedCommitment should be stored in database");

		let stored = stored_commitment.unwrap();

		// Verify the stored data matches the returned result
		assert_eq!(stored.commitment.commitment_type, result.commitment.commitment_type);
		assert_eq!(stored.commitment.payload, result.commitment.payload);
		assert_eq!(stored.commitment.request_hash, result.commitment.request_hash);
		assert_eq!(stored.commitment.slasher, result.commitment.slasher);
		assert_eq!(stored.nonce, result.nonce);
		assert_eq!(stored.signing_id, result.signing_id);
		assert_eq!(stored.signature, result.signature);

		// Verify the request hash matches what we calculated
		assert_eq!(stored.commitment.request_hash, expected_request_hash);

		Ok(())
	}

	#[tokio::test]
	async fn test_database_persistence_across_multiple_requests() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create multiple requests with different parameters
		let test_requests = vec![
			(11111, test_helpers::create_valid_signed_tx()),
			(22222, test_helpers::create_valid_signed_tx()),
			(33333, test_helpers::create_valid_signed_tx()),
		];

		let mut request_hashes = Vec::new();
		let mut expected_results = Vec::new();

		// Process all requests and collect their hashes and results
		for (slot, signed_tx) in test_requests {
			let payload = test_helpers::create_valid_inclusion_payload(slot, signed_tx)?;
			let request = test_helpers::create_valid_commitment_request(
				test_helpers::create_valid_commitment_type(),
				payload,
				test_helpers::create_valid_slasher(),
			);

			let request_hash = request.request_hash()?;
			let result = harness.test_commitment_request(request).await?;

			request_hashes.push(request_hash);
			expected_results.push(result);
		}

		// Verify all commitments are stored in the database
		for (i, request_hash) in request_hashes.iter().enumerate() {
			let stored_commitment = harness.context.database().get_commitment(request_hash).unwrap();
			assert!(stored_commitment.is_some(), "Commitment {} should be stored in database", i);

			let stored = stored_commitment.unwrap();
			let expected = &expected_results[i];

			// Verify the stored data matches the expected result
			assert_eq!(stored.commitment.commitment_type, expected.commitment.commitment_type);
			assert_eq!(stored.commitment.payload, expected.commitment.payload);
			assert_eq!(stored.commitment.request_hash, expected.commitment.request_hash);
			assert_eq!(stored.commitment.slasher, expected.commitment.slasher);
			assert_eq!(stored.nonce, expected.nonce);
			assert_eq!(stored.signing_id, expected.signing_id);
			assert_eq!(stored.signature, expected.signature);
		}

		Ok(())
	}

	#[tokio::test]
	async fn test_database_retrieval_nonexistent_hash() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Try to retrieve a commitment with a non-existent hash
		let nonexistent_hash = alloy::primitives::B256::from_slice(&[0x99; 32]);
		let retrieved_commitment = harness.context.database().get_commitment(&nonexistent_hash).unwrap();

		assert!(retrieved_commitment.is_none(), "Should return None for non-existent commitment");

		Ok(())
	}

	#[tokio::test]
	async fn test_request_hash_consistency() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a test request
		let payload = test_helpers::create_valid_inclusion_payload(99999, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		// Calculate request hash before processing
		let expected_hash = request.request_hash()?;

		// Process the request
		let result = harness.test_commitment_request(request).await?;

		// Verify the request hash in the result matches our calculation
		assert_eq!(result.commitment.request_hash, expected_hash);

		// Verify we can retrieve the commitment using the calculated hash
		let retrieved = harness.context.database().get_commitment(&expected_hash).unwrap();
		assert!(retrieved.is_some(), "Should be able to retrieve using calculated hash");

		Ok(())
	}

	#[tokio::test]
	async fn test_database_serialization_roundtrip() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create and process a request
		let payload = test_helpers::create_valid_inclusion_payload(77777, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		let result = harness.test_commitment_request(request).await?;
		let request_hash = result.commitment.request_hash;

		// Retrieve from database
		let stored = harness.context.database().get_commitment(&request_hash).unwrap().unwrap();

		// Verify serialization/deserialization roundtrip maintains data integrity
		assert_eq!(stored.commitment.commitment_type, result.commitment.commitment_type);
		assert_eq!(stored.commitment.payload, result.commitment.payload);
		assert_eq!(stored.commitment.request_hash, result.commitment.request_hash);
		assert_eq!(stored.commitment.slasher, result.commitment.slasher);
		assert_eq!(stored.nonce, result.nonce);
		assert_eq!(stored.signing_id, result.signing_id);
		assert_eq!(stored.signature, result.signature);

		// Verify the signature is properly preserved
		assert!(!stored.signature.to_string().is_empty(), "Signature should not be empty after roundtrip");
		assert_eq!(stored.signature, result.signature, "Signature should match exactly after roundtrip");

		Ok(())
	}
}
