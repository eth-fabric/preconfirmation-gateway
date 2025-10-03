use jsonrpsee::{Extensions, types::Params};
use preconfirmation_gateway::{CommitmentRequest, DatabaseContext, RpcContext, fee_handler};
use serde_json::json;
use rocksdb::{DB, Options};
use std::sync::Arc;
use tempfile::TempDir;

// Create a test database context using RocksDB
fn create_test_context() -> RpcContext {
	let temp_dir = TempDir::new().unwrap();
	let db_path = temp_dir.path().join("test_db");
	
	let mut opts = Options::default();
	opts.create_if_missing(true);
	let db = DB::open(&opts, &db_path).unwrap();
	let database = DatabaseContext::new(Arc::new(db));
	
	// Create a minimal RPC context for testing
	RpcContext {
		database,
		commit_config: Arc::new(tokio::sync::Mutex::new(commit_boost::prelude::StartCommitModuleConfig {
			// Minimal config for testing
			module_id: "test-module".to_string(),
			signing_id: alloy::primitives::B256::ZERO,
			module_type: commit_boost::prelude::ModuleType::Commit,
			docker_image: "test".to_string(),
			env_file: None,
			extra: preconfirmation_gateway::config::InclusionPreconfConfig {
				rpc_server_host: "127.0.0.1".to_string(),
				rpc_server_port: 8080,
				database_url: "./data/rocksdb".to_string(),
				log_level: "info".to_string(),
				enable_method_tracing: false,
				traced_methods: vec![],
				committer_address: "0x0000000000000000000000000000000000000000".to_string(),
			},
			signer_client: None,
		})),
		committer_address: alloy::primitives::Address::ZERO,
	}
}

#[test]
fn test_fee_handler_basic() {
	let context = create_test_context();

	let request = CommitmentRequest {
		commitment_type: 1,
		payload: vec![1, 2, 3, 4, 5],
		slasher: "0x1234567890123456789012345678901234567890".to_string(),
	};

	let params_json = json!([request]);
	let params_string = params_json.to_string();
	let params = Params::new(Some(&params_string));

	let result = fee_handler(params, &context, &Extensions::new());

	assert!(result.is_ok());
	let fee_info = result.unwrap();
	assert_eq!(fee_info.commitment_type, 1);
	assert_eq!(fee_info.fee_payload.len(), 32);
	assert_eq!(fee_info.fee_payload, vec![0u8; 32]);
}

#[test]
fn test_fee_handler_invalid_params() {
	let context = create_test_context();

	// Test with invalid JSON parameters
	let invalid_params_json = json!(["invalid", "params"]);
	let params_string = invalid_params_json.to_string();
	let params = Params::new(Some(&params_string));

	let result = fee_handler(params, &context, &Extensions::new());
	assert!(result.is_err());
}
