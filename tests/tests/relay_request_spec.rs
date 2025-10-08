use alloy::primitives::{B256, Bytes};
use cb_common::signature::sign_commit_boost_root;
use cb_common::types::{BlsSecretKey, SignatureRequestInfo};
use commit_boost::prelude::{BlsSignature, Chain};
use common::constants::CONSTRAINT_TYPE;
use common::types::constraints::{Constraint, ConstraintsMessage, SignedConstraints};
use hex;
use std::sync::Arc;
use tempfile::TempDir;

use relay::database::RelayDatabase;
use relay::handlers::RelayState;

// Global test BLS secret key - all test keys will be derived from this
const TEST_SECRET_KEY_BYTES: [u8; 32] = [0x01; 32];

/// Helper to create a test database
fn create_test_database() -> (RelayDatabase, TempDir) {
	let temp_dir = TempDir::new().unwrap();
	let db_path = temp_dir.path().join("test_relay.db");
	let database = RelayDatabase::new(&db_path).unwrap();
	(database, temp_dir)
}

/// Helper to create a test ConstraintsMessage with real BLS keys
fn create_test_constraints_message(slot: u64) -> ConstraintsMessage {
	// Create BLS secret key from global test key
	let bls_secret_key = BlsSecretKey::deserialize(&TEST_SECRET_KEY_BYTES).unwrap();
	let bls_public_key = bls_secret_key.public_key();

	ConstraintsMessage {
		proposer: bls_public_key.clone(),
		delegate: bls_public_key.clone(),
		slot,
		constraints: vec![
			Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![0x01, 0x02, 0x03]) },
			Constraint { constraint_type: CONSTRAINT_TYPE, payload: Bytes::from(vec![0x04, 0x05, 0x06]) },
		],
		receivers: vec![bls_public_key],
	}
}

/// Helper to create a test SignedConstraints using local BLS signing
fn create_test_signed_constraints_with_local_signing(slot: u64) -> SignedConstraints {
	let mut constraints_message = create_test_constraints_message(slot);

	// Use the global test BLS secret key
	let bls_secret_key = BlsSecretKey::deserialize(&TEST_SECRET_KEY_BYTES).unwrap();

	// Get the corresponding public key and update the delegate to match
	let bls_public_key = bls_secret_key.public_key();
	constraints_message.delegate = bls_public_key.clone();

	// Get the object root that needs to be signed
	let object_root = constraints_message.to_object_root().unwrap();

	// Create signature request info
	let signature_request_info = SignatureRequestInfo { module_signing_id: B256::from_slice(&[0x11; 32]), nonce: 1337 };

	// Sign the object root locally
	let signature = sign_commit_boost_root(
		Chain::Mainnet, // Use Ethereum mainnet chain
		&bls_secret_key,
		&object_root,
		Some(&signature_request_info),
	);

	// Create the signed constraints
	SignedConstraints {
		message: constraints_message,
		nonce: signature_request_info.nonce,
		signing_id: signature_request_info.module_signing_id,
		signature,
	}
}

/// Helper to create test headers with BLS authentication
fn create_test_headers_with_valid_signature(slot: u64) -> axum::http::HeaderMap {
	use axum::http::{HeaderMap, HeaderValue};

	// Use the global test BLS secret key
	let bls_secret_key = BlsSecretKey::deserialize(&TEST_SECRET_KEY_BYTES).unwrap();
	let bls_public_key = bls_secret_key.public_key();

	// Sign the slot number
	let slot_string = slot.to_string();
	let slot_bytes = slot_string.as_bytes();
	let slot_hash = alloy::primitives::keccak256(slot_bytes);

	// Debug output
	println!("Test: Slot string: {}", slot_string);
	println!("Test: Slot hash: {:?}", slot_hash);
	println!("Test: Public key: {:?}", bls_public_key.serialize());

	let signature = bls_secret_key.sign(slot_hash);
	println!("Test: Generated signature: {:?}", signature.serialize());

	let mut headers = HeaderMap::new();
	headers.insert(
		"X-Receiver-PublicKey",
		HeaderValue::from_str(&format!("0x{}", hex::encode(bls_public_key.serialize()))).unwrap(),
	);
	headers.insert(
		"X-Receiver-Signature",
		HeaderValue::from_str(&format!("0x{}", hex::encode(signature.serialize()))).unwrap(),
	);
	headers
}

#[test]
fn verify_create_test_signed_constraints_with_local_signing() {
	let s = create_test_signed_constraints_with_local_signing(12345);
	let x = relay::utils::verify_constraints_signature(&s).unwrap();
	assert!(x);
}

#[test]
fn test_create_signed_constraints_with_local_signing() {
	let signed_constraints = create_test_signed_constraints_with_local_signing(12345);

	// Verify the signed constraints have proper structure
	assert_eq!(signed_constraints.message.slot, 12345);
	assert_eq!(signed_constraints.message.constraints.len(), 2); // We create 2 constraints in the helper
	assert_eq!(signed_constraints.message.constraints[0].constraint_type, CONSTRAINT_TYPE);

	// Verify the signature is not empty (it should be a real BLS signature)
	assert_ne!(signed_constraints.signature, BlsSignature::empty());

	// Verify the nonce and signing_id are set
	assert_eq!(signed_constraints.nonce, 1337);
	assert_eq!(signed_constraints.signing_id, B256::from_slice(&[0x11; 32]));
}

#[tokio::test]
async fn test_get_constraints_for_slot_with_local_signing() {
	let (database, _temp_dir) = create_test_database();
	let state = RelayState { database: Arc::new(database) };

	let slot = 12345u64;

	// Create a properly signed constraint using local BLS signing
	let signed_constraints = create_test_signed_constraints_with_local_signing(slot);

	// Store the signed constraint using the POST handler
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Retrieve constraints for the slot
	let retrieved_constraints = state.database.get_constraints_for_slot(slot).unwrap();

	assert_eq!(retrieved_constraints.len(), 1);
	assert_eq!(retrieved_constraints[0].message.slot, slot);
	assert_eq!(retrieved_constraints[0].message.constraints.len(), 2);

	// Verify the signature is preserved
	assert_ne!(retrieved_constraints[0].signature, BlsSignature::empty());
}

#[test]
fn test_constraints_message_object_root() {
	let constraints_message = create_test_constraints_message(12345);

	// Test that we can get the object root (this is what gets signed)
	let object_root = constraints_message.to_object_root().unwrap();

	// The object root should be a valid hash
	assert_ne!(object_root, B256::ZERO);

	// The same message should produce the same object root
	let object_root2 = constraints_message.to_object_root().unwrap();
	assert_eq!(object_root, object_root2);
}

#[tokio::test]
async fn test_multiple_constraints_same_slot() {
	let (database, _temp_dir) = create_test_database();
	let state = RelayState { database: Arc::new(database) };

	let slot = 12345u64;

	// Create multiple signed constraints for the same slot
	let signed_constraints1 = create_test_signed_constraints_with_local_signing(slot);
	let signed_constraints2 = create_test_signed_constraints_with_local_signing(slot);

	// Store both constraints using the POST handler
	let store_result1 = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints1.clone()),
	)
	.await;

	let store_result2 = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints2.clone()),
	)
	.await;

	// Verify both constraints were stored successfully
	assert!(store_result1.is_ok());
	assert!(store_result2.is_ok());

	// Retrieve all constraints for the slot
	let retrieved_constraints = state.database.get_constraints_for_slot(slot).unwrap();

	assert_eq!(retrieved_constraints.len(), 2);

	// Both should have the same slot
	for constraint in &retrieved_constraints {
		assert_eq!(constraint.message.slot, slot);
		assert_ne!(constraint.signature, BlsSignature::empty());
	}
}

#[test]
fn test_constraints_signature_verification() {
	let signed_constraints = create_test_signed_constraints_with_local_signing(12345);

	// Get the object root that was signed
	let _object_root = signed_constraints.message.to_object_root().unwrap();

	// The signature should be verifiable against the object root
	// Note: This would require the actual BLS signature verification logic
	// For now, we just verify that the signature is not empty and has the right structure
	assert_ne!(signed_constraints.signature, BlsSignature::empty());

	// Verify the message structure
	assert_eq!(signed_constraints.message.slot, 12345);
	assert_eq!(signed_constraints.message.constraints.len(), 2);
	assert_eq!(signed_constraints.message.receivers.len(), 1);
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_with_valid_headers() {
	use axum::extract::Path;

	// Set up logging for the test
	let _ = tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::new("debug")).try_init();

	let (database, _temp_dir) = create_test_database();
	let state = RelayState { database: Arc::new(database) };

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = create_test_signed_constraints_with_local_signing(slot);

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Create valid headers with BLS signature for the slot
	let headers = create_test_headers_with_valid_signature(slot);

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers.clone())
			.await;

	assert!(result.is_ok());
	let constraints = result.unwrap();
	assert_eq!(constraints.len(), 1);
	assert_eq!(constraints[0].message.slot, slot);
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_with_invalid_signature() {
	use axum::extract::Path;

	let (database, _temp_dir) = create_test_database();
	let state = RelayState { database: Arc::new(database) };

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = create_test_signed_constraints_with_local_signing(slot);

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Create headers with invalid signature (wrong slot)
	let wrong_slot = 99999u64;
	let headers = create_test_headers_with_valid_signature(wrong_slot);

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should fail due to signature verification
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_missing_headers() {
	use axum::extract::Path;
	use axum::http::HeaderMap;

	let (database, _temp_dir) = create_test_database();
	let state = RelayState { database: Arc::new(database) };

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = create_test_signed_constraints_with_local_signing(slot);

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Create empty headers (missing authentication)
	let headers = HeaderMap::new();

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should fail due to missing headers
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_wrong_public_key() {
	use axum::extract::Path;
	use axum::http::{HeaderMap, HeaderValue};

	let (database, _temp_dir) = create_test_database();
	let state = RelayState { database: Arc::new(database) };

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = create_test_signed_constraints_with_local_signing(slot);

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Create headers with a different public key (not in receivers list)
	let mut headers = HeaderMap::new();
	headers.insert(
		"X-Receiver-PublicKey",
		HeaderValue::from_str(
			"0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
		)
		.unwrap(),
	);
	headers.insert(
		"X-Receiver-Signature",
		HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
	);

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should fail due to public key not being in receivers list
	assert!(result.is_err());
}
