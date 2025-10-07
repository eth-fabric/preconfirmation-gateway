use alloy::primitives::{Address, B256, Bytes, Signature, b256, hex};
use cb_common::{
	commit::client::SignerClient,
	config::load_module_signing_configs,
	types::{Jwt, ModuleId},
};
use commit_boost::prelude::StartCommitModuleConfig;
use common::config::InclusionPreconfConfig;
use common::types::commitments::InclusionPayload;
use common::types::{Commitment, CommitmentRequest, DatabaseContext, RpcContext, SignedCommitment};
use eyre::Result;
use rand::Rng;
use rocksdb::{DB, Options};
use std::{collections::HashMap, sync::Arc};
use tempfile::TempDir;

// Test constants
pub const MODULE_ID: &str = "inclusion-preconf-module";
pub const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");
pub const PUBKEY: [u8; 48] =
	hex!("883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4");

/// Starts a local signer server for testing and reconstructs a StartCommitModuleConfig
/// This function allows unit tests to start a local signer service and get a properly configured
/// StartCommitModuleConfig that can be used to test signing functionality.
pub async fn start_local_signer_server(
	module_id: &str,
	signing_id: B256,
	admin_secret: &str,
	port: u16,
) -> Result<StartCommitModuleConfig<()>> {
	use cb_tests::{signer_service, utils};

	utils::setup_test_env();

	let mut cfg = utils::get_commit_boost_config(utils::get_pbs_static_config(utils::get_pbs_config(0)));

	let module_id = ModuleId(module_id.to_string());

	cfg.modules = Some(vec![utils::create_module_config(module_id.clone(), signing_id)]);

	let jwts = HashMap::from([(module_id.clone(), admin_secret.to_string())]);

	let mod_cfgs = load_module_signing_configs(&cfg, &jwts)?;

	let start_config = signer_service::start_server(port, &mod_cfgs, admin_secret.to_string(), false).await?;
	let jwt_config = mod_cfgs.get(&module_id).expect("JWT config for test module not found");

	// Reconstruct StartCommitModuleConfig using the same URL and JWT secret as the local signer
	let signer_url = format!("http://{}", start_config.endpoint)
		.parse()
		.map_err(|e| eyre::eyre!("Failed to parse signer URL: {}", e))?;

	let module_jwt = Jwt(jwt_config.jwt_secret.clone());

	// Create SignerClient with the same parameters as the local signer
	let signer_client = SignerClient::new(signer_url, None, module_jwt, module_id.clone())?;

	// Use the chain from the config
	let chain = cfg.chain;

	Ok(StartCommitModuleConfig { id: module_id, chain, signer_client, extra: () })
}

/// Starts a local signer server for testing with custom configuration
pub async fn start_local_signer_server_with_config(
	module_id: &str,
	signing_id: B256,
	admin_secret: &str,
	port: u16,
	app_config: InclusionPreconfConfig,
) -> Result<StartCommitModuleConfig<InclusionPreconfConfig>> {
	use cb_tests::{signer_service, utils};

	utils::setup_test_env();

	let mut cfg = utils::get_commit_boost_config(utils::get_pbs_static_config(utils::get_pbs_config(0)));

	let module_id = ModuleId(module_id.to_string());

	cfg.modules = Some(vec![utils::create_module_config(module_id.clone(), signing_id)]);

	let jwts = HashMap::from([(module_id.clone(), admin_secret.to_string())]);

	let mod_cfgs = load_module_signing_configs(&cfg, &jwts)?;

	let start_config = signer_service::start_server(port, &mod_cfgs, admin_secret.to_string(), false).await?;
	let jwt_config = mod_cfgs.get(&module_id).expect("JWT config for test module not found");

	// Reconstruct StartCommitModuleConfig using the same URL and JWT secret as the local signer
	let signer_url = format!("http://{}", start_config.endpoint)
		.parse()
		.map_err(|e| eyre::eyre!("Failed to parse signer URL: {}", e))?;

	let module_jwt = Jwt(jwt_config.jwt_secret.clone());

	// Create SignerClient with the same parameters as the local signer
	let signer_client = SignerClient::new(signer_url, None, module_jwt, module_id.clone())?;

	// Use the chain from the config
	let chain = cfg.chain;

	Ok(StartCommitModuleConfig { id: module_id, chain, signer_client, extra: app_config })
}

/// Consolidated test helpers for commitment testing
/// This module provides shared helper functions for both commitment_request and commitment_result tests
pub mod test_helpers {
	use super::*;
	use common::constants::COMMITMENT_TYPE;

	/// Creates a valid inclusion payload
	pub fn create_valid_inclusion_payload(slot: u64, signed_tx: Vec<u8>) -> eyre::Result<Bytes> {
		let inclusion_payload = InclusionPayload { slot, signed_tx: signed_tx.into() };
		inclusion_payload.abi_encode()
	}

	/// Creates a valid signed transaction (RLP-encoded)
	/// This creates a realistic EIP-1559 transaction for testing
	pub fn create_valid_signed_tx() -> Vec<u8> {
		// Use the function from utils.rs
		commitments::utils::create_valid_signed_transaction().to_vec()
	}

	/// Creates a valid signed transaction with a specific nonce for testing
	pub fn create_valid_signed_tx_with_nonce(nonce: u64) -> Vec<u8> {
		use alloy::consensus::{Signed, TxEip1559, TxEnvelope};
		use alloy::primitives::{Address, Bytes, Signature, TxKind, U256};
		use alloy::rlp::Encodable;

		let tx = TxEip1559 {
			chain_id: 1,
			nonce,
			gas_limit: 21000,
			max_fee_per_gas: 20_000_000_000u128,
			max_priority_fee_per_gas: 2_000_000_000u128,
			to: TxKind::Call(Address::from([0x01; 20])),
			value: U256::from(1_000_000_000_000_000_000u64),
			input: Bytes::new(),
			access_list: Default::default(),
		};

		// Create a mock signature (all zeros for testing)
		let signature = Signature::from_raw(&[0u8; 65]).unwrap();
		let signed_tx = Signed::new_unhashed(tx, signature);
		let envelope = TxEnvelope::Eip1559(signed_tx);

		// Encode to bytes
		let mut buf = Vec::new();
		envelope.encode(&mut buf);
		buf
	}

	/// Creates a valid commitment request
	pub fn create_valid_commitment_request(
		commitment_type: u64,
		payload: Bytes,
		slasher: Address,
	) -> CommitmentRequest {
		CommitmentRequest { commitment_type, payload, slasher }
	}

	/// Creates a valid slasher address
	pub fn create_valid_slasher() -> Address {
		Address::random()
	}

	pub fn create_valid_commitment_type() -> u64 {
		COMMITMENT_TYPE
	}

	/// Creates an invalid slasher address (zero address)
	pub fn create_invalid_slasher() -> Address {
		Address::ZERO
	}

	/// Creates an invalid commitment type
	pub fn create_invalid_commitment_type() -> u64 {
		COMMITMENT_TYPE + 1 // Invalid commitment type
	}

	/// Creates an empty payload
	pub fn create_empty_payload() -> Bytes {
		Bytes::new()
	}

	/// Creates an invalid payload (not ABI-encoded InclusionPayload)
	pub fn create_invalid_payload() -> Bytes {
		Bytes::from(vec![0x01, 0x02, 0x03]) // Not ABI-encoded
	}

	/// Creates a valid signed commitment for testing
	pub fn create_signed_commitment_with_mock_signature(
		commitment_type: u64,
		payload: Bytes,
		request_hash: B256,
		slasher: Address,
		nonce: u64,
		signing_id: B256,
	) -> SignedCommitment {
		let commitment = Commitment { commitment_type, payload, request_hash, slasher };

		SignedCommitment {
			commitment,
			nonce,
			signing_id,
			signature: Signature::from_bytes_and_parity(&[0x01; 64], true),
		}
	}

	/// Creates a valid request hash
	pub fn create_valid_request_hash() -> B256 {
		B256::from_slice(&[0x11; 32])
	}

	/// Creates another valid request hash
	pub fn create_another_valid_request_hash() -> B256 {
		B256::from_slice(&[0x22; 32])
	}

	/// Creates a non-existent request hash
	pub fn create_nonexistent_request_hash() -> B256 {
		B256::from_slice(&[0x99; 32])
	}

	/// Creates a valid signing ID
	pub fn create_valid_signing_id() -> B256 {
		B256::from_slice(&[0x33; 32])
	}

	/// Creates a valid payload
	pub fn create_valid_payload() -> Bytes {
		Bytes::from(vec![0x01, 0x02, 0x03, 0x04, 0x05])
	}

	/// Creates another valid payload
	pub fn create_another_valid_payload() -> Bytes {
		Bytes::from(vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE])
	}

	/// Creates a valid nonce
	pub fn create_valid_nonce() -> u64 {
		12345
	}

	/// Creates another valid nonce
	pub fn create_another_valid_nonce() -> u64 {
		67890
	}

	/// Creates a test RPC context with a temporary RocksDB database and local signer server
	/// This function provides a complete test environment for RPC handler tests
	/// The port is randomly generated to avoid conflicts between concurrent tests
	pub async fn create_test_context() -> Result<RpcContext> {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");

		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		// Generate a random port to avoid conflicts
		let mut rng = rand::thread_rng();
		let port = rng.gen_range(20000..65535);

		// Start local signer server to get proper config
		let commit_config = start_local_signer_server(MODULE_ID, SIGNING_ID, "test-admin-secret", port).await?;

		// TODO: Get these from configuration - using defaults for tests
		// Use a valid BLS public key for testing
		let bls_public_key = cb_common::utils::bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;
		let relay_url = "https://relay.example.com".to_string();
		let api_key = None::<String>;

		Ok(RpcContext {
			database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			committer_address: Address::ZERO,
			bls_public_key,
			relay_url,
			api_key,
		})
	}

	/// Creates a test RPC context with InclusionPreconfConfig for server tests
	/// This function provides a complete test environment for server tests that need the full config
	pub async fn create_test_context_with_config() -> Result<RpcContext<InclusionPreconfConfig>> {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");

		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		// Generate a random port to avoid conflicts
		let mut rng = rand::thread_rng();
		let rpc_port = rng.gen_range(3000..65535);
		let constraints_port = rpc_port + 1;

		// Create test config
		let app_config = InclusionPreconfConfig {
			rpc_server_host: "127.0.0.1".to_string(),
			rpc_server_port: rpc_port,
			database_url: "./test_db".to_string(),
			log_level: "info".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],
			committer_address: "0x0000000000000000000000000000000000000000".to_string(),
			constraints_server_port: constraints_port,
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: None,
			constraints_bls_public_key:
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
			constraints_delegate_public_key:
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
		};

		// Start local signer server with config
		let commit_config = start_local_signer_server_with_config(
			MODULE_ID,
			SIGNING_ID,
			"test-admin-secret",
			rpc_port,
			app_config.clone(),
		)
		.await?;

		// Use a valid BLS public key for testing
		let bls_public_key = cb_common::utils::bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;
		let relay_url = "https://relay.example.com".to_string();
		let api_key = None::<String>;

		Ok(RpcContext {
			database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			committer_address: Address::ZERO,
			bls_public_key,
			relay_url,
			api_key,
		})
	}
}
