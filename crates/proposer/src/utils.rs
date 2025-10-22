use alloy::primitives::{Address, Bytes, B256};
use cb_common::types::BlsPublicKey;
use commit_boost::prelude::StartCommitModuleConfig;
use common::beacon::{BeaconApiClient, HttpClient};
use common::config::ProposerConfig as ProposerConfigTrait;
use common::signer::call_bls_signer;
use common::types::beacon::BeaconTiming;
use common::types::constraints::{Delegation, SignedDelegation};
use constraints::client::ConstraintsClientTrait;
use eyre::{Context, Result};
use tracing::{debug, info, warn};

use crate::config::ProposerConfig;

/// Process proposer lookahead to find upcoming duties and sign delegations
///
/// This function checks the beacon chain for proposer duties in the current and next epoch.
/// If the configured proposer is assigned to a slot, it creates, signs, and posts a delegation
/// to the relay.
pub async fn process_lookahead<H: HttpClient, C: ConstraintsClientTrait>(
	beacon_client: &BeaconApiClient<H>,
	constraints_client: &C,
	commit_config: &mut StartCommitModuleConfig<ProposerConfig>,
	current_slot: u64,
) -> Result<()> {
	let config = &commit_config.extra;

	// Parse our proposer's BLS public key
	let our_pubkey =
		parse_bls_public_key(&config.proposer_bls_public_key).context("Failed to parse proposer BLS public key")?;

	// Calculate current and next epoch
	let current_epoch = BeaconTiming::slot_to_epoch(current_slot);
	let next_epoch = current_epoch + 1;

	// Check duties for both current and next epoch
	for epoch in [current_epoch, next_epoch] {
		match process_epoch_duties(beacon_client, constraints_client, commit_config, epoch, current_slot, &our_pubkey)
			.await
		{
			Ok(posted_count) => {
				if posted_count > 0 {
					info!("Posted {} delegation(s) for epoch {}", posted_count, epoch);
				} else {
					debug!("No duties found for our proposer in epoch {}", epoch);
				}
			}
			Err(e) => {
				warn!("Error processing epoch {} duties: {}", epoch, e);
			}
		}
	}

	Ok(())
}

/// Process duties for a specific epoch
async fn process_epoch_duties<H: HttpClient, C: ConstraintsClientTrait>(
	beacon_client: &BeaconApiClient<H>,
	constraints_client: &C,
	commit_config: &mut StartCommitModuleConfig<ProposerConfig>,
	epoch: u64,
	current_slot: u64,
	our_pubkey: &BlsPublicKey,
) -> Result<usize> {
	// Get proposer duties for this epoch
	let duties = beacon_client.get_proposer_duties(epoch).await?;

	let mut posted_count = 0;

	// Parse module_signing_id from config
	let module_signing_id = commit_config
		.extra
		.module_signing_id()
		.parse::<B256>()
		.context("Failed to parse module_signing_id from config")?;

	// Check each duty to see if it's for our proposer
	for duty in duties.data {
		let duty_pubkey = duty.parse_pubkey().context("Failed to parse duty pubkey")?;
		let duty_slot = duty.parse_slot().context("Failed to parse duty slot")?;

		// Only process duties that:
		// 1. Match our proposer key
		// 2. Are in the future (slot > current_slot)
		if duty_pubkey == *our_pubkey && duty_slot > current_slot {
			info!("Found proposer duty for slot {}", duty_slot);

			// Create and sign delegation
			let delegation = create_delegation(&commit_config.extra, duty_slot)?;
			let signed_delegation = sign_delegation(commit_config, &delegation, &module_signing_id).await?;

			// Post to relay
			constraints_client
				.post_delegation(&signed_delegation)
				.await
				.context("Failed to post delegation to relay")?;

			info!("Successfully posted delegation for slot {}", duty_slot);
			posted_count += 1;
		}
	}

	Ok(posted_count)
}

/// Create a delegation message from configuration
fn create_delegation(config: &ProposerConfig, slot: u64) -> Result<Delegation> {
	let proposer =
		parse_bls_public_key(&config.proposer_bls_public_key).context("Failed to parse proposer BLS public key")?;

	let delegate =
		parse_bls_public_key(&config.delegate_bls_public_key).context("Failed to parse delegate BLS public key")?;

	let committer: Address = config.committer_address.parse().context("Failed to parse committer address")?;

	Ok(Delegation { proposer, delegate, committer, slot, metadata: Bytes::new() })
}

/// Sign a delegation message using the consensus BLS key
async fn sign_delegation(
	commit_config: &mut StartCommitModuleConfig<ProposerConfig>,
	delegation: &Delegation,
	module_signing_id: &B256,
) -> Result<SignedDelegation> {
	// Get the delegation message hash
	let message_hash = delegation.to_message_hash().context("Failed to compute delegation message hash")?;

	// Sign using the consensus BLS signer
	let bls_response = call_bls_signer(commit_config, message_hash, delegation.proposer.clone(), module_signing_id)
		.await
		.context("Failed to sign delegation with BLS signer")?;

	Ok(SignedDelegation {
		message: delegation.clone(),
		nonce: bls_response.nonce,
		signing_id: bls_response.module_signing_id,
		signature: bls_response.signature,
	})
}

/// Parse a BLS public key from hex string
fn parse_bls_public_key(hex_str: &str) -> Result<BlsPublicKey> {
	let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
	let bytes = hex::decode(hex_str).context("Failed to decode BLS public key hex")?;

	if bytes.len() != 48 {
		eyre::bail!("Invalid BLS public key length: expected 48 bytes, got {}", bytes.len());
	}

	let mut pubkey_bytes = [0u8; 48];
	pubkey_bytes.copy_from_slice(&bytes);

	BlsPublicKey::deserialize(&pubkey_bytes).map_err(|e| eyre::eyre!("Failed to deserialize BLS public key: {:?}", e))
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_PROPOSER_KEY: &str =
		"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6";
	const TEST_DELEGATE_KEY: &str =
		"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754";
	const TEST_COMMITTER: &str = "0x1111111111111111111111111111111111111111";

	#[test]
	fn test_parse_bls_public_key() {
		let key = parse_bls_public_key(TEST_PROPOSER_KEY).unwrap();
		assert_eq!(key.serialize().len(), 48);

		// Test without 0x prefix
		let key_no_prefix = parse_bls_public_key(TEST_PROPOSER_KEY.strip_prefix("0x").unwrap()).unwrap();
		assert_eq!(key, key_no_prefix);
	}

	#[test]
	fn test_parse_bls_public_key_invalid() {
		// Too short
		assert!(parse_bls_public_key("0xaf6e96").is_err());

		// Invalid hex
		assert!(parse_bls_public_key("0xZZZZ").is_err());
	}

	#[test]
	fn test_create_delegation() {
		let config = ProposerConfig {
			proposer_bls_public_key: TEST_PROPOSER_KEY.to_string(),
			delegate_bls_public_key: TEST_DELEGATE_KEY.to_string(),
			committer_address: TEST_COMMITTER.to_string(),
			relay_url: "http://localhost:3001".to_string(),
			relay_api_key: None,
			beacon_api_url: "https://test.com".to_string(),
			beacon_genesis_timestamp: 1606824023,
			poll_interval_seconds: 60,
			module_signing_id: "0x1111111111111111111111111111111111111111111111111111111111111111".to_string(),
		};

		let delegation = create_delegation(&config, 12345).unwrap();

		assert_eq!(delegation.slot, 12345);
		assert_eq!(delegation.proposer, parse_bls_public_key(TEST_PROPOSER_KEY).unwrap());
		assert_eq!(delegation.delegate, parse_bls_public_key(TEST_DELEGATE_KEY).unwrap());
		assert_eq!(delegation.committer, TEST_COMMITTER.parse::<Address>().unwrap());
	}

	// Note: Full integration tests with mocked constraints client are in the integration test suite
	// Unit tests here focus on pure functions that don't require async mocking
}
