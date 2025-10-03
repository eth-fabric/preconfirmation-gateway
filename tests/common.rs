use alloy::primitives::{Address, B256, b256, hex};
use cb_common::{
	commit::client::SignerClient,
	config::load_module_signing_configs,
	types::{Jwt, ModuleId},
};
use commit_boost::prelude::StartCommitModuleConfig;
use eyre::Result;
use preconfirmation_gateway::{DatabaseContext, RpcContext};
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
