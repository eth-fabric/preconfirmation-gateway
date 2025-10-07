use axum::{extract::State, http::StatusCode, response::Json};
use eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use super::client::ConstraintsClient;
use super::utils::{create_constraints_message, create_signed_constraints};
use common::types::{ConstraintCapabilities, RpcContext};

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
}
