use common::beacon::BeaconApiClient;
use common::config::BeaconApiConfig;
use common::db::create_database;
use common::slot_timer::SlotTimer;
use common::types::DatabaseContext;
use relay::{ProposerLookaheadConfig, ProposerLookaheadTask, config::RelayConfig, run_relay_server, setup_logging};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	// Load configuration
	let config_path = env::args().nth(1).unwrap_or_else(|| "config/relay.toml".to_string());

	let config = RelayConfig::from_file(&config_path)?;

	// Setup logging
	setup_logging(&config.relay.log_level)?;

	info!("Starting relay service (relay server + proposer lookahead task)");

	// Create slot timer
	let slot_timer = SlotTimer::new(config.relay.genesis_timestamp);

	// Open database (shared between proposer lookahead and relay server)
	let db = create_database(config.relay.database_path.to_str().ok_or_else(|| eyre::eyre!("Invalid database path"))?)?;
	let database = DatabaseContext::new(db);

	// Create beacon API client for proposer lookahead
	let beacon_config = BeaconApiConfig {
		primary_endpoint: config.relay.beacon_api_url.clone(),
		fallback_endpoints: vec![],
		request_timeout_secs: 30,
		genesis_time: config.relay.genesis_timestamp, //todo should be fetched from beacon API
	};
	let beacon_client = BeaconApiClient::with_default_client(beacon_config)?;

	// Create proposer lookahead task configuration
	let proposer_lookahead_config = ProposerLookaheadConfig {
		update_interval_seconds: config.relay.lookahead_update_interval,
		lookahead_window: config.relay.lookahead_window,
	};

	// Create and spawn proposer lookahead task
	let proposer_lookahead_task =
		ProposerLookaheadTask::new(proposer_lookahead_config, slot_timer.clone(), database.clone(), beacon_client);

	info!("Starting proposer lookahead task");
	tokio::spawn(async move {
		if let Err(e) = proposer_lookahead_task.run().await {
			tracing::error!("Proposer lookahead task error: {}", e);
		}
	});

	// Run relay server (this will block until shutdown)
	// Pass the already-opened database to avoid opening it twice
	info!("Starting relay server on port {}", config.relay.port);
	run_relay_server(config, database).await?;

	Ok(())
}
