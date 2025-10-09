use axum::{extract::State, http::StatusCode, response::Json};
use eyre::Result;
use tracing::{error, info, warn};

use super::client::ConstraintsClient;
use crate::utils::{
	create_constraints_error_response, create_constraints_message, create_signed_constraints, parse_bls_public_key,
	parse_bls_public_key_with_status_code, parse_bls_public_keys,
};
use commit_boost::prelude::BlsPublicKey;
use common::types::{
	ConstraintCapabilities, HealthResponse, ProcessConstraintsRequest, ProcessConstraintsResponse,
	ProcessDelegationsRequest, ProcessDelegationsResponse, RpcContext, SignedDelegation,
};

/// Validate a process constraints request and extract BLS public keys
pub fn validate_process_constraints_request(
	request: &ProcessConstraintsRequest,
) -> Result<(BlsPublicKey, BlsPublicKey, Vec<BlsPublicKey>), StatusCode> {
	// Parse BLS public keys using the simpler parsing functions
	let bls_public_key = match parse_bls_public_key(&request.bls_public_key, "delegate") {
		Ok(key) => key,
		Err(e) => {
			error!("Invalid delegate BLS public key: {}", e);
			return Err(StatusCode::BAD_REQUEST);
		}
	};

	let proposer_public_key = match parse_bls_public_key(&request.proposer_public_key, "proposer") {
		Ok(key) => key,
		Err(e) => {
			error!("Invalid proposer BLS public key: {}", e);
			return Err(StatusCode::BAD_REQUEST);
		}
	};

	// Parse receiver BLS public keys
	let receivers = match parse_bls_public_keys(&request.receivers, "receiver") {
		Ok(keys) => keys,
		Err(e) => {
			error!("Invalid receiver BLS public keys: {}", e);
			return Err(StatusCode::BAD_REQUEST);
		}
	};

	Ok((bls_public_key, proposer_public_key, receivers))
}

/// Scheduler endpoint: Process constraints for a specific slot
pub async fn process_constraints_handler<T>(
	State(context): State<RpcContext<T>>,
	Json(request): Json<ProcessConstraintsRequest>,
) -> Result<Json<ProcessConstraintsResponse>, StatusCode> {
	info!("Processing constraints for slot {} with BLS public key", request.slot);

	// Validate request and extract BLS public keys
	let (bls_public_key, proposer_public_key, receivers) = match validate_process_constraints_request(&request) {
		Ok(keys) => keys,
		Err(status_code) => return Err(status_code),
	};

	// 1. Get constraints for the specific slot
	let slot_constraints = match context.database().get_constraints_for_slot(request.slot) {
		Ok(constraints) => constraints,
		Err(e) => {
			error!("Failed to get constraints for slot {}: {}", request.slot, e);
			return create_constraints_error_response(
				request.slot,
				&format!("Failed to get constraints for slot: {}", e),
			);
		}
	};

	if slot_constraints.is_empty() {
		info!("No constraints found for slot {}", request.slot);
		return Ok(Json(ProcessConstraintsResponse {
			success: true,
			slot: request.slot,
			processed_count: 0,
			signed_constraints: None,
			message: format!("No constraints found for slot {}", request.slot),
		}));
	}

	// 2. Create constraints message from slot constraints
	let constraints_message = match create_constraints_message(
		slot_constraints.clone(),
		proposer_public_key,
		bls_public_key.clone(), // delegate is the same as the signing key
		request.slot,
		receivers,
	) {
		Ok(message) => message,
		Err(e) => {
			error!("Failed to create constraints message for slot {}: {}", request.slot, e);
			return create_constraints_error_response(
				request.slot,
				&format!("Failed to create constraints message: {}", e),
			);
		}
	};

	// 3. Sign the constraints message with the provided BLS public key
	let signed_constraints =
		match create_signed_constraints(&constraints_message, context.commit_config.clone(), bls_public_key).await {
			Ok(signed) => signed,
			Err(e) => {
				error!("Failed to sign constraints message for slot {}: {}", request.slot, e);
				return create_constraints_error_response(
					request.slot,
					&format!("Failed to sign constraints message: {}", e),
				);
			}
		};

	// 4. Send to relay using the client
	let client = ConstraintsClient::new(context.relay_url.clone(), context.api_key.clone());

	match client.post_constraints(&signed_constraints).await {
		Ok(_) => {
			info!("Successfully sent constraints for slot {} to relay", request.slot);

			// 5. Mark constraints as sent (atomic operation)
			let mut all_marked = true;
			for (constraint_id, _) in &slot_constraints {
				if let Err(e) = context.database().mark_constraint_sent(constraint_id) {
					error!("Failed to mark constraint {} as sent: {}", constraint_id, e);
					all_marked = false;
					// Continue processing other constraints
				}
			}

			if !all_marked {
				warn!("Some constraints could not be marked as sent for slot {}", request.slot);
			}

			Ok(Json(ProcessConstraintsResponse {
				success: true,
				slot: request.slot,
				processed_count: constraints_message.constraints.len(),
				signed_constraints: Some(signed_constraints),
				message: format!(
					"Successfully processed {} constraints for slot {}",
					constraints_message.constraints.len(),
					request.slot
				),
			}))
		}
		Err(e) => {
			error!("Failed to send constraints for slot {} to relay: {}", request.slot, e);
			create_constraints_error_response(request.slot, &format!("Failed to send constraints to relay: {}", e))
		}
	}
}

/// Health check endpoint
pub async fn health_handler<T>(State(_context): State<RpcContext<T>>) -> Result<Json<HealthResponse>, StatusCode> {
	let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

	Ok(Json(HealthResponse { status: "healthy".to_string(), timestamp }))
}

/// Get constraint capabilities
pub async fn capabilities_handler<T>(
	State(_context): State<RpcContext<T>>,
) -> Result<Json<ConstraintCapabilities>, StatusCode> {
	// TODO: Make this configurable or read from database
	let capabilities = ConstraintCapabilities {
        constraint_types: vec![1, 2, 3, 4, 5], // Example constraint types
    };

	Ok(Json(capabilities))
}

/// Scheduler endpoint: Process delegations for a specific slot and delegate
pub async fn process_delegations_handler<T>(
	State(context): State<RpcContext<T>>,
	Json(request): Json<ProcessDelegationsRequest>,
) -> Result<Json<ProcessDelegationsResponse>, StatusCode> {
	info!("Processing delegations request for slot {} and delegate {}", request.slot, request.delegate_bls_public_key);

	// Parse the delegate BLS public key
	let delegate_bls_public_key =
		match parse_bls_public_key_with_status_code(&request.delegate_bls_public_key, "delegate") {
			Ok(key) => key,
			Err(status_code) => return Err(status_code),
		};

	// Create client and fetch delegations for the slot
	let client = ConstraintsClient::new(context.relay_url.clone(), context.api_key.clone());

	let delegations = match client.get_delegations_for_slot(request.slot).await {
		Ok(delegations) => delegations,
		Err(e) => {
			error!("Failed to get delegations for slot {}: {}", request.slot, e);
			return Err(StatusCode::INTERNAL_SERVER_ERROR);
		}
	};

	info!("Retrieved {} delegations for slot {}", delegations.len(), request.slot);

	// Store total count before filtering
	let total_delegations = delegations.len();

	// Filter delegations that match the delegate BLS public key
	let matching_delegations: Vec<SignedDelegation> = delegations
		.into_iter()
		.filter(|signed_delegation| {
			// Compare delegate BLS public keys
			signed_delegation.message.delegate == delegate_bls_public_key
		})
		.collect();

	let matching_count = matching_delegations.len();
	info!("Found {} matching delegations for delegate {}", matching_count, request.delegate_bls_public_key);

	// Store matching delegations in the database
	for delegation in &matching_delegations {
		if let Err(e) = context.database.store_delegation(request.slot, delegation) {
			error!("Failed to store delegation for slot {}: {}", request.slot, e);
			// Continue processing other delegations even if one fails
		} else {
			info!("Stored delegation for slot {} with committer {}", request.slot, delegation.message.committer);
		}
	}

	Ok(Json(ProcessDelegationsResponse {
		success: true,
		slot: request.slot,
		total_delegations,
		matching_delegations,
		message: format!("Found {} matching delegations out of {} total", matching_count, total_delegations),
	}))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process_constraints_response() {
		let response = ProcessConstraintsResponse {
			success: true,
			slot: 12345,
			processed_count: 5,
			signed_constraints: None,
			message: "Test message".to_string(),
		};

		assert!(response.success);
		assert_eq!(response.slot, 12345);
		assert_eq!(response.processed_count, 5);
		assert_eq!(response.message, "Test message");
	}

	#[test]
	fn test_process_constraints_request() {
		let request = ProcessConstraintsRequest {
			slot: 12345,
			bls_public_key: "0x1234567890abcdef".to_string(),
			proposer_public_key: "0xabcdef1234567890".to_string(),
			receivers: vec!["0x1111111111111111".to_string(), "0x2222222222222222".to_string()],
		};

		assert_eq!(request.slot, 12345);
		assert_eq!(request.bls_public_key, "0x1234567890abcdef");
		assert_eq!(request.proposer_public_key, "0xabcdef1234567890");
		assert_eq!(request.receivers.len(), 2);
	}

	#[test]
	fn test_health_response() {
		let response = HealthResponse { status: "healthy".to_string(), timestamp: 1234567890 };

		assert_eq!(response.status, "healthy");
		assert_eq!(response.timestamp, 1234567890);
	}

	#[test]
	fn test_process_delegations_request() {
		let request = ProcessDelegationsRequest {
			slot: 12345,
			delegate_bls_public_key:
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
		};

		assert_eq!(request.slot, 12345);
		assert_eq!(
			request.delegate_bls_public_key,
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
		);
	}

	#[test]
	fn test_process_delegations_response() {
		let response = ProcessDelegationsResponse {
			success: true,
			slot: 12345,
			total_delegations: 10,
			matching_delegations: vec![],
			message: "Found 0 matching delegations out of 10 total".to_string(),
		};

		assert!(response.success);
		assert_eq!(response.slot, 12345);
		assert_eq!(response.total_delegations, 10);
		assert_eq!(response.matching_delegations.len(), 0);
		assert_eq!(response.message, "Found 0 matching delegations out of 10 total");
	}
}
