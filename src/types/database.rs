use crate::types::SignedCommitment;
use alloy::primitives::B256;
use anyhow::Result;
use rocksdb::DB;
use std::sync::Arc;

/// Database context that provides access to the RocksDB database
#[derive(Clone)]
pub struct DatabaseContext {
	db: Arc<DB>,
}

impl DatabaseContext {
	/// Create a new DatabaseContext with the given database
	pub fn new(db: Arc<DB>) -> Self {
		Self { db }
	}

	/// Get a reference to the database
	pub fn db(&self) -> &Arc<DB> {
		&self.db
	}

	/// Store a key-value pair in the database
	pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
		self.db.put(key, value)?;
		Ok(())
	}

	/// Get a value by key from the database
	pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
		match self.db.get(key)? {
			Some(value) => Ok(Some(value)),
			None => Ok(None),
		}
	}

	/// Delete a key from the database
	pub fn delete(&self, key: &[u8]) -> Result<()> {
		self.db.delete(key)?;
		Ok(())
	}

	/// Perform a health check on the database
	pub async fn db_healthcheck(&self) -> Result<()> {
		// Test by putting and getting a test value
		let test_key = b"test_connection";
		let test_value = b"test_value";

		self.put(test_key, test_value)?;
		let retrieved = self.get(test_key)?;

		if let Some(value) = retrieved {
			if value == test_value {
				self.delete(test_key)?; // Clean up test data
				Ok(())
			} else {
				Err(anyhow::anyhow!("Database test failed: value mismatch"))
			}
		} else {
			Err(anyhow::anyhow!("Database test failed: value not found"))
		}
	}

	/// Store a signed commitment in the database
	pub fn store_commitment(&self, request_hash: &B256, signed_commitment: &SignedCommitment) -> Result<()> {
		let key = format!("commitment:{}", request_hash);
		let value = serde_json::to_vec(signed_commitment)?;
		self.put(key.as_bytes(), &value)
	}

	/// Retrieve a signed commitment from the database
	pub fn get_commitment(&self, request_hash: &B256) -> Result<Option<SignedCommitment>> {
		let key = format!("commitment:{}", request_hash);
		match self.get(key.as_bytes())? {
			Some(value) => {
				let signed_commitment: SignedCommitment = serde_json::from_slice(&value)?;
				Ok(Some(signed_commitment))
			}
			None => Ok(None),
		}
	}

	/// Store fee information for a commitment request
	pub fn store_fee_info(&self, request_hash: &B256, fee_info: &crate::types::FeeInfo) -> Result<()> {
		let key = format!("fee:{}", request_hash);
		let value = serde_json::to_vec(fee_info)?;
		self.put(key.as_bytes(), &value)
	}

	/// Retrieve fee information for a commitment request
	pub fn get_fee_info(&self, request_hash: &B256) -> Result<Option<crate::types::FeeInfo>> {
		let key = format!("fee:{}", request_hash);
		match self.get(key.as_bytes())? {
			Some(value) => {
				let fee_info: crate::types::FeeInfo = serde_json::from_slice(&value)?;
				Ok(Some(fee_info))
			}
			None => Ok(None),
		}
	}

	/// Store slot information
	pub fn store_slot_info(&self, slot: u64, slot_info: &crate::types::SlotInfo) -> Result<()> {
		let key = format!("slot:{}", slot);
		let value = serde_json::to_vec(slot_info)?;
		self.put(key.as_bytes(), &value)
	}

	/// Retrieve slot information
	pub fn get_slot_info(&self, slot: u64) -> Result<Option<crate::types::SlotInfo>> {
		let key = format!("slot:{}", slot);
		match self.get(key.as_bytes())? {
			Some(value) => {
				let slot_info: crate::types::SlotInfo = serde_json::from_slice(&value)?;
				Ok(Some(slot_info))
			}
			None => Ok(None),
		}
	}
}

impl std::fmt::Debug for DatabaseContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("DatabaseContext").field("db", &"<RocksDB Database>").finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::rpc::Offering;
	use crate::types::{Commitment, FeeInfo, SignedCommitment, SlotInfo};
	use alloy::primitives::{Address, B256, Bytes, Signature};
	use rocksdb::{DB, Options};
	use tempfile::TempDir;

	fn create_test_db() -> (TempDir, DatabaseContext) {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");

		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let db_context = DatabaseContext::new(Arc::new(db));

		(temp_dir, db_context)
	}

	#[test]
	fn test_basic_operations() {
		let (_temp_dir, db) = create_test_db();

		// Test put and get
		let key = b"test_key";
		let value = b"test_value";

		db.put(key, value).unwrap();
		let retrieved = db.get(key).unwrap();
		assert_eq!(retrieved, Some(value.to_vec()));

		// Test delete
		db.delete(key).unwrap();
		let retrieved_after_delete = db.get(key).unwrap();
		assert_eq!(retrieved_after_delete, None);
	}

	#[test]
	fn test_commitment_storage() {
		let (_temp_dir, db) = create_test_db();

		let request_hash = B256::from_slice(&[1u8; 32]);
		let commitment = Commitment {
			commitment_type: 1,
			payload: Bytes::from(vec![0x01, 0x02, 0x03]),
			request_hash,
			slasher: Address::from([2u8; 20]),
		};

		let signed_commitment = SignedCommitment {
			commitment,
			nonce: 12345,
			signing_id: B256::from_slice(&[3u8; 32]),
			signature: Signature::from_bytes_and_parity(&[4u8; 64], true),
		};

		// Store commitment
		db.store_commitment(&request_hash, &signed_commitment).unwrap();

		// Retrieve commitment
		let retrieved = db.get_commitment(&request_hash).unwrap();
		assert!(retrieved.is_some());

		let retrieved_commitment = retrieved.unwrap();
		assert_eq!(retrieved_commitment.commitment.commitment_type, 1);
		assert_eq!(retrieved_commitment.commitment.payload, Bytes::from(vec![0x01, 0x02, 0x03]));
		assert_eq!(retrieved_commitment.commitment.request_hash, request_hash);
		assert_eq!(retrieved_commitment.commitment.slasher, Address::from([2u8; 20]));
		assert_eq!(retrieved_commitment.nonce, 12345);
		assert_eq!(retrieved_commitment.signing_id, B256::from_slice(&[3u8; 32]));
	}

	#[test]
	fn test_commitment_not_found() {
		let (_temp_dir, db) = create_test_db();

		let request_hash = B256::from_slice(&[1u8; 32]);
		let retrieved = db.get_commitment(&request_hash).unwrap();
		assert!(retrieved.is_none());
	}

	#[test]
	fn test_fee_info_storage() {
		let (_temp_dir, db) = create_test_db();

		let request_hash = B256::from_slice(&[5u8; 32]);
		let fee_info = FeeInfo { fee_payload: Bytes::from(vec![0x05, 0x06, 0x07]), commitment_type: 2 };

		// Store fee info
		db.store_fee_info(&request_hash, &fee_info).unwrap();

		// Retrieve fee info
		let retrieved = db.get_fee_info(&request_hash).unwrap();
		assert!(retrieved.is_some());

		let retrieved_fee = retrieved.unwrap();
		assert_eq!(retrieved_fee.fee_payload, Bytes::from(vec![0x05, 0x06, 0x07]));
		assert_eq!(retrieved_fee.commitment_type, 2);
	}

	#[test]
	fn test_fee_info_not_found() {
		let (_temp_dir, db) = create_test_db();

		let request_hash = B256::from_slice(&[6u8; 32]);
		let retrieved = db.get_fee_info(&request_hash).unwrap();
		assert!(retrieved.is_none());
	}

	#[test]
	fn test_slot_info_storage() {
		let (_temp_dir, db) = create_test_db();

		let slot = 12345;
		let offering = Offering { chain_id: 1, commitment_types: vec![1, 2, 3] };
		let slot_info = SlotInfo { slot, offerings: vec![offering] };

		// Store slot info
		db.store_slot_info(slot, &slot_info).unwrap();

		// Retrieve slot info
		let retrieved = db.get_slot_info(slot).unwrap();
		assert!(retrieved.is_some());

		let retrieved_slot = retrieved.unwrap();
		assert_eq!(retrieved_slot.slot, 12345);
		assert_eq!(retrieved_slot.offerings.len(), 1);
		assert_eq!(retrieved_slot.offerings[0].chain_id, 1);
		assert_eq!(retrieved_slot.offerings[0].commitment_types, vec![1, 2, 3]);
	}

	#[test]
	fn test_slot_info_not_found() {
		let (_temp_dir, db) = create_test_db();

		let slot = 99999;
		let retrieved = db.get_slot_info(slot).unwrap();
		assert!(retrieved.is_none());
	}

	#[test]
	fn test_multiple_commitments() {
		let (_temp_dir, db) = create_test_db();

		// Store multiple commitments
		for i in 0..5 {
			let request_hash = B256::from_slice(&[i; 32]);
			let commitment = Commitment {
				commitment_type: i as u64,
				payload: Bytes::from(vec![i as u8]),
				request_hash,
				slasher: Address::from([i; 20]),
			};

			let signed_commitment = SignedCommitment {
				commitment,
				nonce: i as u64 * 1000,
				signing_id: B256::from_slice(&[i; 32]),
				signature: Signature::from_bytes_and_parity(&[i; 64], true),
			};

			db.store_commitment(&request_hash, &signed_commitment).unwrap();
		}

		// Retrieve and verify each commitment
		for i in 0..5 {
			let request_hash = B256::from_slice(&[i; 32]);
			let retrieved = db.get_commitment(&request_hash).unwrap();
			assert!(retrieved.is_some());

			let commitment = retrieved.unwrap();
			assert_eq!(commitment.commitment.commitment_type, i as u64);
			assert_eq!(commitment.commitment.payload, Bytes::from(vec![i as u8]));
			assert_eq!(commitment.nonce, i as u64 * 1000);
		}
	}

	#[test]
	fn test_key_prefixes() {
		let (_temp_dir, db) = create_test_db();

		// Test that different key prefixes don't interfere
		let request_hash = B256::from_slice(&[7u8; 32]);

		// Store commitment
		let commitment = Commitment { commitment_type: 1, payload: Bytes::new(), request_hash, slasher: Address::ZERO };
		let signed_commitment = SignedCommitment {
			commitment,
			nonce: 1,
			signing_id: B256::ZERO,
			signature: Signature::from_bytes_and_parity(&[0u8; 64], true),
		};
		db.store_commitment(&request_hash, &signed_commitment).unwrap();

		// Store fee info with same request hash
		let fee_info = FeeInfo { fee_payload: Bytes::from(vec![0x08]), commitment_type: 1 };
		db.store_fee_info(&request_hash, &fee_info).unwrap();

		// Both should be retrievable
		assert!(db.get_commitment(&request_hash).unwrap().is_some());
		assert!(db.get_fee_info(&request_hash).unwrap().is_some());

		// Keys should be different
		let commitment_key = format!("commitment:{}", request_hash);
		let fee_key = format!("fee:{}", request_hash);
		assert_ne!(commitment_key, fee_key);
	}

	#[test]
	fn test_serialization_roundtrip() {
		let (_temp_dir, db) = create_test_db();

		let request_hash = B256::from_slice(&[8u8; 32]);
		let original_commitment = Commitment {
			commitment_type: 42,
			payload: Bytes::from(vec![0xAA, 0xBB, 0xCC]),
			request_hash,
			slasher: Address::from([0xDD; 20]),
		};

		let original_signed = SignedCommitment {
			commitment: original_commitment,
			nonce: 98765,
			signing_id: B256::from_slice(&[0xEE; 32]),
			signature: Signature::from_bytes_and_parity(&[0xFF; 64], false),
		};

		// Store and retrieve
		db.store_commitment(&request_hash, &original_signed).unwrap();
		let retrieved = db.get_commitment(&request_hash).unwrap().unwrap();

		// Verify all fields match exactly
		assert_eq!(retrieved.commitment.commitment_type, original_signed.commitment.commitment_type);
		assert_eq!(retrieved.commitment.payload, original_signed.commitment.payload);
		assert_eq!(retrieved.commitment.request_hash, original_signed.commitment.request_hash);
		assert_eq!(retrieved.commitment.slasher, original_signed.commitment.slasher);
		assert_eq!(retrieved.nonce, original_signed.nonce);
		assert_eq!(retrieved.signing_id, original_signed.signing_id);
		assert_eq!(retrieved.signature, original_signed.signature);
	}

	#[tokio::test]
	async fn test_db_healthcheck() {
		let (_temp_dir, db) = create_test_db();

		// Test the health check method
		db.db_healthcheck().await.unwrap();

		// Verify test data was cleaned up
		let test_key = b"test_connection";
		let retrieved = db.get(test_key).unwrap();
		assert!(retrieved.is_none());
	}
}
