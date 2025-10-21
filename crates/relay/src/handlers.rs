use axum::{extract::Path, extract::State, http::HeaderMap, http::StatusCode, response::Json};
use commit_boost::prelude::verify_proposer_commitment_signature_bls_for_message;
use common::types::{
	ConstraintCapabilities, GetDelegationsResponse, HealthResponse, SignedConstraints, SignedDelegation,
};
use hex;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::utils::{
	validate_constraints_message, validate_delegation_message, validate_is_proposer, verify_constraints_signature,
	verify_delegation_signature,
};
use common::types::database::DatabaseContext;

/// Extract and parse BLS signature, public key, nonce, and signing_id from headers
fn extract_bls_auth_headers(
	headers: &HeaderMap,
) -> Result<
	(commit_boost::prelude::BlsPublicKey, commit_boost::prelude::BlsSignature, u64, alloy::primitives::B256),
	StatusCode,
> {
	// Extract required headers
	let signature_header = headers.get("X-Receiver-Signature").ok_or_else(|| {
		error!("Missing X-Receiver-Signature header");
		StatusCode::BAD_REQUEST
	})?;

	let public_key_header = headers.get("X-Receiver-PublicKey").ok_or_else(|| {
		error!("Missing X-Receiver-PublicKey header");
		StatusCode::BAD_REQUEST
	})?;

	let nonce_header = headers.get("X-Receiver-Nonce").ok_or_else(|| {
		error!("Missing X-Receiver-Nonce header");
		StatusCode::BAD_REQUEST
	})?;

	let signing_id_header = headers.get("X-Receiver-SigningId").ok_or_else(|| {
		error!("Missing X-Receiver-SigningId header");
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

	// Parse nonce
	let nonce_str = nonce_header.to_str().map_err(|_| {
		error!("Invalid X-Receiver-Nonce header");
		StatusCode::BAD_REQUEST
	})?;

	let nonce = nonce_str.parse::<u64>().map_err(|e| {
		error!("Invalid nonce format: {}", e);
		error!("Nonce string: {}", nonce_str);
		StatusCode::BAD_REQUEST
	})?;

	// Parse signing_id
	let signing_id_str = signing_id_header.to_str().map_err(|_| {
		error!("Invalid X-Receiver-SigningId header");
		StatusCode::BAD_REQUEST
	})?;

	let signing_id_hex = signing_id_str.strip_prefix("0x").unwrap_or(signing_id_str);
	let signing_id_bytes = hex::decode(signing_id_hex).map_err(|e| {
		error!("Invalid signing_id hex: {}", e);
		StatusCode::BAD_REQUEST
	})?;

	if signing_id_bytes.len() != 32 {
		error!("Invalid signing_id length: expected 32 bytes, got {}", signing_id_bytes.len());
		return Err(StatusCode::BAD_REQUEST);
	}

	let signing_id = alloy::primitives::B256::from_slice(&signing_id_bytes);

	Ok((public_key, bls_signature, nonce, signing_id))
}

/// Shared state for relay handlers
#[derive(Clone)]
pub struct RelayState {
	pub database: Arc<DatabaseContext>,
	pub config: crate::config::RelayConfig,
	pub slot_timer: common::slot_timer::SlotTimer,
}

/// POST /delegation - Store a verified delegation
pub async fn store_delegation_handler(
	State(state): State<RelayState>,
	Json(delegation): Json<SignedDelegation>,
) -> StatusCode {
	info!("Storing delegation for slot {}", delegation.message.slot);

	// Validate delegation message structure
	if validate_delegation_message(&delegation.message, &state.slot_timer).is_err() {
		error!("Invalid delegation message structure");
		return StatusCode::BAD_REQUEST;
	}

	info!("Delegation message validated");

	// Verify BLS signature using the proposer public key from the message
	match verify_delegation_signature(&delegation, state.config.relay.chain) {
		Ok(true) => {
			info!("Delegation signature verification successful");
		}
		Ok(false) => {
			error!("Delegation signature verification failed");
			return StatusCode::UNAUTHORIZED;
		}
		Err(e) => {
			error!("Error verifying delegation signature: {}", e);
			return StatusCode::BAD_REQUEST;
		}
	}

	info!("Delegation signature verified");

	// Validate that the proposer is scheduled for this slot
	match validate_is_proposer(&delegation.message.proposer, delegation.message.slot, &state.database) {
		Ok(true) => {
			info!("Proposer validation successful for slot {}", delegation.message.slot);
		}
		Ok(false) => {
			error!("Proposer public key is not scheduled for slot {}", delegation.message.slot);
			return StatusCode::FORBIDDEN;
		}
		Err(e) => {
			error!("Error validating proposer for slot {}: {}", delegation.message.slot, e);
			return StatusCode::INTERNAL_SERVER_ERROR;
		}
	}

	info!("Proposer is scheduled for this slot");

	// Check for existing delegation to prevent equivocation
	match state.database.get_delegation_for_slot(delegation.message.slot) {
		Ok(Some(_existing_delegation)) => {
			error!(
				"Delegation already exists for slot {} - rejecting to prevent equivocation",
				delegation.message.slot
			);
			return StatusCode::CONFLICT;
		}
		Ok(None) => {
			debug!("No existing delegation found for slot {}", delegation.message.slot);
		}
		Err(e) => {
			error!("Error checking for existing delegation for slot {}: {}", delegation.message.slot, e);
			return StatusCode::INTERNAL_SERVER_ERROR;
		}
	}

	info!("No existing delegation found for this slot");

	// Store delegation in database
	match state.database.store_delegation(delegation.message.slot, &delegation) {
		Ok(_) => {
			info!("Delegation stored successfully for slot {}", delegation.message.slot);
			StatusCode::OK
		}
		Err(e) => {
			error!("Failed to store delegation: {}", e);
			StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}

/// GET /delegations/{slot} - Get delegations for a slot
pub async fn get_delegations_handler(
	State(state): State<RelayState>,
	axum::extract::Path(slot): axum::extract::Path<u64>,
) -> Result<Json<GetDelegationsResponse>, StatusCode> {
	info!("Getting delegations for slot {}", slot);

	match state.database.get_delegation_for_slot(slot) {
		Ok(Some(delegation)) => {
			info!("Retrieved delegation for slot {}", slot);
			Ok(Json(GetDelegationsResponse { delegations: vec![delegation] }))
		}
		Ok(None) => {
			info!("No delegation found for slot {}", slot);
			Ok(Json(GetDelegationsResponse { delegations: vec![] }))
		}
		Err(e) => {
			error!("Failed to get delegation for slot {}: {}", slot, e);
			Err(StatusCode::INTERNAL_SERVER_ERROR)
		}
	}
}

/// POST /constraints - Store verified constraints
pub async fn store_constraints_handler(
	State(state): State<RelayState>,
	Json(signed_constraints): Json<SignedConstraints>,
) -> StatusCode {
	info!("Storing constraints for slot {}", signed_constraints.message.slot);

	// Validate constraints message structure
	if validate_constraints_message(&signed_constraints.message, &state.slot_timer).is_err() {
		error!("Invalid constraints message structure");
		return StatusCode::BAD_REQUEST;
	}

	// Verify BLS signature using the delegate public key from the message
	match verify_constraints_signature(&signed_constraints, state.config.relay.chain) {
		Ok(true) => {
			info!("Constraints signature verification successful");
		}
		Ok(false) => {
			error!("Constraints signature verification failed");
			return StatusCode::UNAUTHORIZED;
		}
		Err(e) => {
			error!("Error verifying constraints signature: {}", e);
			return StatusCode::BAD_REQUEST;
		}
	}

	// Verify a delegation exists and is for the correct gateway
	match state.database.get_delegation_for_slot(signed_constraints.message.slot) {
		Ok(Some(delegation)) => {
			if delegation.message.delegate != signed_constraints.message.delegate {
				error!("Delegation for slot {} is not for the correct gateway", signed_constraints.message.slot);
				return StatusCode::FORBIDDEN;
			}
			info!("Delegation exists for slot {}", signed_constraints.message.slot);
		}
		Ok(None) => {
			error!("No delegation found for slot {}", signed_constraints.message.slot);
			return StatusCode::NOT_FOUND;
		}
		Err(e) => {
			error!("Error checking for delegation for slot {}: {}", signed_constraints.message.slot, e);
			return StatusCode::INTERNAL_SERVER_ERROR;
		}
	}

	// Store signed constraints in database
	match state.database.store_signed_constraints(&signed_constraints) {
		Ok(_) => {
			info!("Signed constraints stored successfully for slot {}", signed_constraints.message.slot);
			StatusCode::OK
		}
		Err(e) => {
			error!("Failed to store signed constraints: {}", e);
			StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}

/// GET /constraints/v0/relay/constraints/{slot} - Get constraints for a slot with BLS signature verification
pub async fn get_constraints_for_slot_handler(
	State(state): State<RelayState>,
	Path(slot): Path<u64>,
	headers: HeaderMap,
) -> Result<Json<Vec<SignedConstraints>>, StatusCode> {
	info!("Getting constraints for slot {}", slot);

	// Get current slot to check if target slot has passed
	let current_slot = state.slot_timer.get_current_slot();

	// If we're at slot_target + 1 or beyond, bypass authentication
	if current_slot > slot {
		info!("Slot {} has passed (current slot: {}), bypassing authentication", slot, current_slot);

		// Simply fetch and return all constraints for this slot
		let signed_constraints = match state.database.get_signed_constraints_for_slot(slot) {
			Ok(constraints) => {
				info!("Retrieved {} signed constraints from database for slot {}", constraints.len(), slot);
				constraints
			}
			Err(e) => {
				error!("Failed to get signed constraints for slot {}: {}", slot, e);
				return Err(StatusCode::INTERNAL_SERVER_ERROR);
			}
		};

		info!("Returning {} constraints for past slot {}", signed_constraints.len(), slot);
		return Ok(Json(signed_constraints));
	}

	// Slot has not passed yet - enforce authentication
	info!("Slot {} has not passed yet (current slot: {}), enforcing authentication", slot, current_slot);
	info!("Headers count: {}", headers.len());

	// Extract and parse BLS signature, public key, nonce, and signing_id from headers
	let (public_key, bls_signature, nonce, signing_id) = extract_bls_auth_headers(&headers)?;

	// Compute slot hash for signature verification
	let slot_hash = alloy::primitives::keccak256(slot.to_string().as_bytes());

	// Verify caller's signature against the slot hash using standardized commit-boost verification
	let is_valid = verify_proposer_commitment_signature_bls_for_message(
		state.config.relay.chain,
		&public_key,
		&slot_hash,
		&bls_signature,
		&signing_id,
		nonce,
	);

	// Debug output
	debug!("DEBUG: Slot: {}", slot);
	debug!("DEBUG: Slot hash: {:?}", slot_hash);
	debug!("DEBUG: Public key: {:?}", public_key.serialize());
	debug!("DEBUG: Signature: {:?}", bls_signature.serialize());
	debug!("DEBUG: Nonce: {}", nonce);
	debug!("DEBUG: Signing ID: {:?}", signing_id);
	debug!("DEBUG: Signature verification result: {}", is_valid);

	if !is_valid {
		error!("Invalid BLS signature for slot {}", slot);
		return Err(StatusCode::BAD_REQUEST);
	}

	// Get signed constraints from database
	let signed_constraints = match state.database.get_signed_constraints_for_slot(slot) {
		Ok(constraints) => {
			debug!("DEBUG: Retrieved {} signed constraints from database for slot {}", constraints.len(), slot);
			constraints
		}
		Err(e) => {
			error!("Failed to get signed constraints for slot {}: {}", slot, e);
			return Err(StatusCode::INTERNAL_SERVER_ERROR);
		}
	};

	// Filter constraints where the public key is in the receivers list
	// If receivers list is empty, allow access to all constraints (public endpoint)
	let mut authorized_constraints = Vec::new();
	for signed_constraints_item in signed_constraints {
		if signed_constraints_item.message.receivers.is_empty()
			|| signed_constraints_item.message.receivers.iter().any(|receiver| receiver == &public_key)
		{
			authorized_constraints.push(signed_constraints_item);
		}
	}

	info!("Returning {} authorized constraints for slot {}", authorized_constraints.len(), slot);
	Ok(Json(authorized_constraints))
}

/// GET /constraints/v0/builder/capabilities - Get constraint capabilities
pub async fn capabilities_handler(State(state): State<RelayState>) -> Result<Json<ConstraintCapabilities>, StatusCode> {
	info!("Getting constraint capabilities");

	let capabilities = ConstraintCapabilities { constraint_types: state.config.relay.constraint_capabilities.clone() };

	info!("Returning constraint capabilities: {:?}", capabilities.constraint_types);
	Ok(Json(capabilities))
}

/// GET /health - Health check endpoint
pub async fn health_handler(State(state): State<RelayState>) -> Result<Json<HealthResponse>, StatusCode> {
	match state.database.db_healthcheck().await {
		Ok(_) => {
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
	fn create_test_database() -> (DatabaseContext, TempDir) {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_relay.db");
		let mut opts = rocksdb::Options::default();
		opts.create_if_missing(true);
		let db = Arc::new(rocksdb::DB::open(&opts, &db_path).unwrap());
		let database = DatabaseContext::new(db);
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
	#[allow(dead_code)]
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
	fn test_extract_bls_auth_headers_with_all_fields() {
		// Use a valid 48-byte BLS public key
		let public_key =
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		// Use a valid 96-byte BLS signature (this is a real BLS signature from commit-boost test data)
		let signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

		let mut headers = create_test_headers(public_key, signature);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("42").unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str("0x0202020202020202020202020202020202020202020202020202020202020202").unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);

		assert!(result.is_ok());
		let (parsed_pubkey, parsed_sig, nonce, signing_id) = result.unwrap();
		assert_eq!(
			parsed_pubkey.serialize().as_slice(),
			hex::decode(public_key.strip_prefix("0x").unwrap()).unwrap().as_slice()
		);
		assert_eq!(
			parsed_sig.serialize().as_slice(),
			hex::decode(signature.strip_prefix("0x").unwrap()).unwrap().as_slice()
		);
		assert_eq!(nonce, 42);
		assert_eq!(signing_id, alloy::primitives::B256::from_slice(&[0x02; 32]));
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

		let mut headers = create_test_headers(public_key, signature);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("123").unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str("0x0303030303030303030303030303030303030303030303030303030303030303").unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);

		assert!(result.is_ok());
	}

	#[test]
	fn test_extract_bls_auth_headers_public_key_without_0x_prefix() {
		let public_key =
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
		let signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

		let mut headers = create_test_headers(public_key, signature);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("456").unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str("0x0404040404040404040404040404040404040404040404040404040404040404").unwrap(),
		);

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

	#[tokio::test]
	async fn test_capabilities_handler() {
		let (database, _temp_dir) = create_test_database();
		let config = crate::config::RelayConfig::default();
		let slot_timer = common::slot_timer::SlotTimer::new(config.relay.genesis_timestamp);
		let state = RelayState { database: Arc::new(database), config, slot_timer };

		let result = capabilities_handler(State(state)).await;
		assert!(result.is_ok());

		let capabilities = result.unwrap();
		assert_eq!(capabilities.constraint_types, vec![1]);
	}

	#[test]
	fn test_extract_bls_auth_headers_missing_nonce() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		);
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str("0x0101010101010101010101010101010101010101010101010101010101010101").unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_missing_signing_id() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("1337").unwrap());

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_invalid_nonce_format() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("not_a_number").unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str("0x0101010101010101010101010101010101010101010101010101010101010101").unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_invalid_signing_id_format() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("1337").unwrap());
		headers.insert("X-Receiver-SigningId", HeaderValue::from_str("not_hex").unwrap());

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_invalid_signing_id_length() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("1337").unwrap());
		headers.insert("X-Receiver-SigningId", HeaderValue::from_str("0x0101").unwrap()); // Too short

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[test]
	fn test_extract_bls_auth_headers_success() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str("1337").unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str("0x0101010101010101010101010101010101010101010101010101010101010101").unwrap(),
		);

		let result = extract_bls_auth_headers(&headers);
		assert!(result.is_ok());
		let (_, _, nonce, signing_id) = result.unwrap();
		assert_eq!(nonce, 1337);
		assert_eq!(signing_id, alloy::primitives::B256::from_slice(&[0x01; 32]));
	}

	#[tokio::test]
	async fn test_get_constraints_past_slot_bypasses_authentication() {
		use commit_boost::prelude::BlsSignature;

		let (database, _temp_dir) = create_test_database();

		// Create a slot timer with a genesis timestamp that makes the current slot high
		// Genesis timestamp far in the past so current slot is very high
		let genesis_timestamp = 1606824023; // Beacon chain genesis
		let slot_timer = common::slot_timer::SlotTimer::new(genesis_timestamp);
		let current_slot = slot_timer.get_current_slot();

		// Create a constraint for a past slot (current_slot - 10)
		let past_slot = current_slot.saturating_sub(10);

		let constraints_message = ConstraintsMessage {
			proposer:
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			delegate:
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			slot: past_slot,
			constraints: vec![],
			receivers: vec![
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			],
		};

		let signed_constraints = SignedConstraints {
			message: constraints_message,
			nonce: 1337,
			signing_id: "0x3078313233340000000000000000000000000000000000000000000000000000".parse().unwrap(),
			signature: BlsSignature::empty(),
		};

		// Store constraints in database
		Arc::new(database.clone()).store_signed_constraints(&signed_constraints).unwrap();

		let config = crate::config::RelayConfig::default();
		let state = RelayState { database: Arc::new(database), config, slot_timer };

		// Call handler without auth headers
		let headers = HeaderMap::new();
		let result = get_constraints_for_slot_handler(State(state), Path(past_slot), headers).await;

		// Should succeed without authentication
		assert!(result.is_ok());
		let constraints = result.unwrap();
		assert_eq!(constraints.0.len(), 1);
	}

	#[tokio::test]
	async fn test_get_constraints_current_slot_requires_authentication() {
		let (database, _temp_dir) = create_test_database();

		// Create a slot timer for current slot
		let genesis_timestamp = 1606824023;
		let slot_timer = common::slot_timer::SlotTimer::new(genesis_timestamp);
		let current_slot = slot_timer.get_current_slot();

		let config = crate::config::RelayConfig::default();
		let state = RelayState { database: Arc::new(database), config, slot_timer };

		// Call handler without auth headers for current slot
		let headers = HeaderMap::new();
		let result = get_constraints_for_slot_handler(State(state), Path(current_slot), headers).await;

		// Should fail due to missing authentication
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_constraints_future_slot_requires_authentication() {
		let (database, _temp_dir) = create_test_database();

		// Create a slot timer for future slot
		let genesis_timestamp = 1606824023;
		let slot_timer = common::slot_timer::SlotTimer::new(genesis_timestamp);
		let current_slot = slot_timer.get_current_slot();
		let future_slot = current_slot + 100;

		let config = crate::config::RelayConfig::default();
		let state = RelayState { database: Arc::new(database), config, slot_timer };

		// Call handler without auth headers for future slot
		let headers = HeaderMap::new();
		let result = get_constraints_for_slot_handler(State(state), Path(future_slot), headers).await;

		// Should fail due to missing authentication
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_constraints_past_slot_returns_all_constraints_regardless_of_receivers() {
		use commit_boost::prelude::BlsSignature;

		let (database, _temp_dir) = create_test_database();

		// Create a slot timer with genesis far in the past
		let genesis_timestamp = 1606824023;
		let slot_timer = common::slot_timer::SlotTimer::new(genesis_timestamp);
		let current_slot = slot_timer.get_current_slot();

		// Create a constraint for a past slot with specific receivers
		let past_slot = current_slot.saturating_sub(10);

		// Create constraint with a specific receiver list
		let receiver_pubkey: commit_boost::prelude::BlsPublicKey =
			"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
				.parse()
				.unwrap();

		let constraints_message = ConstraintsMessage {
			proposer:
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			delegate:
				"0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"
					.parse()
					.unwrap(),
			slot: past_slot,
			constraints: vec![],
			receivers: vec![receiver_pubkey], // Specific receiver
		};

		let signed_constraints = SignedConstraints {
			message: constraints_message,
			nonce: 1337,
			signing_id: "0x3078313233340000000000000000000000000000000000000000000000000000".parse().unwrap(),
			signature: BlsSignature::empty(),
		};

		// Store constraints in database
		Arc::new(database.clone()).store_signed_constraints(&signed_constraints).unwrap();

		let config = crate::config::RelayConfig::default();
		let state = RelayState { database: Arc::new(database), config, slot_timer };

		// Call handler without auth headers (so no matching public key)
		let headers = HeaderMap::new();
		let result = get_constraints_for_slot_handler(State(state), Path(past_slot), headers).await;

		// Should succeed and return all constraints (no filtering by receivers)
		assert!(result.is_ok());
		let constraints = result.unwrap();
		assert_eq!(constraints.0.len(), 1);
		assert_eq!(constraints.0[0].message.receivers.len(), 1);
	}
}
