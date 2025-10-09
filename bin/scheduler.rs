use commit_boost::prelude::*;
use common::db::{DatabaseType, create_database};
use common::{config, types};
use eyre::Result;
use scheduler::{ConstraintsTask, DelegationTask, DelegationTaskConfig, SlotTimer, TaskCoordinator};
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	info!("Starting scheduler service");

	// Load consolidated configuration using commit-boost's config loader
	let commit_config = load_commit_module_config::<config::InclusionPreconfConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	let app_config = &commit_config.extra;
	info!("Using Ethereum genesis timestamp: {}", app_config.eth_genesis_timestamp);

	// Create slot timer
	let slot_timer = SlotTimer::new(app_config.eth_genesis_timestamp);

	// Create delegations database
	let delegations_db = create_database(&commit_config, DatabaseType::Delegations)
		.map_err(|e| eyre::eyre!("Failed to create delegations database: {}", e))?;
	let delegations_database = types::DatabaseContext::new(delegations_db);

	// Create task coordinator
	let mut coordinator = TaskCoordinator::new();

	// Create delegation task configuration
	let delegation_config = DelegationTaskConfig {
		check_interval_seconds: 1, // 1 second interval as requested
		lookahead_window: 64,      // 64 slots lookahead
	};

	// Create delegation task
	let delegation_task =
		DelegationTask::new(delegation_config, app_config.clone(), slot_timer.clone(), delegations_database.clone());

	// Create constraints task
	let constraints_task = ConstraintsTask::new(app_config.clone(), slot_timer.clone(), delegations_database.clone());

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
