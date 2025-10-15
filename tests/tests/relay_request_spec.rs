use alloy::primitives::{B256, Bytes};
use axum::http::{HeaderMap, HeaderValue};
use commit_boost::prelude::{BlsSignature, Chain};
use common::constants::CONSTRAINT_TYPE;
use common::types::constraints::{Constraint, ConstraintsMessage, SignedConstraints};
use common::types::database::DatabaseContext;
use constraints::create_signed_constraints;
use hex;
use integration_tests::test_common::start_local_signer_server;
use rand::Rng;
use relay::handlers::RelayState;
use std::sync::Arc;
use tempfile::TempDir;

/// Test harness for relay request testing
/// This provides a clean interface for testing different relay request scenarios
struct RelayRequestTestHarness {
	context: common::types::RpcContext,
	proposer_bls_public_key: commit_boost::prelude::BlsPublicKey,
	gateway_bls_public_key: commit_boost::prelude::BlsPublicKey,
	_temp_dir: TempDir,
}

impl RelayRequestTestHarness {
	/// Creates a new test harness with a properly configured context
	async fn new() -> eyre::Result<Self> {
		// Create test database
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_relay.db");
		let mut opts = rocksdb::Options::default();
		opts.create_if_missing(true);
		let db = Arc::new(rocksdb::DB::open(&opts, &db_path).unwrap());
		let database = DatabaseContext::new(db);

		// Use the same BLS public key as the signer service is configured with
		let proposer_bls_public_key =
			cb_common::types::BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY).unwrap();

		// Start local signer server for testing
		let commit_config = start_local_signer_server(
			"test_relay_module",
			RelayRequestTestHarness::signing_id(),
			"test_secret",
			rand::thread_rng().gen_range(3000..65535),
		)
		.await
		.expect("Failed to start local signer server");
		assert_eq!(commit_config.chain, RelayRequestTestHarness::chain());

		// Create pricing database
		let pricing_db_path = temp_dir.path().join("pricing.db");
		let mut pricing_opts = rocksdb::Options::default();
		pricing_opts.create_if_missing(true);
		let pricing_db = Arc::new(rocksdb::DB::open(&pricing_opts, &pricing_db_path).unwrap());
		let pricing_database = DatabaseContext::new(pricing_db);

		// Create context with the database
		let context = common::types::RpcContext {
			database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			pricing_database,
			bls_public_key: proposer_bls_public_key.clone(),
			relay_url: "http://localhost:8080".to_string(),
			api_key: Some("test-api-key".to_string()),
			slot_timer: common::slot_timer::SlotTimer::new(12),
		};

		// Generate proxy BLS key for gateway using the signer service
		let gateway_bls_public_key = {
			let mut commit_config_guard = context.commit_config.lock().await;
			commit_config_guard
				.signer_client
				.generate_proxy_key_bls(proposer_bls_public_key.clone())
				.await
				.map_err(|e| eyre::eyre!("Failed to generate proxy BLS key: {}", e))?
				.message
				.proxy
		};

		Ok(Self { context, proposer_bls_public_key, gateway_bls_public_key, _temp_dir: temp_dir })
	}

	fn signing_id() -> B256 {
		B256::from_slice(&[0x11; 32])
	}

	fn chain() -> Chain {
		Chain::Hoodi
	}

	/// Creates a test ConstraintsMessage with the harness's configured keys
	fn create_test_constraints_message(
		&self,
		slot: u64,
		num_constraints: usize,
		public_receivers: bool,
	) -> ConstraintsMessage {
		let receivers = if public_receivers { vec![] } else { vec![self.gateway_bls_public_key.clone()] };
		ConstraintsMessage {
			proposer: self.proposer_bls_public_key.clone(),
			delegate: self.gateway_bls_public_key.clone(),
			slot,
			constraints: vec![
				Constraint {
					constraint_type: CONSTRAINT_TYPE,
					payload: Bytes::from(vec![0x01, 0x02, 0x03])
				};
				num_constraints
			],
			receivers,
		}
	}

	/// Creates a test SignedConstraints using the proxy BLS signer
	async fn create_test_signed_constraints(
		&self,
		slot: u64,
		num_constraints: usize,
		public_receivers: bool,
	) -> SignedConstraints {
		let constraints_message = self.create_test_constraints_message(slot, num_constraints, public_receivers);

		// Create the signed constraints
		let signed_constraints = create_signed_constraints(
			&constraints_message,
			self.context.commit_config.clone(),
			self.gateway_bls_public_key.clone(),
		)
		.await
		.unwrap();

		signed_constraints
	}

	/// Creates test headers with BLS authentication using the proxy signer
	async fn create_test_headers_with_valid_signature(&self, slot: u64) -> axum::http::HeaderMap {
		// Get the commit config from context
		let mut commit_config = self.context.commit_config.lock().await;

		// Compute slot hash for signing
		let slot_hash = alloy::primitives::keccak256(slot.to_string().as_bytes());

		// Call the proxy BLS signer to sign the slot hash
		let bls_response =
			common::signer::call_proxy_bls_signer(&mut *commit_config, slot_hash, self.gateway_bls_public_key.clone())
				.await
				.expect("Failed to sign slot hash with proxy BLS signer");

		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(&format!("0x{}", hex::encode(self.gateway_bls_public_key.serialize()))).unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str(&format!("0x{}", hex::encode(bls_response.signature.serialize()))).unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str(&bls_response.nonce.to_string()).unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str(&format!("0x{}", hex::encode(bls_response.module_signing_id.as_slice()))).unwrap(),
		);
		headers
	}

	/// Creates a RelayState for testing
	fn create_relay_state(&self) -> RelayState {
		RelayState { database: Arc::new(self.context.database.clone()), config: relay::config::RelayConfig::default() }
	}
}

#[tokio::test]
async fn test_create_signed_constraints() {
	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let signed_constraints = harness.create_test_signed_constraints(12345, 2, true).await;

	// Verify the signed constraints have proper structure
	assert_eq!(signed_constraints.message.slot, 12345);
	assert_eq!(signed_constraints.message.constraints.len(), 2); // We create 2 constraints in the helper
	assert_eq!(signed_constraints.message.constraints[0].constraint_type, CONSTRAINT_TYPE);

	// Verify the signature is not empty (it should be a real BLS signature)
	assert_ne!(signed_constraints.signature, BlsSignature::empty());

	// Verify the nonce and signing_id are set
	assert_eq!(signed_constraints.signing_id, RelayRequestTestHarness::signing_id());

	// Verify signature is valid
	assert!(relay::utils::verify_constraints_signature(&signed_constraints, RelayRequestTestHarness::chain()).unwrap());
}

#[tokio::test]
async fn test_store_constraints_handler() {
	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create a properly signed constraint using local BLS signing
	let signed_constraints = harness.create_test_signed_constraints(slot, 2, true).await;

	// Store the signed constraint using the POST handler
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Retrieve constraints for the slot
	let retrieved_constraints = state.database.get_signed_constraints_for_slot(slot).unwrap();

	assert_eq!(retrieved_constraints.len(), 1);
	assert_eq!(retrieved_constraints[0].message.slot, slot);
	assert_eq!(retrieved_constraints[0].message.constraints.len(), 2);

	// Verify the signature is preserved
	assert_ne!(retrieved_constraints[0].signature, BlsSignature::empty());
}

#[tokio::test]
async fn test_store_constraints_handler_multiple_constraints_same_slot() {
	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create multiple signed constraints for the same slot
	let signed_constraints1 = harness.create_test_signed_constraints(slot, 1, true).await;
	let signed_constraints2 = harness.create_test_signed_constraints(slot, 2, true).await;

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
	let retrieved_constraints = state.database.get_signed_constraints_for_slot(slot).unwrap();

	assert_eq!(retrieved_constraints.len(), 2);

	// Both should have the same slot
	for constraint in &retrieved_constraints {
		assert_eq!(constraint.message.slot, slot);
		assert_ne!(constraint.signature, BlsSignature::empty());
	}
}

#[tokio::test]
async fn test_verify_constraints_signature_comprehensive() {
	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");

	// Test 1: Valid signature should pass verification
	let signed_constraints = harness.create_test_signed_constraints(12345, 2, true).await;
	let verification_result =
		relay::utils::verify_constraints_signature(&signed_constraints, RelayRequestTestHarness::chain());
	assert!(verification_result.is_ok());
	assert!(verification_result.unwrap());

	// Test 2: Test with different slot numbers
	let signed_constraints_slot_100 = harness.create_test_signed_constraints(100, 1, true).await;
	let verification_result =
		relay::utils::verify_constraints_signature(&signed_constraints_slot_100, RelayRequestTestHarness::chain());
	assert!(verification_result.is_ok());
	assert!(verification_result.unwrap());

	// Test 3: Test with different number of constraints
	let signed_constraints_many = harness.create_test_signed_constraints(200, 5, true).await;
	let verification_result =
		relay::utils::verify_constraints_signature(&signed_constraints_many, RelayRequestTestHarness::chain());
	assert!(verification_result.is_ok());
	assert!(verification_result.unwrap());

	// Test 4: Test with empty constraints (should still work if message is valid)
	let signed_constraints_empty = harness.create_test_signed_constraints(300, 0, true).await;
	let verification_result =
		relay::utils::verify_constraints_signature(&signed_constraints_empty, RelayRequestTestHarness::chain());
	assert!(verification_result.is_ok());
	assert!(verification_result.unwrap());
}

#[tokio::test]
async fn test_verify_constraints_signature_edge_cases() {
	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");

	// Test 1: Very large slot number
	let signed_constraints_large_slot = harness.create_test_signed_constraints(u64::MAX, 1, true).await;
	let verification_result =
		relay::utils::verify_constraints_signature(&signed_constraints_large_slot, RelayRequestTestHarness::chain());
	assert!(verification_result.is_ok());
	assert!(verification_result.unwrap());

	// Test 2: Slot 0
	let signed_constraints_slot_zero = harness.create_test_signed_constraints(0, 1, true).await;
	let verification_result =
		relay::utils::verify_constraints_signature(&signed_constraints_slot_zero, RelayRequestTestHarness::chain());
	assert!(verification_result.is_ok());
	assert!(verification_result.unwrap());
}

#[tokio::test]
async fn test_get_constraints_handler_with_receivers() {
	use axum::extract::Path;

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create a test SignedConstraints
	let signed_constraints = harness.create_test_signed_constraints(slot, 1, false).await;

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Create valid headers with BLS signature for the slot using the proxy signer
	let headers = harness.create_test_headers_with_valid_signature(slot).await;

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

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = harness.create_test_signed_constraints(slot, 1, true).await;

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
	let headers = harness.create_test_headers_with_valid_signature(wrong_slot).await;

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

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = harness.create_test_signed_constraints(slot, 1, false).await;

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

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create and store a signed constraint using the POST handler
	let signed_constraints = harness.create_test_signed_constraints(slot, 1, false).await;

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
	headers.insert("X-Receiver-Nonce", HeaderValue::from_str("1337").unwrap());
	headers.insert(
		"X-Receiver-SigningId",
		HeaderValue::from_str("0x0101010101010101010101010101010101010101010101010101010101010101").unwrap(),
	);

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should fail due to public key not being in receivers list
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_mixed_access_scenarios() {
	use axum::extract::Path;

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create and store multiple constraints with different access patterns

	// 1. Public constraint (empty receivers)
	let mut public_constraints = harness.create_test_signed_constraints(slot, 1, true).await;
	public_constraints.message.receivers = vec![]; // Empty receivers = public access
	public_constraints.signing_id = B256::from_slice(&[0x11; 32]); // Unique signing ID

	let store_public = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(public_constraints.clone()),
	)
	.await;
	assert!(store_public.is_ok());

	// 2. Restricted constraint (specific receivers)
	let restricted_constraints = harness.create_test_signed_constraints(slot, 2, false).await;

	let store_restricted = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(restricted_constraints.clone()),
	)
	.await;
	assert!(store_restricted.is_ok());

	// Create headers with a valid BLS signature for the slot using the proxy signer
	let headers = harness.create_test_headers_with_valid_signature(slot).await;

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should succeed and return both constraints:
	// - Public constraint (empty receivers) - accessible to anyone
	// - Restricted constraint (specific receivers) - accessible because caller's key is in receivers
	assert!(result.is_ok());
	let constraints = result.unwrap();
	assert_eq!(constraints.len(), 2);

	// Find the public constraint (empty receivers)
	let public_constraint = constraints.iter().find(|c| c.message.receivers.is_empty()).unwrap();
	assert_eq!(public_constraint.message.receivers.len(), 0);

	// Find the restricted constraint (specific receivers)
	let restricted_constraint = constraints.iter().find(|c| !c.message.receivers.is_empty()).unwrap();
	assert_eq!(restricted_constraint.message.receivers.len(), 1);

	// The gateway public key should be in the receivers list
	assert_eq!(restricted_constraint.message.receivers[0], harness.gateway_bls_public_key);
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_unauthorized_access() {
	use axum::extract::Path;

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create and store a signed constraint with specific receivers (restricted access)
	let signed_constraints = harness.create_test_signed_constraints(slot, 1, false).await;
	// The default test constraints already have a specific receiver list

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
			"0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba",
		)
		.unwrap(),
	);
	headers.insert(
		"X-Receiver-Signature",
		HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
	);
	headers.insert("X-Receiver-Nonce", HeaderValue::from_str("9999").unwrap());
	headers.insert(
		"X-Receiver-SigningId",
		HeaderValue::from_str("0x9999999999999999999999999999999999999999999999999999999999999999").unwrap(),
	);

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should because the signature is invalid
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_public_access_empty_receivers() {
	use axum::extract::Path;

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12345u64;

	// Create and store a signed constraint with empty receivers (public access)
	let mut signed_constraints = harness.create_test_signed_constraints(slot, 1, true).await;
	signed_constraints.message.receivers = vec![]; // Empty receivers = public access

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	let _stored = store_result.unwrap();

	// Create headers with a valid BLS signature for the slot using the proxy signer
	let headers = harness.create_test_headers_with_valid_signature(slot).await;

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should succeed and return the public constraint
	assert!(result.is_ok());
	let constraints = result.unwrap();
	assert_eq!(constraints.len(), 1);
	assert_eq!(constraints[0].message.receivers.len(), 0); // Empty receivers = public
}

#[tokio::test]
async fn test_get_constraints_for_slot_handler_restricted_access_with_receivers() {
	use axum::extract::Path;

	let harness = RelayRequestTestHarness::new().await.expect("Failed to create test harness");
	let state = harness.create_relay_state();

	let slot = 12346u64;

	// Create and store a signed constraint with specific receivers (restricted access)
	let signed_constraints = harness.create_test_signed_constraints(slot, 1, false).await;
	// The default test constraints already have a specific receiver list

	// Use the store_constraints_handler to store the constraint
	let store_result = relay::handlers::store_constraints_handler(
		axum::extract::State(state.clone()),
		axum::Json(signed_constraints.clone()),
	)
	.await;

	// Verify the constraint was stored successfully
	assert!(store_result.is_ok());

	// Create headers with a valid BLS signature for the slot using the proxy signer
	let headers = harness.create_test_headers_with_valid_signature(slot).await;

	// Call the handler
	let result =
		relay::handlers::get_constraints_for_slot_handler(axum::extract::State(state), Path(slot), headers).await;

	// Should succeed and return the constraint because the caller's key is in the receivers list
	assert!(result.is_ok());
	let constraints = result.unwrap();
	assert_eq!(constraints.len(), 1);
	assert!(constraints[0].message.receivers.len() > 0); // Has specific receivers
}
