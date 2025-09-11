use std::net::SocketAddr;

use jsonrpsee::server::Server;
use tracing_subscriber::util::SubscriberInitExt;

mod db;
mod rpc;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut filter = tracing_subscriber::EnvFilter::try_from_default_env()?;
	filter = filter.add_directive("jsonrpsee[method_call{name = \"commitmentRequest\"}]=trace".parse()?);
	filter = filter.add_directive("jsonrpsee[method_call{name = \"commitmentResult\"}]=trace".parse()?);
	filter = filter.add_directive("jsonrpsee[method_call{name = \"slots\"}]=trace".parse()?);
	filter = filter.add_directive("jsonrpsee[method_call{name = \"fee\"}]=trace".parse()?);

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	// Initialize database connection pool
	let db_pool = db::create_pool().await?;
	db::test_connection(&db_pool).await?;
	let db_context = types::DatabaseContext::new(db_pool);

	// Create RPC context with database context
	let rpc_context = types::RpcContext::new(db_context);

	run_server(rpc_context).await?;

	Ok(())
}

async fn run_server(rpc_context: types::RpcContext) -> anyhow::Result<()> {
	let server = Server::builder().build("127.0.0.1:8080".parse::<SocketAddr>()?).await?;
	let module = rpc::setup_rpc_methods(rpc_context)?;

	let addr = server.local_addr()?;
	tracing::info!("Starting RPC server on {}", addr);
	let handle = server.start(module);

	// Run the server indefinitely, waiting for incoming requests
	handle.stopped().await;

	Ok(())
}
