use alloy::primitives::{B256, Bytes};
use common::constants::CONSTRAINT_TYPE;
use common::types::constraints::Constraint;
use integration_tests::test_common::TestHarness;

/// Integration tests for relay HTTP server
/// These tests launch the live HTTP server and make real HTTP calls
///
/// Test pattern:
/// 1. Setup: Launch relay service
/// 2. Act: Make HTTP calls via ClientHarness
/// 3. Assert: Verify HTTP responses and side effects

// ===== DELEGATION HTTP TESTS =====

#[tokio::test]
async fn test_post_delegation_success() {
	// Launch relay service (lookahead is auto-populated)
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Create delegation
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();

	// Act: Post delegation via HTTP
	client.post_delegation(&signed_delegation).await.unwrap();
}

#[tokio::test]
async fn test_post_delegation_multiple() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let current_slot = harness.context.slot_timer.get_current_slot();

	// Post multiple delegations for different slots
	for i in 0..3 {
		let slot = current_slot + i;
		let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
		let signed_delegation =
			harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();

		client.post_delegation(&signed_delegation).await.unwrap();
	}
}

#[tokio::test]
async fn test_post_delegation_different_delegates() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Post delegation for gateway_one
	let slot1 = harness.context.slot_timer.get_current_slot() + 1;
	let delegation1 = harness.create_delegation(slot1, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation1 =
		harness.create_signed_delegation(&delegation1, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation1).await.unwrap();

	// Post delegation for gateway_two
	let slot2 = slot1 + 1;
	let delegation2 = harness.create_delegation(slot2, harness.gateway_bls_two.clone(), harness.committer_two);
	let signed_delegation2 =
		harness.create_signed_delegation(&delegation2, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation2).await.unwrap();
}

#[tokio::test]
async fn test_post_delegation_invalid_signature() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Create delegation but sign with wrong key (gateway instead of proposer)
	let slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.gateway_bls_one.clone()).await.unwrap();

	// Act: Try to post with invalid signature
	let result = client.post_delegation(&signed_delegation).await;

	// Should fail with HTTP error
	assert!(result.is_err());
}

// ===== CONSTRAINTS HTTP TESTS =====

#[tokio::test]
async fn test_post_constraints_success() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Step 1: Post delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Step 2: Store constraints in local database
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3, 4, 5]) };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();

	// Step 3: Process constraints (signs and posts to relay)
	let result = harness.process_constraints(slot, vec![]).await;

	// Should succeed
	assert!(result.is_ok());
	let response = result.unwrap();
	assert!(response.success);
}

// todo
#[tokio::test]
async fn test_post_constraints_without_delegation() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let slot = harness.context.slot_timer.get_current_slot() + 100000000; // No delegation for this slot

	// Store constraints in local database without posting delegation first
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();

	// Act: Try to process constraints without delegation
	let result = harness.process_constraints(slot, vec![]).await;

	// Should fail due to missing delegation (relay returns 404)
	assert!(result.is_ok()); // The process itself succeeds
	assert!(!result.unwrap().success); // ...but relay rejected it
}

#[tokio::test]
async fn test_post_constraints_multiple_same_slot() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Post delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Store multiple constraints for same slot
	for i in 0..3 {
		let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![i as u8; 5]) };
		let constraint_id = B256::random();
		harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();
	}

	// Process all constraints at once
	let result = harness.process_constraints(slot, vec![]).await.unwrap();
	assert!(result.success);
}

// ===== GET CONSTRAINTS TESTS =====

#[tokio::test]
async fn test_get_constraints_success() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Setup: Post delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Store and process constraints
	let payload = Bytes::from(vec![1, 2, 3, 4, 5]);
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: payload.clone() };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();

	let result = harness.process_constraints(slot, vec![]).await.unwrap();
	assert!(result.success);

	// Act: Get constraints via HTTP
	let headers = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_one.clone()).await;
	let result = client.get_constraints(slot, headers).await;

	assert!(result.is_ok());
	let constraints_list = result.unwrap();
	assert_eq!(constraints_list.len(), 1);
	assert_eq!(constraints_list[0].message.slot, slot);
	assert_eq!(constraints_list[0].message.constraints.len(), 1);
	assert_eq!(constraints_list[0].message.constraints[0].payload, payload);
}

#[tokio::test]
async fn test_get_constraints_multiple() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Setup: Post delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Store multiple constraints
	for i in 0..3 {
		let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![i as u8; 5]) };
		let constraint_id = B256::random();
		harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();
	}

	// Process all constraints
	let result = harness.process_constraints(slot, vec![]).await.unwrap();
	assert!(result.success);

	// Act: Get all constraints for slot
	let headers = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_one.clone()).await;
	let result = client.get_constraints(slot, headers).await;

	assert!(result.is_ok());
	let constraints_list = result.unwrap();
	assert_eq!(constraints_list.len(), 1); // All constraints are batched into one signed message
}

#[tokio::test]
async fn test_get_constraints_nonexistent_slot() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Try to get constraints for slot without any data
	let nonexistent_slot = 99999;
	let headers = harness.create_headers_with_valid_signature(nonexistent_slot, harness.gateway_bls_one.clone()).await;
	let result = client.get_constraints(nonexistent_slot, headers).await;

	assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_get_constraints_missing_auth_headers() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Try to get constraints without auth headers
	let empty_headers = axum::http::HeaderMap::new();
	let result = client.get_constraints(slot, empty_headers).await;

	// Should fail due to missing auth headers
	assert!(result.is_err());
}

// ===== FULL WORKFLOW TESTS =====

#[tokio::test]
async fn test_full_relay_workflow() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Step 1: Post delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Step 2: Store and process constraints
	let payload = Bytes::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: payload.clone() };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();

	let result = harness.process_constraints(slot, vec![]).await.unwrap();
	assert!(result.success);

	// Step 3: Retrieve constraints
	let headers = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_one.clone()).await;
	let retrieved = client.get_constraints(slot, headers).await.unwrap();

	// Verify full workflow
	assert_eq!(retrieved.len(), 1);
	assert_eq!(retrieved[0].message.slot, slot);
	assert_eq!(retrieved[0].message.constraints[0].payload, payload);
}

#[tokio::test]
async fn test_concurrent_constraint_posts() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let relay_url = harness.relay_service.as_ref().unwrap().url.clone();

	// Post delegations for multiple slots
	let client = harness.create_client_harness();
	let current_slot = harness.context.slot_timer.get_current_slot();
	for i in 0..5 {
		let slot = current_slot + i;
		let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
		let signed_delegation =
			harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
		client.post_delegation(&signed_delegation).await.unwrap();

		// Store constraint for this slot
		let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![i as u8; 5]) };
		let constraint_id = B256::random();
		harness.context.database.store_constraint(slot, &constraint_id, &constraint).unwrap();
	}

	// Process all constraints concurrently
	let mut handles = vec![];
	for i in 0..5 {
		let slot = current_slot + i;
		let gateway = harness.gateway_bls_one.clone();
		let proposer = harness.proposer_bls_public_key.clone();
		let database = harness.context.database.clone();
		let commit_config = harness.context.commit_config.clone();
		let relay_url_clone = relay_url.clone();

		let handle = tokio::spawn(async move {
			gateway::constraints::process_constraints(
				slot,
				gateway,
				proposer,
				vec![],
				&database,
				commit_config,
				relay_url_clone,
				None,
			)
			.await
		});
		handles.push(handle);
	}

	// Wait for all to complete
	for handle in handles {
		let result = handle.await.unwrap().unwrap();
		assert!(result.success);
	}
}

#[tokio::test]
async fn test_relay_server_handles_errors_gracefully() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Test various error conditions

	// 1. Process constraints without delegation
	let invalid_slot = harness.context.slot_timer.get_current_slot() + 100000000;
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(invalid_slot, &constraint_id, &constraint).unwrap();

	let result1 = harness.process_constraints(invalid_slot, vec![]).await.unwrap();
	assert!(!result1.success); // Should fail due to missing delegation

	// 2. Get constraints for nonexistent slot
	let headers = harness.create_headers_with_valid_signature(invalid_slot, harness.gateway_bls_one.clone()).await;
	let result2 = client.get_constraints(invalid_slot, headers).await;
	// Either empty or error is acceptable
	match result2 {
		Ok(constraints) => assert!(constraints.is_empty()),
		Err(_) => {}
	}

	// Server should still be responsive after errors
	// Post a valid delegation
	let valid_slot = harness.context.slot_timer.get_current_slot() + 1;
	let delegation = harness.create_delegation(valid_slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();
}

#[tokio::test]
async fn test_get_constraints_current_slot_requires_authentication() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Use current slot
	let current_slot = harness.context.slot_timer.get_current_slot();

	// Setup: Post delegation for current slot
	let delegation = harness.create_delegation(current_slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Store and process constraints
	let payload = Bytes::from(vec![1, 2, 3, 4, 5]);
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: payload.clone() };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(current_slot, &constraint_id, &constraint).unwrap();

	let result = harness.process_constraints(current_slot, vec![]).await.unwrap();
	assert!(result.success);

	// Act: Try to get constraints WITHOUT auth headers (should fail for current slot)
	let empty_headers = axum::http::HeaderMap::new();
	let result = client.get_constraints(current_slot, empty_headers).await;

	// Assert: Should fail due to missing authentication
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_constraints_future_slot_requires_authentication() {
	let harness = TestHarness::builder().with_relay_port(None).build().await.unwrap();
	let client = harness.create_client_harness();

	// Use a future slot
	let current_slot = harness.context.slot_timer.get_current_slot();
	let future_slot = current_slot + 100;

	// Setup: Post delegation for future slot
	let delegation = harness.create_delegation(future_slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	client.post_delegation(&signed_delegation).await.unwrap();

	// Store and process constraints
	let payload = Bytes::from(vec![1, 2, 3, 4, 5]);
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: payload.clone() };
	let constraint_id = B256::random();
	harness.context.database.store_constraint(future_slot, &constraint_id, &constraint).unwrap();

	let result = harness.process_constraints(future_slot, vec![]).await.unwrap();
	assert!(result.success);

	// Act: Try to get constraints WITHOUT auth headers (should fail for future slot)
	let empty_headers = axum::http::HeaderMap::new();
	let result = client.get_constraints(future_slot, empty_headers).await;

	// Assert: Should fail due to missing authentication
	assert!(result.is_err());
}

// NOTE: Testing that past slots ignore receivers[] list is covered in unit tests
