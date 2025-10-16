use alloy::primitives::Bytes;
use common::constants::CONSTRAINT_TYPE;
use common::types::constraints::{Constraint, ConstraintsMessage, SignedConstraints};
use integration_tests::test_common::TestHarness;

/// Tests for relay handlers (delegations and constraints)
/// These tests do NOT launch services - they test handler logic directly
/// Data is written directly to databases to isolate the behavior under test
///
/// The test pattern is:
/// 1. Setup: Create test data (delegations, constraints)
/// 2. Act: Store/retrieve via database or call handlers directly
/// 3. Assert: Verify expected behavior

// ===== DELEGATION TESTS =====

#[tokio::test]
async fn test_store_and_retrieve_delegation() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();

	// Store in database
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Retrieve
	let retrieved = harness.context.database.get_delegation_for_slot(slot).unwrap();

	assert!(retrieved.is_some());
	let retrieved_delegation = retrieved.unwrap();
	assert_eq!(retrieved_delegation.message.slot, slot);
	assert_eq!(retrieved_delegation.message.delegate, harness.gateway_bls_one);
	assert_eq!(retrieved_delegation.message.committer, harness.committer_one);

	println!("✅ Delegation stored and retrieved successfully");
}

#[tokio::test]
async fn test_delegation_signature_valid() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create and sign delegation
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();

	// Verify the BLS signature is not empty
	assert_ne!(signed_delegation.signature, commit_boost::prelude::BlsSignature::empty());

	// Verify signature can be validated
	let chain = commit_boost::prelude::Chain::Hoodi;
	let is_valid = relay::utils::verify_delegation_signature(&signed_delegation, chain).unwrap();
	assert!(is_valid);

	println!("✅ Delegation signature is valid");
}

#[tokio::test]
async fn test_multiple_delegations_different_slots() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Create and store delegations for different slots
	for i in 0..3 {
		let slot = 12345 + i;
		let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
		let signed_delegation =
			harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
		harness.context.database.store_delegation(slot, &signed_delegation).unwrap();
	}

	// Retrieve each delegation
	for i in 0..3 {
		let slot = 12345 + i;
		let retrieved = harness.context.database.get_delegation_for_slot(slot).unwrap();
		assert!(retrieved.is_some());
		assert_eq!(retrieved.unwrap().message.slot, slot);
	}

	println!("✅ Multiple delegations stored and retrieved correctly");
}

#[tokio::test]
async fn test_delegation_with_different_delegates() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Create delegation for gateway_one
	let slot1 = 12345;
	let delegation1 = harness.create_delegation(slot1, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation1 =
		harness.create_signed_delegation(&delegation1, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot1, &signed_delegation1).unwrap();

	// Create delegation for gateway_two
	let slot2 = 12346;
	let delegation2 = harness.create_delegation(slot2, harness.gateway_bls_two.clone(), harness.committer_two);
	let signed_delegation2 =
		harness.create_signed_delegation(&delegation2, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot2, &signed_delegation2).unwrap();

	// Retrieve and verify
	let retrieved1 = harness.context.database.get_delegation_for_slot(slot1).unwrap().unwrap();
	let retrieved2 = harness.context.database.get_delegation_for_slot(slot2).unwrap().unwrap();

	assert_eq!(retrieved1.message.delegate, harness.gateway_bls_one);
	assert_eq!(retrieved2.message.delegate, harness.gateway_bls_two);
	assert_eq!(retrieved1.message.committer, harness.committer_one);
	assert_eq!(retrieved2.message.committer, harness.committer_two);

	println!("✅ Delegations with different delegates handled correctly");
}

#[tokio::test]
async fn test_retrieve_nonexistent_delegation() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Try to retrieve delegation for slot that doesn't exist
	let nonexistent_slot = 99999;
	let result = harness.context.database.get_delegation_for_slot(nonexistent_slot).unwrap();

	assert!(result.is_none());
	println!("✅ Correctly returns None for nonexistent delegation");
}

// ===== CONSTRAINTS TESTS =====

#[tokio::test]
async fn test_create_signed_constraints() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create constraint
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3, 4, 5]) };

	// Create constraints message
	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint.clone()],
		receivers: vec![], // Empty receivers = public
	};

	// Sign constraints using BLS signer
	let message_hash = constraints_message.to_message_hash().unwrap();
	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	// Verify structure
	assert_eq!(signed_constraints.message.slot, slot);
	assert_eq!(signed_constraints.message.constraints.len(), 1);
	assert_eq!(signed_constraints.message.constraints[0].constraint_type, CONSTRAINT_TYPE);
	assert_ne!(signed_constraints.signature, commit_boost::prelude::BlsSignature::empty());

	println!("✅ Signed constraints created successfully");
}

#[tokio::test]
async fn test_constraints_signature_valid() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create and sign constraints
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint],
		receivers: vec![],
	};
	let message_hash = constraints_message.to_message_hash().unwrap();

	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	// Verify signature
	let chain = commit_boost::prelude::Chain::Hoodi;
	let is_valid = relay::utils::verify_constraints_signature(&signed_constraints, chain).unwrap();
	assert!(is_valid);

	println!("✅ Constraints signature is valid");
}

#[tokio::test]
async fn test_store_and_retrieve_constraints() {
	let harness = TestHarness::builder().build().await.unwrap();
	let state = harness.create_relay_state();
	let slot = 12345;

	// Create constraints
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3, 4]) };
	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint],
		receivers: vec![],
	};
	let message_hash = constraints_message.to_message_hash().unwrap();

	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	// Store constraints
	state.database.store_signed_constraints(&signed_constraints).unwrap();

	// Retrieve constraints
	let retrieved = state.database.get_signed_constraints_for_slot(slot).unwrap();

	assert_eq!(retrieved.len(), 1);
	assert_eq!(retrieved[0].message.slot, slot);
	assert_eq!(retrieved[0].message.constraints.len(), 1);

	println!("✅ Constraints stored and retrieved successfully");
}

#[tokio::test]
async fn test_multiple_constraints_same_slot() {
	let harness = TestHarness::builder().build().await.unwrap();
	let state = harness.create_relay_state();
	let slot = 12345;

	// Store multiple different constraints for same slot
	for i in 0..3 {
		let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![i as u8; 5]) };
		let constraints_message = ConstraintsMessage {
			proposer: harness.proposer_bls_public_key.clone(),
			delegate: harness.gateway_bls_one.clone(),
			slot,
			constraints: vec![constraint],
			receivers: vec![],
		};
		let message_hash = constraints_message.to_message_hash().unwrap();

		let response = {
			let mut config = harness.context.commit_config.lock().await;
			common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
				.await
				.unwrap()
		};

		let signed_constraints = SignedConstraints {
			message: constraints_message,
			nonce: response.nonce,
			signing_id: response.module_signing_id,
			signature: response.signature,
		};

		state.database.store_signed_constraints(&signed_constraints).unwrap();
	}

	// Retrieve all constraints for slot
	let retrieved = state.database.get_signed_constraints_for_slot(slot).unwrap();

	assert_eq!(retrieved.len(), 3);
	println!("✅ Multiple constraints for same slot handled correctly");
}

#[tokio::test]
async fn test_constraints_with_multiple_constraint_items() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create constraints message with multiple constraints
	let constraint1 = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let constraint2 = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![4, 5, 6]) };
	let constraint3 = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![7, 8, 9]) };

	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint1, constraint2, constraint3],
		receivers: vec![],
	};

	let message_hash = constraints_message.to_message_hash().unwrap();

	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	assert_eq!(signed_constraints.message.constraints.len(), 3);
	println!("✅ Constraints with multiple items created successfully");
}

#[tokio::test]
async fn test_public_vs_private_constraints() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create public constraints (empty receivers)
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let public_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint.clone()],
		receivers: vec![], // Empty = public
	};

	// Create private constraints (specific receivers)
	let private_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint],
		receivers: vec![harness.gateway_bls_one.clone()], // Specific receiver = private
	};

	// Both should be signable
	let public_hash = public_message.to_message_hash().unwrap();
	let private_hash = private_message.to_message_hash().unwrap();

	// Hashes should be different
	assert_ne!(public_hash, private_hash);

	println!("✅ Public and private constraints produce different hashes");
}

#[tokio::test]
async fn test_retrieve_constraints_for_nonexistent_slot() {
	let harness = TestHarness::builder().build().await.unwrap();
	let state = harness.create_relay_state();

	// Try to retrieve constraints for slot that doesn't exist
	let nonexistent_slot = 99999;
	let result = state.database.get_signed_constraints_for_slot(nonexistent_slot).unwrap();

	assert!(result.is_empty());
	println!("✅ Correctly returns empty for nonexistent constraints");
}

// ===== AUTH HEADER TESTS =====

#[tokio::test]
async fn test_create_auth_headers() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create auth headers with BLS signature
	let headers = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_one.clone()).await;

	// Verify headers are present
	assert!(headers.contains_key("x-receiver-publickey"));
	assert!(headers.contains_key("x-receiver-signature"));
	assert!(headers.contains_key("x-receiver-nonce"));
	assert!(headers.contains_key("x-receiver-signingid"));

	println!("✅ Auth headers created successfully");
}

#[tokio::test]
async fn test_headers_with_different_signers() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create headers with gateway_one
	let headers1 = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_one.clone()).await;

	// Create headers with gateway_two
	let headers2 = harness.create_headers_with_valid_signature(slot, harness.gateway_bls_two.clone()).await;

	// Signatures should be different
	let sig1 = headers1.get("x-receiver-signature").unwrap();
	let sig2 = headers2.get("x-receiver-signature").unwrap();
	assert_ne!(sig1, sig2);

	println!("✅ Different signers produce different auth headers");
}

// ===== INTEGRATION BETWEEN DELEGATIONS AND CONSTRAINTS =====

#[tokio::test]
async fn test_delegation_required_for_constraints() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Store delegation first
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Now create constraints
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint],
		receivers: vec![],
	};
	let message_hash = constraints_message.to_message_hash().unwrap();

	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	// Both delegation and constraints exist for the slot
	let delegation_exists = harness.context.database.get_delegation_for_slot(slot).unwrap().is_some();
	assert!(delegation_exists);

	// Can verify constraints signature
	let chain = commit_boost::prelude::Chain::Hoodi;
	let is_valid = relay::utils::verify_constraints_signature(&signed_constraints, chain).unwrap();
	assert!(is_valid);

	println!("✅ Delegation and constraints workflow works correctly");
}

#[tokio::test]
async fn test_constraint_without_delegation() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 99999;

	// No delegation stored for this slot
	let delegation = harness.context.database.get_delegation_for_slot(slot).unwrap();
	assert!(delegation.is_none());

	// Can still create constraints (validation happens in handler)
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![1, 2, 3]) };
	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint],
		receivers: vec![],
	};
	let message_hash = constraints_message.to_message_hash().unwrap();

	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	// Constraints can be created, but handler would reject without delegation
	assert_ne!(signed_constraints.signature, commit_boost::prelude::BlsSignature::empty());
	println!("✅ Constraints can be created without delegation (handler enforcement needed)");
}

#[tokio::test]
async fn test_delegation_fields_preserved() {
	let harness = TestHarness::builder().build().await.unwrap();
	let slot = 12345;

	// Create delegation with specific values
	let delegation = harness.create_delegation(slot, harness.gateway_bls_one.clone(), harness.committer_one);
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await.unwrap();

	// Store
	harness.context.database.store_delegation(slot, &signed_delegation).unwrap();

	// Retrieve and verify all fields
	let retrieved = harness.context.database.get_delegation_for_slot(slot).unwrap().unwrap();

	assert_eq!(retrieved.message.proposer, harness.proposer_bls_public_key);
	assert_eq!(retrieved.message.delegate, harness.gateway_bls_one);
	assert_eq!(retrieved.message.committer, harness.committer_one);
	assert_eq!(retrieved.message.slot, slot);
	assert_eq!(retrieved.nonce, signed_delegation.nonce);
	assert_eq!(retrieved.signing_id, signed_delegation.signing_id);
	assert_eq!(retrieved.signature, signed_delegation.signature);

	println!("✅ All delegation fields preserved after storage");
}

#[tokio::test]
async fn test_constraints_fields_preserved() {
	let harness = TestHarness::builder().build().await.unwrap();
	let state = harness.create_relay_state();
	let slot = 12345;

	// Create constraints
	let payload = Bytes::from(vec![1, 2, 3, 4, 5]);
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: payload.clone() };
	let constraints_message = ConstraintsMessage {
		proposer: harness.proposer_bls_public_key.clone(),
		delegate: harness.gateway_bls_one.clone(),
		slot,
		constraints: vec![constraint],
		receivers: vec![],
	};
	let message_hash = constraints_message.to_message_hash().unwrap();

	let response = {
		let mut config = harness.context.commit_config.lock().await;
		common::signer::call_proxy_bls_signer(&mut *config, message_hash, harness.gateway_bls_one.clone())
			.await
			.unwrap()
	};

	let signed_constraints = SignedConstraints {
		message: constraints_message,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	// Store
	state.database.store_signed_constraints(&signed_constraints).unwrap();

	// Retrieve and verify
	let retrieved = state.database.get_signed_constraints_for_slot(slot).unwrap();
	assert_eq!(retrieved.len(), 1);

	let retrieved_constraints = &retrieved[0];
	assert_eq!(retrieved_constraints.message.slot, slot);
	assert_eq!(retrieved_constraints.message.constraints.len(), 1);
	assert_eq!(retrieved_constraints.message.constraints[0].constraint_type, CONSTRAINT_TYPE);
	assert_eq!(retrieved_constraints.message.constraints[0].payload, payload);
	assert_eq!(retrieved_constraints.message.receivers.len(), 0); // Empty receivers = public
	assert_eq!(retrieved_constraints.nonce, signed_constraints.nonce);
	assert_eq!(retrieved_constraints.signing_id, signed_constraints.signing_id);
	assert_eq!(retrieved_constraints.signature, signed_constraints.signature);

	println!("✅ All constraints fields preserved after storage");
}
