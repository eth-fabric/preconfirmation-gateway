use alloy::primitives::B256;
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use common::constants::CONSTRAINT_TYPE;
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

	// 1. Get the message hash
	let message_hash = message.to_message_hash()?;

	// 2. Call the proxy_bls signer
	let response = {
		let mut commit_config = commit_config.lock().await;
		signer::call_proxy_bls_signer(&mut *commit_config, message_hash, bls_public_key).await?
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
pub fn create_constraints_message(
	pending_constraints: Vec<(B256, Constraint)>,
	proposer: BlsPublicKey,
	delegate: BlsPublicKey,
	slot: u64,
	receivers: Vec<BlsPublicKey>,
) -> Result<ConstraintsMessage> {
	info!("Creating constraints message from {} pending constraints", pending_constraints.len());

	if pending_constraints.is_empty() {
		return Err(eyre::eyre!("No pending constraints to process"));
	}

	let constraints: Vec<Constraint> = pending_constraints.into_iter().map(|(_, constraint)| constraint).collect();

	let message = ConstraintsMessage { proposer, delegate, slot, constraints, receivers };

	info!("Created constraints message with {} constraints", message.constraints.len());
	Ok(message)
}

/// Validates a constraint
pub fn validate_constraint(constraint: &Constraint) -> Result<()> {
	if constraint.constraint_type != CONSTRAINT_TYPE {
		return Err(eyre::eyre!("Invalid constraint type: {}", constraint.constraint_type));
	}

	if constraint.payload.is_empty() {
		return Err(eyre::eyre!("Empty constraint payload"));
	}

	// todo other validation logic

	Ok(())
}

/// Parse a BLS public key from hex string with error handling
pub fn parse_bls_public_key(hex_string: &str, field_name: &str) -> Result<BlsPublicKey> {
	cb_common::utils::bls_pubkey_from_hex(hex_string)
		.map_err(|e| eyre::eyre!("Invalid {} BLS public key: {}", field_name, e))
}

/// Parse multiple BLS public keys from hex strings with error handling
pub fn parse_bls_public_keys(hex_strings: &[String], field_name: &str) -> Result<Vec<BlsPublicKey>> {
	let mut keys = Vec::new();
	for (index, hex_string) in hex_strings.iter().enumerate() {
		let key = cb_common::utils::bls_pubkey_from_hex(hex_string)
			.map_err(|e| eyre::eyre!("Invalid {} BLS public key at index {}: {}", field_name, index, e))?;
		keys.push(key);
	}
	Ok(keys)
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
