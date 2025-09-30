use alloy::primitives::{Address, B256, Bytes, Signature, keccak256};
use eyre::Result;
use preconfirmation_gateway::rpc::utils;
use preconfirmation_gateway::types::rpc::InclusionPayload;
use preconfirmation_gateway::types::{Commitment, CommitmentRequest, SignedCommitment};

#[tokio::test]
async fn test_abi_encode_commitment_request() -> Result<()> {
	let commitment_request = CommitmentRequest { commitment_type: 1, payload: Bytes::new(), slasher: Address::ZERO };

	let encoded = commitment_request.abi_encode()?;

	// Verify the encoded data is not empty
	assert!(!encoded.is_empty());

	// Verify the structure contains our data
	let encoded_bytes = encoded.as_ref();
	assert!(encoded_bytes.len() > 0);

	assert_eq!(keccak256(&encoded).to_string(), "0xf61a6130b6ebfffcb3738e03fe820e4b883b623ec3ab7657ffbf385b2e94edba");
	Ok(())
}

#[tokio::test]
async fn test_validate_commitment_request() -> Result<()> {
	// Test valid request with proper InclusionPayload
	let inclusion_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01, 0x02]) };
	let encoded_payload = inclusion_payload.abi_encode()?;

	let valid_request = CommitmentRequest {
		commitment_type: 1,
		payload: encoded_payload,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	assert!(utils::validate_commitment_request(&valid_request).is_ok());

	// Test invalid commitment type
	let invalid_type_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01]) };
	let invalid_type_encoded = invalid_type_payload.abi_encode()?;
	let invalid_type_request = CommitmentRequest {
		commitment_type: 0,
		payload: invalid_type_encoded,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	assert!(utils::validate_commitment_request(&invalid_type_request).is_err());

	// Test empty payload
	let empty_payload_request = CommitmentRequest {
		commitment_type: 1,
		payload: Bytes::new(),
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	assert!(utils::validate_commitment_request(&empty_payload_request).is_err());

	// Test zero address
	let zero_address_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01]) };
	let zero_address_encoded = zero_address_payload.abi_encode()?;
	let zero_address_request =
		CommitmentRequest { commitment_type: 1, payload: zero_address_encoded, slasher: Address::ZERO };

	assert!(utils::validate_commitment_request(&zero_address_request).is_err());

	Ok(())
}

#[tokio::test]
async fn test_abi_encode_commitment() -> Result<()> {
	let commitment =
		Commitment { commitment_type: 1, payload: Bytes::new(), request_hash: B256::ZERO, slasher: Address::ZERO };

	let encoded = commitment.abi_encode()?;

	// Verify the encoded data is not empty
	assert!(!encoded.is_empty());

	// Verify the structure contains our data
	let encoded_bytes = encoded.as_ref();
	assert!(encoded_bytes.len() > 0);

	assert_eq!(keccak256(&encoded).to_string(), "0x53c39bc1f3870634c4f9a8b63b2e4e3a914acc9da71f4d0093df91eabd9247a9");
	Ok(())
}

#[tokio::test]
async fn test_create_mock_signed_commitment() -> Result<()> {
	let request = CommitmentRequest {
		commitment_type: 1,
		payload: Bytes::from(vec![0x01, 0x02, 0x03]),
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	let signed_commitment = utils::create_mock_signed_commitment(&request);

	// Verify the commitment matches the request
	assert_eq!(signed_commitment.commitment.commitment_type, request.commitment_type);
	assert_eq!(signed_commitment.commitment.payload, request.payload);
	assert_eq!(signed_commitment.commitment.slasher, request.slasher);

	// Verify signature is not empty
	assert!(!signed_commitment.signature.to_string().is_empty());

	println!("Mock signed commitment: {:?}", signed_commitment);
	Ok(())
}

#[tokio::test]
async fn test_sign_commitment_mock() -> Result<()> {
	let commitment = Commitment {
		commitment_type: 1,
		payload: Bytes::from(vec![0x01, 0x02]),
		request_hash: B256::ZERO,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	let signature = utils::sign_commitment(&commitment, "mock_private_key").await?;

	// Verify signature is valid
	assert!(!signature.to_string().is_empty());

	println!("Mock signature: {:?}", signature);
	Ok(())
}

#[tokio::test]
async fn test_create_signed_commitment() -> Result<()> {
	let request = CommitmentRequest {
		commitment_type: 2,
		payload: Bytes::from(vec![0x04, 0x05, 0x06]),
		slasher: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".parse()?,
	};

	let signed_commitment = utils::create_signed_commitment(&request, "mock_key").await?;

	// Verify the structure
	assert_eq!(signed_commitment.commitment.commitment_type, 2);
	assert_eq!(signed_commitment.commitment.payload, request.payload);
	assert_eq!(signed_commitment.commitment.slasher, request.slasher);

	// Verify signature exists
	assert!(!signed_commitment.signature.to_string().is_empty());

	println!("Signed commitment: {:?}", signed_commitment);
	Ok(())
}

#[test]
fn test_validate_request_hash() -> Result<()> {
	// Test valid hash
	let valid_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
	let parsed_hash = utils::validate_request_hash(valid_hash)?;
	assert_eq!(parsed_hash.to_string(), valid_hash);

	// Test invalid hash (too short)
	let invalid_hash = "0x1234";
	assert!(utils::validate_request_hash(invalid_hash).is_err());

	// Test invalid hash (no 0x prefix)
	let no_prefix_hash = "1234567890123456789012345678901234567890123456789012345678901234";
	assert!(utils::validate_request_hash(no_prefix_hash).is_err());

	Ok(())
}

#[test]
fn test_parse_address() -> Result<()> {
	// Test valid address
	let valid_address = "0x1234567890123456789012345678901234567890";
	let parsed_address = utils::parse_address(valid_address)?;
	assert_eq!(parsed_address.to_string(), valid_address);

	// Test invalid address (too short)
	let invalid_address = "0x1234";
	assert!(utils::parse_address(invalid_address).is_err());

	// Test invalid address (no 0x prefix)
	let no_prefix_address = "1234567890123456789012345678901234567890";
	assert!(utils::parse_address(no_prefix_address).is_err());

	Ok(())
}

#[test]
fn test_is_valid_commitment_type() {
	// Test valid types
	assert!(utils::is_valid_commitment_type(1));
	assert!(utils::is_valid_commitment_type(2));
	assert!(utils::is_valid_commitment_type(3));
	assert!(utils::is_valid_commitment_type(4));
	assert!(utils::is_valid_commitment_type(5));

	// Test invalid types
	assert!(!utils::is_valid_commitment_type(0));
	assert!(!utils::is_valid_commitment_type(6));
	assert!(!utils::is_valid_commitment_type(100));
}

#[test]
fn test_create_empty_bytes() {
	let empty_bytes = utils::create_empty_bytes(32);
	assert_eq!(empty_bytes.len(), 32);
	assert!(empty_bytes.iter().all(|&b| b == 0));

	let empty_bytes_100 = utils::create_empty_bytes(100);
	assert_eq!(empty_bytes_100.len(), 100);
	assert!(empty_bytes_100.iter().all(|&b| b == 0));
}

#[test]
fn test_format_error() {
	let error_msg = utils::format_error("TestContext", "TestError");
	assert_eq!(error_msg, "TestContext: TestError");
}

#[tokio::test]
async fn test_calculate_fee_info() -> Result<()> {
	let request = CommitmentRequest {
		commitment_type: 3,
		payload: Bytes::from(vec![0x07, 0x08, 0x09]),
		slasher: "0x9876543210987654321098765432109876543210".parse()?,
	};

	let fee_info = utils::calculate_fee_info(&request);

	assert_eq!(fee_info.commitment_type, request.commitment_type);
	assert_eq!(fee_info.fee_payload, Bytes::new());

	println!("Fee info: {:?}", fee_info);
	Ok(())
}

#[test]
fn test_verify_commitment_signature_mock() -> Result<()> {
	let commitment = Commitment {
		commitment_type: 1,
		payload: Bytes::from(vec![0x01, 0x02]),
		request_hash: B256::ZERO,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	let signature = "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        .parse()?;

	let expected_signer = "0x1234567890123456789012345678901234567890".parse()?;

	// Note: This will fail with mock signature, but tests the function structure
	match utils::verify_commitment_signature(&commitment, &signature, &expected_signer) {
		Ok(is_valid) => {
			println!("Signature verification result: {}", is_valid);
			// With mock signature, this should be false
			assert!(!is_valid);
		}
		Err(e) => {
			println!("Signature verification failed as expected: {}", e);
			// This is expected with mock signature
		}
	}
	Ok(())
}

#[test]
fn test_abi_encode_inclusion_payload() -> Result<()> {
	let payload = InclusionPayload { slot: 12345, signed_tx: Bytes::from(vec![0x01, 0x02, 0x03, 0x04]) };

	let encoded = payload.abi_encode()?;

	// Verify the encoded data is not empty
	assert!(!encoded.is_empty());

	println!("Encoded InclusionPayload: {:?}", encoded);
	Ok(())
}

#[test]
fn test_abi_decode_inclusion_payload() -> Result<()> {
	let original_payload = InclusionPayload { slot: 67890, signed_tx: Bytes::from(vec![0x05, 0x06, 0x07, 0x08, 0x09]) };

	// Encode first
	let encoded = original_payload.abi_encode()?;

	// Then decode
	let decoded = InclusionPayload::abi_decode(&encoded)?;

	// Verify round-trip
	assert_eq!(decoded.slot, original_payload.slot);
	assert_eq!(decoded.signed_tx, original_payload.signed_tx);

	println!("Round-trip test passed: slot={}, signed_tx_len={}", decoded.slot, decoded.signed_tx.len());
	Ok(())
}

#[test]
fn test_validate_commitment_request_with_inclusion_payload() -> Result<()> {
	// Create a valid InclusionPayload
	let inclusion_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]) };

	// Encode it
	let encoded_payload = inclusion_payload.abi_encode()?;

	// Create a valid commitment request
	let valid_request = CommitmentRequest {
		commitment_type: 1,
		payload: encoded_payload,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	// Should pass validation
	assert!(utils::validate_commitment_request(&valid_request).is_ok());

	// Test invalid slot (0)
	let invalid_slot_payload = InclusionPayload {
		slot: 0, // Invalid
		signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]),
	};
	let invalid_slot_encoded = invalid_slot_payload.abi_encode()?;
	let invalid_slot_request = CommitmentRequest {
		commitment_type: 1,
		payload: invalid_slot_encoded,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};
	assert!(utils::validate_commitment_request(&invalid_slot_request).is_err());

	// Test empty signed_tx
	let empty_tx_payload = InclusionPayload {
		slot: 100,
		signed_tx: Bytes::new(), // Invalid
	};
	let empty_tx_encoded = empty_tx_payload.abi_encode()?;
	let empty_tx_request = CommitmentRequest {
		commitment_type: 1,
		payload: empty_tx_encoded,
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};
	assert!(utils::validate_commitment_request(&empty_tx_request).is_err());

	println!("InclusionPayload validation tests passed");
	Ok(())
}

#[test]
fn test_validate_commitment_request_invalid_payload_format() -> Result<()> {
	// Create a request with invalid payload (not ABI-encoded InclusionPayload)
	let invalid_request = CommitmentRequest {
		commitment_type: 1,
		payload: Bytes::from(vec![0x01, 0x02, 0x03]), // Not ABI-encoded
		slasher: "0x1234567890123456789012345678901234567890".parse()?,
	};

	// Should fail validation due to invalid payload format
	assert!(utils::validate_commitment_request(&invalid_request).is_err());

	println!("Invalid payload format test passed");
	Ok(())
}
