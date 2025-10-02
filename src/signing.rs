use commit_boost::prelude::{StartCommitModuleConfig, commit::{request::SignProxyRequest, response::EcdsaSignResponse}, verify_proposer_commitment_signature_ecdsa_for_message};
use alloy::primitives::{B256, Signature, Address};
use eyre::{Result, WrapErr};
use tracing::{debug, info, error};

use crate::constants::SIGNING_ID;
use crate::types::{Commitment, CommitmentRequest, SignedCommitment};

/// Calls the proxy_ecdsa signer to sign a hash
pub async fn call_proxy_ecdsa_signer<T>(
    commit_config: &mut StartCommitModuleConfig<T>,
    commitment_hash: B256, 
    committer: Address
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

/// Creates a mock signed commitment for testing/development
pub fn create_mock_signed_commitment(request: &CommitmentRequest) -> SignedCommitment {
    let commitment = create_mock_commitment(request);
    let signature = create_mock_signature();

    SignedCommitment { 
        commitment, 
        nonce: 0, // Mock nonce
        signing_id: SIGNING_ID,
        signature 
    }
}

/// Creates a mock commitment for testing/development
fn create_mock_commitment(request: &CommitmentRequest) -> Commitment {
    debug!("Creating mock commitment for type: {}", request.commitment_type);
    
    Commitment {
        commitment_type: request.commitment_type,
        payload: request.payload.clone(),
        request_hash: B256::ZERO, // Mock hash
        slasher: request.slasher,
    }
}

/// Creates a mock signature for testing/development
fn create_mock_signature() -> Signature {
    "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        .parse()
        .expect("Failed to parse mock signature - this should never happen")
}
