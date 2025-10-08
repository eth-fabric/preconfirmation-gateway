use relay::{config::RelayConfig, run_relay_server, setup_logging};
use std::env;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	// Load configuration
	let config_path = env::args().nth(1).unwrap_or_else(|| "config/relay.toml".to_string());

	let config = RelayConfig::from_file(&config_path)?;

	// Setup logging
	setup_logging(&config.relay.log_level)?;

	// Run server
	run_relay_server(config).await?;

	Ok(())
}
