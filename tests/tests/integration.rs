use alloy::primitives::B256;
use common::SlotTimer;
use common::types::{CommitmentRequest, SignedCommitment};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use rand::Rng;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;

use integration_tests::test_common::{MODULE_ID, SIGNING_ID, test_helpers};

/// Integration test for the full RPC server flow
/// This test instantiates the RPC server, starts a local signer, and tests the complete flow
/// of commitmentRequest -> commitmentResult RPC calls
#[tokio::test]
async fn test_full_rpc_server_flow() -> eyre::Result<()> {
	// Setup test environment
	let test_env = TestEnvironment::setup().await?;

	// Test the complete flow
	test_commitment_request_flow(&test_env).await?;
	test_commitment_result_flow(&test_env).await?;
	test_end_to_end_flow(&test_env).await?;

	Ok(())
}

/// Test environment that sets up the RPC server with local signer
struct TestEnvironment {
	client: HttpClient,
	server_handle: tokio::task::JoinHandle<()>,
	temp_dir: TempDir,
	test_slots: Vec<u64>,
}

impl TestEnvironment {
	/// Sets up the complete test environment including RPC server and local signer
	async fn setup() -> eyre::Result<Self> {
		// Create temporary directory for database
		let temp_dir = TempDir::new()?;
		let db_path = temp_dir.path().join("test_db");

		// Generate random ports for signer server and RPC server
		let mut rng = rand::thread_rng();
		let signer_port = rng.gen_range(20000..29999);
		let rpc_port = rng.gen_range(30000..39999);

		// Create test configuration
		let app_config = common::config::InclusionPreconfConfig {
			commitments_server_host: "127.0.0.1".to_string(),
			commitments_server_port: rpc_port,
			commitments_database_url: db_path.to_string_lossy().to_string(),
			constraints_database_url: format!("{}_constraints", db_path.to_string_lossy()),
			delegations_database_url: format!("{}_delegations", db_path.to_string_lossy()),
			pricing_database_url: format!("{}_pricing", db_path.to_string_lossy()),
			log_level: "debug".to_string(),
			enable_method_tracing: true,
			traced_methods: vec![
				"commitmentRequest".to_string(),
				"commitmentResult".to_string(),
				"slots".to_string(),
				"fee".to_string(),
			],
			// Constraints config fields
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: None,
			constraints_bls_public_key:
				"010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101"
					.to_string(),
			constraints_delegate_public_key:
				"030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303"
					.to_string(),
			eth_genesis_timestamp: 1606824023,
			constraints_receivers: vec![
				"020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202"
					.to_string(),
			],
		};

		// Start local signer server with test configuration
		let commit_config = integration_tests::test_common::start_local_signer_server_with_config(
			MODULE_ID,
			SIGNING_ID,
			"test-admin-secret",
			signer_port,
			app_config,
		)
		.await?;

		// Create database
		let db = create_test_database(&db_path)?;
		let database = common::types::DatabaseContext::new(std::sync::Arc::new(db));

		// Generate proxy key for committer
		let test_bls_pubkey = cb_common::types::BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY)
			.map_err(|e| eyre::eyre!("Failed to deserialize BLS public key: {:?}", e))?;
		let mut commit_config_guard = commit_config;
		let proxy_address = commit_config_guard
			.signer_client
			.generate_proxy_key_ecdsa(test_bls_pubkey)
			.await
			.map_err(|e| eyre::eyre!("Failed to generate proxy key: {}", e))?;
		let committer_address = proxy_address.message.proxy;

		// Create test slots relative to current time
		let slot_timer = common::SlotTimer::new(1606824023);
		let current_slot = slot_timer.get_current_slot();
		let test_slots: Vec<u64> = (current_slot..current_slot + 10).collect();

		// Set up test delegations for the test slots
		setup_test_delegations_with_slots(&database, &committer_address, &test_slots).await?;

		// Create RPC context with constraints fields
		// Use a valid BLS public key for testing
		let bls_public_key = cb_common::utils::bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;
		let relay_url = "https://relay.example.com".to_string();
		let api_key = None::<String>;

		// Create slot timer with test genesis timestamp
		let slot_timer = SlotTimer::new(1606824023);

		let rpc_context = common::types::RpcContext::new(
			database,
			commit_config_guard,
			bls_public_key,
			relay_url,
			api_key,
			slot_timer,
		);

		// Start RPC server
		let server_address = format!("127.0.0.1:{}", rpc_port);
		let server_handle = tokio::spawn(async move {
			if let Err(e) = commitments::server::run_server(rpc_context).await {
				eprintln!("RPC server error: {}", e);
			}
		});

		// Wait for server to start
		sleep(Duration::from_millis(100)).await;

		// Create HTTP client to connect to the server
		let server_url = format!("http://{}", server_address);
		let client = HttpClientBuilder::default()
			.request_timeout(Duration::from_secs(30))
			.build(&server_url)
			.map_err(|e| eyre::eyre!("Failed to create HTTP client: {}", e))?;

		Ok(Self { client, server_handle, temp_dir, test_slots })
	}
}

/// Sets up test delegations for provided test slots
async fn setup_test_delegations_with_slots(
	database: &common::types::DatabaseContext,
	committer_address: &alloy::primitives::Address,
	test_slots: &[u64],
) -> eyre::Result<()> {
	use alloy::primitives::{B256, Bytes};
	use commit_boost::prelude::{BlsPublicKey, BlsSignature};
	use common::types::constraints::SignedDelegation;

	for slot in test_slots {
		// Create a mock delegation for testing
		let mock_delegation = SignedDelegation {
			message: common::types::constraints::Delegation {
				proposer: BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY)
					.map_err(|e| eyre::eyre!("Failed to create BLS public key: {:?}", e))?,
				delegate: BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY)
					.map_err(|e| eyre::eyre!("Failed to create BLS public key: {:?}", e))?,
				committer: *committer_address,
				slot: *slot,
				metadata: Bytes::new(),
			},
			nonce: 1,
			signing_id: B256::default(),
			signature: BlsSignature::deserialize(&[0u8; 96])
				.map_err(|e| eyre::eyre!("Failed to create BLS signature: {:?}", e))?,
		};

		// Store the delegation in the database
		database
			.store_delegation(*slot, &mock_delegation)
			.map_err(|e| eyre::eyre!("Failed to store test delegation for slot {}: {}", slot, e))?;
	}

	Ok(())
}

/// Creates a test RocksDB database
fn create_test_database(db_path: &std::path::Path) -> eyre::Result<rocksdb::DB> {
	use rocksdb::{DB, Options};

	let mut opts = Options::default();
	opts.create_if_missing(true);
	opts.create_missing_column_families(true);

	let db = DB::open(&opts, db_path)?;
	Ok(db)
}

/// Tests the commitmentRequest RPC method
async fn test_commitment_request_flow(env: &TestEnvironment) -> eyre::Result<()> {
	println!("Testing commitmentRequest RPC method...");

	// Create a valid commitment request using the first test slot
	let test_slot = env.test_slots[0];
	let payload = test_helpers::create_valid_inclusion_payload(test_slot, test_helpers::create_valid_signed_tx())?;
	let slasher = test_helpers::create_valid_slasher();
	let commitment_type = test_helpers::create_valid_commitment_type();

	let request = CommitmentRequest { commitment_type, payload, slasher };

	// Make RPC request to the server
	let response: SignedCommitment = env
		.client
		.request("commitmentRequest", (request,))
		.await
		.map_err(|e| eyre::eyre!("RPC request failed: {}", e))?;

	// Verify the response
	assert_eq!(response.commitment.commitment_type, commitment_type);
	assert!(response.nonce > 0, "Nonce should be greater than 0");
	assert!(!response.signature.to_string().is_empty(), "Signature should not be empty");
	assert_eq!(response.commitment.slasher, slasher);

	println!("commitmentRequest test passed");
	println!("  - Commitment type: {}", response.commitment.commitment_type);
	println!("  - Nonce: {}", response.nonce);
	println!("  - Request hash: {}", response.commitment.request_hash);

	Ok(())
}

/// Tests the commitmentResult RPC method
async fn test_commitment_result_flow(env: &TestEnvironment) -> eyre::Result<()> {
	println!("Testing commitmentResult RPC method...");

	// First, create a commitment request to get a request hash using the second test slot
	let test_slot = env.test_slots[1];
	let payload = test_helpers::create_valid_inclusion_payload(test_slot, test_helpers::create_valid_signed_tx())?;
	let slasher = test_helpers::create_valid_slasher();
	let commitment_type = test_helpers::create_valid_commitment_type();

	let request = CommitmentRequest { commitment_type, payload, slasher };

	// Call commitmentRequest to store the commitment
	let signed_commitment: SignedCommitment = env
		.client
		.request("commitmentRequest", (request,))
		.await
		.map_err(|e| eyre::eyre!("RPC request failed: {}", e))?;

	let request_hash = signed_commitment.commitment.request_hash;

	// Now call commitmentResult with the request hash
	let result: SignedCommitment = env
		.client
		.request("commitmentResult", (request_hash,))
		.await
		.map_err(|e| eyre::eyre!("RPC request failed: {}", e))?;

	// Verify the result matches the original commitment
	assert_eq!(result.commitment.commitment_type, signed_commitment.commitment.commitment_type);
	assert_eq!(result.commitment.payload, signed_commitment.commitment.payload);
	assert_eq!(result.commitment.request_hash, signed_commitment.commitment.request_hash);
	assert_eq!(result.commitment.slasher, signed_commitment.commitment.slasher);
	assert_eq!(result.nonce, signed_commitment.nonce);
	assert_eq!(result.signing_id, signed_commitment.signing_id);
	assert_eq!(result.signature, signed_commitment.signature);

	println!("commitmentResult test passed");
	println!("  - Retrieved commitment type: {}", result.commitment.commitment_type);
	println!("  - Retrieved nonce: {}", result.nonce);
	println!("  - Retrieved request hash: {}", result.commitment.request_hash);

	Ok(())
}

/// Tests the complete end-to-end flow
async fn test_end_to_end_flow(env: &TestEnvironment) -> eyre::Result<()> {
	println!("Testing complete end-to-end flow...");

	// Create multiple commitment requests using test slots
	let test_cases = vec![
		(env.test_slots[0], test_helpers::create_valid_signed_tx()),
		(env.test_slots[1], test_helpers::create_valid_signed_tx()),
		(env.test_slots[2], test_helpers::create_valid_signed_tx()),
	];

	let mut request_hashes = Vec::new();

	// Process all commitment requests
	for (slot, signed_tx) in test_cases {
		let payload = test_helpers::create_valid_inclusion_payload(slot, signed_tx)?;
		let slasher = test_helpers::create_valid_slasher();
		let commitment_type = test_helpers::create_valid_commitment_type();

		let request = CommitmentRequest { commitment_type, payload, slasher };

		// Call commitmentRequest via RPC
		let signed_commitment: SignedCommitment = env
			.client
			.request("commitmentRequest", (request,))
			.await
			.map_err(|e| eyre::eyre!("RPC request failed: {}", e))?;

		request_hashes.push(signed_commitment.commitment.request_hash);

		println!("  - Processed commitment for slot {} with hash {}", slot, signed_commitment.commitment.request_hash);
	}

	// Retrieve all commitments using commitmentResult
	for (i, request_hash) in request_hashes.iter().enumerate() {
		let result: SignedCommitment = env
			.client
			.request("commitmentResult", (request_hash,))
			.await
			.map_err(|e| eyre::eyre!("RPC request failed: {}", e))?;

		assert_eq!(result.commitment.request_hash, *request_hash);
		println!("  - Retrieved commitment {} with hash {}", i + 1, request_hash);
	}

	// Test error case - non-existent request hash
	let nonexistent_hash = B256::from_slice(&[0x99; 32]);
	let error_result = env.client.request::<SignedCommitment, _>("commitmentResult", (nonexistent_hash,)).await;

	assert!(error_result.is_err(), "Should return error for non-existent request hash");
	println!("  - Correctly handled non-existent request hash");

	println!("✓ End-to-end flow test passed");

	Ok(())
}

/// Test error handling for commitmentResult with invalid request hash
#[tokio::test]
async fn test_commitment_result_error_handling() -> eyre::Result<()> {
	let test_env = TestEnvironment::setup().await?;

	// Test with non-existent request hash
	let nonexistent_hash = B256::from_slice(&[0x99; 32]);
	let result = test_env.client.request::<SignedCommitment, _>("commitmentResult", (nonexistent_hash,)).await;

	assert!(result.is_err(), "Should return error for non-existent request hash");

	if let Err(e) = result {
		println!("✓ Correctly returned error: {}", e);
	}

	Ok(())
}

/// Test multiple concurrent commitment requests
#[tokio::test]
async fn test_concurrent_commitment_requests() -> eyre::Result<()> {
	let test_env = TestEnvironment::setup().await?;

	// Create multiple concurrent requests
	let mut handles = Vec::new();

	for i in 0..5 {
		let client = test_env.client.clone();
		let test_slot = test_env.test_slots[i];
		let handle = tokio::spawn(async move {
			let payload =
				test_helpers::create_valid_inclusion_payload(test_slot, test_helpers::create_valid_signed_tx())
					.unwrap();
			let slasher = test_helpers::create_valid_slasher();
			let commitment_type = test_helpers::create_valid_commitment_type();

			let request = CommitmentRequest { commitment_type, payload, slasher };

			client.request::<SignedCommitment, _>("commitmentRequest", (request,)).await
		});
		handles.push(handle);
	}

	// Wait for all requests to complete
	let mut results = Vec::new();
	for handle in handles {
		let result = handle.await??;
		results.push(result);
	}

	// Verify all requests succeeded
	assert_eq!(results.len(), 5);
	for (i, result) in results.iter().enumerate() {
		assert_eq!(result.commitment.commitment_type, 1);
		assert!(result.nonce > 0);
		println!("  - Concurrent request {} completed with nonce {}", i + 1, result.nonce);
	}

	println!("✓ Concurrent commitment requests test passed");

	Ok(())
}

/// Test server health and basic connectivity
#[tokio::test]
async fn test_server_health() -> eyre::Result<()> {
	let test_env = TestEnvironment::setup().await?;

	// Test that the server is working correctly using the first test slot
	let test_slot = test_env.test_slots[0];
	let payload = test_helpers::create_valid_inclusion_payload(test_slot, test_helpers::create_valid_signed_tx())?;
	let slasher = test_helpers::create_valid_slasher();
	let commitment_type = test_helpers::create_valid_commitment_type();

	let request = CommitmentRequest { commitment_type, payload, slasher };

	let response: SignedCommitment = test_env
		.client
		.request("commitmentRequest", (request,))
		.await
		.map_err(|e| eyre::eyre!("RPC request failed: {}", e))?;

	assert!(response.nonce > 0);
	println!("✓ Server health test passed - RPC server is working correctly");

	Ok(())
}
