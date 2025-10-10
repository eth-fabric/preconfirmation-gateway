use commit_boost::prelude::*;
use common::db::{DatabaseType, create_database};
use common::{config, types};
use eyre::Result;
use scheduler::{ConstraintsTask, DelegationTask, DelegationTaskConfig, SlotTimer, TaskCoordinator};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	info!("Starting scheduler service");

	// Load consolidated configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<config::InclusionPreconfConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	let app_config = commit_config.extra.clone();
	info!("Using Ethereum genesis timestamp: {}", app_config.eth_genesis_timestamp);

	// Create slot timer
	let slot_timer = SlotTimer::new(app_config.eth_genesis_timestamp);

	// Create delegations database
	let delegations_db = create_database(&commit_config, DatabaseType::Delegations)
		.map_err(|e| eyre::eyre!("Failed to create delegations database: {}", e))?;
	let delegations_database = types::DatabaseContext::new(delegations_db);

	// Create shared commit config for tasks
	let shared_commit_config = Arc::new(Mutex::new(commit_config));

	// Parse BLS public key for constraints
	let bls_public_key = cb_common::utils::bls_pubkey_from_hex(&app_config.constraints_bls_public_key)
		.map_err(|e| eyre::eyre!("Failed to parse BLS public key: {}", e))?;

	// Create task coordinator
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
		bls_public_key,
		app_config.constraints_relay_url.clone(),
		app_config.constraints_api_key.clone(),
	);

	// Spawn delegation task
	coordinator.spawn_task("delegation_task".to_string(), move || async move { delegation_task.run().await });

	// Spawn constraints task
	coordinator.spawn_task("constraints_task".to_string(), move || async move { constraints_task.run().await });

	info!("Scheduler initialized with {} tasks", coordinator.task_count());

	// Run scheduler loop - tasks run independently with their own timing
	loop {
		let current_slot = slot_timer.get_current_slot();
		info!("Current slot: {}", current_slot);

		// Tasks run independently, just log current slot periodically
		tokio::time::sleep(Duration::from_secs(12)).await;
	}
}
