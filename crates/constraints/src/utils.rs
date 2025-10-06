use alloy::primitives::{B256, Signature, U256, keccak256};
use commit_boost::prelude::{BlsPublicKey, BlsSignature, StartCommitModuleConfig};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

use blst::*;
use urc::i_registry::BLS::{G1Point, G2Point};

use common::signing;
use common::types::constraints::{ConstraintsMessage, SignedConstraints};

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
		signing::call_proxy_bls_signer(&mut *commit_config, object_root, bls_public_key).await?
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
pub fn validate_constraints_message(message: &ConstraintsMessage) -> Result<()> {
	// todo

	Ok(())
}
