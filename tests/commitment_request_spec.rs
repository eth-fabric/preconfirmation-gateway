use alloy::primitives::Address;
use jsonrpsee::Extensions;
use jsonrpsee::types::Params;
use preconfirmation_gateway::rpc::handlers::commitment_request_handler;
use preconfirmation_gateway::types::CommitmentRequest;
use preconfirmation_gateway::types::commitments::InclusionPayload;
use std::sync::Arc;

mod common;
use common::{PUBKEY, create_test_context};

#[tokio::test]
async fn test_commitment_request_handler() -> eyre::Result<()> {
	// Create test context with random port
	let mut context = create_test_context().await?;

	// Generate a proxy key for the committer using the signer from the context
	let test_bls_pubkey = cb_common::types::BlsPublicKey::deserialize(&PUBKEY).unwrap();
	let mut commit_config = context.commit_config.lock().await;
	let proxy_address = commit_config
		.signer_client
		.generate_proxy_key_ecdsa(test_bls_pubkey)
		.await
		.map_err(|e| eyre::eyre!("Failed to generate ECDSA proxy key: {}", e))?;
	let committer_address = proxy_address.message.proxy;
	drop(commit_config); // Release the lock

	// Update the context with the committer address
	context.committer_address = committer_address;

	// Create a test inclusion payload
	let inclusion_payload = InclusionPayload { slot: 12345, signed_tx: vec![0x01, 0x02, 0x03, 0x04].into() };

	// ABI-encode the payload
	let encoded_payload = inclusion_payload.abi_encode()?;

	// Create a test commitment request
	let test_request =
		CommitmentRequest { commitment_type: 1, payload: encoded_payload, slasher: Address::from([0x2; 20]) };

	// Serialize the request to JSON for the params
	let request_json = serde_json::to_string(&test_request)?;
	let params = Params::new(Some(&request_json));

	// Call the handler
	let result = commitment_request_handler::<()>(params, Arc::new(context), Extensions::default()).await;

	// Verify the result
	match result {
		Ok(signed_commitment) => {
			println!("Successfully processed commitment request");
			println!("   Commitment type: {}", signed_commitment.commitment.commitment_type);
			println!("   Nonce: {}", signed_commitment.nonce);
			println!("   Signature: {}", signed_commitment.signature);

			// Basic assertions
			assert_eq!(signed_commitment.commitment.commitment_type, 1);
			assert!(signed_commitment.nonce > 0, "Nonce should be greater than 0");
			assert!(!signed_commitment.signature.to_string().is_empty(), "Signature should not be empty");
		}
		Err(e) => {
			panic!("Handler failed: {}", e);
		}
	}

	Ok(())
}
