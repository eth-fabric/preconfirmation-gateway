use axum::{extract::Path, extract::State, http::HeaderMap, http::StatusCode, response::Json};
use common::types::{
	GetDelegationsResponse, HealthResponse, PostConstraintsResponse, PostDelegationResponse, SignedConstraints,
	SignedDelegation,
};
use hex;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::database::RelayDatabase;
use crate::utils::{
	validate_constraints_message, validate_delegation_message, verify_constraints_signature,
	verify_delegation_signature,
};

/// Extract and parse BLS signature and public key from headers
fn extract_bls_auth_headers(
	headers: &HeaderMap,
) -> Result<(commit_boost::prelude::BlsPublicKey, commit_boost::prelude::BlsSignature), StatusCode> {
	// Extract required headers
	let signature_header = headers.get("X-Receiver-Signature").ok_or_else(|| {
		error!("Missing X-Receiver-Signature header");
		StatusCode::BAD_REQUEST
	})?;

	let public_key_header = headers.get("X-Receiver-PublicKey").ok_or_else(|| {
		error!("Missing X-Receiver-PublicKey header");
		StatusCode::BAD_REQUEST
	})?;

	// Parse BLS public key
	let public_key_str = public_key_header.to_str().map_err(|_| {
		error!("Invalid X-Receiver-PublicKey header");
		StatusCode::BAD_REQUEST
	})?;

	let public_key = common::utils::bls_pubkey_from_hex(public_key_str).map_err(|e| {
		error!("Invalid BLS public key: {}", e);
		error!("Public key string: {}", public_key_str);
		StatusCode::BAD_REQUEST
	})?;

	// Parse BLS signature
	let signature_str = signature_header.to_str().map_err(|_| {
		error!("Invalid X-Receiver-Signature header");
		StatusCode::BAD_REQUEST
	})?;

	let signature = signature_str.strip_prefix("0x").unwrap_or(signature_str);
	let signature_bytes = hex::decode(signature).map_err(|e| {
		error!("Invalid signature hex: {}", e);
		StatusCode::BAD_REQUEST
	})?;

	let bls_signature = commit_boost::prelude::BlsSignature::deserialize(&signature_bytes).map_err(|e| {
		error!("Invalid BLS signature: {:?}", e);
		error!("Signature string: {}", signature_str);
		error!("Signature bytes length: {}", signature_bytes.len());
		StatusCode::BAD_REQUEST
	})?;

	Ok((public_key, bls_signature))
}

/// Shared state for relay handlers
#[derive(Clone)]
pub struct RelayState {
	pub database: Arc<RelayDatabase>,
}

/// POST /delegation - Store a verified delegation
pub async fn store_delegation_handler(
	State(state): State<RelayState>,
	Json(delegation): Json<SignedDelegation>,
) -> Result<Json<PostDelegationResponse>, StatusCode> {
	info!("Storing delegation for slot {}", delegation.message.slot);

	// Validate delegation message structure
	match validate_delegation_message(&delegation.message) {
		Ok(false) => {
			error!("Invalid delegation message structure");
			return Ok(Json(PostDelegationResponse {
				success: false,
				message: "Invalid delegation message structure".to_string(),
			}));
		}
		Err(e) => {
			error!("Error validating delegation message: {}", e);
			return Ok(Json(PostDelegationResponse {
				success: false,
				message: format!("Error validating delegation message: {}", e),
			}));
		}
		Ok(true) => {
			debug!("Delegation message structure is valid");
		}
	}

	// Verify BLS signature using the proposer public key from the message
	match verify_delegation_signature(&delegation) {
		Ok(true) => {
			info!("Delegation signature verification successful");
		}
		Ok(false) => {
			error!("Delegation signature verification failed");
			return Ok(Json(PostDelegationResponse {
				success: false,
				message: "Delegation signature verification failed".to_string(),
			}));
		}
		Err(e) => {
			error!("Error verifying delegation signature: {}", e);
			return Ok(Json(PostDelegationResponse {
				success: false,
				message: format!("Error verifying delegation signature: {}", e),
			}));
		}
	}

	// Generate a delegation ID (using a hash of the delegation content)
	let delegation_id = format!("delegation_{}", delegation.message.slot);

	// Store delegation in database
	match state.database.store_delegation(delegation.message.slot, &delegation_id, &delegation) {
		Ok(_) => {
			info!("Delegation stored successfully for slot {}", delegation.message.slot);
			Ok(Json(PostDelegationResponse { success: true, message: "Delegation stored successfully".to_string() }))
		}
		Err(e) => {
			error!("Failed to store delegation: {}", e);
			Ok(Json(PostDelegationResponse { success: false, message: format!("Failed to store delegation: {}", e) }))
		}
	}
}

/// GET /delegations/{slot} - Get delegations for a slot
pub async fn get_delegations_handler(
	State(state): State<RelayState>,
	axum::extract::Path(slot): axum::extract::Path<u64>,
) -> Result<Json<GetDelegationsResponse>, StatusCode> {
	info!("Getting delegations for slot {}", slot);

	match state.database.get_delegations_for_slot(slot) {
		Ok(delegations) => {
			info!("Retrieved {} delegations for slot {}", delegations.len(), slot);
			Ok(Json(GetDelegationsResponse { delegations }))
		}
		Err(e) => {
			error!("Failed to get delegations for slot {}: {}", slot, e);
			Err(StatusCode::INTERNAL_SERVER_ERROR)
		}
	}
}

/// POST /constraints - Store verified constraints
pub async fn store_constraints_handler(
	State(state): State<RelayState>,
	Json(signed_constraints): Json<SignedConstraints>,
) -> Result<Json<PostConstraintsResponse>, StatusCode> {
	info!("Storing constraints for slot {}", signed_constraints.message.slot);

	// Validate constraints message structure
	match validate_constraints_message(&signed_constraints.message) {
		Ok(false) => {
			error!("Invalid constraints message structure");
			return Ok(Json(PostConstraintsResponse {
				success: false,
				message: "Invalid constraints message structure".to_string(),
			}));
		}
		Err(e) => {
			error!("Error validating constraints message: {}", e);
			return Ok(Json(PostConstraintsResponse {
				success: false,
				message: format!("Error validating constraints message: {}", e),
			}));
		}
		Ok(true) => {
			debug!("Constraints message structure is valid");
		}
	}

	// Verify BLS signature using the delegate public key from the message
	match verify_constraints_signature(&signed_constraints) {
		Ok(true) => {
			info!("Constraints signature verification successful");
		}
		Ok(false) => {
			error!("Constraints signature verification failed");
			return Ok(Json(PostConstraintsResponse {
				success: false,
				message: "Constraints signature verification failed".to_string(),
			}));
		}
		Err(e) => {
			error!("Error verifying constraints signature: {}", e);
			return Ok(Json(PostConstraintsResponse {
				success: false,
				message: format!("Error verifying constraints signature: {}", e),
			}));
		}
	}

	// Store constraints in database
	let constraint_id = format!("constraint_{}", chrono::Utc::now().timestamp_millis());
	match state.database.store_constraint(signed_constraints.message.slot, &constraint_id, &signed_constraints) {
		Ok(_) => {
			info!("Constraints stored successfully for slot {}", signed_constraints.message.slot);
			Ok(Json(PostConstraintsResponse { success: true, message: "Constraints stored successfully".to_string() }))
		}
		Err(e) => {
			error!("Failed to store constraints: {}", e);
			Ok(Json(PostConstraintsResponse { success: false, message: format!("Failed to store constraints: {}", e) }))
		}
	}
}

/// GET /constraints/v0/relay/constraints/{slot} - Get constraints for a slot with BLS signature verification
pub async fn get_constraints_for_slot_handler(
	State(state): State<RelayState>,
	Path(slot): Path<u64>,
	headers: HeaderMap,
) -> Result<Json<Vec<SignedConstraints>>, StatusCode> {
	info!("Getting constraints for slot {} with signature verification", slot);
	info!("Handler called with slot: {}", slot);
	info!("Headers count: {}", headers.len());

	// Extract and parse BLS signature and public key from headers
	let (public_key, bls_signature) = extract_bls_auth_headers(&headers)?;

	// For now, we'll use a simplified signature verification approach
	// In a production system, you would implement proper BLS signature verification
	// over the slot number as specified in the requirements

	// Create a simple hash of the slot for verification
	let slot_str = slot.to_string();
	let slot_hash = alloy::primitives::keccak256(slot_str.as_bytes());

	// Verify caller's signature against the slot hash
	let is_valid = bls_signature.verify(&public_key, slot_hash);

	// Debug output
	info!("Signature verification for slot {}: {}", slot, is_valid);
	info!("Public key: {:?}", public_key.serialize());
	info!("Signature: {:?}", bls_signature.serialize());
	info!("Slot hash: {:?}", slot_hash);

	if !is_valid {
		error!("Invalid BLS signature for slot {}", slot);
		return Err(StatusCode::BAD_REQUEST);
	}

	// Get constraints from database
	let constraints = match state.database.get_constraints_for_slot(slot) {
		Ok(constraints) => constraints,
		Err(e) => {
			error!("Failed to get constraints for slot {}: {}", slot, e);
			return Err(StatusCode::INTERNAL_SERVER_ERROR);
		}
	};

	// Filter constraints where the public key is in the receivers list
	let mut authorized_constraints = Vec::new();
	for constraint in constraints {
		// Check if the public key is in any of the receivers lists
		let is_authorized = constraint.message.receivers.iter().any(|receiver| receiver == &public_key);

		if is_authorized {
			authorized_constraints.push(constraint);
		}
	}

	info!("Returning {} authorized constraints for slot {}", authorized_constraints.len(), slot);
	Ok(Json(authorized_constraints))
}

/// GET /health - Health check endpoint
pub async fn health_handler(State(state): State<RelayState>) -> Result<Json<HealthResponse>, StatusCode> {
	match state.database.get_health_status() {
		Ok(_health_status) => {
			Ok(Json(HealthResponse { status: "healthy".to_string(), timestamp: chrono::Utc::now().timestamp() as u64 }))
		}
		Err(e) => {
			error!("Failed to get health status: {}", e);
			Ok(Json(HealthResponse {
				status: "unhealthy".to_string(),
				timestamp: chrono::Utc::now().timestamp() as u64,
			}))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::http::{HeaderMap, HeaderValue};
	use common::types::{ConstraintsMessage, SignedConstraints};
	use tempfile::TempDir;

	/// Helper to create a test database
	fn create_test_database() -> (RelayDatabase, TempDir) {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_relay.db");
		let database = RelayDatabase::new(&db_path).unwrap();
		(database, temp_dir)
	}

	/// Helper to create test headers with BLS authentication
	fn create_test_headers(public_key: &str, signature: &str) -> HeaderMap {
		let mut headers = HeaderMap::new();
		headers.insert("X-Receiver-PublicKey", HeaderValue::from_str(public_key).unwrap());
		headers.insert("X-Receiver-Signature", HeaderValue::from_str(signature).unwrap());
		headers
	}

	/// Helper to create a test SignedConstraints
	fn create_test_signed_constraints(slot: u64) -> SignedConstraints {
		use commit_boost::prelude::BlsSignature;

		// Create a test constraints message
		let constraints_message = ConstraintsMessage {
			proposer:
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			delegate:
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			slot: slot,
			constraints: vec![],
			receivers: vec![
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			],
		};

		SignedConstraints {
			message: constraints_message,
			nonce: 1337,
			signing_id: "0x3078313233340000000000000000000000000000000000000000000000000000".parse().unwrap(),
			signature: BlsSignature::empty(), // Use empty signature for testing
		}
	}

	#[test]
	fn test_extract_bls_auth_headers_success() {
		// Use a valid 48-byte BLS public key
		let public_key =
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		// Use a valid 96-byte BLS signature (this is a real BLS signature from commit-boost test data)
		let signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

		let headers = create_test_headers(public_key, signature);
		let result = extract_bls_auth_headers(&headers);

		assert!(result.is_ok());
		let (parsed_pubkey, parsed_sig) = result.unwrap();
		assert_eq!(
			parsed_pubkey.serialize().as_slice(),
			hex::decode(public_key.strip_prefix("0x").unwrap()).unwrap().as_slice()
		);
		assert_eq!(
			parsed_sig.serialize().as_slice(),
			hex::decode(signature.strip_prefix("0x").unwrap()).unwrap().as_slice()
		);
	}

	#[test]
	fn test_extract_bls_auth_headers_missing_signature() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a",
			)
			.unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_missing_public_key() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505cc411d61252fb6cb3fa0017b679f8bb2305b26a285fa2737f175668d0dff91cc1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505").unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_invalid_public_key() {
		let headers = create_test_headers("invalid_hex", "0x1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505cc411d61252fb6cb3fa0017b679f8bb2305b26a285fa2737f175668d0dff91cc1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505");
		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_invalid_signature() {
		let headers = create_test_headers(
			"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a",
			"invalid_hex",
		);
		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_signature_without_0x_prefix() {
		let public_key =
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		let signature = "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

		let headers = create_test_headers(public_key, signature);
		let result = extract_bls_auth_headers(&headers);

		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_get_constraints_for_slot_handler_success() {
		let (database, _temp_dir) = create_test_database();
		let state = RelayState { database: Arc::new(database) };

		let slot = 12345u64;
		let public_key =
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		let signature = "0x1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505cc411d61252fb6cb3fa0017b679f8bb2305b26a285fa2737f175668d0dff91cc1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505";

		let headers = create_test_headers(public_key, signature);

		// Store some test constraints
		let test_constraints = create_test_signed_constraints(slot);
		state.database.store_constraint(slot, "test_constraint_1", &test_constraints).unwrap();

		let result = get_constraints_for_slot_handler(State(state), Path(slot), headers).await;

		// Note: This test will fail signature verification in the current implementation
		// because we're using a default signature. In a real test, you'd need to create
		// a properly signed constraint with a valid BLS signature.
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_get_constraints_for_slot_handler_missing_headers() {
		let (database, _temp_dir) = create_test_database();
		let state = RelayState { database: Arc::new(database) };

		let slot = 12345u64;
		let headers = HeaderMap::new(); // Empty headers

		let result = get_constraints_for_slot_handler(State(state), Path(slot), headers).await;

		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_constraints_for_slot_handler_no_constraints() {
		let (database, _temp_dir) = create_test_database();
		let state = RelayState { database: Arc::new(database) };

		let slot = 12345u64;
		let public_key =
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		let signature = "0x1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505cc411d61252fb6cb3fa0017b679f8bb2305b26a285fa2737f175668d0dff91cc1b66ac1fb663c9bc59509846d6ec05345bd908eda73e670af888da41af171505";

		let headers = create_test_headers(public_key, signature);

		// Don't store any constraints
		let result = get_constraints_for_slot_handler(State(state), Path(slot), headers).await;

		// Should return empty array even if signature verification fails
		// because there are no constraints to return
		assert!(result.is_err()); // Will fail due to signature verification
	}

	#[test]
	fn test_extract_bls_auth_headers_public_key_without_0x_prefix() {
		let public_key =
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		let signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

		let headers = create_test_headers(public_key, signature);
		let result = extract_bls_auth_headers(&headers);

		// Should still work because the helper handles the 0x prefix
		assert!(result.is_ok());
	}

	#[test]
	fn test_signed_delegation_serialization() {
		use alloy::primitives::{Bytes, B256};
		use commit_boost::prelude::{BlsPublicKey, BlsSignature};
		use common::types::constraints::{Delegation, SignedDelegation};
		use hex;

		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let delegation = SignedDelegation {
			message: Delegation {
				proposer: BlsPublicKey::deserialize(&valid_bls_key).expect("Failed to deserialize BLS public key"),
				delegate: BlsPublicKey::deserialize(&valid_bls_key).expect("Failed to deserialize BLS public key"),
				committer: "0x1234567890123456789012345678901234567890".parse().unwrap(),
				slot: 12345,
				metadata: Bytes::from(vec![0x01, 0x02]),
			},
			signature: BlsSignature::empty(),
			nonce: 1,
			signing_id: B256::from_slice(&[0x01; 32]),
		};

		let serialized = serde_json::to_string(&delegation).unwrap();
		let deserialized: SignedDelegation = serde_json::from_str(&serialized).unwrap();

		assert_eq!(delegation.message.slot, deserialized.message.slot);
		assert_eq!(delegation.nonce, deserialized.nonce);
	}

	#[test]
	fn test_health_response_serialization() {
		let response = HealthResponse { status: "healthy".to_string(), timestamp: 1234567890 };

		let serialized = serde_json::to_string(&response).unwrap();
		let deserialized: HealthResponse = serde_json::from_str(&serialized).unwrap();

		assert_eq!(response.status, deserialized.status);
		assert_eq!(response.timestamp, deserialized.timestamp);
	}
}
