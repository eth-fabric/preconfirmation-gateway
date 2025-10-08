use eyre::Result;
use scheduler::{SchedulerConfig, SlotTimer, TaskCoordinator};
use std::time::Duration;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	info!("Starting scheduler service");

	// Load scheduler config
	let config = SchedulerConfig::mainnet();
	info!("Using Ethereum genesis timestamp: {}", config.eth_genesis_timestamp);

	// Create slot timer
	let slot_timer = SlotTimer::new(config.eth_genesis_timestamp);

	// Create task coordinator
	let mut coordinator = TaskCoordinator::new();

	// Spawn tasks here
	coordinator.spawn_task("delegation_task".to_string(), || async {
		// Delegation processing logic
		info!("Processing delegations");
		Ok(())
	});

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
