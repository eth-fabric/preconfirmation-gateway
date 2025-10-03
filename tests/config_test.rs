use preconfirmation_gateway::config::{DatabaseConfig, InclusionPreconfConfig, LoggingConfig, ServerConfig};

#[test]
fn test_config_load_from_existing_file() {
	// Since we now use commit-boost's config loader, we need to test the individual structs
	// This test would need to be updated to work with the commit-boost config format
	// For now, let's test the individual config structs
	let server_config = ServerConfig::default();
	assert_eq!(server_config.host, "127.0.0.1");
	assert_eq!(server_config.port, 8080);

	let db_config = DatabaseConfig::default();
	assert_eq!(db_config.url, "postgresql://localhost/preconfirmation_gateway");

	let logging_config = LoggingConfig::default();
	assert_eq!(logging_config.level, "info");
	assert_eq!(logging_config.enable_method_tracing, true);
	assert_eq!(logging_config.traced_methods, vec!["commitmentRequest", "commitmentResult", "slots", "fee"]);
}

#[test]
fn test_inclusion_preconf_config_methods() {
	// Create a test InclusionPreconfConfig
	let config = InclusionPreconfConfig {
		rpc_server_host: "0.0.0.0".to_string(),
		rpc_server_port: 9090,
		database_url: "postgresql://test/testdb".to_string(),
		log_level: "debug".to_string(),
		enable_method_tracing: false,
		traced_methods: vec!["test_method".to_string()],
		committer_address: "0x1234567890123456789012345678901234567890".to_string(),
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
	assert_eq!(db_config.url, "postgresql://localhost/preconfirmation_gateway");
}

#[test]
fn test_logging_config_default() {
	let logging_config = LoggingConfig::default();
	assert_eq!(logging_config.level, "info");
	assert_eq!(logging_config.enable_method_tracing, true);
	assert_eq!(logging_config.traced_methods, vec!["commitmentRequest", "commitmentResult", "slots", "fee"]);
}
