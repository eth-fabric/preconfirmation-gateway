use alloy::primitives::B256;
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

use common::signer;
use common::types::constraints::{Constraint, ConstraintsMessage, SignedConstraints};

/// Creates a properly signed constraints message using BLS
pub async fn create_signed_constraints<T>(
	message: &ConstraintsMessage,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	bls_public_key: BlsPublicKey,
) -> Result<SignedConstraints> {
	debug!("Creating signed constraints with proper BLS signing");

	// 1. Get the object root
	let object_root = message.to_object_root()?;

	// 2. Call the proxy_bls signer
	let response = {
		let mut commit_config = commit_config.lock().await;
		signer::call_proxy_bls_signer(&mut *commit_config, object_root, bls_public_key).await?
	};
	debug!("Received response from proxy_bls: {:?}", response);

	let signed_constraints = SignedConstraints {
		message: message.clone(),
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature,
	};

	debug!("Signed constraints created successfully");
	Ok(signed_constraints)
}

/// Validates a constraints message
pub fn validate_constraints_message(_message: &ConstraintsMessage) -> Result<()> {
	// TODO: Implement proper validation
	// For now, just return Ok
	Ok(())
}

/// Creates a ConstraintsMessage from pending constraints
pub fn create_constraints_message(pending_constraints: Vec<(B256, Constraint)>) -> Result<ConstraintsMessage> {
	info!("Creating constraints message from {} pending constraints", pending_constraints.len());

	if pending_constraints.is_empty() {
		return Err(eyre::eyre!("No pending constraints to process"));
	}

	// TODO: Extract proposer, delegate, slot, and receivers from constraints
	// For now, using placeholder values - these will be replaced with proper values
	// Note: These are dummy values that will be replaced with real ones from config
	let proposer = cb_common::utils::bls_pubkey_from_hex(
		"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
	)
	.unwrap(); // TODO: Get from config or constraints
	let delegate = cb_common::utils::bls_pubkey_from_hex(
		"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
	)
	.unwrap(); // TODO: Get from config or constraints
	let slot = 0; // TODO: Get current slot
	let receivers = vec![]; // TODO: Get from config or constraints

	let constraints: Vec<Constraint> = pending_constraints.into_iter().map(|(_, constraint)| constraint).collect();

	let message = ConstraintsMessage { proposer, delegate, slot, constraints, receivers };

	info!("Created constraints message with {} constraints", message.constraints.len());
	Ok(message)
}

/// Validates a constraint
pub fn validate_constraint(constraint: &Constraint) -> Result<()> {
	if constraint.constraint_type == 0 {
		return Err(eyre::eyre!("Invalid constraint type: 0"));
	}

	if constraint.payload.is_empty() {
		return Err(eyre::eyre!("Empty constraint payload"));
	}

	Ok(())
}

/// Validates multiple constraints
pub fn validate_constraints(constraints: &[Constraint]) -> Result<()> {
	for (i, constraint) in constraints.iter().enumerate() {
		if let Err(e) = validate_constraint(constraint) {
			return Err(eyre::eyre!("Invalid constraint at index {}: {}", i, e));
		}
	}

	Ok(())
}
