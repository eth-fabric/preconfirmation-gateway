use alloy::primitives::Address;
use common::constants::{COMMITMENT_TYPE, CONSTRAINT_TYPE};
use integration_tests::test_common::TestHarness;

/// End-to-end integration tests with both commitments and relay services running
///
/// These tests verify the complete workflow and interaction between services
/// in realistic production scenarios.

// ===== FULL WORKFLOW TESTS =====

#[tokio::test]
async fn test_complete_preconfirmation_workflow() {
	// Launch both services
	let harness = TestHarness::builder()
		.with_commitments_port(None) // Launch commitments service with auto-assigned port
		.with_relay_port(None) // Launch relay service with auto-assigned port
		.build()
		.await
		.unwrap();

	let client = harness.create_client_harness();
	let slot = harness.context.slot_timer.get_current_slot() + 1;

	// === STEP 1: Post delegation to relay ===
	println!("Step 1: Posting delegation to relay...");
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();

	client.post_delegation(&signed_delegation).await.unwrap();
	println!("Delegation posted to relay");

	// === STEP 2: Gateway processes delegations ===
	println!("\nStep 2: Gateway processing delegations from relay...");
	let delegation_response = harness.process_delegations(slot).await.unwrap();

	assert!(delegation_response.success, "Delegation processing should succeed");
	assert_eq!(delegation_response.matching_delegations.len(), 1, "Should find 1 matching delegation");
	println!("Gateway processed delegations: found {} matching", delegation_response.matching_delegations.len());

	// Verify delegation is stored in local database
	let stored_delegation = harness.context.database.get_delegation_for_slot(slot).unwrap();
	assert!(stored_delegation.is_some(), "Delegation should be stored in local DB");
	println!("Delegation stored in local database");

	// === STEP 3: Make commitment request (writes constraint to local DB) ===
	println!("\nStep 3: Making commitment request...");
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let commitment_request = harness.create_commitment_request(slot, signed_tx.clone(), slasher).unwrap();

	let commitment_result = client.commitment_request(&commitment_request).await.unwrap();

	assert_eq!(commitment_result.commitment.commitment_type, COMMITMENT_TYPE);
	assert_eq!(commitment_result.commitment.slasher, slasher);
	assert!(commitment_result.nonce > 0);
	println!("Commitment created with hash: {:?}", commitment_result.commitment.request_hash);

	// Verify commitment is stored
	let stored_commitment =
		harness.context.database.get_commitment_by_hash(&commitment_result.commitment.request_hash).unwrap();
	assert!(stored_commitment.is_some(), "Commitment should be stored");
	println!("Commitment stored in database");

	// Verify constraint was written to local DB (this happens during commitment creation)
	let constraints_for_slot = harness.context.database.get_constraints_for_slot(slot).unwrap();
	assert!(!constraints_for_slot.is_empty(), "Constraints should be written to local DB");
	println!("Constraint written to local database: {} constraints", constraints_for_slot.len());

	// === STEP 4: Process constraints (posts to relay) ===
	println!("\nStep 4: Processing and posting constraints to relay...");
	let process_result = harness.process_constraints(slot, vec![]).await.unwrap();

	assert!(process_result.success, "Constraint processing should succeed");
	assert_eq!(process_result.processed_count, 1, "Should process 1 constraint");
	println!("Processed {} constraints and posted to relay", process_result.processed_count);

	// === STEP 5: Retrieve commitment result ===
	println!("\nStep 5: Retrieving commitment result...");
	let retrieved_commitment = client.commitment_result(&commitment_result.commitment.request_hash).await.unwrap();

	assert_eq!(retrieved_commitment.commitment.request_hash, commitment_result.commitment.request_hash);
	assert_eq!(retrieved_commitment.commitment.slasher, slasher);
	assert_eq!(retrieved_commitment.nonce, commitment_result.nonce);
	assert_eq!(retrieved_commitment.signature, commitment_result.signature);
	println!("Commitment result retrieved and verified");

	// === STEP 6: Retrieve constraints from relay ===
	println!("\nStep 6: Retrieving constraints from relay...");
	let headers = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_one.clone()).await;
	let relay_constraints = client.get_constraints(slot, headers).await.unwrap();

	assert_eq!(relay_constraints.len(), 1, "Should retrieve 1 signed constraint message");
	println!("Retrieved {} constraint messages from relay", relay_constraints.len());

	// === STEP 7: Verify all constraints are correct ===
	println!("\nStep 7: Verifying constraint correctness...");
	let signed_constraints = &relay_constraints[0];

	// Verify constraint message structure
	assert_eq!(signed_constraints.message.slot, slot);
	assert_eq!(signed_constraints.message.proposer, harness.proposer_bls_public_key);
	assert_eq!(signed_constraints.message.delegate, harness.gateway_bls_one);
	assert_eq!(signed_constraints.message.constraints.len(), 1);
	println!("Constraint message structure verified");

	// Verify constraint details
	let constraint = &signed_constraints.message.constraints[0];
	assert_eq!(constraint.constraint_type, CONSTRAINT_TYPE);
	assert!(!constraint.payload.is_empty());
	println!("Constraint type and payload verified");

	// Verify signature is present (not all zeros)
	let sig_bytes = signed_constraints.signature.serialize();
	assert!(!sig_bytes.iter().all(|&b| b == 0), "Signature should not be all zeros");
	assert!(signed_constraints.nonce > 0);
	println!("Constraint signature verified");

	// === FINAL VERIFICATION ===
	println!("\n=== Final Verification ===");
	println!("Full preconfirmation workflow completed successfully!");
	println!("   - Delegation posted and processed");
	println!("   - Commitment created and retrievable");
	println!("   - Constraints written, processed, and posted to relay");
	println!("   - All data verified across both services");
}
