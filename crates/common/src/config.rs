use serde::{Deserialize, Serialize};

/// Configuration for Beacon API integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconApiConfig {
	/// Primary beacon node endpoint (e.g., Alchemy)
	pub primary_endpoint: String,
	/// Fallback beacon node endpoints
	pub fallback_endpoints: Vec<String>,
	/// Request timeout in seconds
	pub request_timeout_secs: u64,
	/// Beacon chain genesis time (Unix timestamp)
	pub genesis_time: u64,
}

// Consolidated configuration that will be loaded by load_commit_module_config
#[derive(Debug, Clone, Deserialize)]
pub struct InclusionPreconfConfig {
	// Commitments Server configuration
	pub commitments_server_host: String,
	pub commitments_server_port: u16,

	// Database configuration
	pub commitments_database_url: String,
	pub constraints_database_url: String,
	pub delegations_database_url: String,
	pub pricing_database_url: String,

	// Logging configuration
	pub log_level: String,
	pub enable_method_tracing: bool,
	pub traced_methods: Vec<String>,

	// Constraints configuration
	pub constraints_relay_url: String,
	pub constraints_api_key: Option<String>,
	pub constraints_bls_public_key: String,
	pub constraints_delegate_public_key: String,

	// Scheduler configuration
	pub eth_genesis_timestamp: u64,

	// Constraints receivers configuration
	pub constraints_receivers: Vec<String>,
}

// Configuration access methods
impl InclusionPreconfConfig {
	// New methods for accessing specific database URLs
	pub fn commitments_database_url(&self) -> &str {
		&self.commitments_database_url
	}

	pub fn constraints_database_url(&self) -> &str {
		&self.constraints_database_url
	}

	pub fn delegations_database_url(&self) -> &str {
		&self.delegations_database_url
	}

	pub fn pricing_database_url(&self) -> &str {
		&self.pricing_database_url
	}

	// Scheduler configuration access
	pub fn eth_genesis_timestamp(&self) -> u64 {
		self.eth_genesis_timestamp
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Test configuration loading and validation
	#[test]
	fn test_constraints_configuration() {
		let config = InclusionPreconfConfig {
			commitments_server_host: "127.0.0.1".to_string(),
			commitments_server_port: 9090,
			commitments_database_url: "test.db".to_string(),
			constraints_database_url: "constraints.db".to_string(),
			delegations_database_url: "delegations.db".to_string(),
			pricing_database_url: "pricing.db".to_string(),
			log_level: "debug".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: Some("test-api-key".to_string()),
			constraints_bls_public_key:
				"010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101"
					.to_string(),
			constraints_delegate_public_key:
				"030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303"
					.to_string(),
			eth_genesis_timestamp: 1606824023, // Mainnet genesis
			constraints_receivers: vec![
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
			],
		};

		// Test direct field access
		assert_eq!(config.constraints_relay_url, "https://relay.example.com");
		assert_eq!(config.constraints_api_key, Some("test-api-key".to_string()));

		// Test scheduler config access
		assert_eq!(config.eth_genesis_timestamp(), 1606824023);
	}
}
