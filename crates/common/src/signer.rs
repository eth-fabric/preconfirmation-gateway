use alloy::primitives::{Address, B256};
use commit_boost::prelude::{
	BlsPublicKey, StartCommitModuleConfig,
	commit::{
		request::{EncryptionScheme, SignProxyRequest},
		response::{BlsSignResponse, EcdsaSignResponse},
	},
	verify_proposer_commitment_signature_ecdsa_for_message,
};
use eyre::{Result, WrapErr};
use tracing::{debug, error, info};

use crate::constants::SIGNING_ID;

/// Calls the proxy_ecdsa signer to sign a hash
pub async fn call_proxy_ecdsa_signer<T>(
	commit_config: &mut StartCommitModuleConfig<T>,
	commitment_hash: B256,
	committer: Address,
) -> Result<EcdsaSignResponse> {
	debug!("Calling proxy_ecdsa signer for commitment hash: {:?}", commitment_hash);

	let proxy_request_ecdsa = SignProxyRequest::builder(committer).with_root(commitment_hash);

	// Make the actual API call to the signer service
	let proxy_response_ecdsa = commit_config
		.signer_client
		.request_proxy_signature_ecdsa(proxy_request_ecdsa)
		.await
		.wrap_err("Failed to request proxy signature from signer service")?;

	match verify_proposer_commitment_signature_ecdsa_for_message(
		commit_config.chain,
		&committer,
		&commitment_hash,
		&proxy_response_ecdsa.signature,
		&SIGNING_ID,
		proxy_response_ecdsa.nonce,
	) {
		Ok(_) => info!("Signature verified successfully"),
		Err(err) => error!(%err, "Signature verification failed"),
	};

	Ok(proxy_response_ecdsa)
}

/// Calls the proxy_bls signer to sign a hash
pub async fn call_proxy_bls_signer<T>(
	commit_config: &mut StartCommitModuleConfig<T>,
	constraints_hash: B256,
	bls_public_key: BlsPublicKey,
) -> Result<BlsSignResponse> {
	debug!("Calling proxy_bls signer for constraints hash: {:?}", constraints_hash);

	let proxy_request_bls = SignProxyRequest::builder(bls_public_key).with_root(constraints_hash);

	// Make the actual API call to the signer service
	let proxy_response_bls = commit_config
		.signer_client
		.request_proxy_signature_bls(proxy_request_bls)
		.await
		.wrap_err("Failed to request proxy BLS signature from signer service")?;

	// Note: BLS signature verification would be different from ECDSA
	// For now, we'll log the response without verification
	// TODO: Add BLS signature verification when available
	// todo use verify_proposer_commitment_signature_bls_for_data
	info!("BLS signature received (verification pending)");

	Ok(proxy_response_bls)
}

/// Generates a proxy key using the signer client
pub async fn generate_proxy_key_ecdsa<T>(
	commit_config: &mut StartCommitModuleConfig<T>,
	bls_public_key: BlsPublicKey,
) -> Result<Address> {
	debug!("Generating ECDSA proxy key for BLS public key: {:?}", bls_public_key);

	let signed_delegation = commit_config
		.signer_client
		.generate_proxy_key_ecdsa(bls_public_key)
		.await
		.wrap_err("Failed to generate ECDSA proxy key")?;

	let proxy_address = signed_delegation.message.proxy;
	info!("Generated ECDSA proxy key: {:?}", proxy_address);
	Ok(proxy_address)
}

/// Generates a BLS proxy key using the signer client
pub async fn generate_proxy_key_bls<T>(
	commit_config: &mut StartCommitModuleConfig<T>,
	bls_public_key: BlsPublicKey,
) -> Result<BlsPublicKey> {
	debug!("Generating BLS proxy key for BLS public key: {:?}", bls_public_key);

	let signed_delegation = commit_config
		.signer_client
		.generate_proxy_key_bls(bls_public_key)
		.await
		.wrap_err("Failed to generate BLS proxy key")?;

	let proxy_key = signed_delegation.message.proxy;
	info!("Generated BLS proxy key: {:?}", proxy_key);
	Ok(proxy_key)
}
