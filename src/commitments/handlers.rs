use jsonrpsee::Extensions;
use jsonrpsee::core::RpcResult;
use tracing::{error, info, instrument};

use super::super::types::{CommitmentRequest, FeeInfo, RpcContext, SignedCommitment, SlotInfoResponse};
use super::utils;
use alloy::primitives::B256;

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

	// Create and sign the commitment using ECDSA with commit-boost
	let commit_config = _context.commit_config.clone();
	let committer_address = _context.committer_address;
	let signed_commitment = match utils::create_signed_commitment(&request, commit_config, committer_address).await {
		Ok(commitment) => commitment,
		Err(e) => {
			error!("Failed to create signed commitment: {}", e);
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Failed to create signed commitment",
				Some(format!("{}", e)),
			));
		}
	};

	// Store the signed commitment in the database
	let request_hash = signed_commitment.commitment.request_hash;
	if let Err(e) = _context.database().store_commitment(&request_hash, &signed_commitment) {
		error!("Failed to store commitment in database: {}", e);
		return Err(jsonrpsee::types::error::ErrorObject::owned(
			-32603, // Internal error
			"Failed to store commitment",
			Some(format!("{}", e)),
		));
	}

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

	// Retrieve the commitment from the database
	match _context.database().get_commitment(&request_hash) {
		Ok(Some(signed_commitment)) => {
			info!("Commitment result request processed successfully");
			Ok(signed_commitment)
		}
		Ok(None) => {
			error!("Commitment not found for request hash: {}", request_hash);
			Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Commitment not found",
				Some(format!("No commitment found for request hash: {}", request_hash)),
			))
		}
		Err(e) => {
			error!("Failed to retrieve commitment from database: {}", e);
			Err(jsonrpsee::types::error::ErrorObject::owned(
				-32603, // Internal error
				"Database error",
				Some(format!("{}", e)),
			))
		}
	}
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
	let fee_info = match utils::calculate_fee_info(&request) {
		Ok(fee_info) => fee_info,
		Err(e) => {
			error!("Failed to calculate fee info: {}", e);
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid request",
				Some(format!("{}", e)),
			));
		}
	};

	// Store fee information in the database
	let request_hash = match request.request_hash() {
		Ok(hash) => hash,
		Err(e) => {
			error!("Failed to calculate request hash: {}", e);
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid request",
				Some(format!("{}", e)),
			));
		}
	};

	if let Err(e) = _context.database().store_fee_info(&request_hash, &fee_info) {
		error!("Failed to store fee info in database: {}", e);
		return Err(jsonrpsee::types::error::ErrorObject::owned(
			-32603, // Internal error
			"Failed to store fee info",
			Some(format!("{}", e)),
		));
	}

	info!("Fee request processed successfully");
	Ok(fee_info)
}
