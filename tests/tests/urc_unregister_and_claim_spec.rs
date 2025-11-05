use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use commit_boost::prelude::Chain;
use eyre::Result;

#[tokio::test]
async fn unregister_then_claim_collateral_zero_delay() -> Result<()> {
	let anvil = Anvil::new().spawn();
	let overrides = integration_tests::urc_helpers::URCRegistryOverrides {
		fraud_proof_window: Some(0),
		opt_in_delay: Some(0),
		slash_window: Some(0),
		unregistration_delay: Some(0),
	};
	let urc_addr =
		integration_tests::urc_helpers::deploy_urc_to_anvil_with_chain_and_overrides(&anvil, Chain::Hoodi, overrides)
			.await?;

	let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
	let wallet = EthereumWallet::from(signer);
	let provider = ProviderBuilder::new().wallet(wallet.clone()).connect_http(anvil.endpoint_url());
	let registry = urc::registry::Registry::new(urc_addr, provider.clone());

	// Register
	let mut commit_cfg = integration_tests::test_common::start_local_signer_server(
		integration_tests::test_common::MODULE_ID,
		integration_tests::test_common::SIGNING_ID,
		"test-admin-secret",
		20014,
		(),
	)
	.await?;
	let consensus_bls = cb_common::types::BlsPublicKey::deserialize(&integration_tests::test_common::PUBKEY)
		.map_err(|e| eyre::eyre!("deserialize bls pubkey failed: {e:?}"))?;
	let signed_reg = proposer::sign_registration(
		&mut commit_cfg,
		&consensus_bls,
		wallet.default_signer().address(),
		&integration_tests::test_common::SIGNING_ID,
	)
	.await?;
	let regs_binding = vec![signed_reg.as_sol_type()?];
	let owner = wallet.default_signer().address();
	registry
		.register(regs_binding.clone(), owner, integration_tests::test_common::SIGNING_ID)
		.value(U256::from(1_000_000_000_000_000_000u128))
		.send()
		.await?
		.watch()
		.await?;
	let proof = registry
		.getRegistrationProof(regs_binding.clone(), owner, U256::from(0u64), integration_tests::test_common::SIGNING_ID)
		.call()
		.await?;
	let reg_root = proof.registrationRoot;

	// Unregister and claimCollateral (unregistration_delay=0)
	registry.unregister(reg_root).send().await?.watch().await?;
	registry.claimCollateral(reg_root).send().await?.watch().await?;

	let op = registry.getOperatorData(reg_root).call().await?;
	assert!(op.deleted);
	Ok(())
}
