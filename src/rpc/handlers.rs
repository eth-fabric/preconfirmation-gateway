use jsonrpsee::Extensions;
use jsonrpsee::core::RpcResult;
use tracing::{error, info, instrument};

use super::super::types::{Commitment, CommitmentRequest, FeeInfo, RpcContext, SignedCommitment, SlotInfoResponse};
use super::utils;
use alloy::primitives::{Address, B256, Bytes};

#[instrument(name = "commitment_request", skip(_context, _extensions))]
pub async fn commitment_request_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: std::sync::Arc<RpcContext<T>>,
	_extensions: Extensions,
) -> RpcResult<SignedCommitment> {
	info!("Processing commitment request");
	let request: CommitmentRequest = params.parse()?;

	// Validate the commitment request
	if let Err(e) = utils::validate_commitment_request(&request) {
		error!("Invalid commitment request: {}", e);
		return Err(jsonrpsee::types::error::ErrorObject::owned(
			-32602, // Invalid params
			"Invalid commitment request",
			Some(format!("{}", e)),
		));
	}

	// Database is now available via _context.database
	// Example usage: _context.database.with_client(|client| { /* database operations */ }).await?;
	// Or use the convenience method: _context.with_database(|client| { /* database operations */ }).await?;
	// Or get direct client access: _context.database_client();

	// Create and sign the commitment using ECDSA with commit-boost
	let commit_config = _context.commit_config.clone();
	let committer_address = _context.committer_address;
	let signed_commitment = match utils::create_signed_commitment(&request, commit_config, committer_address).await {
		Ok(commitment) => commitment,
		Err(e) => {
			error!("Failed to create signed commitment: {}", e);
			// For now, return a mock commitment on error
			utils::create_mock_signed_commitment(&request)
		}
	};

	info!("Commitment request processed successfully");
	Ok(signed_commitment)
}

#[instrument(name = "commitment_result", skip(_context, _extensions))]
pub fn commitment_result_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: &RpcContext<T>,
	_extensions: &Extensions,
) -> RpcResult<SignedCommitment> {
	info!("Processing commitment result request");
	let request_hash: B256 = params.one()?;

	// TODO: Implement actual commitment retrieval logic
	let commitment = Commitment { commitment_type: 1, payload: Bytes::new(), request_hash, slasher: Address::ZERO };

	let signed_commitment = SignedCommitment {
		commitment,
		nonce: 0, // Mock nonce
		signing_id: B256::from_slice(&[0u8; 32]), // Mock signing_id
		signature: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".parse().unwrap()
	};

	info!("Commitment result request processed successfully");
	Ok(signed_commitment)
}

#[instrument(name = "slots", skip(_context, _extensions))]
pub fn slots_handler<T>(
	_params: jsonrpsee::types::Params<'_>,
	_context: &RpcContext<T>,
	_extensions: &Extensions,
) -> RpcResult<SlotInfoResponse> {
	info!("Processing slots request");
	// TODO: Implement actual slots logic
	let response = SlotInfoResponse { slots: vec![] };

	info!("Slots request processed successfully");
	Ok(response)
}

#[instrument(name = "fee", skip(_context, _extensions))]
pub fn fee_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: &RpcContext<T>,
	_extensions: &Extensions,
) -> RpcResult<FeeInfo> {
	info!("Processing fee request");
	let request: CommitmentRequest = params.parse()?;

	// Use helper function to calculate fee
	let fee_info = utils::calculate_fee_info(&request);

	info!("Fee request processed successfully");
	Ok(fee_info)
}
