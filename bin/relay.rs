use common::beacon::BeaconApiClient;
use common::config::BeaconApiConfig;
use common::db::create_database;
use common::slot_timer::SlotTimer;
use common::types::DatabaseContext;
use lazy_static::lazy_static;
use prometheus::{IntCounter, Registry, opts};
use relay::{ProposerLookaheadConfig, ProposerLookaheadTask, config::RelayConfig, run_relay_server, setup_logging};
use std::env;
use tracing::info;

lazy_static! {
	pub static ref MY_CUSTOM_REGISTRY: Registry =
		Registry::new_custom(Some("inclusion-preconf-relay".to_string()), None).unwrap();
	pub static ref SIG_RECEIVED_COUNTER: IntCounter =
		IntCounter::with_opts(opts!("sig_received_total", "Number of OS signals received")).unwrap();
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
	// Load configuration

	// Remember to register all your metrics before starting the process
	MY_CUSTOM_REGISTRY.register(Box::new(SIG_RECEIVED_COUNTER.clone()))?;
	let config_path = env::args().nth(1).unwrap_or_else(|| "config/relay.toml".to_string());

	let config = RelayConfig::from_file(&config_path)?;

	// Setup logging
	setup_logging(&config.relay.log_level)?;

	info!("Starting relay service (relay server + proposer lookahead task)");

	// Create slot timer
	let slot_timer = SlotTimer::new(config.relay.genesis_timestamp);

	// Open database for proposer lookahead
	let db = create_database(config.relay.database_path.to_str().ok_or_else(|| eyre::eyre!("Invalid database path"))?)?;
	let database = DatabaseContext::new(db);

	// Create beacon API client for proposer lookahead
	let beacon_config = BeaconApiConfig {
		primary_endpoint: std::env::var("BEACON_API_URL")
			.unwrap_or_else(|_| "https://ethereum-beacon-api.publicnode.com".to_string()),
		fallback_endpoints: vec![],
		request_timeout_secs: 30,
		genesis_time: config.relay.genesis_timestamp,
	};
	let beacon_client = BeaconApiClient::with_default_client(beacon_config)?;

	// Create proposer lookahead task configuration
	let proposer_lookahead_config = ProposerLookaheadConfig {
		update_interval_seconds: 60, // Update every 60 seconds
		lookahead_window: 64,        // 64 slots lookahead
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
	info!("Starting relay server on port {}", config.relay.port);
	run_relay_server(config).await?;

	Ok(())
}
