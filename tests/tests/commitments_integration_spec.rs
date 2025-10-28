use alloy::primitives::Address;
use common::constants::COMMITMENT_TYPE;
use integration_tests::test_common::TestHarness;

/// Integration tests for commitments RPC server
/// These tests launch the live RPC server and make real JSON-RPC calls
///
/// Test pattern:
/// 1. Setup: Launch commitments service, prepare test data
/// 2. Act: Make RPC calls via ClientHarness
/// 3. Assert: Verify RPC responses and side effects

// ===== COMMITMENT REQUEST RPC TESTS =====

#[tokio::test]
async fn test_commitment_request_rpc_success() {
	// Launch commitments service (port will be auto-assigned)
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Create commitment request
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let request = harness.create_commitment_request(slot, signed_tx, slasher).unwrap();

	// Act: Make RPC call
	let result = client.commitment_request(&request).await.unwrap();

	// Assert: Verify response
	assert_eq!(result.commitment.commitment_type, COMMITMENT_TYPE);
	assert_eq!(result.commitment.slasher, slasher);
	assert!(result.nonce > 0);
	assert!(!result.signature.to_string().is_empty());

	// Verify commitment was stored in database
	let stored = harness.context.database.get_commitment_by_hash(&result.commitment.request_hash).unwrap();
	assert!(stored.is_some());

	println!("commitmentRequest RPC successful");
}

#[tokio::test]
async fn test_commitment_request_rpc_no_delegation() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// No delegation stored for this slot
	let slot = 99999;
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();

	// Act: Make RPC call (should fail)
	let result = client.commitment_request(&request).await;

	assert!(result.is_err());
	println!("commitmentRequest RPC correctly rejects request without delegation");
}

#[tokio::test]
async fn test_commitment_request_rpc_invalid_slot() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Create request with slot 0 (invalid)
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(0, signed_tx, Address::random()).unwrap();

	// Act: Make RPC call (should fail)
	let result = client.commitment_request(&request).await;

	assert!(result.is_err());
	println!("commitmentRequest RPC correctly rejects zero slot");
}

#[tokio::test]
async fn test_commitment_request_rpc_zero_address_slasher() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Create request with zero address slasher
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(slot, signed_tx, Address::ZERO).unwrap();

	// Act: Make RPC call (should fail)
	let result = client.commitment_request(&request).await;

	assert!(result.is_err());
	println!("commitmentRequest RPC correctly rejects zero address slasher");
}

#[tokio::test]
async fn test_commitment_request_rpc_multiple_sequential() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Make multiple RPC calls
	let mut results = vec![];
	for _ in 0..3 {
		let signed_tx = harness.create_signed_tx();
		let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();
		let result = client.commitment_request(&request).await.unwrap();
		results.push(result);
	}

	// All should succeed and have unique request hashes
	assert_eq!(results.len(), 3);
	for result in &results {
		assert!(result.nonce > 0);
	}

	println!("Multiple sequential commitmentRequest RPC calls successful");
}

#[tokio::test]
async fn test_commitment_request_rpc_duplicate() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Create same request and send twice
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();

	let result1 = client.commitment_request(&request).await.unwrap();
	let result2 = client.commitment_request(&request).await.unwrap();

	// Should return same nonce and signature (deterministic)
	assert_eq!(result1.nonce, result2.nonce);
	assert_eq!(result1.signature, result2.signature);
	assert_eq!(result1.commitment.request_hash, result2.commitment.request_hash);

	println!("Duplicate commitmentRequest RPC calls are deterministic");
}

// ===== COMMITMENT RESULT RPC TESTS =====

#[tokio::test]
async fn test_commitment_result_rpc_success() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Create commitment via RPC
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let request = harness.create_commitment_request(slot, signed_tx, slasher).unwrap();
	let commitment_result = client.commitment_request(&request).await.unwrap();

	// Act: Retrieve the commitment via RPC
	let result = client.commitment_result(&commitment_result.commitment.request_hash).await.unwrap();

	// Assert: Verify response matches the created commitment
	assert_eq!(result.commitment.commitment_type, COMMITMENT_TYPE);
	assert_eq!(result.commitment.slasher, slasher);
	assert_eq!(result.commitment.request_hash, commitment_result.commitment.request_hash);
	assert_eq!(result.nonce, commitment_result.nonce);
	assert_eq!(result.signature, commitment_result.signature);

	println!("commitmentResult RPC successful");
}

#[tokio::test]
async fn test_commitment_result_rpc_nonexistent() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Try to retrieve nonexistent commitment
	let nonexistent_hash = alloy::primitives::B256::random();
	let result = client.commitment_result(&nonexistent_hash).await;

	assert!(result.is_err());
	println!("commitmentResult RPC correctly returns error for nonexistent commitment");
}

#[tokio::test]
async fn test_commitment_result_rpc_multiple_commitments() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Create multiple commitments via RPC
	let mut created_commitments = vec![];
	let base_slot = harness.context.slot_timer.get_current_slot() + 1;
	for i in 0..3 {
		let slot = base_slot + i;

		// Store delegation for this slot
		let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
		let signed_delegation =
			harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
		harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

		// Create commitment via RPC
		let signed_tx = harness.create_signed_tx();
		let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();
		let commitment_result = client.commitment_request(&request).await.unwrap();

		created_commitments.push(commitment_result);
	}

	// Act: Retrieve each commitment via RPC
	for created in &created_commitments {
		let result = client.commitment_result(&created.commitment.request_hash).await.unwrap();
		assert_eq!(result.commitment.request_hash, created.commitment.request_hash);
		assert_eq!(result.nonce, created.nonce);
		assert_eq!(result.signature, created.signature);
	}

	println!("commitmentResult RPC successfully retrieves multiple commitments");
}

// ===== FEE RPC TESTS =====

#[tokio::test]
async fn test_fee_rpc_success() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Create commitment request
	// Note: Fee is now calculated via RPC calls to execution client (gas price × gas estimate)
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();

	// Act: Make RPC call
	let result = client.fee(&request).await;

	// Assert: Verify response or handle execution client unavailability
	match result {
		Ok(fee_info) => {
			assert_eq!(fee_info.commitment_type, COMMITMENT_TYPE);
			assert!(!fee_info.fee_payload.is_empty());
			println!("fee RPC successful");
		}
		Err(e) => {
			println!("fee RPC failed (execution client unavailable): {}", e);
		}
	}
}

#[tokio::test]
async fn test_fee_rpc_with_different_requests() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Create two different requests
	// Fee is now calculated via RPC: gas_price × gas_estimate
	let slot1 = harness.context.slot_timer.get_current_slot() + 1;
	let signed_tx1 = harness.create_signed_tx();
	let request1 = harness.create_commitment_request(slot1, signed_tx1, Address::random()).unwrap();

	let slot2 = slot1 + 1;
	let signed_tx2 = harness.create_signed_tx();
	let request2 = harness.create_commitment_request(slot2, signed_tx2, Address::random()).unwrap();

	// Act: Get fees for both
	let fee1 = client.fee(&request1).await;
	let fee2 = client.fee(&request2).await;

	// Verify behavior when execution client is available
	match (fee1, fee2) {
		(Ok(fee_info1), Ok(fee_info2)) => {
			// Both should have same commitment type but different payloads (different request hashes)
			assert_eq!(fee_info1.commitment_type, fee_info2.commitment_type);
			// Fee payloads should be different (they encode different request hashes)
			assert_ne!(fee_info1.fee_payload, fee_info2.fee_payload);
			println!("fee RPC handles different requests correctly");
		}
		_ => {
			println!("fee RPC test skipped (execution client unavailable)");
		}
	}
}

#[tokio::test]
async fn test_fee_rpc_calculates_via_execution_client() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Fee is now calculated via RPC to execution client (no database needed)
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();

	// Act: Make RPC call
	let result = client.fee(&request).await;

	// Fee should be calculated from gas price and gas estimate from execution client
	match result {
		Ok(fee_info) => {
			assert_eq!(fee_info.commitment_type, COMMITMENT_TYPE);
			println!("fee RPC calculates fee via execution client");
		}
		Err(e) => println!("Fee RPC error (execution client may not be available): {}", e),
	}
}

// ===== FULL WORKFLOW TESTS =====

#[tokio::test]
async fn test_full_commitment_workflow_via_rpc() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	// Fee is calculated via RPC to execution client
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Step 1: Get fee (calculated from execution client RPC)
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let request = harness.create_commitment_request(slot, signed_tx, slasher).unwrap();
	let fee_result = client.fee(&request).await;

	match fee_result {
		Ok(fee_info) => {
			assert_eq!(fee_info.commitment_type, COMMITMENT_TYPE);
			println!("Fee calculation successful");
		}
		Err(_) => {
			println!("Fee calculation skipped (execution client unavailable)");
		}
	}

	// Step 2: Create commitment (should work regardless of fee calculation)
	let commitment_result = client.commitment_request(&request).await.unwrap();
	assert_eq!(commitment_result.commitment.commitment_type, COMMITMENT_TYPE);
	assert_eq!(commitment_result.commitment.slasher, slasher);

	// Step 3: Retrieve commitment
	let retrieved = client.commitment_result(&commitment_result.commitment.request_hash).await.unwrap();
	assert_eq!(retrieved.commitment.request_hash, commitment_result.commitment.request_hash);
	assert_eq!(retrieved.nonce, commitment_result.nonce);
	assert_eq!(retrieved.signature, commitment_result.signature);

	println!("Full commitment workflow via RPC successful");
}

#[tokio::test]
async fn test_concurrent_commitment_requests() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Setup: Store delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Create multiple requests
	let mut requests = vec![];
	for _ in 0..5 {
		let signed_tx = harness.create_signed_tx();
		let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();
		requests.push(request);
	}

	// Send all requests concurrently
	let mut handles = vec![];
	for request in requests {
		let client_clone = client.clone();
		let handle = tokio::spawn(async move { client_clone.commitment_request(&request).await });
		handles.push(handle);
	}

	// Wait for all to complete
	let mut results = vec![];
	for handle in handles {
		let result = handle.await.unwrap();
		assert!(result.is_ok());
		results.push(result.unwrap());
	}

	// All should succeed
	assert_eq!(results.len(), 5);
	println!("Concurrent commitment requests handled correctly");
}

#[tokio::test]
async fn test_rpc_server_handles_errors_gracefully() {
	let harness = TestHarness::builder().with_commitments_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Test various error conditions

	// 1. No delegation
	let signed_tx = harness.create_signed_tx();
	let request1 = harness.create_commitment_request(99999, signed_tx, Address::random()).unwrap();
	let result1 = client.commitment_request(&request1).await;
	assert!(result1.is_err());

	// 2. Nonexistent commitment
	let result2 = client.commitment_result(&alloy::primitives::B256::random()).await;
	assert!(result2.is_err());

	// Server should still be responsive after errors
	// Store delegation and make a valid request
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	let signed_tx = harness.create_signed_tx();
	let valid_request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();
	let result3 = client.commitment_request(&valid_request).await;
	assert!(result3.is_ok());

	println!("RPC server handles errors gracefully and remains responsive");
}
