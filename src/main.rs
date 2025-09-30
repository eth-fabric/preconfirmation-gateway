mod config;
mod db;
mod rpc;
mod server;
mod types;

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

	// Create RPC context with database context and private key
	// TODO: Load private key from config or environment variable
	let private_key = "0x1234567890123456789012345678901234567890123456789012345678901234".to_string();
	let rpc_context = types::RpcContext::new(db_context, private_key);

	server::run_server(rpc_context, &config).await?;

	Ok(())
}
