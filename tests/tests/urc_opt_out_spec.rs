use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::{U256, address};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use commit_boost::prelude::Chain;
use eyre::Result;

#[tokio::test]
async fn opt_out_of_slasher_happy_path() -> Result<()> {
	// Deploy URC with zero delays for faster testing
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

	// Owner wallet/provider (anvil key0)
	let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
	let wallet = EthereumWallet::from(signer);
	let provider = ProviderBuilder::new().wallet(wallet.clone()).connect_http(anvil.endpoint_url());
	let registry = urc::registry::Registry::new(urc_addr, provider.clone());

	// Spin local signer and sign a registration
	let mut commit_cfg = integration_tests::test_common::start_local_signer_server(
		integration_tests::test_common::MODULE_ID,
		integration_tests::test_common::SIGNING_ID,
		"test-admin-secret",
		20012,
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

	// Register with 1 ETH
	let owner = wallet.default_signer().address();
	registry
		.register(regs_binding.clone(), owner, integration_tests::test_common::SIGNING_ID)
		.value(U256::from(1_000_000_000_000_000_000u128))
		.send()
		.await?
		.watch()
		.await?;

	// Compute registration root
	let proof = registry
		.getRegistrationProof(regs_binding.clone(), owner, U256::from(0u64), integration_tests::test_common::SIGNING_ID)
		.call()
		.await?;
	let reg_root = proof.registrationRoot;

	// Opt-in then opt-out
	let slasher = address!("00000000000000000000000000000000000000AA");
	let committer = address!("00000000000000000000000000000000000000BB");
	registry.optInToSlasher(reg_root, slasher, committer).send().await?.watch().await?;
	registry.optOutOfSlasher(reg_root, slasher).send().await?.watch().await?;

	// Verify commitment state
	let sc = registry.getSlasherCommitment(reg_root, slasher).call().await?;
	assert!(sc.optedOutAt >= sc.optedInAt);
	Ok(())
}
