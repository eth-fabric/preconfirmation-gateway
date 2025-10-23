use alloy::node_bindings::Anvil;
use alloy::primitives::{Address, B256, U256};
use eyre::Result;
use integration_tests::test_common::TestHarnessBuilder;
use integration_tests::urc_helpers::deploy_urc_to_anvil;
use proposer::{send_registration_transaction, sign_registrations};
use tracing::info;

#[tokio::test]
#[ignore] // Requires forge to be installed
async fn test_urc_registration_end_to_end() -> Result<()> {
	info!("Starting end-to-end URC registration test");

	// 1. Start Anvil
	let anvil = Anvil::new().spawn();
	info!("Anvil started at: {}", anvil.endpoint_url());

	// 2. Deploy URC using forge script
	info!("Deploying URC contract...");
	let urc_address = deploy_urc_to_anvil(&anvil).await?;
	info!("URC deployed at: {:?}", urc_address);

	// 3. Set up proposer test harness with signer
	let harness = TestHarnessBuilder::default().build_proposer_harness().await?;

	// 4. Get the owner address (use first Anvil address)
	let owner = anvil.addresses()[0];
	info!("Using owner address: {:?}", owner);

	// 5. Parse module signing ID from config
	let mut commit_config_guard = harness.commit_config.lock().await;
	let module_signing_id: B256 = commit_config_guard.extra.module_signing_id.parse()?;

	// 6. Sign registrations for all BLS keys
	info!("Signing registrations...");
	let inputs = sign_registrations(&mut *commit_config_guard, owner, &module_signing_id).await?;

	info!("Successfully signed {} registrations", inputs.registrations.len());
	assert!(!inputs.registrations.is_empty(), "Should have at least one registration");

	// Verify registration structure
	for (i, reg) in inputs.registrations.iter().enumerate() {
		info!("  Registration {}: pubkey={}, nonce={}", i + 1, reg.pubkey.as_hex_string(), reg.nonce);
		assert!(!reg.signature.is_empty(), "Signature should not be empty");
	}

	// 7. Send registration transaction
	info!("Sending registration transaction...");
	let keystore_path = "./data/keystores/keys/anvil-0";
	let password = "password";
	let collateral = U256::from(1_000_000_000_000_000_000u64); // 1 ETH

	let tx_hash = send_registration_transaction(
		&inputs,
		urc_address,
		&anvil.endpoint_url().to_string(),
		keystore_path,
		password,
		collateral,
	)
	.await?;

	info!("Registration transaction sent: {:?}", tx_hash);

	// 8. Verify transaction was mined (tx_hash should be non-zero)
	assert_ne!(tx_hash, B256::ZERO, "Transaction hash should not be zero");

	info!("✅ End-to-end URC registration test passed!");

	Ok(())
}

#[tokio::test]
#[ignore] // Requires forge to be installed
async fn test_sign_single_registration() -> Result<()> {
	info!("Testing single registration signing");

	// Set up proposer test harness
	let harness = TestHarnessBuilder::default().build_proposer_harness().await?;

	// Create test owner address
	let owner = Address::repeat_byte(0x42);
	let mut commit_config_guard = harness.commit_config.lock().await;
	let module_signing_id: B256 = commit_config_guard.extra.module_signing_id.parse()?;

	// Sign registrations
	let inputs = sign_registrations(&mut *commit_config_guard, owner, &module_signing_id).await?;

	// Verify
	assert!(!inputs.registrations.is_empty(), "Should have registrations");
	assert_eq!(inputs.owner, owner, "Owner should match");
	assert_eq!(inputs.signing_id, module_signing_id, "Signing ID should match");

	// Verify each registration has required fields
	for reg in &inputs.registrations {
		assert!(!reg.pubkey.as_hex_string().is_empty(), "Pubkey should be set");
		assert!(!reg.signature.is_empty(), "Signature should be set");
		assert_ne!(reg.nonce, 0, "Nonce should be non-zero");
	}

	info!("Single registration signing test passed!");

	Ok(())
}
