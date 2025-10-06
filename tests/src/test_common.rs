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
		Address::from([0x2; 20])
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

		Ok(RpcContext {
			database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			committer_address: Address::ZERO,
		})
	}
}
