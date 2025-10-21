use std::net::SocketAddr;

use eyre::Result;
use jsonrpsee::server::Server;

use super::methods;
use crate::CommitmentsServerState;
use common::config::{CommitmentsConfig, GatewayConfig};

// Functions for working with commit-boost configuration
pub async fn run_server<T>(rpc_context: CommitmentsServerState<T>) -> Result<()>
where
	T: Send + Sync + 'static + GatewayConfig,
{
	// Extract the server address from the commit config
	let commit_config = rpc_context.commit_config.lock().await;
	let commitments_config = commit_config.extra.commitments_config();
	let server_address = format!("{}:{}", commitments_config.server_host(), commitments_config.server_port());
	drop(commit_config); // Release the lock

	let server = Server::builder().build(server_address.parse::<SocketAddr>()?).await?;
	let module = methods::setup_commitment_methods(rpc_context)?;

	let addr = server.local_addr()?;
	tracing::info!("Starting RPC server on {}", addr);
	let handle = server.start(module);

	// Run the server indefinitely, waiting for incoming requests
	handle.stopped().await;

	Ok(())
}
