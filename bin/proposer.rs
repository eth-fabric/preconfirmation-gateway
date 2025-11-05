use axum::{Router, routing::get};
use eyre::Result;
use proposer::cli;
use proposer::metrics::metrics_handler;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	// Spawn lightweight metrics server exposing only /metrics
	let metrics_addr = std::env::var("PROPOSER_METRICS_ADDR").unwrap_or_else(|_| "0.0.0.0:9902".to_string());
	let metrics_addr: std::net::SocketAddr = metrics_addr.parse().unwrap_or_else(|_| "0.0.0.0:9902".parse().unwrap());
	tokio::spawn(async move {
		let app = Router::new().route("/metrics", get(metrics_handler));
		match tokio::net::TcpListener::bind(metrics_addr).await {
			Ok(listener) => {
				if let Err(e) = axum::serve(listener, app).await {
					tracing::error!("proposer metrics server error: {}", e);
				}
			}
			Err(e) => tracing::error!("failed to bind proposer metrics listener: {}", e),
		}
	});

	// Run CLI in background; exit gracefully on shutdown signal
	let cli_task = tokio::spawn(async move { cli::run().await });

	// Wait for Docker shutdown signal
	let _ = common::utils::wait_for_signal().await;

	// Stop CLI task
	cli_task.abort();

	Ok(())
}

// CLI handlers moved to `crates/proposer/src/cli.rs`
