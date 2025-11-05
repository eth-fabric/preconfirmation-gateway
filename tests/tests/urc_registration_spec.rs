use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::{U256, address};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use eyre::Result;

#[tokio::test]
async fn slash_registration_happy_reverts_and_unhappy_succeeds() -> Result<()> {
	// 1) Start Anvil and deploy URC
	let anvil = Anvil::new().spawn();
	let urc_addr = integration_tests::urc_helpers::deploy_urc_to_anvil(&anvil)
		.await
		.map_err(|e| eyre::eyre!("deploy urc failed: {}", e))?;

	// 2) Build provider with anvil key0
	let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
	let wallet = EthereumWallet::from(signer);
	let provider = ProviderBuilder::new().wallet(wallet.clone()).connect_http(anvil.endpoint_url());

	// 3) Create Registry binding
	let registry = urc::registry::Registry::new(urc_addr, provider.clone());

	// 4) Start local signer and get an operator BLS key
	let mut commit_cfg = integration_tests::test_common::start_local_signer_server(
		integration_tests::test_common::MODULE_ID,
		integration_tests::test_common::SIGNING_ID,
		"test-admin-secret",
		20001,
		(),
	)
	.await?;

	let consensus_bls = cb_common::types::BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY)
		.map_err(|e| eyre::eyre!("deserialize bls pubkey failed: {:?}", e))?;

	// Helper to convert SignedRegistration -> binding type
	let to_binding =
		|r: &common::types::urc::SignedRegistration| -> eyre::Result<urc::registry::IRegistry::SignedRegistration> {
			r.as_sol_type()
		};

	// 5a) Happy path: register with owner_x; slashRegistration should revert
	let owner_x = wallet.default_signer().address();
	let signed_reg_x = proposer::urc_registration::sign_registration(
		&mut commit_cfg,
		&consensus_bls,
		owner_x,
		&integration_tests::test_common::SIGNING_ID,
	)
	.await?;
	let regs_x = vec![signed_reg_x];

	let regs_binding_x = regs_x.iter().map(to_binding).collect::<eyre::Result<Vec<_>>>()?;
	registry
		.register(regs_binding_x.clone(), owner_x, integration_tests::test_common::SIGNING_ID)
		.value(U256::from(1_000_000_000_000_000_000u128))
		.send()
		.await?
		.watch()
		.await?;

	let proof_ok = registry
		.getRegistrationProof(
			regs_binding_x.clone(),
			owner_x,
			U256::from(0u64),
			integration_tests::test_common::SIGNING_ID,
		)
		.call()
		.await?;

	// Should revert because signature is valid for owner_x
	assert!(registry.slashRegistration(proof_ok).call().await.is_err());

	// 5b) Unhappy path: register with owner_y != owner_x; slashRegistration should succeed
	let owner_y = address!("0000000000000000000000000000000000001337");
	registry
		.register(regs_binding_x.clone(), owner_y, integration_tests::test_common::SIGNING_ID)
		.value(U256::from(1_000_000_000_000_000_000u128))
		.send()
		.await?
		.watch()
		.await?;

	let proof_bad = registry
		.getRegistrationProof(regs_binding_x, owner_y, U256::from(0u64), integration_tests::test_common::SIGNING_ID)
		.call()
		.await?;

	let slashed = registry.slashRegistration(proof_bad).call().await?;
	assert!(slashed > U256::ZERO);
	println!("slashed: {}", slashed);

	Ok(())
}
