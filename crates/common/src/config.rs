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
pub struct CommitterConfig {
	pub address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsConfig {
	pub server_port: u16,
	pub relay_url: String,
	pub api_key: Option<String>,
	pub bls_public_key: String,
	pub proposer_public_key: String,
	pub delegate_public_key: String,
}

// Consolidated configuration that will be loaded by load_commit_module_config
#[derive(Debug, Clone, Deserialize)]
pub struct InclusionPreconfConfig {
	// RPC Server configuration
	pub rpc_server_host: String,
	pub rpc_server_port: u16,

	// Database configuration
	pub database_url: String,

	// Logging configuration
	pub log_level: String,
	pub enable_method_tracing: bool,
	pub traced_methods: Vec<String>,

	// Committer configuration
	pub committer_address: String,

	// Constraints configuration
	pub constraints_server_port: u16,
	pub constraints_relay_url: String,
	pub constraints_api_key: Option<String>,
	pub constraints_bls_public_key: String,
	pub constraints_proposer_public_key: String,
	pub constraints_delegate_public_key: String,
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

impl Default for CommitterConfig {
	fn default() -> Self {
		Self { address: "0x0000000000000000000000000000000000000000".to_string() }
	}
}

impl Default for ConstraintsConfig {
	fn default() -> Self {
		Self {
			server_port: 8081,
			relay_url: "https://relay.example.com".to_string(),
			api_key: None,
			bls_public_key:
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string(),
			proposer_public_key:
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
		ServerConfig { host: self.rpc_server_host.clone(), port: self.rpc_server_port }
	}

	pub fn database(&self) -> DatabaseConfig {
		DatabaseConfig { url: self.database_url.clone() }
	}

	pub fn logging(&self) -> LoggingConfig {
		LoggingConfig {
			level: self.log_level.clone(),
			enable_method_tracing: self.enable_method_tracing,
			traced_methods: self.traced_methods.clone(),
		}
	}

	pub fn committer(&self) -> CommitterConfig {
		CommitterConfig { address: self.committer_address.clone() }
	}

	pub fn constraints(&self) -> ConstraintsConfig {
		ConstraintsConfig {
			server_port: self.constraints_server_port,
			relay_url: self.constraints_relay_url.clone(),
			api_key: self.constraints_api_key.clone(),
			bls_public_key: self.constraints_bls_public_key.clone(),
			proposer_public_key: self.constraints_proposer_public_key.clone(),
			delegate_public_key: self.constraints_delegate_public_key.clone(),
		}
	}

	pub fn database_url(&self) -> &str {
		&self.database_url
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Test configuration loading and validation
	#[test]
	fn test_constraints_configuration() {
		let config = InclusionPreconfConfig {
			rpc_server_host: "127.0.0.1".to_string(),
			rpc_server_port: 9090,
			database_url: "test.db".to_string(),
			log_level: "debug".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],
			committer_address: "0x0000000000000000000000000000000000000000".to_string(),
			constraints_server_port: 8080,
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: Some("test-api-key".to_string()),
			constraints_bls_public_key:
				"010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101"
					.to_string(),
			constraints_proposer_public_key:
				"020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202"
					.to_string(),
			constraints_delegate_public_key:
				"030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303"
					.to_string(),
		};

		// Test constraints config access
		let constraints_config = config.constraints();
		assert_eq!(constraints_config.server_port, 8080);
		assert_eq!(constraints_config.relay_url, "https://relay.example.com");
		assert_eq!(constraints_config.api_key, Some("test-api-key".to_string()));
	}
}
