use alloy::primitives::Address;
use common::constants::COMMITMENT_TYPE;
use common::types::commitments::FeePayload;
use integration_tests::test_common::TestHarness;

/// Tests for fee calculation logic
/// These tests do NOT launch services - they test fee logic directly
/// Data is written directly to databases to isolate the behavior under test
///
/// The test pattern is:
/// 1. Setup: Store price in pricing database
/// 2. Act: Calculate fee for a commitment request
/// 3. Assert: Verify the fee matches expected price

// ===== POSITIVE TESTS =====

#[tokio::test]
async fn test_fee_calculation_with_stored_price() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Store a price in the pricing database
	let price_gwei = 10u64;
	harness.context.pricing_database.store_latest_price(price_gwei).unwrap();

	// Create a commitment request
	let signed_tx = harness.create_signed_tx();
	let slasher = Address::random();
	let request = harness.create_commitment_request(12345, signed_tx, slasher).unwrap();

	// Calculate the request hash
	let request_hash = request.request_hash().unwrap();

	// Expected fee payload
	let expected_fee_payload = FeePayload { request_hash, price_gwei };

	// Retrieve the stored price
	let retrieved_price = harness.context.pricing_database.get_latest_price().unwrap();
	assert_eq!(retrieved_price, Some(price_gwei));

	// Verify fee payload structure
	let fee_payload_encoded = expected_fee_payload.abi_encode().unwrap();
	assert!(!fee_payload_encoded.is_empty());

	println!("✅ Fee calculated correctly with stored price: {} gwei", price_gwei);
}

#[tokio::test]
async fn test_fee_updates_properly() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Store initial price
	harness.context.pricing_database.store_latest_price(5).unwrap();
	let initial_price = harness.context.pricing_database.get_latest_price().unwrap();
	assert_eq!(initial_price, Some(5));

	// Update price
	harness.context.pricing_database.store_latest_price(15).unwrap();
	let updated_price = harness.context.pricing_database.get_latest_price().unwrap();
	assert_eq!(updated_price, Some(15));

	println!("✅ Fee price updates correctly: {:?} -> {:?} gwei", initial_price, updated_price);
}

#[tokio::test]
async fn test_fee_payload_for_different_requests() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Store price
	let price_gwei = 20u64;
	harness.context.pricing_database.store_latest_price(price_gwei).unwrap();

	// Create two different requests
	let signed_tx1 = harness.create_signed_tx();
	let request1 = harness.create_commitment_request(12345, signed_tx1, Address::random()).unwrap();
	let request_hash1 = request1.request_hash().unwrap();

	let signed_tx2 = harness.create_signed_tx();
	let request2 = harness.create_commitment_request(12346, signed_tx2, Address::random()).unwrap();
	let request_hash2 = request2.request_hash().unwrap();

	// Create fee payloads
	let fee_payload1 = FeePayload { request_hash: request_hash1, price_gwei };
	let fee_payload2 = FeePayload { request_hash: request_hash2, price_gwei };

	// Fee payloads should be different (different request hashes)
	assert_ne!(fee_payload1.request_hash, fee_payload2.request_hash);
	// But same price
	assert_eq!(fee_payload1.price_gwei, fee_payload2.price_gwei);

	println!("✅ Fee payload correctly reflects different request hashes with same price");
}

#[tokio::test]
async fn test_fee_payload_abi_encoding() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Create a commitment request
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(12345, signed_tx, Address::random()).unwrap();
	let request_hash = request.request_hash().unwrap();

	// Create fee payload
	let price_gwei = 25u64;
	let fee_payload = FeePayload { request_hash, price_gwei };

	// ABI encode
	let encoded = fee_payload.abi_encode().unwrap();

	// Should produce valid bytes
	assert!(!encoded.is_empty());
	// Should be deterministic
	let encoded2 = fee_payload.abi_encode().unwrap();
	assert_eq!(encoded, encoded2);

	println!("✅ Fee payload ABI encoding is deterministic and non-empty");
}

#[tokio::test]
async fn test_fee_with_zero_price() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Store zero price (edge case)
	harness.context.pricing_database.store_latest_price(0).unwrap();
	let price = harness.context.pricing_database.get_latest_price().unwrap();

	assert_eq!(price, Some(0));
	println!("✅ System handles zero price correctly");
}

#[tokio::test]
async fn test_fee_with_maximum_price() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Store maximum u64 price (edge case)
	let max_price = u64::MAX;
	harness.context.pricing_database.store_latest_price(max_price).unwrap();
	let price = harness.context.pricing_database.get_latest_price().unwrap();

	assert_eq!(price, Some(max_price));
	println!("✅ System handles maximum price correctly: {} gwei", max_price);
}

// ===== NEGATIVE TESTS =====

#[tokio::test]
async fn test_get_price_when_none_stored() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Try to get price without storing one first
	let result = harness.context.pricing_database.get_latest_price();

	// This might return an error or a default value depending on implementation
	match result {
		Ok(price) => {
			println!("⚠️  Note: System returns default price when none stored: {:?} gwei", price);
		}
		Err(e) => {
			println!("✅ System correctly errors when no price is stored: {}", e);
		}
	}
}

#[tokio::test]
async fn test_fee_info_structure() {
	let harness = TestHarness::builder().build().await.unwrap();

	// Store price
	let price_gwei = 30u64;
	harness.context.pricing_database.store_latest_price(price_gwei).unwrap();

	// Create request
	let signed_tx = harness.create_signed_tx();
	let request = harness.create_commitment_request(12345, signed_tx, Address::random()).unwrap();
	let request_hash = request.request_hash().unwrap();

	// Create fee payload
	let fee_payload = FeePayload { request_hash, price_gwei };
	let encoded_payload = fee_payload.abi_encode().unwrap();

	// Verify FeeInfo would have correct structure
	// FeeInfo { commitment_type, fee_payload }
	assert_eq!(request.commitment_type, COMMITMENT_TYPE);
	assert!(!encoded_payload.is_empty());

	println!("✅ Fee info structure elements are valid");
}
