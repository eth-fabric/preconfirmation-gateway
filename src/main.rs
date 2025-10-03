mod commitments;
mod config;
mod constants;
mod db;
mod server;
mod signing;
mod types;

use commit_boost::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load consolidated configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<config::InclusionPreconfConfig>()
		.map_err(|e| anyhow::anyhow!("Failed to load commit module config: {}", e))?;

	// Setup logging with configuration
	server::setup_logging(&commit_config)?;

	// Initialize RocksDB database
	let db = db::create_database(&commit_config)?;
	db::db_healthcheck(&db).await?;
	let db_context = types::DatabaseContext::new(db);

	// Parse committer address from config
	let committer_address = commit_config
		.extra
		.committer_address
		.parse()
		.map_err(|e| anyhow::anyhow!("Invalid committer address in config: {}", e))?;

	// Create RPC context with database context, commit config, and committer address
	let rpc_context = types::RpcContext::new(db_context, commit_config, committer_address);

	server::run_server(rpc_context).await?;

	Ok(())
}
