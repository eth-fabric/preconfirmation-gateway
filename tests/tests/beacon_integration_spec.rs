//! Integration tests for beacon node integration with proposer lookahead

use common::slot_timer::SlotTimer;
use common::types::DatabaseContext;
use eyre::Result;
use integration_tests::test_common::TestHarness;
use relay::proposer_lookahead::{ProposerLookaheadConfig, ProposerLookaheadTask};
use rocksdb::{DB, Options};
use std::sync::Arc;
use tempfile::TempDir;

/// Test that ProposerLookaheadTask populates the database using mock beacon client
#[tokio::test]
async fn test_beacon_integration_populates_lookahead() -> Result<()> {
	// Build test harness (no services needed, just keys and database)
	let harness = TestHarness::builder().build().await?;

	// Create a temporary database for this test
	let temp_dir = TempDir::new()?;
	let db_path = temp_dir.path().join("test_beacon_db");
	let mut opts = Options::default();
	opts.create_if_missing(true);
	let db = DB::open(&opts, &db_path)?;
	let database = DatabaseContext::new(Arc::new(db));

	// Create slot timer
	let slot_timer = SlotTimer::new(1606824023);

	// Define epoch for test (epoch 3 = slots 96-127)
	let epoch = 3u64;
	let start_slot = 96u64; // First slot in epoch 3
	let end_slot = 127u64; // Last slot in epoch 3

	// Create a mock beacon client configured with the harness's proposer key for this epoch
	let mock_beacon_client = harness.create_mock_beacon_api_client(epoch)?;

	// Create ProposerLookaheadTask with mock beacon client
	let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

	let task = ProposerLookaheadTask::new(config, slot_timer, database.clone(), mock_beacon_client);

	// Call populate_lookahead without a test key (will fetch from mock beacon)
	task.populate_lookahead(epoch, None).await?;

	// Verify that the proposer keys were stored in the database for all slots in the epoch
	for slot in start_slot..=end_slot {
		let proposer = database.get_proposer_lookahead(slot)?;
		assert!(proposer.is_some(), "Proposer key should be stored for slot {}", slot);
		assert_eq!(
			proposer.unwrap(),
			harness.proposer_bls_public_key,
			"Stored key should match harness proposer key for slot {}",
			slot
		);
	}

	println!(
		"✓ Beacon integration successfully populated lookahead for epoch {} (slots {}-{})",
		epoch, start_slot, end_slot
	);
	Ok(())
}

// Note: End-to-end tests for store_delegation with beacon-populated lookahead
// are covered in relay_integration_spec.rs which properly handles database initialization
// before the relay service starts. The beacon integration itself is thoroughly tested
// in test_beacon_integration_populates_lookahead above.

/// Test beacon fallback when primary fails
#[tokio::test]
async fn test_beacon_fallback_on_primary_failure() -> Result<()> {
	use common::beacon::{BeaconApiClient, HttpResponse, MockHttpClient};
	use common::config::BeaconApiConfig;
	use common::types::beacon::{BeaconTiming, ProposerDutiesResponse, ValidatorDuty};

	// Build test harness
	let harness = TestHarness::builder().build().await?;

	// Create a temporary database
	let temp_dir = TempDir::new()?;
	let db_path = temp_dir.path().join("test_beacon_fallback_db");
	let mut opts = Options::default();
	opts.create_if_missing(true);
	let db = DB::open(&opts, &db_path)?;
	let database = DatabaseContext::new(Arc::new(db));

	let slot_timer = SlotTimer::new(1606824023);
	let epoch = 3u64; // Epoch 3 = slots 96-127
	let test_slot = 100u64; // A slot within epoch 3 to verify

	// Create mock HTTP client that fails on primary, succeeds on fallback
	let mut mock_http = MockHttpClient::new();

	let proposer_key = harness.proposer_bls_public_key.clone();
	let pubkey_bytes = proposer_key.serialize();
	let pubkey_hex = format!("0x{}", hex::encode(pubkey_bytes));

	// Create duties for all slots in the epoch
	let mut duties_data = Vec::new();
	let start_slot = BeaconTiming::epoch_to_first_slot(epoch);
	let end_slot = BeaconTiming::epoch_to_last_slot(epoch);
	for slot in start_slot..=end_slot {
		duties_data.push(ValidatorDuty {
			validator_index: slot.to_string(),
			pubkey: pubkey_hex.clone(),
			slot: slot.to_string(),
		});
	}

	// First call (primary) fails, second call (fallback) succeeds
	let mut call_count = 0;
	mock_http.expect_get().times(2).returning(move |url| {
		call_count += 1;
		if call_count == 1 {
			// Primary endpoint fails
			assert!(url.contains("primary-beacon.test"));
			Err(eyre::eyre!("Primary endpoint failed"))
		} else {
			// Fallback endpoint succeeds
			assert!(url.contains("fallback-beacon.test"));
			assert!(url.contains(&format!("eth/v1/validator/duties/proposer/{}", epoch)));
			let response =
				ProposerDutiesResponse { execution_optimistic: false, finalized: true, data: duties_data.clone() };
			Ok(HttpResponse { status: 200, body: serde_json::to_vec(&response).unwrap() })
		}
	});

	// Create beacon config with primary and fallback
	let beacon_config = BeaconApiConfig {
		primary_endpoint: "https://primary-beacon.test".to_string(),
		fallback_endpoints: vec!["https://fallback-beacon.test".to_string()],
		request_timeout_secs: 30,
		genesis_time: 1606824023,
	};

	let beacon_client = BeaconApiClient::new(beacon_config, mock_http)?;

	// Create ProposerLookaheadTask
	let config = ProposerLookaheadConfig { update_interval_seconds: 60, lookahead_window: 64 };

	let task = ProposerLookaheadTask::new(config, slot_timer, database.clone(), beacon_client);

	// Populate lookahead for epoch 3 - should succeed using fallback
	task.populate_lookahead(epoch, None).await?;

	// Verify lookahead was populated for a slot in the epoch
	let proposer = database.get_proposer_lookahead(test_slot)?;
	assert!(proposer.is_some(), "Lookahead should be populated from fallback");
	assert_eq!(proposer.unwrap(), harness.proposer_bls_public_key);

	println!("✓ Beacon fallback successfully handled primary failure for epoch {}", epoch);
	Ok(())
}
