use serde::Deserialize;

/// Configuration for the proposer service
#[derive(Debug, Clone, Deserialize)]
pub struct ProposerConfig {
	/// Gateway delegate BLS public key
	pub delegate_bls_public_key: String,

	/// Committer address (ECDSA address)
	pub committer_address: String,

	/// Relay URL to post delegations to
	pub relay_url: String,

	/// Optional API key for relay authentication
	pub relay_api_key: Option<String>,

	/// Beacon API URL for fetching proposer duties
	pub beacon_api_url: String,

	/// Beacon chain genesis timestamp (Unix timestamp)
	pub beacon_genesis_timestamp: u64,

	/// How often to poll for proposer duties (in seconds)
	pub poll_interval_seconds: u64,

	/// Module signing ID for this proposer instance
	pub module_signing_id: String,

	/// URC owner address for registration
	pub urc_owner: String,

	/// Execution RPC endpoint for sending transactions
	pub execution_rpc_url: String,

	/// Collateral amount to send with registration (in wei)
	pub registration_collateral_wei: String,

	/// Path to the RocksDB database for storing delegations (for equivocation prevention)
	pub delegation_db_path: String,
}

impl common::config::ProposerConfig for ProposerConfig {
	fn delegate_bls_public_key(&self) -> &str {
		&self.delegate_bls_public_key
	}

	fn relay_url(&self) -> &str {
		&self.relay_url
	}

	fn beacon_api_url(&self) -> &str {
		&self.beacon_api_url
	}

	fn genesis_timestamp(&self) -> u64 {
		self.beacon_genesis_timestamp
	}

	fn module_signing_id(&self) -> &str {
		&self.module_signing_id
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_proposer_config_structure() {
		// Verify the config can be deserialized from JSON
		let json = r#"{
			"delegate_bls_public_key": "0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
			"committer_address": "0x1111111111111111111111111111111111111111",
			"relay_url": "http://localhost:3001",
			"relay_api_key": null,
			"beacon_api_url": "https://ethereum-beacon-api.publicnode.com",
			"beacon_genesis_timestamp": 1606824023,
			"poll_interval_seconds": 60,
			"module_signing_id": "0x1111111111111111111111111111111111111111111111111111111111111111",
			"urc_owner": "0x2222222222222222222222222222222222222222",
			"execution_rpc_url": "http://localhost:8545",
			"registration_collateral_wei": "1000000000000000000",
			"delegation_db_path": "data/proposer-delegations-rocksdb"
		}"#;

		let config: ProposerConfig = serde_json::from_str(json).unwrap();
		assert_eq!(config.poll_interval_seconds, 60);
		assert_eq!(config.beacon_genesis_timestamp, 1606824023);
	}
}
