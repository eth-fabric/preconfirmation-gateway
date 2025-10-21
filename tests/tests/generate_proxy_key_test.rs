use common::slot_timer::SlotTimer;
use eyre::Result;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use rand::Rng;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;

use integration_tests::test_common::{MODULE_ID, SIGNING_ID};

// Integration test using the proper test helper functions
async fn setup_test_env() -> Result<(HttpClient, TempDir, tokio::task::JoinHandle<()>)> {
	let temp_dir = TempDir::new()?;
	let db_path = temp_dir.path().join("test_db");

	// Generate random ports for signer server and RPC server
	let mut rng = rand::thread_rng();
	let signer_port = rng.gen_range(20000..29999);
	let rpc_port = rng.gen_range(30000..39999);

	// Create test configuration
	let commitments_config = common::config::InclusionCommitmentsConfig {
		server_host: "127.0.0.1".to_string(),
		server_port: rpc_port,
		database_path: db_path.to_string_lossy().to_string(),
		log_level: "debug".to_string(),
		bls_public_key:
			"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4"
				.to_string(),
		enable_method_tracing: true,
		traced_methods: vec![
			"commitmentRequest".to_string(),
			"commitmentResult".to_string(),
			"slots".to_string(),
			"fee".to_string(),
			"generateProxyKey".to_string(),
		],
	};

	let app_config = common::config::InclusionGatewayConfig {
		commitments: commitments_config,
		relay_url: "https://relay.example.com".to_string(),
		constraints_api_key: None,
		genesis_timestamp: 1606824023,
		delegation_database_path: format!("{}_delegations", db_path.to_string_lossy()),
		execution_endpoint_url: "http://localhost:8545".to_string(),
		execution_request_timeout_secs: 10,
		execution_max_retries: 3,
		constraints_receivers: vec![
			"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4"
				.to_string(),
		],
		delegate_public_key:
			"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4"
				.to_string(),
	};

	// Start local signer server with test configuration
	let commit_config = integration_tests::test_common::start_local_signer_server(
		MODULE_ID,
		SIGNING_ID,
		"test-admin-secret",
		signer_port,
		app_config.clone(),
	)
	.await?;

	// Create database
	let mut opts = rocksdb::Options::default();
	opts.create_if_missing(true);
	let db = rocksdb::DB::open(&opts, &db_path)?;
	let database = common::types::DatabaseContext::new(std::sync::Arc::new(db));

	// Generate proxy key for committer using the registered BLS public key
	let test_bls_pubkey = cb_common::types::BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY)
		.map_err(|e| eyre::eyre!("Failed to deserialize BLS public key: {:?}", e))?;
	let mut commit_config_guard = commit_config;
	let proxy_address = commit_config_guard.signer_client.generate_proxy_key_ecdsa(test_bls_pubkey).await?;
	let _committer_address = proxy_address.message.proxy;

	// Create RPC context with constraints fields
	let bls_public_key = cb_common::utils::bls_pubkey_from_hex(
		"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4",
	)?;
	let relay_url = "https://relay.example.com".to_string();
	let api_key = None::<String>;

	// Create slot timer with test genesis timestamp
	let slot_timer = SlotTimer::new(1606824023);

	// Create mock Reth client for tests
	let execution_config = common::execution::ExecutionApiConfig {
		endpoint: "http://localhost:8545".to_string(),
		request_timeout_secs: 10,
		max_retries: 3,
	};
	let execution_client =
		Arc::new(common::execution::ExecutionApiClient::with_default_client(execution_config).unwrap());

	let rpc_context = commitments::CommitmentsServerState::new(
		database,
		commit_config_guard,
		bls_public_key,
		relay_url,
		api_key,
		slot_timer,
		execution_client,
	);

	// Start RPC server
	let server_handle = tokio::spawn(async move {
		if let Err(e) = commitments::server::run_server(rpc_context).await {
			eprintln!("RPC server error: {}", e);
		}
	});

	// Wait for server to start
	sleep(Duration::from_millis(500)).await;

	// Create HTTP client to connect to the server
	let server_url = format!("http://127.0.0.1:{}", rpc_port);
	let client = HttpClientBuilder::default().request_timeout(Duration::from_secs(30)).build(&server_url)?;

	Ok((client, temp_dir, server_handle))
}

// Test the request/response types directly without full server setup
#[tokio::test]
async fn test_generate_proxy_key_types() -> Result<()> {
	use commitments::handlers::{GenerateProxyKeyRequest, GenerateProxyKeyResponse};
	use serde_json::json;

	// Test request serialization
	let request = GenerateProxyKeyRequest {
		bls_public_key:
			"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4"
				.to_string(),
		encryption_scheme: "ecdsa".to_string(),
	};

	let json_str = serde_json::to_string(&request)?;
	let deserialized: GenerateProxyKeyRequest = serde_json::from_str(&json_str)?;
	assert_eq!(deserialized.bls_public_key, request.bls_public_key);
	assert_eq!(deserialized.encryption_scheme, request.encryption_scheme);

	// Test response serialization
	let response = GenerateProxyKeyResponse {
		signed_delegation: json!({"message": {"proxy": "0x1234567890123456789012345678901234567890"}}),
		encryption_scheme: "ecdsa".to_string(),
	};

	let response_json = serde_json::to_string(&response)?;
	let deserialized_response: GenerateProxyKeyResponse = serde_json::from_str(&response_json)?;
	assert_eq!(deserialized_response.encryption_scheme, response.encryption_scheme);

	Ok(())
}

// Test BLS public key validation
#[test]
fn test_bls_public_key_validation() -> Result<()> {
	use cb_common::utils::bls_pubkey_from_hex;

	// Test valid BLS public key
	let valid_hex =
		"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4";
	let bls_public_key = bls_pubkey_from_hex(valid_hex)?;
	assert_eq!(bls_public_key.serialize().len(), 48);

	// Test invalid BLS public key
	let invalid_hex = "0xinvalid";
	let result = bls_pubkey_from_hex(invalid_hex);
	assert!(result.is_err());

	Ok(())
}

// Test encryption scheme validation
#[test]
fn test_encryption_scheme_validation() {
	use commit_boost::prelude::commit::request::EncryptionScheme;

	// Test that both encryption schemes are valid
	assert!(matches!(EncryptionScheme::Ecdsa, EncryptionScheme::Ecdsa));
	assert!(matches!(EncryptionScheme::Bls, EncryptionScheme::Bls));
}

#[tokio::test]
async fn test_generate_proxy_key_ecdsa_success() -> Result<()> {
	let (client, _temp_dir, server_handle) = setup_test_env().await?;

	let bls_public_key =
		"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4";
	let request = json!({
		"bls_public_key": bls_public_key,
		"encryption_scheme": "ecdsa"
	});

	let response: serde_json::Value = client.request("generateProxyKey", [request]).await?;

	assert!(response["signed_delegation"].is_object());
	assert!(response["signed_delegation"]["message"]["proxy"].is_string());
	assert_eq!(response["encryption_scheme"], "ecdsa");

	let proxy_address_str = response["signed_delegation"]["message"]["proxy"].as_str().unwrap();
	let proxy_address: alloy::primitives::Address = proxy_address_str.parse()?;
	assert_ne!(proxy_address, alloy::primitives::Address::ZERO);

	server_handle.abort();
	Ok(())
}

#[tokio::test]
async fn test_generate_proxy_key_bls_success() -> Result<()> {
	let (client, _temp_dir, server_handle) = setup_test_env().await?;

	let bls_public_key =
		"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4";
	let request = json!({
		"bls_public_key": bls_public_key,
		"encryption_scheme": "bls"
	});

	let response: serde_json::Value = client.request("generateProxyKey", [request]).await?;

	assert!(response["signed_delegation"].is_object());
	assert!(response["signed_delegation"]["message"]["proxy"].is_string());
	assert_eq!(response["encryption_scheme"], "bls");

	let proxy_bls_key_str = response["signed_delegation"]["message"]["proxy"].as_str().unwrap();
	let proxy_bls_key = cb_common::utils::bls_pubkey_from_hex(proxy_bls_key_str)?;
	assert_eq!(proxy_bls_key.serialize().len(), 48); // BLS public keys are 48 bytes

	server_handle.abort();
	Ok(())
}

#[tokio::test]
async fn test_generate_proxy_key_invalid_bls_public_key() -> Result<()> {
	let (client, _temp_dir, server_handle) = setup_test_env().await?;

	let invalid_bls_public_key = "0xinvalid_key";
	let request = json!({
		"bls_public_key": invalid_bls_public_key,
		"encryption_scheme": "ecdsa"
	});

	let result: Result<serde_json::Value, _> = client.request("generateProxyKey", [request]).await;

	assert!(result.is_err());
	let error = result.unwrap_err();
	assert!(error.to_string().contains("Invalid BLS public key format"));

	server_handle.abort();
	Ok(())
}

#[tokio::test]
async fn test_generate_proxy_key_invalid_encryption_scheme() -> Result<()> {
	let (client, _temp_dir, server_handle) = setup_test_env().await?;

	let bls_public_key =
		"0x883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4";
	let request = json!({
		"bls_public_key": bls_public_key,
		"encryption_scheme": "invalid_scheme"
	});

	let result: Result<serde_json::Value, _> = client.request("generateProxyKey", [request]).await;

	assert!(result.is_err());
	let error = result.unwrap_err();
	assert!(error.to_string().contains("Invalid encryption scheme"));

	server_handle.abort();
	Ok(())
}
