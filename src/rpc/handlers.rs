use jsonrpsee::Extensions;
use jsonrpsee::core::RpcResult;

use super::{
	context::RpcContext,
	types::{Commitment, CommitmentRequest, FeeInfo, SignedCommitment, SlotInfoResponse},
};

pub fn commitment_request_handler(
	params: jsonrpsee::types::Params<'_>,
	context: &RpcContext,
	_extensions: &Extensions,
) -> RpcResult<SignedCommitment> {
	let request: CommitmentRequest = params.parse()?;

	// Database is now available via context.database
	// Example usage: context.database.with_client(|client| { /* database operations */ })?;
	// Or use the convenience method: context.with_database(|client| { /* database operations */ })?;
	// TODO: Implement actual commitment logic
	let commitment = Commitment {
		commitment_type: request.commitment_type,
		payload: request.payload,
		request_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
		slasher: request.slasher,
	};

	let signed_commitment = SignedCommitment {
		commitment,
		signature: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
	};

	Ok(signed_commitment)
}

pub fn commitment_result_handler(
	params: jsonrpsee::types::Params<'_>,
	context: &RpcContext,
	_extensions: &Extensions,
) -> RpcResult<SignedCommitment> {
	let request_hash: String = params.one()?;

	// TODO: Implement actual commitment retrieval logic
	let commitment = Commitment {
		commitment_type: 1,
		payload: vec![],
		request_hash,
		slasher: "0x0000000000000000000000000000000000000000".to_string(),
	};

	let signed_commitment = SignedCommitment {
		commitment,
		signature: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
	};

	Ok(signed_commitment)
}

pub fn slots_handler(
	_params: jsonrpsee::types::Params<'_>,
	context: &RpcContext,
	_extensions: &Extensions,
) -> RpcResult<SlotInfoResponse> {
	// TODO: Implement actual slots logic
	let response = SlotInfoResponse { slots: vec![] };

	Ok(response)
}

pub fn fee_handler(
	params: jsonrpsee::types::Params<'_>,
	context: &RpcContext,
	_extensions: &Extensions,
) -> RpcResult<FeeInfo> {
	let request: CommitmentRequest = params.parse()?;

	// TODO: Implement actual fee calculation logic
	let fee_info = FeeInfo { fee_payload: vec![0u8; 32], commitment_type: request.commitment_type };

	Ok(fee_info)
}
