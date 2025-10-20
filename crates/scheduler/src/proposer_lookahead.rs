use cb_common::types::BlsSecretKey;
use commit_boost::prelude::BlsPublicKey;
use eyre::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

use common::slot_timer::SlotTimer;
use common::types::DatabaseContext;

/// Configuration for the proposer lookahead task
#[derive(Debug, Clone)]
pub struct ProposerLookaheadConfig {
	/// How often to update the lookahead (in seconds)
	pub update_interval_seconds: u64,
	/// Number of slots to look ahead
	pub lookahead_window: u64,
}

/// Proposer lookahead task that periodically updates the proposer schedule
pub struct ProposerLookaheadTask {
	config: ProposerLookaheadConfig,
	slot_timer: SlotTimer,
	database: DatabaseContext,
}

impl ProposerLookaheadTask {
	/// Create a new proposer lookahead task
	pub fn new(config: ProposerLookaheadConfig, slot_timer: SlotTimer, database: DatabaseContext) -> Self {
		Self { config, slot_timer, database }
	}

	/// Run the proposer lookahead task continuously
	pub async fn run(&self) -> Result<()> {
		info!(
			"Starting proposer lookahead task with {}s update interval and {} slot lookahead",
			self.config.update_interval_seconds, self.config.lookahead_window
		);

		loop {
			if let Err(e) = self.update_lookahead().await {
				error!("Error updating proposer lookahead: {}", e);
			}

			sleep(Duration::from_secs(self.config.update_interval_seconds)).await;
		}
	}

	/// Update the proposer lookahead for upcoming slots
	async fn update_lookahead(&self) -> Result<()> {
		let current_slot = self.slot_timer.get_current_slot();
		let lookahead_end = current_slot + self.config.lookahead_window;

		info!("Updating proposer lookahead for slots {} to {}", current_slot, lookahead_end);

		// TODO: Replace with beacon node API call to fetch actual proposer schedule
		// For now, populate with deterministic fake BLS keys
		self.populate_lookahead(current_slot, lookahead_end, None).await?;

		Ok(())
	}

	/// Populate the proposer lookahead with BLS public keys
	/// This is a public method that can be called from tests or for manual population
	/// If proposer_key is provided, all slots will use that key (useful for testing)
	/// Otherwise, random fake keys are generated per slot
	pub async fn populate_lookahead(
		&self,
		start_slot: u64,
		end_slot: u64,
		proposer_key: Option<BlsPublicKey>,
	) -> Result<()> {
		for slot in start_slot..=end_slot {
			let proposer_pubkey = match &proposer_key {
				Some(key) => key.clone(),
				None => BlsSecretKey::random().public_key(), //todo remove this when beacon node integration is complete
			};

			self.database.store_proposer_lookahead(slot, &proposer_pubkey)?;
		}

		info!("Populated proposer lookahead for slots {} to {}", start_slot, end_slot);
		Ok(())
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
	fn test_proposer_lookahead_config() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };
		assert_eq!(config.update_interval_seconds, 60);
		assert_eq!(config.lookahead_window, 64);
	}

	#[test]
	fn test_proposer_lookahead_task_creation() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

		// Create a mock database context for testing
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let slot_timer = SlotTimer::new(1606824023); // Mainnet genesis
		let _task = ProposerLookaheadTask::new(config, slot_timer, database);
		// Test passes if creation doesn't panic
	}

	#[tokio::test]
	async fn test_populate_lookahead() {
		let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

		// Create a mock database context for testing
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		let slot_timer = SlotTimer::new(1606824023);
		let task = ProposerLookaheadTask::new(config, slot_timer, database.clone());

		// Populate lookahead for slots 100-105
		task.populate_lookahead(100, 105, None).await.unwrap();

		// Verify that keys were stored
		for slot in 100..=105 {
			let proposer = database.get_proposer_lookahead(slot).unwrap();
			assert!(proposer.is_some(), "Proposer key should be stored for slot {}", slot);
		}
	}
}
