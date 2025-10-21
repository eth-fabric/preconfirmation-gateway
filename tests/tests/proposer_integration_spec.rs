use common::types::beacon::BeaconTiming;
use constraints::ConstraintsClient;
use eyre::Result;
use integration_tests::test_common::TestHarness;
use proposer::process_lookahead;

#[tokio::test]
async fn test_proposer_end_to_end() -> Result<()> {
	// 1. Setup proposer test harness with relay service
	let harness = TestHarness::builder().with_relay_port(None).build_proposer_harness().await?;

	// 2. Get current slot and calculate epochs
	let slot_timer = harness.slot_timer();
	let current_slot = slot_timer.get_current_slot();
	let current_epoch = BeaconTiming::slot_to_epoch(current_slot);
	let next_epoch = current_epoch + 1;

	// 3. Create mock beacon client that returns duties for both current and next epoch
	use common::beacon::{HttpResponse, MockHttpClient};
	use common::config::BeaconApiConfig;
	use common::types::beacon::{BeaconTiming as BT, ProposerDutiesResponse, ValidatorDuty};

	let mut combined_mock = MockHttpClient::new();
	let proposer_key = harness.proposer_bls_public_key.clone();

	// Setup expectations for both current and next epoch
	combined_mock.expect_get().times(2).returning(move |url| {
		let epoch = if url.contains(&format!("/{}", current_epoch)) { current_epoch } else { next_epoch };

		let start_slot = BT::epoch_to_first_slot(epoch);
		let end_slot = BT::epoch_to_last_slot(epoch);
		let mut duties = Vec::new();

		for slot in start_slot..=end_slot {
			if slot > current_slot {
				// Only include future slots
				let pubkey_hex = format!("0x{}", hex::encode(proposer_key.serialize()));
				duties.push(ValidatorDuty {
					validator_index: slot.to_string(),
					pubkey: pubkey_hex,
					slot: slot.to_string(),
				});
			}
		}

		let response = ProposerDutiesResponse { execution_optimistic: false, finalized: true, data: duties };
		Ok(HttpResponse { status: 200, body: serde_json::to_vec(&response).unwrap() })
	});

	let beacon_config = BeaconApiConfig {
		primary_endpoint: "https://test-beacon.example.com".to_string(),
		fallback_endpoints: vec![],
		request_timeout_secs: 30,
		genesis_time: 1606824023,
	};
	let beacon_client = common::beacon::BeaconApiClient::new(beacon_config, combined_mock)?;

	// 4. Create constraints client pointing to real relay
	let constraints_client = ConstraintsClient::new(harness.relay_service.as_ref().unwrap().url.clone(), None);

	// 5. Process lookahead (should post delegations to relay)
	let commit_config_guard = harness.commit_config.lock().await;
	let mut commit_config = commit_boost::prelude::StartCommitModuleConfig {
		id: commit_config_guard.id.clone(),
		chain: commit_config_guard.chain,
		signer_client: commit_config_guard.signer_client.clone(),
		extra: commit_config_guard.extra.clone(),
	};
	drop(commit_config_guard); // Release the lock

	process_lookahead(&beacon_client, &constraints_client, &mut commit_config, current_slot).await?;

	// 6. Verify delegations were posted to relay
	// Check the next slot (should have a delegation)
	let next_slot = current_slot + 1;
	let delegations = constraints_client.get_delegations_for_slot(next_slot).await?;

	assert!(!delegations.is_empty(), "Expected at least one delegation for slot {}", next_slot);

	// Verify the delegation is from our proposer
	let found = delegations.iter().any(|d| d.message.proposer == harness.proposer_bls_public_key);
	assert!(found, "Delegation from our proposer not found in relay");

	// Verify the delegation has correct fields
	let our_delegation = delegations.iter().find(|d| d.message.proposer == harness.proposer_bls_public_key).unwrap();
	assert_eq!(our_delegation.message.delegate, harness.delegate_bls_public_key);
	assert_eq!(our_delegation.message.committer, harness.committer_address);
	assert_eq!(our_delegation.message.slot, next_slot);

	Ok(())
}

#[tokio::test]
async fn test_proposer_no_duties() -> Result<()> {
	// Test scenario where proposer has no upcoming duties

	// 1. Setup proposer test harness with relay service
	let harness = TestHarness::builder().with_relay_port(None).build_proposer_harness().await?;

	// 2. Get current slot
	let slot_timer = harness.slot_timer();
	let current_slot = slot_timer.get_current_slot();

	// 3. Create mock beacon client that returns EMPTY duties (no proposer duties)
	use common::beacon::{HttpResponse, MockHttpClient};
	use common::config::BeaconApiConfig;
	use common::types::beacon::ProposerDutiesResponse;

	let mut mock_http = MockHttpClient::new();
	mock_http.expect_get().times(2).returning(move |_url| {
		// Return empty duties for both current and next epoch
		let response = ProposerDutiesResponse { execution_optimistic: false, finalized: true, data: vec![] };
		Ok(HttpResponse { status: 200, body: serde_json::to_vec(&response).unwrap() })
	});

	let beacon_config = BeaconApiConfig {
		primary_endpoint: "https://test-beacon.example.com".to_string(),
		fallback_endpoints: vec![],
		request_timeout_secs: 30,
		genesis_time: 1606824023,
	};
	let beacon_client = common::beacon::BeaconApiClient::new(beacon_config, mock_http)?;

	// 4. Create constraints client
	let constraints_client = ConstraintsClient::new(harness.relay_service.as_ref().unwrap().url.clone(), None);

	// 5. Get initial delegation count
	let next_slot = current_slot + 1;
	let initial_delegations = constraints_client.get_delegations_for_slot(next_slot).await?;
	let initial_count = initial_delegations.len();

	// 6. Run process_lookahead (should NOT post any delegations since there are no duties)
	let commit_config_guard = harness.commit_config.lock().await;
	let mut commit_config = commit_boost::prelude::StartCommitModuleConfig {
		id: commit_config_guard.id.clone(),
		chain: commit_config_guard.chain,
		signer_client: commit_config_guard.signer_client.clone(),
		extra: commit_config_guard.extra.clone(),
	};
	drop(commit_config_guard);

	process_lookahead(&beacon_client, &constraints_client, &mut commit_config, current_slot).await?;

	// 7. Verify no new delegations were posted
	let final_delegations = constraints_client.get_delegations_for_slot(next_slot).await?;
	assert_eq!(final_delegations.len(), initial_count, "No new delegations should be posted when there are no duties");

	Ok(())
}

#[tokio::test]
async fn test_constraints_client_post_delegation() -> Result<()> {
	// Test the post_delegation method of ConstraintsClient with real relay
	// This test uses the standard TestHarness since it's testing the relay API directly

	// 1. Setup test harness with relay service
	let harness = TestHarness::builder().with_relay_port(None).build().await?;

	// 2. Create a delegation using harness helpers
	let current_slot = harness.context.slot_timer.get_current_slot();
	let future_slot = current_slot + 10;

	let delegation = harness.create_delegation(future_slot, harness.gateway_bls_one.clone(), harness.committer_one);

	// 3. Sign the delegation (using proposer as signer for this test)
	let signed_delegation =
		harness.create_signed_delegation(&delegation, harness.proposer_bls_public_key.clone()).await?;

	// 4. Create constraints client and post delegation
	let constraints_client = ConstraintsClient::new(harness.relay_service.as_ref().unwrap().url.clone(), None);

	constraints_client.post_delegation(&signed_delegation).await?;

	// 5. Verify delegation was stored in relay
	let delegations = constraints_client.get_delegations_for_slot(future_slot).await?;

	assert!(!delegations.is_empty(), "Expected at least one delegation");

	// Find our delegation
	let found = delegations.iter().any(|d| {
		d.message.proposer == harness.proposer_bls_public_key
			&& d.message.delegate == harness.gateway_bls_one
			&& d.message.slot == future_slot
	});

	assert!(found, "Posted delegation not found in relay");

	Ok(())
}
