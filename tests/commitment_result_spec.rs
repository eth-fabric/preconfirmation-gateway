use alloy::primitives::B256;
use jsonrpsee::Extensions;
use jsonrpsee::types::Params;
use preconfirmation_gateway::commitments::handlers::commitment_result_handler;
use preconfirmation_gateway::types::SignedCommitment;

mod common;
use common::test_helpers;

/// Test harness for commitment result testing
/// This provides a clean interface for testing different commitment result scenarios
struct CommitmentResultTestHarness {
	context: preconfirmation_gateway::RpcContext,
}

impl CommitmentResultTestHarness {
	/// Creates a new test harness with a properly configured context
	async fn new() -> eyre::Result<Self> {
		let context = test_helpers::create_test_context().await?;
		Ok(Self { context })
	}

	/// Tests a commitment result request and returns the result
	async fn test_commitment_result(&self, request_hash: B256) -> eyre::Result<SignedCommitment> {
		// Create params with the request hash as a single parameter
		let request_json = serde_json::to_string(&request_hash)?;
		let params_json = format!("[{}]", request_json);
		let params = Params::new(Some(&params_json));

		// Call the handler
		let result = commitment_result_handler::<()>(params, &self.context, &Extensions::default());

		match result {
			Ok(signed_commitment) => Ok(signed_commitment),
			Err(e) => Err(eyre::eyre!("Handler failed: {}", e)),
		}
	}

	/// Tests a commitment result request and expects it to fail with a specific error
	async fn test_commitment_result_should_fail(&self, request_hash: B256) -> eyre::Result<()> {
		// Create params with the request hash as a single parameter
		let request_json = serde_json::to_string(&request_hash)?;
		let params_json = format!("[{}]", request_json);
		let params = Params::new(Some(&params_json));

		// Call the handler
		let result = commitment_result_handler::<()>(params, &self.context, &Extensions::default());

		match result {
			Ok(_) => Err(eyre::eyre!("Expected handler to fail but it succeeded")),
			Err(_) => Ok(()), // Expected failure
		}
	}

	/// Pre-populates the database with a signed commitment for testing
	fn pre_populate_commitment(&self, request_hash: B256, signed_commitment: SignedCommitment) -> eyre::Result<()> {
		self.context
			.database()
			.store_commitment(&request_hash, &signed_commitment)
			.map_err(|e| eyre::eyre!("Failed to store commitment: {}", e))?;
		Ok(())
	}

	/// Pre-populates the database with multiple signed commitments for testing
	fn pre_populate_multiple_commitments(&self, commitments: Vec<(B256, SignedCommitment)>) -> eyre::Result<()> {
		for (request_hash, signed_commitment) in commitments {
			self.context
				.database()
				.store_commitment(&request_hash, &signed_commitment)
				.map_err(|e| eyre::eyre!("Failed to store commitment: {}", e))?;
		}
		Ok(())
	}
}

/// Test cases for different scenarios
mod test_cases {
	use super::*;

	#[tokio::test]
	async fn test_valid_commitment_result() -> eyre::Result<()> {
		let harness = CommitmentResultTestHarness::new().await?;

		// Create test data
		let request_hash = test_helpers::create_valid_request_hash();
		let slasher = test_helpers::create_valid_slasher();
		let payload = test_helpers::create_valid_payload();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let nonce = test_helpers::create_valid_nonce();
		let signing_id = test_helpers::create_valid_signing_id();

		// Create and store a signed commitment
		let signed_commitment = test_helpers::create_signed_commitment_with_mock_signature(
			commitment_type,
			payload.clone(),
			request_hash,
			slasher,
			nonce,
			signing_id,
		);

		harness.pre_populate_commitment(request_hash, signed_commitment.clone())?;

		// Test retrieving the commitment
		let result = harness.test_commitment_result(request_hash).await?;

		// Verify the result matches what we stored
		assert_eq!(result.commitment.commitment_type, commitment_type);
		assert_eq!(result.commitment.payload, payload);
		assert_eq!(result.commitment.request_hash, request_hash);
		assert_eq!(result.commitment.slasher, slasher);
		assert_eq!(result.nonce, nonce);
		assert_eq!(result.signing_id, signing_id);
		assert_eq!(result.signature, signed_commitment.signature);

		Ok(())
	}

	#[tokio::test]
	async fn test_nonexistent_commitment() -> eyre::Result<()> {
		let harness = CommitmentResultTestHarness::new().await?;

		// Try to retrieve a commitment that doesn't exist
		let nonexistent_hash = test_helpers::create_nonexistent_request_hash();
		harness.test_commitment_result_should_fail(nonexistent_hash).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_multiple_commitments() -> eyre::Result<()> {
		let harness = CommitmentResultTestHarness::new().await?;

		// Create multiple commitments
		let commitments = vec![
			(
				test_helpers::create_valid_request_hash(),
				test_helpers::create_signed_commitment_with_mock_signature(
					test_helpers::create_valid_commitment_type(),
					test_helpers::create_valid_payload(),
					test_helpers::create_valid_request_hash(),
					test_helpers::create_valid_slasher(),
					test_helpers::create_valid_nonce(),
					test_helpers::create_valid_signing_id(),
				),
			),
			(
				test_helpers::create_another_valid_request_hash(),
				test_helpers::create_signed_commitment_with_mock_signature(
					test_helpers::create_valid_commitment_type(),
					test_helpers::create_another_valid_payload(),
					test_helpers::create_another_valid_request_hash(),
					test_helpers::create_valid_slasher(),
					test_helpers::create_another_valid_nonce(),
					test_helpers::create_valid_signing_id(),
				),
			),
		];

		// Pre-populate the database
		harness.pre_populate_multiple_commitments(commitments.clone())?;

		// Test retrieving each commitment
		for (request_hash, expected_commitment) in commitments {
			let result = harness.test_commitment_result(request_hash).await?;

			// Verify the result matches what we stored
			assert_eq!(result.commitment.commitment_type, expected_commitment.commitment.commitment_type);
			assert_eq!(result.commitment.payload, expected_commitment.commitment.payload);
			assert_eq!(result.commitment.request_hash, expected_commitment.commitment.request_hash);
			assert_eq!(result.commitment.slasher, expected_commitment.commitment.slasher);
			assert_eq!(result.nonce, expected_commitment.nonce);
			assert_eq!(result.signing_id, expected_commitment.signing_id);
			assert_eq!(result.signature, expected_commitment.signature);
		}

		Ok(())
	}
}
