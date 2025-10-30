use axum::{Router, routing::get};
use clap::Parser;
use eyre::Result;
use proposer::cli::{Cli, Commands};
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

	// Parse CLI arguments in main thread (not in spawned task)
	let cli = Cli::parse();

	// Handle commands - for Run, load config in main then spawn work; others execute directly
	match cli.command.unwrap_or(Commands::Run) {
		Commands::Run => {
			// Load config in main thread (critical: not in spawned task)
			tracing::info!("Starting proposer service");
			let config_and_task = proposer::cli::load_daemon_config().await?;
			
			// Spawn daemon work loop as background task
			let daemon_task = tokio::spawn(async move {
				if let Err(e) = proposer::cli::run_daemon_loop(config_and_task).await {
					tracing::error!("Daemon error: {}", e);
					eprintln!("Daemon error: {}", e);
				}
			});

			// Wait for Docker shutdown signal
			let _ = common::utils::wait_for_signal().await;

			// Stop daemon task
			daemon_task.abort();
		}
		command => {
			// For CLI commands (register, opt-in, etc.), execute directly and exit
			proposer::cli::handle_command(command).await?;
		}
	}

	Ok(())
}

// CLI handlers moved to `crates/proposer/src/cli.rs`
