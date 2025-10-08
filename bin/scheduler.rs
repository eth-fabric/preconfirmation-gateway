use common::config::InclusionPreconfConfig;
use eyre::Result;
use scheduler::{SlotTimer, TaskCoordinator};
use std::time::Duration;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	info!("Starting scheduler service");

	// Load scheduler config
	let config = InclusionPreconfConfig {
		commitments_server_host: "127.0.0.1".to_string(),
		commitments_server_port: 8080,
		commitments_database_url: "./data/rocksdb".to_string(),
		constraints_database_url: "./data/constraints-rocksdb".to_string(),
		delegations_database_url: "./data/delegations-rocksdb".to_string(),
		pricing_database_url: "./data/pricing-rocksdb".to_string(),
		log_level: "info".to_string(),
		enable_method_tracing: false,
		traced_methods: vec![],
		committer_address: "0x0000000000000000000000000000000000000000".to_string(),
		constraints_server_host: "127.0.0.1".to_string(),
		constraints_server_port: 8081,
		constraints_relay_url: "https://relay.example.com".to_string(),
		constraints_api_key: None,
		constraints_bls_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		constraints_delegate_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		eth_genesis_timestamp: 1606824023, // Mainnet genesis
	};
	info!("Using Ethereum genesis timestamp: {}", config.eth_genesis_timestamp());

	// Create slot timer
	let slot_timer = SlotTimer::new(config.eth_genesis_timestamp());

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
