use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use eyre::Result;
use jsonrpsee::server::Server;

use super::methods;
use crate::CommitmentsServerState;
use crate::metrics::metrics_handler;
use common::config::{CommitmentsConfig, GatewayConfig};

/// Start the Commitments JSON-RPC server using the provided shared state.
///
/// Resolves the listen address from the commit-config and serves until shutdown.
/// Returns `Ok(())` when the server stops.
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
	// Spawn metrics HTTP server on port+1
	let metrics_addr: SocketAddr = {
		let mut a = addr;
		a.set_port(a.port().saturating_add(1));
		a
	};
	tokio::spawn(async move {
		let app = Router::new().route("/metrics", get(metrics_handler));
		match tokio::net::TcpListener::bind(metrics_addr).await {
			Ok(listener) => {
				if let Err(e) = axum::serve(listener, app).await {
					tracing::error!("metrics server error: {}", e);
				}
			}
			Err(e) => tracing::error!("failed to bind metrics listener: {}", e),
		}
	});
	let handle = server.start(module);

	// Run the server indefinitely, waiting for incoming requests
	handle.stopped().await;

	Ok(())
}
