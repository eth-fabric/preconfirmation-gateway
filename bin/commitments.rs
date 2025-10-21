use cb_common::utils::bls_pubkey_from_hex;
use commit_boost::prelude::*;
use commitments::server;
use common::db::DatabaseType;
use common::execution::{ExecutionApiClient, ExecutionApiConfig};
use common::slot_timer::SlotTimer;
use common::{config, db, types};
use eyre;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	// Load consolidated configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<config::InclusionPreconfConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	// Setup logging with configuration
	server::setup_logging(&commit_config)?;

	// Initialize RocksDB database
	let db = db::create_database(&commit_config, DatabaseType::Commitments)?;
	db::db_healthcheck(&db).await?;
	let db_context = types::DatabaseContext::new(db);

	// Get constraints configuration for BLS keys and relay settings
	let app_config = &commit_config.extra;
	let bls_public_key = bls_pubkey_from_hex(&app_config.constraints_bls_public_key)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;
	let relay_url = app_config.constraints_relay_url.clone();
	let api_key = app_config.constraints_api_key.clone();

	// Create slot timer with genesis timestamp
	let slot_timer = SlotTimer::new(commit_config.extra.eth_genesis_timestamp());

	// Create execution client RPC API for gas price and estimation
	let execution_config = ExecutionApiConfig {
		endpoint: app_config.execution_endpoint_url().to_string(),
		request_timeout_secs: app_config.execution_request_timeout_secs(),
		max_retries: app_config.execution_max_retries(),
	};
	let execution_client = Arc::new(ExecutionApiClient::with_default_client(execution_config)?);

	// Create RPC context with database context and commit config
	let rpc_context = types::RpcContext::new(
		db_context,
		commit_config,
		bls_public_key,
		relay_url,
		api_key,
		slot_timer,
		execution_client,
	);

	server::run_server(rpc_context).await?;

	Ok(())
}
