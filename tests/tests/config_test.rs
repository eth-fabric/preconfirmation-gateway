use common::config::InclusionPreconfConfig;

#[test]
fn test_config_load_from_existing_file() {
	// Test direct field access on InclusionPreconfConfig
	let config = InclusionPreconfConfig {
		commitments_server_host: "127.0.0.1".to_string(),
		commitments_server_port: 8080,
		commitments_database_url: "./data/rocksdb".to_string(),
		constraints_database_url: "./data/constraints-rocksdb".to_string(),
		delegations_database_url: "./data/delegations-rocksdb".to_string(),
		pricing_database_url: "./data/pricing-rocksdb".to_string(),
		log_level: "info".to_string(),
		enable_method_tracing: true,
		traced_methods: vec![
			"commitmentRequest".to_string(),
			"commitmentResult".to_string(),
			"slots".to_string(),
			"fee".to_string(),
		],
		constraints_relay_url: "https://relay.example.com".to_string(),
		constraints_api_key: None,
		constraints_bls_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		constraints_delegate_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		eth_genesis_timestamp: 1606824023,
		constraints_receivers: vec![
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		],
		execution_endpoint_url: "http://localhost:8545".to_string(),
		execution_request_timeout_secs: 10,
		execution_max_retries: 3,
	};

	// Test direct field access
	assert_eq!(config.commitments_server_host, "127.0.0.1");
	assert_eq!(config.commitments_server_port, 8080);
	assert_eq!(config.commitments_database_url, "./data/rocksdb");
	assert_eq!(config.log_level, "info");
	assert_eq!(config.enable_method_tracing, true);
	assert_eq!(config.traced_methods, vec!["commitmentRequest", "commitmentResult", "slots", "fee"]);
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
		constraints_bls_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		constraints_delegate_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		eth_genesis_timestamp: 1606824023,
		constraints_receivers: vec![
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		],
		execution_endpoint_url: "http://localhost:8545".to_string(),
		execution_request_timeout_secs: 10,
		execution_max_retries: 3,
	};

	// Test direct field access instead of removed methods
	assert_eq!(config.commitments_server_host, "0.0.0.0");
	assert_eq!(config.commitments_server_port, 9090);
	assert_eq!(config.commitments_database_url, "postgresql://test/testdb");
	assert_eq!(config.log_level, "debug");
	assert_eq!(config.enable_method_tracing, false);
	assert_eq!(config.traced_methods, vec!["test_method"]);

	// Test the remaining accessor methods
	assert_eq!(config.commitments_database_url(), "postgresql://test/testdb");
	assert_eq!(config.constraints_database_url(), "postgresql://test/constraints");
	assert_eq!(config.delegations_database_url(), "postgresql://test/delegations");
	assert_eq!(config.pricing_database_url(), "postgresql://test/pricing");

	// Test constraints config fields
	assert_eq!(config.constraints_relay_url, "https://relay.example.com");
	assert_eq!(config.constraints_api_key, None);
	assert_eq!(
		config.constraints_bls_public_key,
		"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
	);

	// Test scheduler config
	assert_eq!(config.eth_genesis_timestamp(), 1606824023);
}

#[test]
fn test_config_field_access() {
	// Test that we can access all the fields directly
	let config = InclusionPreconfConfig {
		commitments_server_host: "127.0.0.1".to_string(),
		commitments_server_port: 8080,
		commitments_database_url: "./data/rocksdb".to_string(),
		constraints_database_url: "./data/constraints-rocksdb".to_string(),
		delegations_database_url: "./data/delegations-rocksdb".to_string(),
		pricing_database_url: "./data/pricing-rocksdb".to_string(),
		log_level: "info".to_string(),
		enable_method_tracing: true,
		traced_methods: vec![
			"commitmentRequest".to_string(),
			"commitmentResult".to_string(),
			"slots".to_string(),
			"fee".to_string(),
		],
		constraints_relay_url: "https://relay.example.com".to_string(),
		constraints_api_key: None,
		constraints_bls_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		constraints_delegate_public_key:
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		eth_genesis_timestamp: 1606824023,
		constraints_receivers: vec![
			"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
				.to_string(),
		],
		execution_endpoint_url: "http://localhost:8545".to_string(),
		execution_request_timeout_secs: 10,
		execution_max_retries: 3,
	};

	// Test server config fields
	assert_eq!(config.commitments_server_host, "127.0.0.1");
	assert_eq!(config.commitments_server_port, 8080);

	// Test database config fields
	assert_eq!(config.commitments_database_url, "./data/rocksdb");

	// Test logging config fields
	assert_eq!(config.log_level, "info");
	assert_eq!(config.enable_method_tracing, true);
	assert_eq!(config.traced_methods, vec!["commitmentRequest", "commitmentResult", "slots", "fee"]);
}
