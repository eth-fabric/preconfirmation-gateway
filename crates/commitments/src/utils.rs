use alloy::primitives::{Address, B256, Bytes, Signature, keccak256};
use commit_boost::prelude::StartCommitModuleConfig;
use eyre::{Result, WrapErr};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

use common::constants::{COMMITMENT_TYPE, CONSTRAINT_TYPE};
use common::signer;
use common::types::commitments::{FeePayload, InclusionPayload};
use common::types::{Commitment, CommitmentRequest, Constraint, FeeInfo, SignedCommitment};

/// Helper functions for RPC business logic
/// This module contains utility functions that can be shared across multiple RPC handlers

/// Validates a commitment request and returns the decoded InclusionPayload
pub fn validate_commitment_request(request: &CommitmentRequest) -> Result<InclusionPayload> {
	if request.commitment_type != COMMITMENT_TYPE {
		return Err(eyre::eyre!(
			"Invalid commitment type: expected {}, got {}",
			COMMITMENT_TYPE,
			request.commitment_type
		));
	}

	if request.payload.is_empty() {
		return Err(eyre::eyre!("Payload cannot be empty"));
	}

	if request.slasher == Address::ZERO {
		return Err(eyre::eyre!("Invalid slasher address"));
	}

	// Validate that the payload is a valid InclusionPayload
	match InclusionPayload::abi_decode(&request.payload) {
		Ok(inclusion_payload) => {
			debug!(
				"Decoded InclusionPayload: slot={}, signed_tx_len={}",
				inclusion_payload.slot,
				inclusion_payload.signed_tx.len()
			);

			// Additional validation for InclusionPayload
			if inclusion_payload.slot == 0 {
				return Err(eyre::eyre!("Invalid slot: 0"));
			}

			if inclusion_payload.signed_tx.is_empty() {
				return Err(eyre::eyre!("Signed transaction cannot be empty"));
			}

			// TODO: Should verify:
			// - Transaction is for a valid slot
			// - Enough gas for transaction to be executed successfully

			// Validate signed_tx format and signature
			verify_signed_tx(&inclusion_payload.signed_tx)?;

			debug!("Commitment request validation passed");
			Ok(inclusion_payload)
		}
		Err(e) => {
			return Err(eyre::eyre!("Invalid payload format: {}", e));
		}
	}
}

/// Verifies a signed transaction by decoding it and validating its signature
/// This implementation uses Alloy's consensus types for proper transaction verification
pub fn verify_signed_tx(signed_tx: &Bytes) -> Result<()> {
	debug!("Verifying signed transaction: {:?}", signed_tx);

	// Basic validation: ensure the transaction is not empty
	if signed_tx.is_empty() {
		return Err(eyre::eyre!("Signed transaction cannot be empty"));
	}

	// Try to decode the transaction using Alloy's consensus types
	use alloy::consensus::TxEnvelope;
	use alloy::rlp::Decodable;

	match TxEnvelope::decode(&mut signed_tx.as_ref()) {
		Ok(tx_envelope) => {
			debug!("Successfully decoded transaction envelope");
			debug!("Transaction type: {:?}", tx_envelope.tx_type());

			// Verify the transaction signature
			// We'll implement proper signature verification using Alloy's built-in methods
			debug!("Transaction decoded successfully, type: {:?}", tx_envelope.tx_type());

			// Step 1-4: Extract and verify the signature
			// We'll recover the signer's address from the signature and validate it
			match &tx_envelope {
				TxEnvelope::Legacy(signed_tx) => {
					debug!("Verifying legacy transaction signature");
					// Step 3: Recover the signer's address from the signature
					match signed_tx.recover_signer() {
						Ok(recovered_address) => {
							debug!("Recovered signer address: {:?}", recovered_address);
							debug!("Legacy transaction signature verified");
						}
						Err(e) => {
							return Err(eyre::eyre!("Failed to recover signer from legacy transaction: {}", e));
						}
					}
				}
				TxEnvelope::Eip1559(signed_tx) => {
					debug!("Verifying EIP-1559 transaction signature");
					// Step 3: Recover the signer's address from the signature
					match signed_tx.recover_signer() {
						Ok(recovered_address) => {
							debug!("Recovered signer address: {:?}", recovered_address);
							debug!("EIP-1559 transaction signature verified");
						}
						Err(e) => {
							return Err(eyre::eyre!("Failed to recover signer from EIP-1559 transaction: {}", e));
						}
					}
				}
				TxEnvelope::Eip2930(signed_tx) => {
					debug!("Verifying EIP-2930 transaction signature");
					// Step 3: Recover the signer's address from the signature
					match signed_tx.recover_signer() {
						Ok(recovered_address) => {
							debug!("Recovered signer address: {:?}", recovered_address);
							debug!("EIP-2930 transaction signature verified");
						}
						Err(e) => {
							return Err(eyre::eyre!("Failed to recover signer from EIP-2930 transaction: {}", e));
						}
					}
				}
				TxEnvelope::Eip4844(signed_tx) => {
					debug!("Verifying EIP-4844 transaction signature");
					// Step 3: Recover the signer's address from the signature
					match signed_tx.recover_signer() {
						Ok(recovered_address) => {
							debug!("Recovered signer address: {:?}", recovered_address);
							debug!("EIP-4844 transaction signature verified");
						}
						Err(e) => {
							return Err(eyre::eyre!("Failed to recover signer from EIP-4844 transaction: {}", e));
						}
					}
				}
				TxEnvelope::Eip7702(signed_tx) => {
					debug!("Verifying EIP-7702 transaction signature");
					// Step 3: Recover the signer's address from the signature
					match signed_tx.recover_signer() {
						Ok(recovered_address) => {
							debug!("Recovered signer address: {:?}", recovered_address);
							debug!("EIP-7702 transaction signature verified");
						}
						Err(e) => {
							return Err(eyre::eyre!("Failed to recover signer from EIP-7702 transaction: {}", e));
						}
					}
				}
			}

			debug!("Transaction signature verification passed - signature is valid and signer recovered");
			Ok(())
		}
		Err(e) => {
			debug!("Failed to decode transaction: {}", e);
			Err(eyre::eyre!("Invalid transaction format: {}", e))
		}
	}
}

/// Calculates fee information for a commitment request
pub fn calculate_fee_info(
	request: &CommitmentRequest,
	pricing_database: &common::types::DatabaseContext,
) -> Result<FeeInfo> {
	debug!("Calculating fee for commitment type: {}", request.commitment_type);

	// Read latest price from pricing database
	let price_gwei = get_latest_price_from_database(pricing_database)?;

	let request_hash = request.request_hash()?;

	let fee_payload = FeePayload { request_hash: request_hash, price_gwei: price_gwei };

	debug!("Calculated fee with price {} gwei", price_gwei);
	Ok(FeeInfo { fee_payload: fee_payload.abi_encode()?, commitment_type: request.commitment_type })
}

/// Get the latest price from the pricing database
fn get_latest_price_from_database(pricing_database: &common::types::DatabaseContext) -> Result<u64> {
	match pricing_database.get_latest_price().map_err(|e| eyre::eyre!("Database error: {}", e))? {
		Some(price) => {
			debug!("Retrieved price {} gwei from pricing database", price);
			Ok(price)
		}
		None => Err(eyre::eyre!("No price found in pricing database")),
	}
}

/// Validates a request hash format
pub fn validate_request_hash(hash: &str) -> Result<B256> {
	if hash.len() != 66 || !hash.starts_with("0x") {
		return Err(eyre::eyre!("Invalid hash format. Expected 0x followed by 64 hex characters"));
	}

	hash.parse::<B256>().wrap_err("Failed to parse hash")
}

/// Logs commitment processing information
pub fn log_commitment_processing(commitment_type: u64, slasher: &Address) {
	info!("Processing commitment - Type: {}, Slasher: {}", commitment_type, slasher);
}

/// Helper to convert string to Address with validation
pub fn parse_address(address_str: &str) -> Result<Address> {
	if address_str.len() != 42 || !address_str.starts_with("0x") {
		return Err(eyre::eyre!("Invalid address format"));
	}

	address_str.parse::<Address>().wrap_err("Failed to parse address")
}

/// Helper to create empty bytes of specified length
pub fn create_empty_bytes(length: usize) -> Bytes {
	Bytes::from(vec![0u8; length])
}

/// Validates commitment type against allowed types
pub fn is_valid_commitment_type(commitment_type: u64) -> bool {
	// TODO: Define valid commitment types based on your business logic
	const VALID_TYPES: &[u64] = &[1, 2, 3, 4, 5]; // Example valid types
	VALID_TYPES.contains(&commitment_type)
}

/// Helper to format error messages consistently
pub fn format_error(context: &str, error: &str) -> String {
	format!("{}: {}", context, error)
}

/// Creates a constraint from a commitment request
/// This function creates a constraint with the same payload but using the constraint type
pub fn create_constraint_from_commitment_request(request: &CommitmentRequest, slot: u64) -> Result<Constraint> {
	debug!("Creating constraint from commitment request for slot {}", slot);

	// Create the constraint with the same payload but constraint type
	let constraint = Constraint { constraint_type: CONSTRAINT_TYPE, payload: request.payload.clone() };

	debug!(
		"Created constraint with type {} and payload length {} for slot {}",
		constraint.constraint_type,
		constraint.payload.len(),
		slot
	);
	Ok(constraint)
}

/// Creates a properly signed commitment using ECDSA
pub async fn create_signed_commitment<T>(
	request: &CommitmentRequest,
	commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	committer_address: Address,
) -> Result<SignedCommitment> {
	debug!("Creating signed commitment with proper ECDSA signing");

	// 1. ABI-encode the CommitmentRequest
	let encoded_request = request.abi_encode()?;
	debug!("ABI-encoded CommitmentRequest: {:?}", encoded_request);

	// 2. Keccak256 hash the encoded request
	let request_hash = keccak256(&encoded_request);
	debug!("Keccak256 hash: {:?}", request_hash);

	// 3. Create the commitment
	let commitment = Commitment {
		commitment_type: request.commitment_type,
		payload: request.payload.clone(),
		request_hash: request_hash,
		slasher: request.slasher,
	};

	// 4. Get the object root
	let message_hash = commitment.to_message_hash()?;
	debug!("Object root: {:?}", message_hash);

	// 5. Call the proxy_ecdsa signer
	let response = {
		let mut commit_config = commit_config.lock().await;
		signer::call_proxy_ecdsa_signer(&mut *commit_config, message_hash, committer_address).await?
	};
	debug!("Received response from proxy_ecdsa: {:?}", response);

	// 6. Construct the SignedCommitment
	let signed_commitment = SignedCommitment {
		commitment,
		nonce: response.nonce,
		signing_id: response.module_signing_id,
		signature: response.signature.normalized_s(),
	};

	debug!("Signed commitment created successfully");
	Ok(signed_commitment)
}

/// Validates a signature against a commitment
pub fn verify_commitment_signature(
	commitment: &Commitment,
	signature: &Signature,
	expected_signer: &Address,
) -> Result<bool> {
	debug!("Verifying commitment signature");
	let message_hash = commitment.to_message_hash()?;
	let recovered_address =
		signature.recover_address_from_prehash(&message_hash).wrap_err("Failed to recover address from signature")?;

	Ok(recovered_address == *expected_signer)
}

/// Creates a valid RLP-encoded EIP-1559 transaction with a mock signature
/// This can be reused across tests to generate properly formatted signed transactions
pub fn create_valid_signed_transaction() -> Bytes {
	use alloy::consensus::{Signed, TxEip1559, TxEnvelope};
	use alloy::primitives::{Address, Bytes, Signature, TxKind, U256};
	use alloy::rlp::Encodable;

	let tx = TxEip1559 {
		chain_id: 1,
		nonce: 0,
		gas_limit: 21000,
		max_fee_per_gas: 20_000_000_000u128,
		max_priority_fee_per_gas: 2_000_000_000u128,
		to: TxKind::Call(Address::from([0x01; 20])),
		value: U256::from(1_000_000_000_000_000_000u64),
		input: Bytes::new(),
		access_list: Default::default(),
	};

	// Create a mock signature that will pass the format validation
	let mock_signature = Signature::new(U256::from(1u64), U256::from(2u64), false);
	let signed_tx = Signed::new_unhashed(tx, mock_signature);
	let tx_envelope = TxEnvelope::Eip1559(signed_tx);
	let mut encoded_tx = Vec::new();
	tx_envelope.encode(&mut encoded_tx);
	Bytes::from(encoded_tx)
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::{Address, Bytes};

	#[tokio::test]
	async fn test_validate_commitment_request() -> Result<()> {
		// Test valid request with proper InclusionPayload and properly signed transaction
		let valid_signed_tx = create_valid_signed_transaction();
		let inclusion_payload = InclusionPayload { slot: 100, signed_tx: valid_signed_tx };
		let encoded_payload = inclusion_payload.abi_encode()?;

		let valid_request = CommitmentRequest {
			commitment_type: 1,
			payload: encoded_payload,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		assert!(validate_commitment_request(&valid_request).is_ok());

		// Test invalid commitment type
		let invalid_type_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01]) };
		let invalid_type_encoded = invalid_type_payload.abi_encode()?;
		let invalid_type_request = CommitmentRequest {
			commitment_type: 0,
			payload: invalid_type_encoded,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		assert!(validate_commitment_request(&invalid_type_request).is_err());

		// Test empty payload
		let empty_payload_request = CommitmentRequest {
			commitment_type: 1,
			payload: Bytes::new(),
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		assert!(validate_commitment_request(&empty_payload_request).is_err());

		// Test zero address
		let zero_address_payload = InclusionPayload { slot: 100, signed_tx: Bytes::from(vec![0x01]) };
		let zero_address_encoded = zero_address_payload.abi_encode()?;
		let zero_address_request =
			CommitmentRequest { commitment_type: 1, payload: zero_address_encoded, slasher: Address::ZERO };

		assert!(validate_commitment_request(&zero_address_request).is_err());

		Ok(())
	}

	#[tokio::test]
	async fn test_validate_commitment_request_with_inclusion_payload() -> Result<()> {
		// Create a valid InclusionPayload with properly RLP-encoded EIP-1559 transaction
		let valid_signed_tx = create_valid_signed_transaction();
		let inclusion_payload = InclusionPayload { slot: 100, signed_tx: valid_signed_tx };

		// Encode it
		let encoded_payload = inclusion_payload.abi_encode()?;

		// Create a valid commitment request
		let valid_request = CommitmentRequest {
			commitment_type: 1,
			payload: encoded_payload,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		// Should pass validation
		assert!(validate_commitment_request(&valid_request).is_ok());

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
		assert!(validate_commitment_request(&invalid_slot_request).is_err());

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
		assert!(validate_commitment_request(&empty_tx_request).is_err());

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
		assert!(validate_commitment_request(&invalid_request).is_err());

		println!("Invalid payload format test passed");
		Ok(())
	}

	#[tokio::test]
	async fn test_calculate_fee_info() -> Result<()> {
		use common::types::DatabaseContext;
		use rocksdb::{DB, Options};
		use std::sync::Arc;
		use tempfile::TempDir;

		let request = CommitmentRequest {
			commitment_type: 3,
			payload: Bytes::from(vec![0x07, 0x08, 0x09]),
			slasher: "0x9876543210987654321098765432109876543210".parse()?,
		};

		// Create a test pricing database
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_pricing_db");
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let pricing_database = DatabaseContext::new(Arc::new(db));

		// Store a test price in the database
		pricing_database.store_latest_price(5).map_err(|e| eyre::eyre!("Database error: {}", e))?;

		let fee_info = calculate_fee_info(&request, &pricing_database).unwrap();

		// Test with the price we stored (5 gwei)
		let fee_payload = FeePayload { request_hash: request.request_hash()?, price_gwei: 5 };

		assert_eq!(fee_info.commitment_type, request.commitment_type);
		assert_eq!(fee_info.fee_payload, fee_payload.abi_encode()?);

		println!("Fee info: {:?}", fee_info);
		Ok(())
	}

	#[test]
	fn test_transaction_decoding() -> Result<()> {
		use alloy::consensus::TxEnvelope;
		use alloy::rlp::Decodable;

		// Create a minimal valid transaction (this is a simplified example)
		let tx_bytes = Bytes::from(vec![0x01; 32]); // 32 bytes

		// Try to decode as a transaction envelope
		match TxEnvelope::decode(&mut tx_bytes.as_ref()) {
			Ok(tx_envelope) => {
				println!("Successfully decoded transaction envelope");
				println!("Transaction type: {:?}", tx_envelope.tx_type());
				Ok(())
			}
			Err(e) => {
				println!("Failed to decode transaction: {}", e);
				// This is expected for our test data since it's not a real transaction
				Ok(())
			}
		}
	}

	#[test]
	fn test_verify_signed_tx() -> Result<()> {
		// Test with empty transaction (should fail)
		let empty_tx = Bytes::new();
		assert!(verify_signed_tx(&empty_tx).is_err());

		// Test with invalid transaction data (should fail)
		let invalid_tx = Bytes::from(vec![0x01, 0x02, 0x03]);
		assert!(verify_signed_tx(&invalid_tx).is_err());

		// Test with a properly RLP-encoded transaction (should pass)
		let valid_tx = create_valid_signed_transaction();
		assert!(verify_signed_tx(&valid_tx).is_ok());

		println!("verify_signed_tx tests passed");
		Ok(())
	}

	#[test]
	fn test_validate_commitment_request_with_signed_tx_verification() -> Result<()> {
		// Test that validate_commitment_request now includes signed_tx verification
		// This test ensures that invalid signed transactions are caught

		// Create a valid InclusionPayload with invalid signed_tx (too short)
		let invalid_tx_payload = InclusionPayload {
			slot: 100,
			signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]), // Too short transaction
		};
		let invalid_tx_encoded = invalid_tx_payload.abi_encode()?;
		let invalid_tx_request = CommitmentRequest {
			commitment_type: 1,
			payload: invalid_tx_encoded,
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		// Should fail validation due to invalid signed transaction
		assert!(validate_commitment_request(&invalid_tx_request).is_err());

		println!("Signed transaction verification integration test passed");
		Ok(())
	}
}
