use commit_boost::prelude::*;
use common::beacon::BeaconApiClient;
use common::config::BeaconApiConfig;
use common::db::{DatabaseType, create_database};
use common::slot_timer::SlotTimer;
use common::types::DatabaseContext;
use relay::{ProposerLookaheadConfig, ProposerLookaheadTask, config::RelayConfig, run_relay_server, setup_logging};
use rocksdb::DB;
use std::env;
use std::sync::Arc;
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

	// Open database for proposer lookahead
	let db_path = config.relay.database_path.clone();
	let mut opts = rocksdb::Options::default();
	opts.create_if_missing(true);
	let db = DB::open(&opts, &db_path)?;
	let database = DatabaseContext::new(Arc::new(db));

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
