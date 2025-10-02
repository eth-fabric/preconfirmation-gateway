mod config;
mod db;
mod rpc;
mod server;
mod types;
mod signing;
mod constants;

use commit_boost::prelude::*;
use serde::Deserialize;


struct InclusionPreconfService {
    config: StartCommitModuleConfig<ExtraConfig>
}

// Extra configurations parameters can be set here and will be automatically
// parsed from the .config.toml file These parameters will be in the .extra
// field of the StartModuleConfig<ExtraConfig> struct you get after calling
// `load_commit_module_config::<ExtraConfig>()`
#[derive(Debug, Deserialize)]
struct ExtraConfig {
    sleep_secs: u64,
    #[serde(default = "default_ecdsa")]
    use_ecdsa_keys: bool,
}

fn default_ecdsa() -> bool {
    true
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load configuration
	let config = config::Config::load()?;

	// Setup logging with configuration
	server::setup_logging(&config)?;

	// Initialize database connection pool
	let db_pool = db::create_pool(&config).await?;
	db::test_connection(&db_pool).await?;
	let db_context = types::DatabaseContext::new(db_pool);

	// Load commit-boost configuration
	let commit_config = load_commit_module_config::<ExtraConfig>()
		.map_err(|e| anyhow::anyhow!("Failed to load commit module config: {}", e))?;
	
	// Parse committer address from config
	let committer_address = config.committer.address.parse()
		.map_err(|e| anyhow::anyhow!("Invalid committer address in config: {}", e))?;
	
	// Create RPC context with database context, commit config, and committer address
	let rpc_context = types::RpcContext::new(db_context, commit_config, committer_address);

	server::run_server(rpc_context, &config).await?;

	Ok(())
}
