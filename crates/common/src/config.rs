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

// ==================== Configuration Trait System ====================

/// Commitments service configuration trait
/// Defines the minimal interface required by the commitments RPC server
pub trait CommitmentsConfig {
	fn server_host(&self) -> &str;
	fn server_port(&self) -> u16;
	fn database_url(&self) -> &str;
	fn log_level(&self) -> &str;
	fn bls_public_key(&self) -> &str;
}

/// Gateway configuration trait
/// Defines the interface for gateway orchestration configuration
/// Nests a CommitmentsConfig for the commitments server component
pub trait GatewayConfig {
	type CommitmentsConfig: CommitmentsConfig;

	fn commitments_config(&self) -> &Self::CommitmentsConfig;
	fn relay_url(&self) -> &str;
	fn constraints_api_key(&self) -> Option<&str>;
	fn genesis_timestamp(&self) -> u64;
	fn delegation_database_url(&self) -> &str;
	fn execution_endpoint_url(&self) -> &str;
	fn execution_request_timeout_secs(&self) -> u64;
	fn execution_max_retries(&self) -> u32;
	fn constraints_receivers(&self) -> &[String];
}

/// Proposer configuration trait
/// Defines the interface for proposer service configuration
pub trait ProposerConfig {
	fn proposer_bls_public_key(&self) -> &str;
	fn delegate_bls_public_key(&self) -> &str;
	fn relay_url(&self) -> &str;
	fn beacon_api_url(&self) -> &str;
	fn genesis_timestamp(&self) -> u64;
}

/// Relay configuration trait
/// Defines the interface for relay service configuration
pub trait RelayConfig {
	fn port(&self) -> u16;
	fn database_path(&self) -> &str;
	fn log_level(&self) -> &str;
	fn genesis_timestamp(&self) -> u64;
	fn constraint_capabilities(&self) -> &[u64];
}

// ==================== Concrete Inclusion Preconf Configurations ====================

/// Commitments service configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionCommitmentsConfig {
	pub server_host: String,
	pub server_port: u16,
	pub database_url: String,
	pub log_level: String,
	pub bls_public_key: String,
	// Inclusion-specific extras
	pub enable_method_tracing: bool,
	pub traced_methods: Vec<String>,
}

impl CommitmentsConfig for InclusionCommitmentsConfig {
	fn server_host(&self) -> &str {
		&self.server_host
	}

	fn server_port(&self) -> u16 {
		self.server_port
	}

	fn database_url(&self) -> &str {
		&self.database_url
	}

	fn log_level(&self) -> &str {
		&self.log_level
	}

	fn bls_public_key(&self) -> &str {
		&self.bls_public_key
	}
}

/// Gateway configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionGatewayConfig {
	/// Nested commitments configuration
	pub commitments: InclusionCommitmentsConfig,

	/// Gateway orchestration configuration
	pub relay_url: String,
	pub constraints_api_key: Option<String>,
	pub genesis_timestamp: u64,
	pub delegation_database_url: String,

	/// Execution client configuration
	pub execution_endpoint_url: String,
	pub execution_request_timeout_secs: u64,
	pub execution_max_retries: u32,

	/// Constraints receivers
	pub constraints_receivers: Vec<String>,
	pub delegate_public_key: String,
}

impl GatewayConfig for InclusionGatewayConfig {
	type CommitmentsConfig = InclusionCommitmentsConfig;

	fn commitments_config(&self) -> &Self::CommitmentsConfig {
		&self.commitments
	}

	fn relay_url(&self) -> &str {
		&self.relay_url
	}

	fn constraints_api_key(&self) -> Option<&str> {
		self.constraints_api_key.as_deref()
	}

	fn genesis_timestamp(&self) -> u64 {
		self.genesis_timestamp
	}

	fn delegation_database_url(&self) -> &str {
		&self.delegation_database_url
	}

	fn execution_endpoint_url(&self) -> &str {
		&self.execution_endpoint_url
	}

	fn execution_request_timeout_secs(&self) -> u64 {
		self.execution_request_timeout_secs
	}

	fn execution_max_retries(&self) -> u32 {
		self.execution_max_retries
	}

	fn constraints_receivers(&self) -> &[String] {
		&self.constraints_receivers
	}
}

/// Proposer configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProposerConfig {
	pub proposer_bls_public_key: String,
	pub delegate_bls_public_key: String,
	pub committer_address: String,
	pub relay_url: String,
	pub relay_api_key: Option<String>,
	pub beacon_api_url: String,
	pub beacon_genesis_timestamp: u64,
	pub poll_interval_seconds: u64,
}

impl ProposerConfig for InclusionProposerConfig {
	fn proposer_bls_public_key(&self) -> &str {
		&self.proposer_bls_public_key
	}

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

	// Reth RPC configuration
	pub execution_endpoint_url: String,
	pub execution_request_timeout_secs: u64,
	pub execution_max_retries: u32,
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

	// Reth RPC configuration access
	pub fn execution_endpoint_url(&self) -> &str {
		&self.execution_endpoint_url
	}

	pub fn execution_request_timeout_secs(&self) -> u64 {
		self.execution_request_timeout_secs
	}

	pub fn execution_max_retries(&self) -> u32 {
		self.execution_max_retries
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
			execution_endpoint_url: "http://localhost:8545".to_string(),
			execution_request_timeout_secs: 10,
			execution_max_retries: 3,
		};

		// Test direct field access
		assert_eq!(config.constraints_relay_url, "https://relay.example.com");
		assert_eq!(config.constraints_api_key, Some("test-api-key".to_string()));

		// Test scheduler config access
		assert_eq!(config.eth_genesis_timestamp(), 1606824023);
	}
}
