use cb_common::utils::bls_pubkey_from_hex;
use commit_boost::prelude::*;
use commitments::{CommitmentsServerState, server};
use common::config::{CommitmentsConfig, GatewayConfig, InclusionGatewayConfig};
use common::db::create_database;
use common::slot_timer::SlotTimer;
use common::types;
use eyre::Result;
use gateway::{ConstraintsTask, DelegationTask, DelegationTaskConfig, TaskCoordinator};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	info!("Starting gateway service (commitments server + gateway tasks)");

	// Load gateway configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<InclusionGatewayConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	// Wrap in Arc<Mutex<>> so it can be shared between tasks
	let shared_commit_config = Arc::new(Mutex::new(commit_config));

	// Extract config values
	let gateway_config = {
		let config_guard = shared_commit_config.lock().await;
		config_guard.extra.clone()
	};
	let commitments_config = gateway_config.commitments_config();
	let genesis_timestamp = gateway_config.genesis_timestamp();
	info!("Using Ethereum genesis timestamp: {}", genesis_timestamp);

	// Create slot timer
	let slot_timer = SlotTimer::new(genesis_timestamp);

	// Create databases
	let commitments_db = create_database(commitments_config.database_path())
		.map_err(|e| eyre::eyre!("Failed to create commitments database: {}", e))?;
	let commitments_database = types::DatabaseContext::new(commitments_db);

	let delegations_db = create_database(gateway_config.delegation_database_path())
		.map_err(|e| eyre::eyre!("Failed to create delegations database: {}", e))?;
	let delegations_database = types::DatabaseContext::new(delegations_db);

	// Get BLS public key for commitments server
	let bls_public_key = bls_pubkey_from_hex(commitments_config.bls_public_key())
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;

	// Create provider for commitments server (built on demand in utils)

	// Create commitments server state (CommitmentsServerState will handle wrapping in Arc<Mutex<>>)
	// We need to temporarily take ownership to construct the state, but CommitmentsServerState
	// wraps it in Arc<Mutex<>> internally, so we can't share the same Arc
	// Instead, we reconstruct the config by reading from the shared Arc
	let commitments_state = {
		let config_guard = shared_commit_config.lock().await;
		let commit_config_for_server = StartCommitModuleConfig {
			id: config_guard.id.clone(),
			chain: config_guard.chain,
			signer_client: config_guard.signer_client.clone(),
			extra: config_guard.extra.clone(),
		};
		drop(config_guard);

		CommitmentsServerState::new(
			commitments_database,
			commit_config_for_server,
			bls_public_key,
			gateway_config.relay_url().to_string(),
			gateway_config.constraints_api_key().map(|s| s.to_string()),
			slot_timer.clone(),
		)
	};

	// Spawn commitments RPC server
	info!(
		"Starting commitments RPC server on {}:{}",
		commitments_config.server_host(),
		commitments_config.server_port()
	);
	tokio::spawn(async move {
		if let Err(e) = server::run_server(commitments_state).await {
			tracing::error!("Commitments server error: {}", e);
		}
	});

	// Create task coordinator for gateway tasks
	let mut coordinator = TaskCoordinator::new();

	// Create delegation task configuration
	let delegation_config = DelegationTaskConfig {
		check_interval_seconds: 1, // 1 second interval as requested
		lookahead_window: 64,      // 64 slots lookahead
	};

	// Clone gateway config for tasks
	let gateway_config_clone = gateway_config.clone();
	let gateway_config_clone2 = gateway_config.clone();

	// Create delegation task
	let delegation_task = DelegationTask::new(
		delegation_config,
		gateway_config_clone,
		slot_timer.clone(),
		delegations_database.clone(),
		gateway_config.relay_url().to_string(),
		gateway_config.constraints_api_key().map(|s| s.to_string()),
	);

	// Create constraints task
	let constraints_task = ConstraintsTask::new(
		gateway_config_clone2,
		slot_timer.clone(),
		delegations_database.clone(),
		shared_commit_config.clone(),
		gateway_config.relay_url().to_string(),
		gateway_config.constraints_api_key().map(|s| s.to_string()),
	);

	// Spawn delegation task
	coordinator.spawn_task("delegation_task".to_string(), move || async move { delegation_task.run().await });

	// Spawn constraints task
	coordinator.spawn_task("constraints_task".to_string(), move || async move { constraints_task.run().await });

	info!("Gateway initialized with commitments server and {} gateway tasks", coordinator.task_count());

	// Run scheduler loop - tasks run independently with their own timing
	loop {
		let current_slot = slot_timer.get_current_slot();
		info!("Current slot: {}", current_slot);

		// Tasks run independently, just log current slot periodically
		tokio::time::sleep(Duration::from_secs(12)).await;
	}
}
