use std::net::SocketAddr;

use anyhow::Result;
use jsonrpsee::server::Server;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::InclusionPreconfConfig;
use crate::{rpc, types};
use commit_boost::prelude::StartCommitModuleConfig;

// Functions for working with commit-boost configuration
pub async fn run_server<T: Send + Sync + 'static>(rpc_context: types::RpcContext<T>) -> Result<()>
where
	T: Clone + Into<InclusionPreconfConfig>,
{
	// Extract the server address from the commit config
	let commit_config = rpc_context.commit_config.lock().await;
	let app_config: InclusionPreconfConfig = commit_config.extra.clone().into();
	let server_address = format!("{}:{}", app_config.rpc_server_host, app_config.rpc_server_port);
	drop(commit_config); // Release the lock

	let server = Server::builder().build(server_address.parse::<SocketAddr>()?).await?;
	let module = rpc::setup_rpc_methods(rpc_context)?;

	let addr = server.local_addr()?;
	tracing::info!("Starting RPC server on {}", addr);
	let handle = server.start(module);

	// Run the server indefinitely, waiting for incoming requests
	handle.stopped().await;

	Ok(())
}

pub fn setup_logging(commit_config: &StartCommitModuleConfig<InclusionPreconfConfig>) -> Result<()> {
	let app_config = &commit_config.extra;
	let mut filter = tracing_subscriber::EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&app_config.log_level));

	if app_config.enable_method_tracing {
		for method in &app_config.traced_methods {
			let directive = format!("jsonrpsee[method_call{{name = \"{}\"}}]=trace", method);
			filter = filter.add_directive(directive.parse()?);
		}
	}

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;
	Ok(())
}
