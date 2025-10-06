use alloy::primitives::{Address, B256};
use commit_boost::prelude::{
	StartCommitModuleConfig,
	commit::{request::SignProxyRequest, response::EcdsaSignResponse},
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
