use jsonrpsee::Extensions;
use jsonrpsee::core::RpcResult;
use tracing::{error, info, instrument};

use super::utils;
use alloy::primitives::B256;
use commit_boost::prelude::commit::request::EncryptionScheme;
use common::types::{CommitmentRequest, FeeInfo, RpcContext, SignedCommitment, SlotInfoResponse};

#[instrument(name = "commitment_request", skip(_context, _extensions))]
pub async fn commitment_request_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: std::sync::Arc<RpcContext<T>>,
	_extensions: Extensions,
) -> RpcResult<SignedCommitment> {
	info!("Processing commitment request");
	let request: CommitmentRequest = params.one()?;

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

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GenerateProxyKeyRequest {
	pub bls_public_key: String,
	pub encryption_scheme: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GenerateProxyKeyResponse {
	pub signed_delegation: serde_json::Value, // Will contain the SignedProxyDelegation
	pub encryption_scheme: String,
}

#[instrument(name = "generate_proxy_key", skip(_context, _extensions))]
pub async fn generate_proxy_key_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: std::sync::Arc<RpcContext<T>>,
	_extensions: Extensions,
) -> RpcResult<GenerateProxyKeyResponse> {
	info!("Processing generate proxy key request");
	let request: GenerateProxyKeyRequest = params.one()?;

	// Parse BLS public key
	let bls_public_key = match cb_common::utils::bls_pubkey_from_hex(&request.bls_public_key) {
		Ok(key) => key,
		Err(e) => {
			error!("Invalid BLS public key format: {}", e);
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid BLS public key format",
				Some(format!("{}", e)),
			));
		}
	};

	// Parse encryption scheme
	let encryption_scheme = match request.encryption_scheme.to_lowercase().as_str() {
		"ecdsa" => EncryptionScheme::Ecdsa,
		"bls" => EncryptionScheme::Bls,
		_ => {
			error!("Invalid encryption scheme: {}", request.encryption_scheme);
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid encryption scheme",
				Some(format!("Expected 'ecdsa' or 'bls', got: {}", request.encryption_scheme)),
			));
		}
	};

	// Generate proxy key
	let commit_config = _context.commit_config.clone();
	let signed_delegation = {
		let mut commit_config = commit_config.lock().await;
		match encryption_scheme {
			EncryptionScheme::Ecdsa => {
				let delegation =
					commit_config.signer_client.generate_proxy_key_ecdsa(bls_public_key).await.map_err(|e| {
						error!("Failed to generate ECDSA proxy key: {}", e);
						jsonrpsee::types::error::ErrorObject::owned(
							-32603, // Internal error
							"Failed to generate ECDSA proxy key",
							Some(format!("{}", e)),
						)
					})?;
				serde_json::to_value(delegation).map_err(|e| {
					error!("Failed to serialize ECDSA delegation: {}", e);
					jsonrpsee::types::error::ErrorObject::owned(
						-32603, // Internal error
						"Failed to serialize delegation",
						Some(format!("{}", e)),
					)
				})?
			}
			EncryptionScheme::Bls => {
				let delegation =
					commit_config.signer_client.generate_proxy_key_bls(bls_public_key).await.map_err(|e| {
						error!("Failed to generate BLS proxy key: {}", e);
						jsonrpsee::types::error::ErrorObject::owned(
							-32603, // Internal error
							"Failed to generate BLS proxy key",
							Some(format!("{}", e)),
						)
					})?;
				serde_json::to_value(delegation).map_err(|e| {
					error!("Failed to serialize BLS delegation: {}", e);
					jsonrpsee::types::error::ErrorObject::owned(
						-32603, // Internal error
						"Failed to serialize delegation",
						Some(format!("{}", e)),
					)
				})?
			}
		}
	};

	info!("Proxy key generated successfully");
	Ok(GenerateProxyKeyResponse { signed_delegation, encryption_scheme: request.encryption_scheme })
}
