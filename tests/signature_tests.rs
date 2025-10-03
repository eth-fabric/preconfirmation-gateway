use alloy::primitives::{Address, B256, b256, hex};
use cb_common::commit::request::EncryptionScheme;
use commit_boost::prelude::*;
use eyre::Result;

mod common;
use common::start_local_signer_server;

const MODULE_ID: &str = "inclusion-preconf-module";
const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");
const ADMIN_SECRET: &str = "test-admin-secret";
const PORT: u16 = 20200;
const PUBKEY: [u8; 48] =
	hex!("883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4");

/// Helper function to generate a proxy key using a local signer service
/// This encapsulates the pattern of starting a signer, making a request, and extracting the proxy address
pub async fn generate_proxy_key_with_local_signer(
	bls_pubkey: BlsPublicKey,
	scheme: EncryptionScheme,
) -> Result<String> {
	// Use the new function to get the commit config
	let mut commit_config = start_local_signer_server(MODULE_ID, SIGNING_ID, ADMIN_SECRET, PORT).await?;

	// Use the appropriate SignerClient method based on the scheme
	let proxy_address = match scheme {
		EncryptionScheme::Ecdsa => {
			let signed_delegation = commit_config
				.signer_client
				.generate_proxy_key_ecdsa(bls_pubkey)
				.await
				.map_err(|e| eyre::eyre!("Failed to generate ECDSA proxy key: {}", e))?;
			signed_delegation.message.proxy.to_string()
		}
		EncryptionScheme::Bls => {
			let signed_delegation = commit_config
				.signer_client
				.generate_proxy_key_bls(bls_pubkey)
				.await
				.map_err(|e| eyre::eyre!("Failed to generate BLS proxy key: {}", e))?;
			signed_delegation.message.proxy.to_string()
		}
	};

	Ok(proxy_address)
}

#[tokio::test]
async fn test_local_generate_proxy_key_ecdsa() -> Result<()> {
	let test_bls_pubkey = BlsPublicKey::deserialize(&PUBKEY).unwrap();

	// Use the helper function to generate a proxy key
	let proxy_address = generate_proxy_key_with_local_signer(test_bls_pubkey, EncryptionScheme::Ecdsa).await?;

	// Verify the proxy address is not empty
	assert!(!proxy_address.is_empty(), "Proxy address should not be empty");

	println!("✅ Generated proxy address: {}", proxy_address);

	Ok(())
}

#[tokio::test]
async fn test_generate_proxy_key_helper_function() -> Result<()> {
	// Test the helper function with different BLS public keys and schemes
	let test_bls_pubkey = BlsPublicKey::deserialize(&PUBKEY).unwrap();

	// Test ECDSA scheme
	let ecdsa_proxy = generate_proxy_key_with_local_signer(test_bls_pubkey.clone(), EncryptionScheme::Ecdsa).await?;
	assert!(!ecdsa_proxy.is_empty(), "ECDSA proxy address should not be empty");

	// Decode the ECDSA proxy address into an Address type
	let ecdsa_address =
		ecdsa_proxy.parse::<Address>().map_err(|e| eyre::eyre!("Failed to parse ECDSA proxy address: {}", e))?;
	println!("Decoded ECDSA Address: {}", ecdsa_address);

	// Test that we can generate multiple proxy keys (they should be different)
	let ecdsa_proxy2 = generate_proxy_key_with_local_signer(test_bls_pubkey.clone(), EncryptionScheme::Ecdsa).await?;
	assert_ne!(ecdsa_proxy, ecdsa_proxy2, "Multiple proxy keys should be different");

	// Decode the second ECDSA proxy address
	let ecdsa_address2 = ecdsa_proxy2
		.parse::<Address>()
		.map_err(|e| eyre::eyre!("Failed to parse second ECDSA proxy address: {}", e))?;
	println!("Decoded second ECDSA Address: {}", ecdsa_address2);

	// Verify the addresses are different
	assert_ne!(ecdsa_address, ecdsa_address2, "Decoded addresses should be different");

	// Test BLS scheme
	let bls_proxy = generate_proxy_key_with_local_signer(test_bls_pubkey.clone(), EncryptionScheme::Bls).await?;
	assert!(!bls_proxy.is_empty(), "BLS proxy address should not be empty");
	println!("BLS proxy address: {}", bls_proxy);

	// Decode the BLS proxy into a BlsPublicKey
	let bls_pubkey =
		bls_proxy.parse::<BlsPublicKey>().map_err(|e| eyre::eyre!("Failed to parse BLS proxy public key: {}", e))?;
	println!("Decoded BLS PublicKey: {}", bls_pubkey);

	// Verify the BLS public key is different from the original
	assert_ne!(bls_pubkey, test_bls_pubkey, "Generated BLS proxy should be different from original");

	Ok(())
}

#[tokio::test]
async fn test_call_proxy_ecdsa_signer_with_local_signer() -> Result<()> {
	use alloy::primitives::{Address, b256};
	use cb_common::commit::request::EncryptionScheme;
	use preconfirmation_gateway::signing::call_proxy_ecdsa_signer;

	// Start the local signer server and get the reconstructed StartCommitModuleConfig
	let mut commit_config = start_local_signer_server(MODULE_ID, SIGNING_ID, ADMIN_SECRET, PORT).await?;

	// First, generate a proxy key for the committer
	let test_bls_pubkey = BlsPublicKey::deserialize(&PUBKEY).unwrap();
	let proxy_address = generate_proxy_key_with_local_signer(test_bls_pubkey, EncryptionScheme::Ecdsa).await?;

	// Parse the proxy address as the committer
	let committer =
		proxy_address.parse::<Address>().map_err(|e| eyre::eyre!("Failed to parse proxy address: {}", e))?;

	// Create a test commitment hash
	let commitment_hash = b256!("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");

	println!("Testing call_proxy_ecdsa_signer with local signer");
	println!("   Commitment hash: {:?}", commitment_hash);
	println!("   Committer (proxy): {}", committer);

	// Call the proxy_ecdsa signer function
	let response = call_proxy_ecdsa_signer(&mut commit_config, commitment_hash, committer).await?;

	// Verify the response
	assert!(response.nonce > 0, "Nonce should be greater than 0");
	assert_eq!(response.module_signing_id, SIGNING_ID, "Module signing ID should match");

	// Just verify that we got a response (the signature verification is done in the signing function)
	println!("   Signature received: {:?}", response.signature);

	println!("Successfully called proxy_ecdsa signer with local signer");
	println!("   Signature: {:?}", response.signature);
	println!("   Nonce: {}", response.nonce);
	println!("   Module signing ID: {:?}", response.module_signing_id);

	Ok(())
}
