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

	/// Store a constraint in the database
	pub fn store_constraint(&self, constraint_id: &B256, constraint: &crate::types::Constraint) -> Result<()> {
		let key = format!("constraint:{}", constraint_id);
		let value = serde_json::to_vec(constraint)?;
		self.put(key.as_bytes(), &value)
	}

	/// Retrieve a constraint from the database
	pub fn get_constraint(&self, constraint_id: &B256) -> Result<Option<crate::types::Constraint>> {
		let key = format!("constraint:{}", constraint_id);
		match self.get(key.as_bytes())? {
			Some(value) => {
				let constraint: crate::types::Constraint = serde_json::from_slice(&value)?;
				Ok(Some(constraint))
			}
			None => Ok(None),
		}
	}

	/// Get all pending constraints (not yet sent to relay)
	pub fn get_pending_constraints(&self) -> Result<Vec<(B256, crate::types::Constraint)>> {
		let mut pending_constraints = Vec::new();

		// Scan for constraint:* keys
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);

		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8_lossy(&key);

			// Check if this is a constraint key
			if key_str.starts_with("constraint:") {
				let constraint_id_str = &key_str[11..]; // Remove "constraint:" prefix

				// Parse the constraint ID
				if let Ok(constraint_id) = constraint_id_str.parse::<B256>() {
					// Check if this constraint has been sent
					let status_key = format!("constraint_status:{}", constraint_id);
					let status = self.get(status_key.as_bytes())?;

					// If no status or status is not "sent", it's pending
					if status.is_none() || status != Some(b"sent".to_vec()) {
						// Deserialize the constraint
						if let Ok(constraint) = serde_json::from_slice::<crate::types::Constraint>(&value) {
							pending_constraints.push((constraint_id, constraint));
						}
					}
				}
			}
		}

		Ok(pending_constraints)
	}

	/// Mark constraint as sent to relay
	pub fn mark_constraint_sent(&self, constraint_id: &B256) -> Result<()> {
		let key = format!("constraint_status:{}", constraint_id);
		self.put(key.as_bytes(), b"sent")
	}

	/// Get constraint status
	pub fn get_constraint_status(&self, constraint_id: &B256) -> Result<Option<String>> {
		let key = format!("constraint_status:{}", constraint_id);
		match self.get(key.as_bytes())? {
			Some(value) => Ok(Some(String::from_utf8_lossy(&value).to_string())),
			None => Ok(None),
		}
	}

	/// Get all constraints (pending and sent)
	pub fn get_all_constraints(&self) -> Result<Vec<(B256, crate::types::Constraint)>> {
		let mut all_constraints = Vec::new();

		// Scan for constraint:* keys
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);

		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8_lossy(&key);

			// Check if this is a constraint key
			if key_str.starts_with("constraint:") {
				let constraint_id_str = &key_str[11..]; // Remove "constraint:" prefix

				// Parse the constraint ID
				if let Ok(constraint_id) = constraint_id_str.parse::<B256>() {
					// Deserialize the constraint
					if let Ok(constraint) = serde_json::from_slice::<crate::types::Constraint>(&value) {
						all_constraints.push((constraint_id, constraint));
					}
				}
			}
		}

		Ok(all_constraints)
	}

	/// Delete constraint and its status
	pub fn delete_constraint(&self, constraint_id: &B256) -> Result<()> {
		let constraint_key = format!("constraint:{}", constraint_id);
		let status_key = format!("constraint_status:{}", constraint_id);

		self.delete(constraint_key.as_bytes())?;
		self.delete(status_key.as_bytes())?;

		Ok(())
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
	use crate::types::commitments::Offering;
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

	/// Test database operations for constraints
	#[tokio::test]
	async fn test_constraint_database_operations() {
		let (_temp_dir, database) = create_test_db();

		// Create test constraint
		let constraint_id = B256::random();
		let constraint = crate::types::Constraint { constraint_type: 1, payload: Bytes::from(vec![1, 2, 3, 4]) };

		// Test storing constraint
		database.store_constraint(&constraint_id, &constraint).unwrap();

		// Test retrieving constraint
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_some());
		assert_eq!(retrieved.unwrap().constraint_type, constraint.constraint_type);

		// Test getting pending constraints
		let pending = database.get_pending_constraints().unwrap();
		assert_eq!(pending.len(), 1);
		assert_eq!(pending[0].0, constraint_id);

		// Test marking constraint as sent
		database.mark_constraint_sent(&constraint_id).unwrap();

		// Test that constraint is no longer pending
		let pending_after = database.get_pending_constraints().unwrap();
		assert_eq!(pending_after.len(), 0);

		// Test constraint status
		let status = database.get_constraint_status(&constraint_id).unwrap();
		assert_eq!(status, Some("sent".to_string()));

		// Test deleting constraint
		database.delete_constraint(&constraint_id).unwrap();
		let deleted = database.get_constraint(&constraint_id).unwrap();
		assert!(deleted.is_none());
	}

	/// Test constraint status edge cases
	#[tokio::test]
	async fn test_constraint_status_edge_cases() {
		let (_temp_dir, database) = create_test_db();

		let constraint_id = B256::random();
		let constraint = crate::types::Constraint { constraint_type: 2, payload: Bytes::from(vec![5, 6, 7, 8]) };

		// Test getting status for non-existent constraint
		let status = database.get_constraint_status(&constraint_id).unwrap();
		assert_eq!(status, None);

		// Store constraint but don't mark as sent
		database.store_constraint(&constraint_id, &constraint).unwrap();
		let status = database.get_constraint_status(&constraint_id).unwrap();
		assert_eq!(status, None);

		// Mark as sent and verify status
		database.mark_constraint_sent(&constraint_id).unwrap();
		let status = database.get_constraint_status(&constraint_id).unwrap();
		assert_eq!(status, Some("sent".to_string()));
	}

	/// Test multiple constraints with different statuses
	#[tokio::test]
	async fn test_multiple_constraints_with_statuses() {
		let (_temp_dir, database) = create_test_db();

		// Create multiple constraints
		let constraint1_id = B256::random();
		let constraint1 = crate::types::Constraint { constraint_type: 1, payload: Bytes::from(vec![1, 2, 3]) };

		let constraint2_id = B256::random();
		let constraint2 = crate::types::Constraint { constraint_type: 2, payload: Bytes::from(vec![4, 5, 6]) };

		let constraint3_id = B256::random();
		let constraint3 = crate::types::Constraint { constraint_type: 3, payload: Bytes::from(vec![7, 8, 9]) };

		// Store all constraints
		database.store_constraint(&constraint1_id, &constraint1).unwrap();
		database.store_constraint(&constraint2_id, &constraint2).unwrap();
		database.store_constraint(&constraint3_id, &constraint3).unwrap();

		// All should be pending initially
		let pending = database.get_pending_constraints().unwrap();
		assert_eq!(pending.len(), 3);

		// Mark constraint2 as sent
		database.mark_constraint_sent(&constraint2_id).unwrap();

		// Now only 2 should be pending
		let pending = database.get_pending_constraints().unwrap();
		assert_eq!(pending.len(), 2);
		assert!(pending.iter().any(|(id, _)| *id == constraint1_id));
		assert!(pending.iter().any(|(id, _)| *id == constraint3_id));
		assert!(!pending.iter().any(|(id, _)| *id == constraint2_id));

		// Test get_all_constraints returns all 3
		let all_constraints = database.get_all_constraints().unwrap();
		assert_eq!(all_constraints.len(), 3);
	}

	/// Test constraint serialization and deserialization
	#[tokio::test]
	async fn test_constraint_serialization_roundtrip() {
		let (_temp_dir, database) = create_test_db();

		let constraint_id = B256::random();
		let original_constraint = crate::types::Constraint {
			constraint_type: 42,
			payload: Bytes::from(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]),
		};

		// Store constraint
		database.store_constraint(&constraint_id, &original_constraint).unwrap();

		// Retrieve and verify
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_some());
		let retrieved_constraint = retrieved.unwrap();
		assert_eq!(retrieved_constraint.constraint_type, original_constraint.constraint_type);
		assert_eq!(retrieved_constraint.payload, original_constraint.payload);
	}

	/// Test constraint deletion removes both constraint and status
	#[tokio::test]
	async fn test_constraint_deletion_complete() {
		let (_temp_dir, database) = create_test_db();

		let constraint_id = B256::random();
		let constraint = crate::types::Constraint { constraint_type: 1, payload: Bytes::from(vec![1, 2, 3, 4]) };

		// Store constraint and mark as sent
		database.store_constraint(&constraint_id, &constraint).unwrap();
		database.mark_constraint_sent(&constraint_id).unwrap();

		// Verify both constraint and status exist
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_some());
		let status = database.get_constraint_status(&constraint_id).unwrap();
		assert_eq!(status, Some("sent".to_string()));

		// Delete constraint
		database.delete_constraint(&constraint_id).unwrap();

		// Verify both constraint and status are gone
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_none());
		let status = database.get_constraint_status(&constraint_id).unwrap();
		assert_eq!(status, None);
	}

	/// Test constraint with empty payload
	#[tokio::test]
	async fn test_constraint_empty_payload() {
		let (_temp_dir, database) = create_test_db();

		let constraint_id = B256::random();
		let constraint = crate::types::Constraint { constraint_type: 0, payload: Bytes::new() };

		database.store_constraint(&constraint_id, &constraint).unwrap();
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_some());
		let retrieved_constraint = retrieved.unwrap();
		assert_eq!(retrieved_constraint.constraint_type, 0);
		assert_eq!(retrieved_constraint.payload, Bytes::new());
	}

	/// Test constraint with large payload
	#[tokio::test]
	async fn test_constraint_large_payload() {
		let (_temp_dir, database) = create_test_db();

		let constraint_id = B256::random();
		let large_payload = Bytes::from(vec![0x42; 1000]); // 1000 bytes
		let constraint = crate::types::Constraint { constraint_type: 999, payload: large_payload.clone() };

		database.store_constraint(&constraint_id, &constraint).unwrap();
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_some());
		let retrieved_constraint = retrieved.unwrap();
		assert_eq!(retrieved_constraint.constraint_type, 999);
		assert_eq!(retrieved_constraint.payload, large_payload);
	}

	/// Test constraint ID parsing edge cases
	#[tokio::test]
	async fn test_constraint_id_parsing() {
		let (_temp_dir, database) = create_test_db();

		// Test with zero constraint ID
		let zero_id = B256::ZERO;
		let constraint = crate::types::Constraint { constraint_type: 1, payload: Bytes::from(vec![1]) };

		database.store_constraint(&zero_id, &constraint).unwrap();
		let retrieved = database.get_constraint(&zero_id).unwrap();
		assert!(retrieved.is_some());

		// Test with max constraint ID
		let max_id = B256::from([0xFF; 32]);
		let constraint2 = crate::types::Constraint { constraint_type: 2, payload: Bytes::from(vec![2]) };

		database.store_constraint(&max_id, &constraint2).unwrap();
		let retrieved = database.get_constraint(&max_id).unwrap();
		assert!(retrieved.is_some());
	}
}
