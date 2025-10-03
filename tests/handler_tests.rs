use alloy::primitives::{Address, B256, b256, hex};
use jsonrpsee::Extensions;
use jsonrpsee::types::Params;
use preconfirmation_gateway::rpc::handlers::commitment_request_handler;
use preconfirmation_gateway::types::rpc::InclusionPayload;
use preconfirmation_gateway::types::{CommitmentRequest, DatabaseContext, RpcContext};
use rocksdb::{DB, Options};
use std::sync::Arc;
use tempfile::TempDir;

mod common;
use common::start_local_signer_server;

const MODULE_ID: &str = "inclusion-preconf-module";
const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");
const ADMIN_SECRET: &str = "test-admin-secret";
const PORT: u16 = 20201; // Use different port to avoid conflicts
const PUBKEY: [u8; 48] =
	hex!("883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4");

#[tokio::test]
async fn test_commitment_request_handler_with_local_signer() -> eyre::Result<()> {
	// Start local signer server
	let mut commit_config = start_local_signer_server(MODULE_ID, SIGNING_ID, ADMIN_SECRET, PORT).await?;

	// First, generate a proxy key for the committer using the local signer
	let test_bls_pubkey = cb_common::types::BlsPublicKey::deserialize(&PUBKEY).unwrap();
	let proxy_address = commit_config
		.signer_client
		.generate_proxy_key_ecdsa(test_bls_pubkey)
		.await
		.map_err(|e| eyre::eyre!("Failed to generate ECDSA proxy key: {}", e))?;
	let committer_address = proxy_address.message.proxy;

	// Create a temporary RocksDB database for testing
	let temp_dir = TempDir::new().unwrap();
	let db_path = temp_dir.path().join("test_db");

	let mut opts = Options::default();
	opts.create_if_missing(true);
	let db = DB::open(&opts, &db_path).unwrap();
	let database = DatabaseContext::new(Arc::new(db));

	// Create RPC context
	let context = Arc::new(RpcContext {
		database,
		commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
		committer_address,
	});

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
	let result = commitment_request_handler::<()>(params, context, Extensions::default()).await;

	// Verify the result
	match result {
		Ok(signed_commitment) => {
			println!("✅ Successfully processed commitment request");
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
