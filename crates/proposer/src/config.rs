use serde::Deserialize;

/// Configuration for the proposer service
#[derive(Debug, Clone, Deserialize)]
pub struct ProposerConfig {
	/// Proposer's own BLS public key (consensus key)
	pub proposer_bls_public_key: String,

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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_proposer_config_structure() {
		// Verify the config can be deserialized from JSON
		let json = r#"{
			"proposer_bls_public_key": "0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			"delegate_bls_public_key": "0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
			"committer_address": "0x1111111111111111111111111111111111111111",
			"relay_url": "http://localhost:3001",
			"relay_api_key": null,
			"beacon_api_url": "https://ethereum-beacon-api.publicnode.com",
			"beacon_genesis_timestamp": 1606824023,
			"poll_interval_seconds": 60
		}"#;

		let config: ProposerConfig = serde_json::from_str(json).unwrap();
		assert_eq!(config.poll_interval_seconds, 60);
		assert_eq!(config.beacon_genesis_timestamp, 1606824023);
	}
}
