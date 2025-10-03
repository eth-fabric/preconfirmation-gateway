use alloy::primitives::B256;
use cb_common::{
	commit::client::SignerClient,
	config::load_module_signing_configs,
	types::{Jwt, ModuleId},
};
use commit_boost::prelude::StartCommitModuleConfig;
use eyre::Result;
use std::collections::HashMap;

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
