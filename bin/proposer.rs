use eyre::Result;
use lazy_static::lazy_static;
use prometheus::{IntCounter, Registry, opts};
use proposer::cli;

lazy_static! {
	pub static ref MY_CUSTOM_REGISTRY: Registry =
		Registry::new_custom(Some("inclusion-preconf-proposer".to_string()), None).unwrap();
	pub static ref SIG_RECEIVED_COUNTER: IntCounter =
		IntCounter::with_opts(opts!("sig_received_total", "Number of OS signals received")).unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	// Remember to register all your metrics before starting the process
	MY_CUSTOM_REGISTRY.register(Box::new(SIG_RECEIVED_COUNTER.clone()))?;

	cli::run().await
}

// CLI handlers moved to `crates/proposer/src/cli.rs`
