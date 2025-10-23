use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, Bytes, B256, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use cb_common::types::BlsPublicKey;
use commit_boost::prelude::StartCommitModuleConfig;
use common::signer::call_bls_signer;
use common::types::constraints::{Registration, SignedRegistration, URCRegisterInputs};
use eyre::{Context, Result};
use tracing::info;

use crate::utils::get_consensus_keys;

pub async fn sign_registration<T>(
	commit_config: &mut StartCommitModuleConfig<T>,
	pubkey: &BlsPublicKey,
	owner: Address,
	module_signing_id: &B256,
) -> Result<SignedRegistration> {
	// 1. Create Registration message
	let registration = Registration { owner };

	// 2. Get message hash
	let message_hash = registration.to_message_hash()?;

	// 3. Sign with BLS using call_bls_signer
	let bls_response = call_bls_signer(commit_config, message_hash, pubkey.clone(), module_signing_id).await?;

	// 4. Return SignedRegistration
	Ok(SignedRegistration { pubkey: pubkey.clone(), signature: bls_response.signature, nonce: bls_response.nonce })
}

/// Generate SignedRegistration for all consensus keys
pub async fn sign_registrations<T>(
	commit_config: &mut StartCommitModuleConfig<T>,
	owner: Address,
	module_signing_id: &B256,
) -> Result<URCRegisterInputs> {
	// 1. Get all consensus keys using existing get_consensus_keys()
	let pubkeys = get_consensus_keys(commit_config).await?;

	info!("Signing registrations for {} consensus keys", pubkeys.len());

	// 2. Sign registration for each key
	let mut registrations = Vec::new();
	for (i, pubkey) in pubkeys.iter().enumerate() {
		info!("  [{}/{}] Signing for pubkey: {}", i + 1, pubkeys.len(), pubkey.as_hex_string());
		let signed_reg = sign_registration(commit_config, pubkey, owner, module_signing_id).await?;
		registrations.push(signed_reg);
	}

	// 3. Return URCRegisterInputs
	Ok(URCRegisterInputs { registrations, owner, signing_id: *module_signing_id })
}

/// Send URC registration transaction
pub async fn send_registration_transaction(
	inputs: &URCRegisterInputs,
	urc_address: Address,
	execution_rpc_url: &str,
	keystore_path: &str,
	password: &str,
	collateral: U256,
) -> Result<B256> {
	// 1. Load keystore and create signer
	// Decrypt the keystore using eth-keystore crate
	let private_key = eth_keystore::decrypt_key(keystore_path, password)?;

	// Create signer from the private key bytes
	let signer = PrivateKeySigner::from_bytes(&B256::from_slice(&private_key))
		.context("Failed to create signer from private key")?;

	let wallet = EthereumWallet::from(signer);

	info!("Loaded wallet from keystore: {:?}", wallet.default_signer().address());

	// 2. Create provider with wallet
	let provider =
		ProviderBuilder::new().wallet(wallet).connect_http(execution_rpc_url.parse().context("Invalid RPC URL")?);

	// 3. Encode calldata for register()
	let calldata = inputs.abi_encode_with_selector()?;

	// 4. Create transaction
	let tx = TransactionRequest::default().with_to(urc_address).with_value(collateral).with_input(calldata);

	info!("Sending URC registration transaction...");
	info!("  URC contract: {:?}", urc_address);
	info!("  Collateral: {} wei", collateral);
	info!("  Number of keys: {}", inputs.registrations.len());

	// 5. Send transaction and wait for receipt
	let pending_tx = provider.send_transaction(tx).await.context("Failed to send transaction")?;

	info!("Transaction sent, waiting for confirmation...");

	let tx_hash = pending_tx.watch().await.context("Failed to confirm transaction")?;

	info!("URC registration transaction confirmed: {:?}", tx_hash);

	Ok(tx_hash)
}
