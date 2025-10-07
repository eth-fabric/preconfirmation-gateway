use axum::{extract::State, http::StatusCode, response::Json};
use eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use super::client::ConstraintsClient;
use super::utils::{create_constraints_message, create_signed_constraints};
use common::types::{ConstraintCapabilities, RpcContext, SignedDelegation};

/// Response for processing constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConstraintsResponse {
	pub success: bool,
	pub processed_count: usize,
	pub message: String,
}

/// Response for health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
	pub status: String,
	pub timestamp: u64,
}

/// Request for processing delegations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDelegationsRequest {
	pub slot: u64,
	pub delegate_bls_public_key: String,
}

/// Response for processing delegations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDelegationsResponse {
	pub success: bool,
	pub slot: u64,
	pub total_delegations: usize,
	pub matching_delegations: Vec<SignedDelegation>,
	pub message: String,
}

/// Scheduler endpoint: Process pending constraints
pub async fn process_constraints_handler<T>(
	State(context): State<RpcContext<T>>,
) -> Result<Json<ProcessConstraintsResponse>, StatusCode> {
	info!("Processing constraints request");

	// 1. Read pending constraints from DB
	let pending_constraints = match context.database().get_pending_constraints() {
		Ok(constraints) => constraints,
		Err(e) => {
			error!("Failed to get pending constraints from database: {}", e);
			return Err(StatusCode::INTERNAL_SERVER_ERROR);
		}
	};

	if pending_constraints.is_empty() {
		info!("No pending constraints to process");
		return Ok(Json(ProcessConstraintsResponse {
			success: true,
			processed_count: 0,
			message: "No pending constraints".to_string(),
		}));
	}

	info!("Found {} pending constraints", pending_constraints.len());

	// 2. Create ConstraintsMessage
	let constraints_message = match create_constraints_message(pending_constraints.clone()) {
		Ok(message) => message,
		Err(e) => {
			error!("Failed to create constraints message: {}", e);
			return Err(StatusCode::INTERNAL_SERVER_ERROR);
		}
	};

	// 3. Sign the message
	let bls_public_key = context.bls_public_key.clone();
	let signed_constraints =
		match create_signed_constraints(&constraints_message, context.commit_config.clone(), bls_public_key).await {
			Ok(signed) => signed,
			Err(e) => {
				error!("Failed to sign constraints message: {}", e);
				return Err(StatusCode::INTERNAL_SERVER_ERROR);
			}
		};

	// 4. Send to relay using client
	let client = ConstraintsClient::new(context.relay_url.clone(), context.api_key.clone());

	match client.post_constraints(&signed_constraints).await {
		Ok(_) => {
			info!("Successfully posted constraints to relay");
		}
		Err(e) => {
			error!("Failed to post constraints to relay: {}", e);
			return Err(StatusCode::INTERNAL_SERVER_ERROR);
		}
	}

	// 5. Mark constraints as sent
	for (constraint_id, _) in &pending_constraints {
		if let Err(e) = context.database().mark_constraint_sent(constraint_id) {
			warn!("Failed to mark constraint {} as sent: {}", constraint_id, e);
		}
	}

	info!("Successfully processed {} constraints", pending_constraints.len());

	Ok(Json(ProcessConstraintsResponse {
		success: true,
		processed_count: pending_constraints.len(),
		message: "Constraints processed successfully".to_string(),
	}))
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
	let delegate_bls_public_key = match cb_common::utils::bls_pubkey_from_hex(&request.delegate_bls_public_key) {
		Ok(key) => key,
		Err(e) => {
			error!("Invalid delegate BLS public key format: {}", e);
			return Err(StatusCode::BAD_REQUEST);
		}
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
		let response =
			ProcessConstraintsResponse { success: true, processed_count: 5, message: "Test message".to_string() };

		assert!(response.success);
		assert_eq!(response.processed_count, 5);
		assert_eq!(response.message, "Test message");
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
