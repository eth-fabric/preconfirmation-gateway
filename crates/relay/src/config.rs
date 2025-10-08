use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the relay server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayConfig {
	pub relay: RelayServerConfig,
	pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayServerConfig {
	pub port: u16,
	pub database_path: PathBuf,
	pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
	pub max_delegations_per_slot: usize,
	pub max_constraints_per_slot: usize,
}

impl Default for RelayConfig {
	fn default() -> Self {
		Self {
			relay: RelayServerConfig {
				port: 8080,
				database_path: PathBuf::from("data/relay-rocksdb"),
				log_level: "info".to_string(),
			},
			storage: StorageConfig { max_delegations_per_slot: 100, max_constraints_per_slot: 1000 },
		}
	}
}

impl RelayConfig {
	/// Load configuration from file
	pub fn from_file(path: &str) -> eyre::Result<Self> {
		let content = std::fs::read_to_string(path)?;
		let config: RelayConfig = toml::from_str(&content)?;
		Ok(config)
	}

	/// Save configuration to file
	pub fn to_file(&self, path: &str) -> eyre::Result<()> {
		let content = toml::to_string_pretty(self)?;
		std::fs::write(path, content)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::TempDir;

	#[test]
	fn test_config_default() {
		let config = RelayConfig::default();
		assert_eq!(config.relay.port, 8080);
		assert_eq!(config.relay.database_path, PathBuf::from("data/relay-rocksdb"));
		assert_eq!(config.storage.max_delegations_per_slot, 100);
	}

	#[test]
	fn test_config_serialization() {
		let config = RelayConfig::default();
		let serialized = toml::to_string(&config).unwrap();
		let deserialized: RelayConfig = toml::from_str(&serialized).unwrap();
		assert_eq!(config.relay.port, deserialized.relay.port);
	}

	#[test]
	fn test_config_file_operations() {
		let temp_dir = TempDir::new().unwrap();
		let config_path = temp_dir.path().join("relay.toml");

		let config = RelayConfig::default();
		config.to_file(config_path.to_str().unwrap()).unwrap();

		let loaded_config = RelayConfig::from_file(config_path.to_str().unwrap()).unwrap();
		assert_eq!(config.relay.port, loaded_config.relay.port);
	}
}
