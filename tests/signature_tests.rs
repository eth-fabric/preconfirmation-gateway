use std::collections::HashMap;

use alloy::primitives::{Address, B256, b256, hex};
use commit_boost::prelude::*;
use cb_common::{
	commit::{constants::GENERATE_PROXY_KEY_PATH, request::{EncryptionScheme, GenerateProxyRequest}},
	config::{load_module_signing_configs, ModuleSigningConfig, StartSignerConfig},
	types::ModuleId,
	utils::create_jwt,
};
use eyre::Result;

const MODULE_ID: &str = "inclusion-preconf-module";
const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");
const ADMIN_SECRET: &str = "test-admin-secret";
const PORT: u16 = 20200;
const PUBKEY: [u8; 48] = hex!(
    "883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4"
);

/// Starts the signer module server on a separate task and returns its configs
pub async fn start_local_signer_server() -> Result<(StartSignerConfig, ModuleSigningConfig)> {
	use cb_tests::{signer_service, utils};

	utils::setup_test_env();
	
	let mut cfg = utils::get_commit_boost_config(utils::get_pbs_static_config(utils::get_pbs_config(0)));

	let module_id = ModuleId(MODULE_ID.to_string());

	cfg.modules = Some(vec![utils::create_module_config(module_id.clone(), SIGNING_ID)]);

	let jwts = HashMap::from([(module_id.clone(), ADMIN_SECRET.to_string())]);

	let mod_cfgs = load_module_signing_configs(&cfg, &jwts)?;

	let start_config = signer_service::start_server(PORT, &mod_cfgs, ADMIN_SECRET.to_string(), false).await?;
	let jwt_config = mod_cfgs.get(&module_id).expect("JWT config for test module not found");

	Ok((start_config, jwt_config.clone()))
}

/// Helper function to generate a proxy key using a local signer service
/// This encapsulates the pattern of starting a signer, making a request, and extracting the proxy address
pub async fn generate_proxy_key_with_local_signer(
	bls_pubkey: BlsPublicKey,
	scheme: EncryptionScheme,
) -> Result<String> {
	let (start_config, jwt_config) = start_local_signer_server().await?;

	let request = GenerateProxyRequest { 
		consensus_pubkey: bls_pubkey, 
		scheme 
	};
	let payload_bytes = serde_json::to_vec(&request)?;

	let jwt = create_jwt(
		&ModuleId(MODULE_ID.to_string()),
		&jwt_config.jwt_secret,
		GENERATE_PROXY_KEY_PATH,
		Some(&payload_bytes),
	)?;
	
	let client = reqwest::Client::new();
	let url = format!("http://{}{}", start_config.endpoint, GENERATE_PROXY_KEY_PATH);
	let response = client.post(&url).json(&request).bearer_auth(&jwt).send().await?;

	// Verify the response is successful
	if !response.status().is_success() {
		return Err(eyre::eyre!("Proxy key generation failed with status: {}", response.status()));
	}

	// Parse the response and extract the proxy address
	let response_json = response.json::<serde_json::Value>().await?;
	let proxy_address = response_json["message"]["proxy"]
		.as_str()
		.ok_or_else(|| eyre::eyre!("Failed to extract proxy address from response"))?;

	Ok(proxy_address.to_string())
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
	println!("✅ ECDSA proxy address: {}", ecdsa_proxy);
	
	// Verify the proxy address looks like a valid Ethereum address
	assert!(ecdsa_proxy.starts_with("0x"), "Proxy address should start with 0x");
	assert_eq!(ecdsa_proxy.len(), 42, "Proxy address should be 42 characters long (0x + 40 hex chars)");
	
	// Decode the ECDSA proxy address into an Address type
	let ecdsa_address = ecdsa_proxy.parse::<Address>()
		.map_err(|e| eyre::eyre!("Failed to parse ECDSA proxy address: {}", e))?;
	println!("✅ Decoded ECDSA Address: {}", ecdsa_address);
	
	// Test that we can generate multiple proxy keys (they should be different)
	let ecdsa_proxy2 = generate_proxy_key_with_local_signer(test_bls_pubkey.clone(), EncryptionScheme::Ecdsa).await?;
	assert_ne!(ecdsa_proxy, ecdsa_proxy2, "Multiple proxy keys should be different");
	println!("✅ Second ECDSA proxy address: {}", ecdsa_proxy2);
	
	// Decode the second ECDSA proxy address
	let ecdsa_address2 = ecdsa_proxy2.parse::<Address>()
		.map_err(|e| eyre::eyre!("Failed to parse second ECDSA proxy address: {}", e))?;
	println!("✅ Decoded second ECDSA Address: {}", ecdsa_address2);
	
	// Verify the addresses are different
	assert_ne!(ecdsa_address, ecdsa_address2, "Decoded addresses should be different");
	
	// Test BLS scheme
	let bls_proxy = generate_proxy_key_with_local_signer(test_bls_pubkey.clone(), EncryptionScheme::Bls).await?;
	assert!(!bls_proxy.is_empty(), "BLS proxy address should not be empty");
	println!("✅ BLS proxy address: {}", bls_proxy);
	
	// For BLS, the response should be a BLS public key (96 characters: 0x + 96 hex chars)
	assert!(bls_proxy.starts_with("0x"), "BLS proxy should start with 0x");
	assert_eq!(bls_proxy.len(), 98, "BLS proxy should be 98 characters long (0x + 96 hex chars)");
	
	// Decode the BLS proxy into a BlsPublicKey
	let bls_pubkey = bls_proxy.parse::<BlsPublicKey>()
		.map_err(|e| eyre::eyre!("Failed to parse BLS proxy public key: {}", e))?;
	println!("✅ Decoded BLS PublicKey: {}", bls_pubkey);
	
	// Verify the BLS public key is different from the original
	assert_ne!(bls_pubkey, test_bls_pubkey, "Generated BLS proxy should be different from original");
	
	println!("✅ Helper function test completed successfully");
	println!("✅ ECDSA scheme: Generated and decoded {} addresses", 2);
	println!("✅ BLS scheme: Generated and decoded 1 BLS public key");
	
	Ok(())
}