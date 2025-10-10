use serde::Deserialize;

// Individual configuration structs for backward compatibility
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
	pub host: String,
	pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
	pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
	pub level: String,
	pub enable_method_tracing: bool,
	pub traced_methods: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsConfig {
	pub server_host: String,
	pub server_port: u16,
	pub relay_url: String,
	pub api_key: Option<String>,
	pub bls_public_key: String,
	pub delegate_public_key: String,
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

// Default implementations for individual config structs
impl Default for ServerConfig {
	fn default() -> Self {
		Self { host: "127.0.0.1".to_string(), port: 8080 }
	}
}

impl Default for DatabaseConfig {
	fn default() -> Self {
		Self { url: "./data/rocksdb".to_string() }
	}
}

impl Default for LoggingConfig {
	fn default() -> Self {
		Self {
			level: "info".to_string(),
			enable_method_tracing: true,
			traced_methods: vec![
				"commitmentRequest".to_string(),
				"commitmentResult".to_string(),
				"slots".to_string(),
				"fee".to_string(),
			],
		}
	}
}

impl Default for ConstraintsConfig {
	fn default() -> Self {
		Self {
			server_host: "127.0.0.1".to_string(),
			server_port: 8081,
			relay_url: "https://relay.example.com".to_string(),
			api_key: None,
			bls_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			delegate_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
		}
	}
}

// Methods to extract individual config structs from InclusionPreconfConfig
impl InclusionPreconfConfig {
	pub fn server(&self) -> ServerConfig {
		ServerConfig { host: self.commitments_server_host.clone(), port: self.commitments_server_port }
	}

	pub fn database(&self) -> DatabaseConfig {
		DatabaseConfig { url: self.commitments_database_url.clone() }
	}

	pub fn logging(&self) -> LoggingConfig {
		LoggingConfig {
			level: self.log_level.clone(),
			enable_method_tracing: self.enable_method_tracing,
			traced_methods: self.traced_methods.clone(),
		}
	}

	pub fn constraints(&self) -> ConstraintsConfig {
		ConstraintsConfig {
			server_host: "127.0.0.1".to_string(), // Not used anymore
			server_port: 8081,                    // Not used anymore
			relay_url: self.constraints_relay_url.clone(),
			api_key: self.constraints_api_key.clone(),
			bls_public_key: self.constraints_bls_public_key.clone(),
			delegate_public_key: self.constraints_delegate_public_key.clone(),
		}
	}

	pub fn database_url(&self) -> &str {
		&self.commitments_database_url
	}

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

		// Test constraints config access
		let constraints_config = config.constraints();

		// Test scheduler config access
		assert_eq!(config.eth_genesis_timestamp(), 1606824023);
		assert_eq!(constraints_config.relay_url, "https://relay.example.com");
		assert_eq!(constraints_config.api_key, Some("test-api-key".to_string()));
	}
}
