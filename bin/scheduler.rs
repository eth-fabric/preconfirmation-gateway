use commit_boost::prelude::*;
use common::db::{DatabaseType, create_database};
use common::{config, types};
use eyre::Result;
use scheduler::{DelegationTask, DelegationTaskConfig, SlotTimer, TaskCoordinator};
use std::time::Duration;
use tracing::{error, info};

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
		constraints_service_url: format!(
			"http://{}:{}",
			app_config.constraints_server_host, app_config.constraints_server_port
		),
		delegate_bls_public_key: app_config.constraints_delegate_public_key.clone(),
	};

	// Create delegation task
	let delegation_task = DelegationTask::new(delegation_config, slot_timer.clone(), delegations_database);

	// Spawn delegation task
	coordinator.spawn_task("delegation_task".to_string(), move || async move { delegation_task.run().await });

	coordinator.spawn_task("constraints_task".to_string(), || async {
		// Constraints processing logic
		info!("Processing constraints");
		Ok(())
	});

	info!("Scheduler initialized with {} tasks", coordinator.task_count());

	// Run scheduler loop
	loop {
		let current_slot = slot_timer.get_current_slot();
		info!("Current slot: {}", current_slot);

		// Wait for constraint window
		if !slot_timer.is_in_constraint_window() {
			let wait_time = slot_timer.time_until_next_constraint_window();
			info!("Waiting for next constraint window: {:?}", wait_time);
			tokio::time::sleep(wait_time).await;
			continue;
		}

		// Process tasks during constraint window
		info!("In constraint window, processing tasks");

		// Wait for all tasks to complete with timeout
		let timeout = Duration::from_secs(8); // 8-second constraint window
		if let Err(e) = coordinator.wait_for_all_with_timeout(timeout).await {
			error!("Task execution failed: {}", e);
		}

		// Wait for next slot
		tokio::time::sleep(Duration::from_secs(12)).await;
	}
}
