use alloy::primitives::{Address, B256};
use common::constants::COMMITMENT_TYPE;
use integration_tests::test_common::{SIGNING_ID, TestHarness};

/// Tests for the commitmentResult RPC handler
/// These tests do NOT launch services - they test retrieval logic directly
/// Data is written directly to databases to isolate the behavior under test
///
/// The test pattern is:
/// 1. Setup: Create commitment and store it directly in database
/// 2. Act: Retrieve commitment from database by hash
/// 3. Assert: Verify the retrieved commitment matches what was stored

// ===== POSITIVE TESTS =====

#[tokio::test]
async fn test_retrieve_commitment_by_request_hash() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Create and store a commitment
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let request = harness.create_commitment_request(slot, signed_tx, slasher).unwrap();
	let signed_commitment = commitments::utils::create_signed_commitment(
		&request,
		harness.context.commit_config.clone(),
		harness.committer_one,
		&SIGNING_ID,
	)
	.await
	.unwrap();

	// Store in database
	let constraint = commitments::utils::create_constraint_from_commitment_request(&request, slot).unwrap();
	harness
		.context
		.commitments_database
		.store_commitment_and_constraint(
			slot,
			&signed_commitment.commitment.request_hash,
			&signed_commitment,
			&constraint,
		)
		.unwrap();

	// Retrieve by request hash
	let retrieved = harness
		.context
		.commitments_database
		.get_commitment_by_hash(&signed_commitment.commitment.request_hash)
		.unwrap();

	assert!(retrieved.is_some());
	let retrieved_commitment = retrieved.unwrap();
	assert_eq!(retrieved_commitment.commitment.commitment_type, COMMITMENT_TYPE);
	assert_eq!(retrieved_commitment.commitment.slasher, slasher);
	assert_eq!(retrieved_commitment.commitment.request_hash, signed_commitment.commitment.request_hash);
	assert_eq!(retrieved_commitment.nonce, signed_commitment.nonce);

	println!(" Commitment retrieved successfully by request hash");
}

#[tokio::test]
async fn test_retrieve_multiple_commitments() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Create and store multiple commitments
	let mut stored_commitments = vec![];

	let base_slot = harness.context.slot_timer.get_current_slot() + 1;
	for i in 0..3 {
		let slot = base_slot + i;
		let signed_tx = harness.create_signed_tx();
		let slasher = Address::random();
		let request = harness.create_commitment_request(slot, signed_tx, slasher).unwrap();
		let signed_commitment = commitments::utils::create_signed_commitment(
			&request,
			harness.context.commit_config.clone(),
			harness.committer_one,
			&SIGNING_ID,
		)
		.await
		.unwrap();

		let constraint = commitments::utils::create_constraint_from_commitment_request(&request, slot).unwrap();
		harness
			.context
			.commitments_database
			.store_commitment_and_constraint(
				slot,
				&signed_commitment.commitment.request_hash,
				&signed_commitment,
				&constraint,
			)
			.unwrap();

		stored_commitments.push(signed_commitment);
	}

	// Retrieve each commitment and verify
	for stored in &stored_commitments {
		let retrieved =
			harness.context.commitments_database.get_commitment_by_hash(&stored.commitment.request_hash).unwrap();

		assert!(retrieved.is_some());
		let retrieved_commitment = retrieved.unwrap();
		assert_eq!(retrieved_commitment.commitment.request_hash, stored.commitment.request_hash);
		assert_eq!(retrieved_commitment.commitment.slasher, stored.commitment.slasher);
		assert_eq!(retrieved_commitment.nonce, stored.nonce);
		assert_eq!(retrieved_commitment.signature, stored.signature);
	}

	println!(" Multiple commitments retrieved successfully");
}

#[tokio::test]
async fn test_commitment_fields_preserved_after_storage() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Create commitment with specific values
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let request = harness.create_commitment_request(slot, signed_tx, slasher).unwrap();
	let signed_commitment = commitments::utils::create_signed_commitment(
		&request,
		harness.context.commit_config.clone(),
		harness.committer_one,
		&SIGNING_ID,
	)
	.await
	.unwrap();

	// Store it
	let constraint = commitments::utils::create_constraint_from_commitment_request(&request, slot).unwrap();
	harness
		.context
		.commitments_database
		.store_commitment_and_constraint(
			slot,
			&signed_commitment.commitment.request_hash,
			&signed_commitment,
			&constraint,
		)
		.unwrap();

	// Retrieve and verify all fields are preserved
	let retrieved = harness
		.context
		.commitments_database
		.get_commitment_by_hash(&signed_commitment.commitment.request_hash)
		.unwrap()
		.unwrap();

	assert_eq!(retrieved.commitment.commitment_type, signed_commitment.commitment.commitment_type);
	assert_eq!(retrieved.commitment.payload, signed_commitment.commitment.payload);
	assert_eq!(retrieved.commitment.request_hash, signed_commitment.commitment.request_hash);
	assert_eq!(retrieved.commitment.slasher, signed_commitment.commitment.slasher);
	assert_eq!(retrieved.nonce, signed_commitment.nonce);
	assert_eq!(retrieved.signing_id, signed_commitment.signing_id);
	assert_eq!(retrieved.signature, signed_commitment.signature);

	println!(" All commitment fields preserved after storage");
}

#[tokio::test]
async fn test_retrieve_commitment_idempotent() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Create and store commitment
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();
	let signed_commitment = commitments::utils::create_signed_commitment(
		&request,
		harness.context.commit_config.clone(),
		harness.committer_one,
		&SIGNING_ID,
	)
	.await
	.unwrap();

	let constraint = commitments::utils::create_constraint_from_commitment_request(&request, slot).unwrap();
	harness
		.context
		.commitments_database
		.store_commitment_and_constraint(
			slot,
			&signed_commitment.commitment.request_hash,
			&signed_commitment,
			&constraint,
		)
		.unwrap();

	// Retrieve multiple times
	let retrieved1 = harness
		.context
		.commitments_database
		.get_commitment_by_hash(&signed_commitment.commitment.request_hash)
		.unwrap()
		.unwrap();
	let retrieved2 = harness
		.context
		.commitments_database
		.get_commitment_by_hash(&signed_commitment.commitment.request_hash)
		.unwrap()
		.unwrap();

	// Should get same data
	assert_eq!(retrieved1.commitment.request_hash, retrieved2.commitment.request_hash);
	assert_eq!(retrieved1.signature, retrieved2.signature);

	println!(" Retrieving commitment is idempotent");
}

// ===== NEGATIVE TESTS =====

#[tokio::test]
async fn test_retrieve_nonexistent_commitment() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Try to retrieve a commitment that doesn't exist
	let nonexistent_hash = B256::random();
	let result = harness.context.commitments_database.get_commitment_by_hash(&nonexistent_hash).unwrap();

	assert!(result.is_none());
	println!(" Correctly returns None for nonexistent commitment");
}

#[tokio::test]
async fn test_retrieve_with_zero_hash() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Try to retrieve with zero hash
	let zero_hash = B256::ZERO;
	let result = harness.context.commitments_database.get_commitment_by_hash(&zero_hash).unwrap();

	assert!(result.is_none());
	println!(" Correctly handles zero hash");
}

#[tokio::test]
async fn test_retrieve_after_multiple_stores() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Store multiple different commitments
	for _ in 0..5 {
		let signed_tx = harness.create_signed_tx();
		let request = harness.create_commitment_request(slot, signed_tx, Address::random()).unwrap();
		let signed_commitment = commitments::utils::create_signed_commitment(
			&request,
			harness.context.commit_config.clone(),
			harness.committer_one,
			&SIGNING_ID,
		)
		.await
		.unwrap();

		let constraint = commitments::utils::create_constraint_from_commitment_request(&request, slot).unwrap();
		harness
			.context
			.commitments_database
			.store_commitment_and_constraint(
				slot,
				&signed_commitment.commitment.request_hash,
				&signed_commitment,
				&constraint,
			)
			.unwrap();
	}

	// Store a specific commitment we'll retrieve
	let signed_tx = harness.create_signed_tx();
	let specific_slasher = Address::random();
	let request = harness.create_commitment_request(slot, signed_tx, specific_slasher).unwrap();
	let target_commitment = commitments::utils::create_signed_commitment(
		&request,
		harness.context.commit_config.clone(),
		harness.committer_one,
		&SIGNING_ID,
	)
	.await
	.unwrap();

	let constraint = commitments::utils::create_constraint_from_commitment_request(&request, slot).unwrap();
	harness
		.context
		.commitments_database
		.store_commitment_and_constraint(
			slot,
			&target_commitment.commitment.request_hash,
			&target_commitment,
			&constraint,
		)
		.unwrap();

	// Retrieve the specific commitment
	let retrieved = harness
		.context
		.commitments_database
		.get_commitment_by_hash(&target_commitment.commitment.request_hash)
		.unwrap();

	assert!(retrieved.is_some());
	let retrieved_commitment = retrieved.unwrap();
	assert_eq!(retrieved_commitment.commitment.slasher, specific_slasher);
	assert_eq!(retrieved_commitment.commitment.request_hash, target_commitment.commitment.request_hash);

	println!(" Can retrieve specific commitment among many");
}

#[tokio::test]
async fn test_different_requests_have_different_hashes() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// Create two different commitments
	let signed_tx1 = harness.create_signed_tx();
	let slasher1 = Address::random();
	let request1 = harness.create_commitment_request(slot, signed_tx1, slasher1).unwrap();
	let commitment1 = commitments::utils::create_signed_commitment(
		&request1,
		harness.context.commit_config.clone(),
		harness.committer_one,
		&SIGNING_ID,
	)
	.await
	.unwrap();

	let signed_tx2 = harness.create_signed_tx();
	let slasher2 = Address::random();
	let request2 = harness.create_commitment_request(slot, signed_tx2, slasher2).unwrap();
	let commitment2 = commitments::utils::create_signed_commitment(
		&request2,
		harness.context.commit_config.clone(),
		harness.committer_one,
		&SIGNING_ID,
	)
	.await
	.unwrap();

	// Different requests should have different hashes
	assert_ne!(commitment1.commitment.request_hash, commitment2.commitment.request_hash);

	println!(" Different commitment requests produce different hashes");
}
