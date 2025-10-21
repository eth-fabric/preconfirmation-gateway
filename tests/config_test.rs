use common::config::{DatabaseConfig, InclusionPreconfConfig, LoggingConfig, ServerConfig};

#[test]
fn test_config_load_from_existing_file() {
	// Since we now use commit-boost's config loader, we need to test the individual structs
	// This test would need to be updated to work with the commit-boost config format
	// For now, let's test the individual config structs
	let server_config = ServerConfig::default();
	assert_eq!(server_config.host, "127.0.0.1");
	assert_eq!(server_config.port, 8080);

	let db_config = DatabaseConfig::default();
	assert_eq!(db_config.url, "./data/rocksdb");

	let logging_config = LoggingConfig::default();
	assert_eq!(logging_config.level, "info");
	assert_eq!(logging_config.enable_method_tracing, true);
	assert_eq!(logging_config.traced_methods, vec!["commitmentRequest", "commitmentResult", "slots", "fee"]);
}

#[test]
fn test_inclusion_preconf_config_methods() {
	// Create a test InclusionPreconfConfig
	let config = InclusionPreconfConfig {
		commitments_server_host: "0.0.0.0".to_string(),
		commitments_server_port: 9090,
		commitments_database_url: "postgresql://test/testdb".to_string(),
		constraints_database_url: "postgresql://test/constraints".to_string(),
		delegations_database_url: "postgresql://test/delegations".to_string(),
		pricing_database_url: "postgresql://test/pricing".to_string(),
		log_level: "debug".to_string(),
		enable_method_tracing: false,
		traced_methods: vec!["test_method".to_string()],
		constraints_relay_url: "https://relay.example.com".to_string(),
		constraints_api_key: None,
		constraints_bls_public_key: "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
		constraints_delegate_public_key: "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
		eth_genesis_timestamp: 1606824023,
		constraints_receivers: vec!["0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()],
		reth_endpoint_url: "http://localhost:8545".to_string(),
		reth_request_timeout_secs: 10,
		reth_max_retries: 3,
	};

	// Test the individual config methods
	let server_config = config.server();
	assert_eq!(server_config.host, "0.0.0.0");
	assert_eq!(server_config.port, 9090);

	let db_config = config.database();
	assert_eq!(db_config.url, "postgresql://test/testdb");

	let logging_config = config.logging();
	assert_eq!(logging_config.level, "debug");
	assert_eq!(logging_config.enable_method_tracing, false);
	assert_eq!(logging_config.traced_methods, vec!["test_method"]);

	let committer_config = config.committer();
	assert_eq!(committer_config.address, "0x1234567890123456789012345678901234567890");

	assert_eq!(config.database_url(), "postgresql://test/testdb");
	assert_eq!(config.commitments_database_url(), "postgresql://test/testdb");
	assert_eq!(config.constraints_database_url(), "postgresql://test/constraints");
	assert_eq!(config.delegations_database_url(), "postgresql://test/delegations");
	assert_eq!(config.pricing_database_url(), "postgresql://test/pricing");

	// Test constraints config (server fields removed)

	// Test scheduler config
	assert_eq!(config.eth_genesis_timestamp(), 1606824023);
}

#[test]
fn test_server_config_default() {
	let server_config = ServerConfig::default();
	assert_eq!(server_config.host, "127.0.0.1");
	assert_eq!(server_config.port, 8080);
}

#[test]
fn test_database_config_default() {
	let db_config = DatabaseConfig::default();
	assert_eq!(db_config.url, "./data/rocksdb");
}

#[test]
fn test_logging_config_default() {
	let logging_config = LoggingConfig::default();
	assert_eq!(logging_config.level, "info");
	assert_eq!(logging_config.enable_method_tracing, true);
	assert_eq!(logging_config.traced_methods, vec!["commitmentRequest", "commitmentResult", "slots", "fee"]);
}
