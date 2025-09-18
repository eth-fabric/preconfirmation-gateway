use std::net::SocketAddr;

use anyhow::Result;
use jsonrpsee::server::Server;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{config, rpc, types};

pub async fn run_server(rpc_context: types::RpcContext, config: &config::Config) -> Result<()> {
	let server = Server::builder().build(server_address(config).parse::<SocketAddr>()?).await?;
	let module = rpc::setup_rpc_methods(rpc_context)?;

	let addr = server.local_addr()?;
	tracing::info!("Starting RPC server on {}", addr);
	let handle = server.start(module);

	// Run the server indefinitely, waiting for incoming requests
	handle.stopped().await;

	Ok(())
}

pub fn setup_logging(config: &config::Config) -> Result<()> {
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

pub fn server_address(config: &config::Config) -> String {
	format!("{}:{}", config.server.host, config.server.port)
}
