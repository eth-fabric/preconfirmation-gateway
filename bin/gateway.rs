use cb_common::utils::bls_pubkey_from_hex;
use commit_boost::prelude::*;
use commitments::{CommitmentsServerState, server};
use common::db::{DatabaseType, create_database};
use common::execution::{ExecutionApiClient, ExecutionApiConfig};
use common::slot_timer::SlotTimer;
use common::{config, types};
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

	// Load consolidated configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<config::InclusionPreconfConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	let app_config = commit_config.extra.clone();
	info!("Using Ethereum genesis timestamp: {}", app_config.eth_genesis_timestamp);

	// Create slot timer
	let slot_timer = SlotTimer::new(app_config.eth_genesis_timestamp);

	// Create databases
	let commitments_db = create_database(&commit_config, DatabaseType::Commitments)
		.map_err(|e| eyre::eyre!("Failed to create commitments database: {}", e))?;
	let commitments_database = types::DatabaseContext::new(commitments_db);

	let delegations_db = create_database(&commit_config, DatabaseType::Delegations)
		.map_err(|e| eyre::eyre!("Failed to create delegations database: {}", e))?;
	let delegations_database = types::DatabaseContext::new(delegations_db);

	// Get BLS public key for commitments server
	let bls_public_key = bls_pubkey_from_hex(&app_config.constraints_bls_public_key)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;

	// Create execution client for commitments server
	let execution_config = ExecutionApiConfig {
		endpoint: app_config.execution_endpoint_url.clone(),
		request_timeout_secs: app_config.execution_request_timeout_secs,
		max_retries: app_config.execution_max_retries,
	};
	let execution_client = Arc::new(ExecutionApiClient::with_default_client(execution_config)?);

	// Create commitments server state
	let commitments_state = CommitmentsServerState::new(
		commitments_database,
		commit_config,
		bls_public_key,
		app_config.constraints_relay_url.clone(),
		app_config.constraints_api_key.clone(),
		slot_timer.clone(),
		execution_client,
	);

	// Spawn commitments RPC server
	info!(
		"Starting commitments RPC server on {}:{}",
		app_config.commitments_server_host, app_config.commitments_server_port
	);
	tokio::spawn(async move {
		if let Err(e) = server::run_server(commitments_state).await {
			tracing::error!("Commitments server error: {}", e);
		}
	});

	// Create shared commit config for gateway tasks
	let shared_commit_config = Arc::new(Mutex::new(load_commit_module_config::<config::InclusionPreconfConfig>()?));

	// Create task coordinator for gateway tasks
	let mut coordinator = TaskCoordinator::new();

	// Create delegation task configuration
	let delegation_config = DelegationTaskConfig {
		check_interval_seconds: 1, // 1 second interval as requested
		lookahead_window: 64,      // 64 slots lookahead
	};

	// Create delegation task
	let delegation_task = DelegationTask::new(
		delegation_config,
		app_config.clone(),
		slot_timer.clone(),
		delegations_database.clone(),
		app_config.constraints_relay_url.clone(),
		app_config.constraints_api_key.clone(),
	);

	// Create constraints task
	let constraints_task = ConstraintsTask::new(
		app_config.clone(),
		slot_timer.clone(),
		delegations_database.clone(),
		shared_commit_config.clone(),
		app_config.constraints_relay_url.clone(),
		app_config.constraints_api_key.clone(),
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
