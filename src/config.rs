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
}

// Default implementations for individual config structs
impl Default for ServerConfig {
	fn default() -> Self {
		Self { host: "127.0.0.1".to_string(), port: 8080 }
	}
}

impl Default for DatabaseConfig {
	fn default() -> Self {
		Self { url: "postgresql://localhost/preconfirmation_gateway".to_string() }
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

	pub fn database_url(&self) -> &str {
		&self.database_url
	}
}
