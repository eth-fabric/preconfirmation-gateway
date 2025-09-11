use std::net::SocketAddr;

use jsonrpsee::server::Server;
use tracing_subscriber::util::SubscriberInitExt;

mod config;
mod db;
mod rpc;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load configuration
	let config = config::Config::load()?;

	// Setup logging with configuration
	setup_logging(&config)?;

	// Initialize database connection pool
	let db_pool = db::create_pool(&config).await?;
	db::test_connection(&db_pool).await?;
	let db_context = types::DatabaseContext::new(db_pool);

	// Create RPC context with database context
	let rpc_context = types::RpcContext::new(db_context);

	run_server(rpc_context, &config).await?;

	Ok(())
}

async fn run_server(rpc_context: types::RpcContext, config: &config::Config) -> anyhow::Result<()> {
	let server = Server::builder().build(config.server_address().parse::<SocketAddr>()?).await?;
	let module = rpc::setup_rpc_methods(rpc_context)?;

	let addr = server.local_addr()?;
	tracing::info!("Starting RPC server on {}", addr);
	let handle = server.start(module);

	// Run the server indefinitely, waiting for incoming requests
	handle.stopped().await;

	Ok(())
}

fn setup_logging(config: &config::Config) -> anyhow::Result<()> {
	let mut filter = tracing_subscriber::EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.logging.level));

	if config.logging.enable_method_tracing {
		for method in &config.logging.traced_methods {
			let directive = format!("jsonrpsee[method_call{{name = \"{}\"}}]=trace", method);
			filter = filter.add_directive(directive.parse()?);
		}
	}

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;
	Ok(())
}
