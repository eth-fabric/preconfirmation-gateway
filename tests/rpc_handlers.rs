use jsonrpsee::{types::Params, Extensions};
use preconfirmation_gateway::{fee_handler, CommitmentRequest, DatabaseContext, RpcContext};
use serde_json::json;

// Create a simple mock database that doesn't require a real connection
fn create_mock_context() -> Option<RpcContext> {
	// Try to create a real database connection for testing
	// If it fails, return None so tests can be skipped gracefully
	std::env::var("TEST_DATABASE_URL")
		.or_else(|_| std::env::var("DATABASE_URL"))
		.ok()
		.and_then(|url| {
			postgres::Client::connect(&url, postgres::NoTls)
				.map(|client| {
					let db_context = DatabaseContext::new(client);
					RpcContext::new(db_context)
				})
				.ok()
		})
}

#[test]
fn test_fee_handler_basic() {
	let Some(context) = create_mock_context() else {
		println!("Skipping test - no database connection available");
		return;
	};

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
fn test_fee_handler_different_commitment_types() {
	let Some(context) = create_mock_context() else {
		println!("Skipping test - no database connection available");
		return;
	};

	for commitment_type in [1, 2, 100, 999] {
		let request = CommitmentRequest {
			commitment_type,
			payload: vec![],
			slasher: "0x0000000000000000000000000000000000000000".to_string(),
		};

		let params_json = json!([request]);
		let params_string = params_json.to_string();
		let params = Params::new(Some(&params_string));

		let result = fee_handler(params, &context, &Extensions::new());

		assert!(result.is_ok());
		let fee_info = result.unwrap();
		assert_eq!(fee_info.commitment_type, commitment_type);
		assert_eq!(fee_info.fee_payload.len(), 32);
	}
}

#[test]
fn test_fee_handler_invalid_params() {
	let Some(context) = create_mock_context() else {
		println!("Skipping test - no database connection available");
		return;
	};

	// Test with invalid JSON parameters
	let invalid_params_json = json!(["invalid", "params"]);
	let params_string = invalid_params_json.to_string();
	let params = Params::new(Some(&params_string));

	let result = fee_handler(params, &context, &Extensions::new());
	assert!(result.is_err());
}