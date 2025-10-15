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
		let context = test_helpers::create_test_context().await?;

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

		Ok(Self { context })
	}

	/// Creates a new test harness with a delegation set up for the given slot
	async fn new_with_delegation(slot: u64) -> eyre::Result<Self> {
		let harness = Self::new().await?;

		// Add a delegation for the slot to the database
		harness.setup_delegation_for_slot(slot).await?;

		Ok(harness)
	}

	/// Sets up a delegation for a specific slot in the database
	async fn setup_delegation_for_slot(&self, slot: u64) -> eyre::Result<()> {
		use alloy::primitives::{B256, Bytes};
		use commit_boost::prelude::BlsSignature;
		use common::types::constraints::SignedDelegation;

		// Use the test BLS public key from constants
		let test_bls_pubkey = cb_common::types::BlsPublicKey::deserialize(&PUBKEY).unwrap();

		// Generate a proxy key for the committer using the signer from the context
		let mut commit_config = self.context.commit_config.lock().await;
		let proxy_address = commit_config
			.signer_client
			.generate_proxy_key_ecdsa(test_bls_pubkey.clone())
			.await
			.map_err(|e| eyre::eyre!("Failed to generate ECDSA proxy key: {}", e))?;
		let committer_address = proxy_address.message.proxy;
		drop(commit_config); // Release the lock

		// Create a mock delegation for testing
		let mock_delegation = SignedDelegation {
			message: common::types::constraints::Delegation {
				proposer: test_bls_pubkey.clone(),
				delegate: test_bls_pubkey,
				committer: committer_address,
				slot,
				metadata: Bytes::new(),
			},
			nonce: 1,
			signing_id: B256::default(),
			signature: BlsSignature::deserialize(&[0u8; 96])
				.map_err(|e| eyre::eyre!("Failed to create BLS signature: {:?}", e))?,
		};

		// Store the delegation in the database
		self.context
			.database
			.store_delegation(slot, &mock_delegation)
			.map_err(|e| eyre::eyre!("Failed to store test delegation: {}", e))?;

		Ok(())
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
	use commit_boost::prelude::tree_hash::TreeHash;

	use super::*;

	#[tokio::test]
	async fn test_valid_commitment_request() -> eyre::Result<()> {
		let slot = 12345;
		let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;

		// Create a valid test request
		let payload = test_helpers::create_valid_inclusion_payload(slot, test_helpers::create_valid_signed_tx())?;
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
	async fn test_commitment_request_without_delegation() -> eyre::Result<()> {
		let harness = CommitmentRequestTestHarness::new().await?;

		// Create a valid test request for a slot without delegation
		// Use a slot that's not in the common test slots list (10000, 10001, 10002, 10003, 10004, 99999, 12345, 67890, 11111, 22222, 33333)
		let slot = 999999;
		let payload = test_helpers::create_valid_inclusion_payload(slot, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		// Test the request - should fail because no delegation exists
		harness.test_commitment_request_should_fail(request).await?;
		println!("Successfully verified that commitment request fails without delegation");

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
		// Test multiple valid requests with different parameters
		let test_cases = vec![
			(12345, test_helpers::create_valid_signed_tx()),
			(67890, test_helpers::create_valid_signed_tx()),
			(11111, test_helpers::create_valid_signed_tx()),
		];

		for (slot, signed_tx) in test_cases {
			let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
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
		// Create a valid test request
		let slot = 12345;
		let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
		let payload = test_helpers::create_valid_inclusion_payload(slot, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		// Calculate the expected request hash
		let expected_request_hash = request.request_hash()?;

		// Test the request
		let result = harness.test_commitment_request(request).await?;

		// Verify the result was stored in the database
		let stored_commitment = harness.context.database().get_commitment(slot, &expected_request_hash).unwrap();
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
		// Create multiple requests with different parameters
		let test_requests = vec![
			(11111, test_helpers::create_valid_signed_tx()),
			(22222, test_helpers::create_valid_signed_tx()),
			(33333, test_helpers::create_valid_signed_tx()),
		];

		let mut request_hashes = Vec::new();
		let mut expected_results = Vec::new();
		let mut harnesses = Vec::new();
		let mut slots = Vec::new();

		// Process all requests and collect their hashes and results
		for (slot, signed_tx) in test_requests {
			let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
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
			harnesses.push(harness);
			slots.push(slot);
		}

		// Verify all commitments are stored in the database
		for (i, request_hash) in request_hashes.iter().enumerate() {
			let stored_commitment = harnesses[i].context.database().get_commitment(slots[i], request_hash).unwrap();
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
		let slot = 12345;

		// Try to retrieve a commitment with a non-existent hash
		let nonexistent_hash = alloy::primitives::B256::from_slice(&[0x99; 32]);
		let retrieved_commitment = harness.context.database().get_commitment(slot, &nonexistent_hash).unwrap();

		assert!(retrieved_commitment.is_none(), "Should return None for non-existent commitment");

		Ok(())
	}

	#[tokio::test]
	async fn test_request_hash_consistency() -> eyre::Result<()> {
		// Create a test request
		let slot = 99999;
		let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
		let payload = test_helpers::create_valid_inclusion_payload(slot, test_helpers::create_valid_signed_tx())?;
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
		let retrieved = harness.context.database().get_commitment(slot, &expected_hash).unwrap();
		assert!(retrieved.is_some(), "Should be able to retrieve using calculated hash");

		Ok(())
	}

	#[tokio::test]
	async fn test_database_serialization_roundtrip() -> eyre::Result<()> {
		// Create and process a request
		let slot = 77777;
		let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
		let payload = test_helpers::create_valid_inclusion_payload(slot, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		let result = harness.test_commitment_request(request).await?;
		let request_hash = result.commitment.request_hash;

		// Retrieve from database
		let stored = harness.context.database().get_commitment(slot, &request_hash).unwrap().unwrap();

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

	#[tokio::test]
	async fn test_constraint_saved_after_commitment() -> eyre::Result<()> {
		// Create and process a commitment request
		let slot = 12345;
		let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
		let payload = test_helpers::create_valid_inclusion_payload(slot, test_helpers::create_valid_signed_tx())?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();
		let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

		// Process the request
		let result = harness.test_commitment_request(request).await?;

		// Verify the commitment was stored
		let request_hash = result.commitment.request_hash;
		let stored_commitment = harness.context.database().get_commitment(slot, &request_hash).unwrap();
		assert!(stored_commitment.is_some(), "Commitment should be stored in database");

		// Verify that a constraint was also saved
		// We need to check if there are any constraints in the database for this slot
		let constraints = harness.context.database().get_constraints_for_slot(slot).unwrap();
		assert!(!constraints.is_empty(), "At least one constraint should be saved");

		// Find the constraint that matches our slot
		let matching_constraint = constraints.iter().find(|(_, constraint)| {
			// Decode the constraint payload to check the slot
			use common::types::commitments::InclusionPayload;
			if let Ok(inclusion_payload) = InclusionPayload::abi_decode(&constraint.payload) {
				inclusion_payload.slot == slot
			} else {
				false
			}
		});

		assert!(matching_constraint.is_some(), "Should find a constraint for slot {}", slot);

		let (_, constraint) = matching_constraint.unwrap();
		assert_eq!(constraint.constraint_type, 1, "Constraint type should be 1 (CONSTRAINT_TYPE)");
		assert_eq!(constraint.payload, result.commitment.payload, "Constraint payload should match commitment payload");

		println!("Successfully verified constraint was saved for slot {}", slot);
		Ok(())
	}

	#[tokio::test]
	async fn test_multiple_constraints_same_slot() -> eyre::Result<()> {
		// Create multiple commitment requests with the same slot but different payloads
		let slot = 54321;
		let harness = CommitmentRequestTestHarness::new_with_delegation(slot).await?;
		let num_requests = 3;

		for i in 0..num_requests {
			// Create different signed transactions for each request by varying the transaction data
			// We'll use the same signed_tx but add some variation to the request to make each unique
			let signed_tx = test_helpers::create_valid_signed_tx();
			let payload = test_helpers::create_valid_inclusion_payload(slot, signed_tx)?;
			// Use different slasher addresses to make each request unique
			let slasher = test_helpers::create_valid_slasher();
			let commitment_type = test_helpers::create_valid_commitment_type();
			let request = test_helpers::create_valid_commitment_request(commitment_type, payload, slasher);

			// Process the request
			let result = harness.test_commitment_request(request).await?;
			println!("Processed commitment request {} for slot {}", i + 1, slot);

			// Verify the commitment was stored
			let request_hash = result.commitment.request_hash;
			let stored_commitment = harness.context.database().get_commitment(slot, &request_hash).unwrap();
			assert!(stored_commitment.is_some(), "Commitment {} should be stored in database", i + 1);
		}

		// Verify that multiple constraints were saved for the same slot
		let constraints = harness.context.database().get_constraints_for_slot(slot).unwrap();

		// All constraints should be for our slot since we're using get_constraints_for_slot
		let slot_constraints: Vec<_> = constraints.iter().collect();

		assert_eq!(slot_constraints.len(), num_requests, "Should have {} constraints for slot {}", num_requests, slot);

		// Verify all constraints have the correct type
		for (_, constraint) in &slot_constraints {
			assert_eq!(constraint.constraint_type, 1, "All constraints should have type 1 (CONSTRAINT_TYPE)");
		}

		println!("Successfully verified {} constraints were saved for slot {}", slot_constraints.len(), slot);
		Ok(())
	}

	#[test]
	/// The expected ecdsa signature for a commitment request
	fn test_fixed_sign_commitment_request() -> eyre::Result<()> {
		use alloy::signers::{SignerSync, local::PrivateKeySigner};
		let local_signer = PrivateKeySigner::from_bytes(&alloy::primitives::B256::from_slice(
			&alloy::primitives::bytes!("0x0501e85d5bc2e95f70efda47409710a7cf01dd02ff238e3efec679b27331d917"),
		))
		.unwrap();

		let commitment_request = CommitmentRequest {
			commitment_type: 1,
			payload: alloy_primitives::Bytes::new(),
			slasher: alloy_primitives::Address::repeat_byte(0x11),
		};

		let commitment = common::Commitment {
			commitment_type: 1,
			payload: commitment_request.payload.clone(),
			request_hash: alloy::primitives::keccak256(commitment_request.abi_encode()?),
			slasher: commitment_request.slasher.clone(),
		};
		let message_hash = commitment.to_message_hash()?;
		assert_eq!(
			message_hash,
			alloy::primitives::B256::from_slice(&alloy::primitives::bytes!(
				"0x310fefd12df340a4d723608aec3e7471ced2abe290a9458a5ba1832b9fc84653"
			))
		);

		let module_signing_id = alloy::primitives::B256::from_slice(&alloy::primitives::bytes!(
			"0xcb005700fab121c00ccbc94db58c04675b7847c38f9583815139d1d98bea0cb0"
		));

		let domain = alloy::primitives::B256::from_slice(&alloy::primitives::bytes!(
			"0x6d6d6f43719103511efa4f1362ff2a50996cccf329cc84cb410c5e5c7d351d03"
		));

		let object_root = cb_common::types::PropCommitSigningInfo {
			data: message_hash,
			module_signing_id: module_signing_id,
			nonce: u64::MAX - 1,
			chain_id: cb_common::types::Chain::Hoodi.id(),
		}
		.tree_hash_root();
		assert_eq!(
			object_root,
			alloy::primitives::B256::from_slice(&alloy::primitives::bytes!(
				"0xfa3897cac4616ff1f5d3a698ebd827d267e460aaa20b8a6a3c8e46c7c01f455d"
			))
		);

		let signing_root = cb_common::types::SigningData { object_root, signing_domain: domain }.tree_hash_root();
		assert_eq!(
			signing_root,
			alloy::primitives::B256::from_slice(&alloy::primitives::bytes!(
				"0x90447b3fac12f371cbb8aa2e1527f2abda7576cd618244c005cdb3c0d63e8a7b"
			))
		);

		let signature = local_signer.sign_hash_sync(&signing_root)?;

		assert_eq!(signature.to_string(), "0xf59f6fbee040a98021f55b8aaff8eecd4f1c1c9723ef7b5b9d09a0c0835bf94e20d90fc12a203bb9fcf81f7e838806bd3e67b82802d0bcd842163bd37c2cce581b".to_string());

		Ok(())
	}
}
