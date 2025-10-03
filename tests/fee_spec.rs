use alloy::primitives::{Address, Bytes};
use jsonrpsee::{Extensions, types::Params};
use preconfirmation_gateway::{CommitmentRequest, fee_handler};
use serde_json::json;

mod common;
use common::test_helpers::create_test_context;

#[tokio::test]
async fn test_fee_handler_basic() -> eyre::Result<()> {
	let context = create_test_context().await?;

	let request = CommitmentRequest {
		commitment_type: 1,
		payload: Bytes::from(vec![1, 2, 3, 4, 5]),
		slasher: "0x1234567890123456789012345678901234567890".parse::<Address>().unwrap(),
	};

	let params_json = json!(request);
	let params_string = params_json.to_string();
	let params = Params::new(Some(&params_string));

	let result = fee_handler(params, &context, &Extensions::new());

	if let Err(e) = &result {
		println!("Fee handler error: {}", e);
	}
	assert!(result.is_ok());
	let fee_info = result.unwrap();
	assert_eq!(fee_info.commitment_type, 1);
	assert_eq!(fee_info.fee_payload.len(), 0); // Currently returns empty bytes as placeholder
	assert_eq!(fee_info.fee_payload, vec![]);

	Ok(())
}

#[tokio::test]
async fn test_fee_handler_invalid_params() -> eyre::Result<()> {
	let context = create_test_context().await?;

	// Test with invalid JSON parameters
	let invalid_params_json = json!(["invalid", "params"]);
	let params_string = invalid_params_json.to_string();
	let params = Params::new(Some(&params_string));

	let result = fee_handler(params, &context, &Extensions::new());
	assert!(result.is_err());

	Ok(())
}
