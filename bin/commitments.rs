use cb_common::utils::bls_pubkey_from_hex;
use commit_boost::prelude::*;
use commitments::server;
use common::db::DatabaseType;
use common::{config, db, types};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load consolidated configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<config::InclusionPreconfConfig>()
		.map_err(|e| anyhow::anyhow!("Failed to load commit module config: {}", e))?;

	// Setup logging with configuration
	server::setup_logging(&commit_config)?;

	// Initialize RocksDB database
	let db = db::create_database(&commit_config, DatabaseType::Commitments)?;
	db::db_healthcheck(&db).await?;
	let db_context = types::DatabaseContext::new(db);

	// Parse committer address from config
	let committer_address = commit_config
		.extra
		.committer_address
		.parse()
		.map_err(|e| anyhow::anyhow!("Invalid committer address in config: {}", e))?;

	// Get constraints configuration for BLS keys and relay settings
	let constraints_config = commit_config.extra.constraints();
	let bls_public_key = bls_pubkey_from_hex(&constraints_config.bls_public_key)
		.map_err(|e| anyhow::anyhow!("Failed to create BLS public key: {}", e))?;
	let relay_url = constraints_config.relay_url.clone();
	let api_key = constraints_config.api_key.clone();

	// Create RPC context with database context, commit config, and committer address
	let rpc_context =
		types::RpcContext::new(db_context, commit_config, committer_address, bls_public_key, relay_url, api_key);

	server::run_server(rpc_context).await?;

	Ok(())
}
