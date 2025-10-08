use serde::{Deserialize, Serialize};

/// Configuration for the scheduler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
	/// Ethereum genesis timestamp
	pub eth_genesis_timestamp: u64,
}

impl Default for SchedulerConfig {
	fn default() -> Self {
		Self {
            eth_genesis_timestamp: 1606824023, // Mainnet genesis
        }
	}
}

impl SchedulerConfig {
	/// Create a new scheduler config
	pub fn new(eth_genesis_timestamp: u64) -> Self {
		Self { eth_genesis_timestamp }
	}

	/// Create config for mainnet
	pub fn mainnet() -> Self {
		Self { eth_genesis_timestamp: 1606824023 }
	}

	/// Create config for testnet
	pub fn testnet() -> Self {
		Self {
            eth_genesis_timestamp: 1695902400, // Holesky testnet
        }
	}
}
