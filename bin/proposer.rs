use commit_boost::prelude::*;
use common::beacon::BeaconApiClient;
use common::config::BeaconApiConfig;
use common::slot_timer::SlotTimer;
use constraints::ConstraintsClient;
use eyre::Result;
use proposer::{ProposerConfig, process_lookahead};
use tokio::time::{Duration, interval};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	info!("Starting proposer service");

	// Load configuration using commit-boost's config loader
	let mut commit_config = load_commit_module_config::<ProposerConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	let config = commit_config.extra.clone();

	info!("Proposer configuration:");
	info!("  Delegate BLS key: {}", config.delegate_bls_public_key);
	info!("  Committer address: {}", config.committer_address);
	info!("  Relay URL: {}", config.relay_url);
	info!("  Beacon API URL: {}", config.beacon_api_url);
	info!("  Poll interval: {} seconds", config.poll_interval_seconds);

	// Create beacon API client
	let beacon_config = BeaconApiConfig {
		primary_endpoint: config.beacon_api_url.clone(),
		fallback_endpoints: vec![],
		request_timeout_secs: 30,
		genesis_time: config.beacon_genesis_timestamp,
	};

	let beacon_client = BeaconApiClient::with_default_client(beacon_config)
		.map_err(|e| eyre::eyre!("Failed to create beacon API client: {}", e))?;

	info!("Beacon API client initialized");

	// Create constraints client for posting delegations to relay
	let constraints_client = ConstraintsClient::new(config.relay_url.clone(), config.relay_api_key.clone());

	// Test relay connectivity
	match constraints_client.health_check().await {
		Ok(true) => info!("Relay health check passed"),
		Ok(false) => error!("Relay health check failed"),
		Err(e) => error!("Failed to check relay health: {}", e),
	}

	// Create slot timer for tracking current slot
	let slot_timer = SlotTimer::new(config.beacon_genesis_timestamp);

	info!("Starting proposer duty polling loop");

	// Set up polling interval
	let mut poll_interval = interval(Duration::from_secs(config.poll_interval_seconds));

	loop {
		poll_interval.tick().await;

		let current_slot = slot_timer.get_current_slot();
		debug!("Checking proposer duties for current slot: {}", current_slot);

		// Process lookahead to find and post delegations
		match process_lookahead(&beacon_client, &constraints_client, &mut commit_config, current_slot).await {
			Ok(_) => {
				debug!("Lookahead processed successfully for slot {}", current_slot);
			}
			Err(e) => {
				error!("Error processing lookahead for slot {}: {}", current_slot, e);
			}
		}
	}
}
