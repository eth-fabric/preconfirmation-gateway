use eyre::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

use common::types::DatabaseContext;
use pricer::PricerService;

/// Configuration for the pricer task
#[derive(Debug, Clone)]
pub struct PricerTaskConfig {
	/// How often to update the price (in seconds)
	pub update_interval_seconds: u64,
}

/// Pricer task that periodically updates gas prices
pub struct PricerTask {
	config: PricerTaskConfig,
	pricer_service: PricerService,
}

impl PricerTask {
	/// Create a new pricer task
	pub fn new(config: PricerTaskConfig, database: DatabaseContext) -> Self {
		let pricer_service = PricerService::new(database);
		Self { config, pricer_service }
	}

	/// Run the pricer task continuously
	pub async fn run(&self) -> Result<()> {
		info!("Starting pricer task with {}s update interval", self.config.update_interval_seconds);

		loop {
			if let Err(e) = self.update_price().await {
				error!("Error in price update: {}", e);
			}

			sleep(Duration::from_secs(self.config.update_interval_seconds)).await;
		}
	}

	/// Update the current price
	async fn update_price(&self) -> Result<()> {
		self.pricer_service.update_price().await
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use common::types::DatabaseContext;
	use rocksdb::{Options, DB};
	use std::sync::Arc;
	use tempfile::TempDir;

	#[test]
	fn test_pricer_task_config_default() {
		let config = PricerTaskConfig { update_interval_seconds: 12 };
		assert_eq!(config.update_interval_seconds, 12);
	}

	#[test]
	fn test_pricer_task_creation() {
		let config = PricerTaskConfig { update_interval_seconds: 12 };

		// Create a mock database context for testing
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let _task = PricerTask::new(config, database);
		// Test passes if creation doesn't panic
	}
}
