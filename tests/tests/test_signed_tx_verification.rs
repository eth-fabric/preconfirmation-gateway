use alloy::primitives::Bytes;
use common::types::InclusionPayload;
use eyre::Result;
use integration_tests::test_common::TestHarness;

#[tokio::test]
async fn test_create_signed_tx_passes_verification() -> Result<()> {
	// Setup harness
	let harness = TestHarness::builder().build().await?;

	// Create a signed transaction using the helper
	let signed_tx_bytes = harness.create_signed_tx();
	let signed_tx = Bytes::from(signed_tx_bytes);

	let inclusion_payload = InclusionPayload { slot: 12345, signed_tx: signed_tx };

	// Verify it passes the signature verification
	let result = inclusion_payload.verify_signature();
	assert!(result.is_ok(), "Signed transaction should pass verification: {:?}", result);

	println!("Signed transaction passed verification");

	Ok(())
}

#[tokio::test]
async fn test_create_commitment_request_with_valid_tx() -> Result<()> {
	// Setup harness
	let harness = TestHarness::builder().build().await?;

	// Create a signed transaction
	let signed_tx = harness.create_signed_tx();

	// Create a commitment request
	let request = harness.create_commitment_request(12345, signed_tx, alloy::primitives::Address::random())?;

	// Validate the request (this internally calls verify_signed_tx)
	let result = commitments::utils::validate_commitment_request(&request);
	assert!(result.is_ok(), "Commitment request should be valid: {:?}", result);

	println!("CommitmentRequest with signed transaction is valid");

	Ok(())
}
