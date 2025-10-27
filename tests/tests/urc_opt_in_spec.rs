use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::{U256, address};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use commit_boost::prelude::Chain;
use eyre::Result;

#[tokio::test]
async fn opt_in_to_slasher_succeeds_with_zero_fraud_window() -> Result<()> {
	// 1) Start Anvil and deploy URC with fraud_proof_window = 0
	let anvil = Anvil::new().spawn();
	let overrides = integration_tests::urc_helpers::URCRegistryOverrides {
		fraud_proof_window: Some(0),
		opt_in_delay: Some(0),
		slash_window: Some(0),
		unregistration_delay: Some(0),
	};
	let urc_addr =
		integration_tests::urc_helpers::deploy_urc_to_anvil_with_chain_and_overrides(&anvil, Chain::Hoodi, overrides)
			.await
			.map_err(|e| eyre::eyre!("deploy urc failed: {}", e))?;

	// 2) Build provider with anvil key0 (this will be the owner)
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
		20002,
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

	// 5) Register one key for owner
	let owner = wallet.default_signer().address();
	let signed_reg = proposer::urc_registration::sign_registration(
		&mut commit_cfg,
		&consensus_bls,
		owner,
		&integration_tests::test_common::SIGNING_ID,
	)
	.await?;
	let regs_binding = vec![to_binding(&signed_reg)?];

	registry
		.register(regs_binding.clone(), owner, integration_tests::test_common::SIGNING_ID)
		.value(U256::from(1_000_000_000_000_000_000u128))
		.send()
		.await?
		.watch()
		.await?;

	// 6) Derive registration root using getRegistrationProof
	let proof = registry
		.getRegistrationProof(regs_binding.clone(), owner, U256::from(0u64), integration_tests::test_common::SIGNING_ID)
		.call()
		.await?;
	let reg_root = proof.registrationRoot;

	// 7) Pre-flight checks similar to CLI (sanity)
	let cfg = registry.getConfig().call().await?;
	assert_eq!(cfg.fraudProofWindow, 0);
	let op_data = registry.getOperatorData(reg_root).call().await?;
	assert_eq!(op_data.owner, owner);
	let already_opted = registry.isOptedIntoSlasher(reg_root, owner).call().await?; // use owner as slasher for test
	assert!(!already_opted);
	let slashed_any = registry.isSlashed_0(reg_root).call().await?;
	assert!(!slashed_any);

	// 8) Opt-in to a slasher with a committer
	let slasher = address!("00000000000000000000000000000000000000AA");
	let committer = address!("00000000000000000000000000000000000000BB");
	registry.optInToSlasher(reg_root, slasher, committer).send().await?.watch().await?;

	// 9) Verify opt-in state
	let is_opted = registry.isOptedIntoSlasher(reg_root, slasher).call().await?;
	assert!(is_opted);

	// Double opt-in should revert
	assert!(registry.optInToSlasher(reg_root, slasher, committer).call().await.is_err());

	Ok(())
}
