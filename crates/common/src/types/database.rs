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
	pub fn store_commitment(&self, slot: u64, request_hash: &B256, signed_commitment: &SignedCommitment) -> Result<()> {
		let key = format!("commitment:{}:{}", slot, request_hash);
		let value = serde_json::to_vec(signed_commitment)?;
		self.put(key.as_bytes(), &value)
	}

	/// Retrieve a signed commitment from the database
	pub fn get_commitment(&self, request_hash: &B256) -> Result<Option<SignedCommitment>> {
		// Search for commitment with any slot (backward compatibility)
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8(key.to_vec())?;

			if key_str.starts_with("commitment:") && key_str.ends_with(&format!(":{}", request_hash)) {
				let signed_commitment: SignedCommitment = serde_json::from_slice(&value)?;
				return Ok(Some(signed_commitment));
			}
		}
		Ok(None)
	}

	/// Delete a commitment by request hash
	pub fn delete_commitment(&self, request_hash: &B256) -> Result<()> {
		// Search for commitment with any slot and delete it
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, _value) = item?;
			let key_str = String::from_utf8(key.to_vec())?;

			if key_str.starts_with("commitment:") && key_str.ends_with(&format!(":{}", request_hash)) {
				return self.delete(&key);
			}
		}
		Ok(())
	}

	/// Get all commitments for a specific slot
	pub fn get_commitments_for_slot(&self, slot: u64) -> Result<Vec<(B256, SignedCommitment)>> {
		let prefix = format!("commitment:{}:", slot);
		let mut commitments = Vec::new();

		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8(key.to_vec())?;

			if key_str.starts_with(&prefix) {
				// Extract request hash from key: "commitment:slot:request_hash"
				let request_hash_str = &key_str[prefix.len()..];
				if let Ok(request_hash) = request_hash_str.parse::<B256>() {
					let signed_commitment: SignedCommitment = serde_json::from_slice(&value)?;
					commitments.push((request_hash, signed_commitment));
				}
			}
		}

		Ok(commitments)
	}

	/// Get all constraints for a specific slot
	pub fn get_constraints_for_slot(&self, slot: u64) -> Result<Vec<(B256, crate::types::Constraint)>> {
		let prefix = format!("constraint:{}:", slot);
		let mut constraints = Vec::new();

		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8(key.to_vec())?;

			if key_str.starts_with(&prefix) {
				// Extract constraint ID from key: "constraint:slot:constraint_id"
				let constraint_id_str = &key_str[prefix.len()..];
				if let Ok(constraint_id) = constraint_id_str.parse::<B256>() {
					let constraint: crate::types::Constraint = serde_json::from_slice(&value)?;
					constraints.push((constraint_id, constraint));
				}
			}
		}

		Ok(constraints)
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
	pub fn store_constraint(
		&self,
		slot: u64,
		constraint_id: &B256,
		constraint: &crate::types::Constraint,
	) -> Result<()> {
		let key = format!("constraint:{}:{}", slot, constraint_id);
		let value = serde_json::to_vec(constraint)?;
		self.put(key.as_bytes(), &value)
	}

	/// Retrieve a constraint from the database
	pub fn get_constraint(&self, constraint_id: &B256) -> Result<Option<crate::types::Constraint>> {
		// Search for constraint with any slot (backward compatibility)
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8(key.to_vec())?;

			if key_str.starts_with("constraint:") && key_str.ends_with(&format!(":{}", constraint_id)) {
				let constraint: crate::types::Constraint = serde_json::from_slice(&value)?;
				return Ok(Some(constraint));
			}
		}
		Ok(None)
	}

	/// Get all pending constraints (not yet sent to relay)
	pub fn get_pending_constraints(&self) -> Result<Vec<(B256, crate::types::Constraint)>> {
		let mut pending_constraints = Vec::new();

		// Scan for constraint:*:* keys (slot-based format)
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);

		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8_lossy(&key);

			// Check if this is a constraint key with slot format: "constraint:slot:constraint_id"
			if key_str.starts_with("constraint:") && key_str.matches(':').count() == 2 {
				// Extract constraint ID from the end of the key
				if let Some(last_colon_pos) = key_str.rfind(':') {
					let constraint_id_str = &key_str[last_colon_pos + 1..];

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

		// Scan for constraint:*:* keys (slot-based format)
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);

		for item in iter {
			let (key, value) = item?;
			let key_str = String::from_utf8_lossy(&key);

			// Check if this is a constraint key with slot format: "constraint:slot:constraint_id"
			if key_str.starts_with("constraint:") && key_str.matches(':').count() == 2 {
				// Extract constraint ID from the end of the key
				if let Some(last_colon_pos) = key_str.rfind(':') {
					let constraint_id_str = &key_str[last_colon_pos + 1..];

					// Parse the constraint ID
					if let Ok(constraint_id) = constraint_id_str.parse::<B256>() {
						// Deserialize the constraint
						if let Ok(constraint) = serde_json::from_slice::<crate::types::Constraint>(&value) {
							all_constraints.push((constraint_id, constraint));
						}
					}
				}
			}
		}

		Ok(all_constraints)
	}

	/// Delete constraint and its status
	pub fn delete_constraint(&self, constraint_id: &B256) -> Result<()> {
		// Search for constraint with any slot and delete it
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, _value) = item?;
			let key_str = String::from_utf8_lossy(&key);

			if key_str.starts_with("constraint:") && key_str.ends_with(&format!(":{}", constraint_id)) {
				self.delete(&key)?;
				break;
			}
		}

		// Also delete the status key
		let status_key = format!("constraint_status:{}", constraint_id);
		self.delete(status_key.as_bytes())?;

		Ok(())
	}

	/// Store a delegation for a specific slot (assumes only one delegation per slot)
	pub fn store_delegation(&self, slot: u64, delegation: &crate::types::constraints::SignedDelegation) -> Result<()> {
		// Use a simple, consistent key format: "delegation:{slot}"
		let key = format!("delegation:{}", slot);
		let value = serde_json::to_vec(delegation)?;
		self.put(key.as_bytes(), &value)
	}

	/// Get the delegation for a specific slot (assumes only one delegation per slot)
	pub fn get_delegation_for_slot(&self, slot: u64) -> Result<Option<crate::types::constraints::SignedDelegation>> {
		// Use direct key lookup instead of iteration
		let key = format!("delegation:{}", slot);

		match self.db.get(key.as_bytes())? {
			Some(value) => {
				let delegation: crate::types::constraints::SignedDelegation = serde_json::from_slice(&value)?;
				Ok(Some(delegation))
			}
			None => Ok(None),
		}
	}

	/// Check if there are any delegations for a specific slot
	pub fn is_delegated(&self, slot: u64) -> Result<bool> {
		// Use direct key lookup instead of iteration
		let key = format!("delegation:{}", slot);
		Ok(self.db.get(key.as_bytes())?.is_some())
	}

	/// Delete all delegations for a specific slot
	pub fn delete_delegations_for_slot(&self, slot: u64) -> Result<()> {
		// Use direct key deletion instead of iteration
		let key = format!("delegation:{}", slot);
		self.delete(key.as_bytes())
	}

	/// Store the latest gas price in the database (overwrites previous)
	pub fn store_latest_price(&self, price_gwei: u64) -> Result<()> {
		let key = "latest_price_gwei";
		let value = price_gwei.to_string();
		self.put(key.as_bytes(), value.as_bytes())
	}

	/// Retrieve the latest gas price from the database
	pub fn get_latest_price(&self) -> Result<Option<u64>> {
		let key = "latest_price_gwei";
		match self.get(key.as_bytes())? {
			Some(value) => {
				let price_str = String::from_utf8(value)?;
				let price = price_str.parse::<u64>()?;
				Ok(Some(price))
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
		db.store_commitment(12345, &request_hash, &signed_commitment).unwrap();

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

			db.store_commitment(12345, &request_hash, &signed_commitment).unwrap();
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
		db.store_commitment(12345, &request_hash, &signed_commitment).unwrap();

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
		db.store_commitment(12345, &request_hash, &original_signed).unwrap();
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
		database.store_constraint(12345, &constraint_id, &constraint).unwrap();

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
		database.store_constraint(12345, &constraint_id, &constraint).unwrap();
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
		database.store_constraint(12345, &constraint1_id, &constraint1).unwrap();
		database.store_constraint(12345, &constraint2_id, &constraint2).unwrap();
		database.store_constraint(12345, &constraint3_id, &constraint3).unwrap();

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
		database.store_constraint(12345, &constraint_id, &original_constraint).unwrap();

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
		database.store_constraint(12345, &constraint_id, &constraint).unwrap();
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

		database.store_constraint(12345, &constraint_id, &constraint).unwrap();
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

		database.store_constraint(12345, &constraint_id, &constraint).unwrap();
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

		database.store_constraint(12345, &zero_id, &constraint).unwrap();
		let retrieved = database.get_constraint(&zero_id).unwrap();
		assert!(retrieved.is_some());

		// Test with max constraint ID
		let max_id = B256::from([0xFF; 32]);
		let constraint2 = crate::types::Constraint { constraint_type: 2, payload: Bytes::from(vec![2]) };

		database.store_constraint(12345, &max_id, &constraint2).unwrap();
		let retrieved = database.get_constraint(&max_id).unwrap();
		assert!(retrieved.is_some());
	}

	/// Test get_commitments_for_slot with multiple commitments
	#[tokio::test]
	async fn test_get_commitments_for_slot() {
		let (_temp_dir, database) = create_test_db();

		let slot = 12345;
		let commitments = vec![
			(B256::from_slice(&[1; 32]), create_test_commitment(1)),
			(B256::from_slice(&[2; 32]), create_test_commitment(2)),
			(B256::from_slice(&[3; 32]), create_test_commitment(3)),
		];

		// Store commitments for the slot
		for (request_hash, commitment) in &commitments {
			database.store_commitment(slot, request_hash, commitment).unwrap();
		}

		// Store a commitment for a different slot
		let other_slot = 54321;
		let other_request_hash = B256::from_slice(&[4; 32]);
		let other_commitment = create_test_commitment(4);
		database.store_commitment(other_slot, &other_request_hash, &other_commitment).unwrap();

		// Get commitments for the target slot
		let retrieved = database.get_commitments_for_slot(slot).unwrap();
		assert_eq!(retrieved.len(), 3);

		// Verify all commitments are present
		let retrieved_hashes: Vec<B256> = retrieved.iter().map(|(hash, _)| *hash).collect();
		assert!(retrieved_hashes.contains(&B256::from_slice(&[1; 32])));
		assert!(retrieved_hashes.contains(&B256::from_slice(&[2; 32])));
		assert!(retrieved_hashes.contains(&B256::from_slice(&[3; 32])));
		assert!(!retrieved_hashes.contains(&B256::from_slice(&[4; 32])));

		// Verify commitment data integrity
		for (request_hash, commitment) in retrieved {
			let original = &commitments.iter().find(|(h, _)| h == &request_hash).unwrap().1;
			assert_eq!(commitment.commitment.commitment_type, original.commitment.commitment_type);
			assert_eq!(commitment.commitment.payload, original.commitment.payload);
		}
	}

	/// Test get_commitments_for_slot with empty slot
	#[tokio::test]
	async fn test_get_commitments_for_slot_empty() {
		let (_temp_dir, database) = create_test_db();

		let slot = 99999;
		let retrieved = database.get_commitments_for_slot(slot).unwrap();
		assert_eq!(retrieved.len(), 0);
	}

	/// Test get_constraints_for_slot with multiple constraints
	#[tokio::test]
	async fn test_get_constraints_for_slot() {
		let (_temp_dir, database) = create_test_db();

		let slot = 12345;
		let constraints = vec![
			(B256::from_slice(&[1; 32]), create_test_constraint(1)),
			(B256::from_slice(&[2; 32]), create_test_constraint(2)),
			(B256::from_slice(&[3; 32]), create_test_constraint(3)),
		];

		// Store constraints for the slot
		for (constraint_id, constraint) in &constraints {
			database.store_constraint(slot, constraint_id, constraint).unwrap();
		}

		// Store a constraint for a different slot
		let other_slot = 54321;
		let other_constraint_id = B256::from_slice(&[4; 32]);
		let other_constraint = create_test_constraint(4);
		database.store_constraint(other_slot, &other_constraint_id, &other_constraint).unwrap();

		// Get constraints for the target slot
		let retrieved = database.get_constraints_for_slot(slot).unwrap();
		assert_eq!(retrieved.len(), 3);

		// Verify all constraints are present
		let retrieved_ids: Vec<B256> = retrieved.iter().map(|(id, _)| *id).collect();
		assert!(retrieved_ids.contains(&B256::from_slice(&[1; 32])));
		assert!(retrieved_ids.contains(&B256::from_slice(&[2; 32])));
		assert!(retrieved_ids.contains(&B256::from_slice(&[3; 32])));
		assert!(!retrieved_ids.contains(&B256::from_slice(&[4; 32])));

		// Verify constraint data integrity
		for (constraint_id, constraint) in retrieved {
			let original = &constraints.iter().find(|(id, _)| id == &constraint_id).unwrap().1;
			assert_eq!(constraint.constraint_type, original.constraint_type);
			assert_eq!(constraint.payload, original.payload);
		}
	}

	/// Test get_constraints_for_slot with empty slot
	#[tokio::test]
	async fn test_get_constraints_for_slot_empty() {
		let (_temp_dir, database) = create_test_db();

		let slot = 99999;
		let retrieved = database.get_constraints_for_slot(slot).unwrap();
		assert_eq!(retrieved.len(), 0);
	}

	/// Test slot-based key format parsing
	#[tokio::test]
	async fn test_slot_based_key_format() {
		let (_temp_dir, database) = create_test_db();

		let slot = 12345;
		let request_hash = B256::from_slice(&[0xAB; 32]);
		let constraint_id = B256::from_slice(&[0xCD; 32]);

		// Test commitment key format: "commitment:slot:request_hash"
		let commitment = create_test_commitment(1);
		database.store_commitment(slot, &request_hash, &commitment).unwrap();

		// Test constraint key format: "constraint:slot:constraint_id"
		let constraint = create_test_constraint(1);
		database.store_constraint(slot, &constraint_id, &constraint).unwrap();

		// Verify both can be retrieved
		let retrieved_commitment = database.get_commitment(&request_hash).unwrap();
		assert!(retrieved_commitment.is_some());

		let retrieved_constraint = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved_constraint.is_some());

		// Verify slot-based retrieval works
		let slot_commitments = database.get_commitments_for_slot(slot).unwrap();
		assert_eq!(slot_commitments.len(), 1);
		assert_eq!(slot_commitments[0].0, request_hash);

		let slot_constraints = database.get_constraints_for_slot(slot).unwrap();
		assert_eq!(slot_constraints.len(), 1);
		assert_eq!(slot_constraints[0].0, constraint_id);
	}

	/// Test get_pending_constraints with slot-based keys
	#[tokio::test]
	async fn test_get_pending_constraints_slot_based() {
		let (_temp_dir, database) = create_test_db();

		let slot1 = 12345;
		let slot2 = 54321;

		// Create constraints for different slots
		let constraint1_id = B256::from_slice(&[1; 32]);
		let constraint1 = create_test_constraint(1);
		database.store_constraint(slot1, &constraint1_id, &constraint1).unwrap();

		let constraint2_id = B256::from_slice(&[2; 32]);
		let constraint2 = create_test_constraint(2);
		database.store_constraint(slot1, &constraint2_id, &constraint2).unwrap();

		let constraint3_id = B256::from_slice(&[3; 32]);
		let constraint3 = create_test_constraint(3);
		database.store_constraint(slot2, &constraint3_id, &constraint3).unwrap();

		// All should be pending initially
		let pending = database.get_pending_constraints().unwrap();
		assert_eq!(pending.len(), 3);

		// Mark one as sent
		database.mark_constraint_sent(&constraint2_id).unwrap();

		// Now only 2 should be pending
		let pending = database.get_pending_constraints().unwrap();
		assert_eq!(pending.len(), 2);

		// Verify the correct constraints are pending
		let pending_ids: Vec<B256> = pending.iter().map(|(id, _)| *id).collect();
		assert!(pending_ids.contains(&constraint1_id));
		assert!(pending_ids.contains(&constraint3_id));
		assert!(!pending_ids.contains(&constraint2_id));
	}

	/// Test get_all_constraints with slot-based keys
	#[tokio::test]
	async fn test_get_all_constraints_slot_based() {
		let (_temp_dir, database) = create_test_db();

		let slot1 = 12345;
		let slot2 = 54321;

		// Create constraints for different slots
		let constraint1_id = B256::from_slice(&[1; 32]);
		let constraint1 = create_test_constraint(1);
		database.store_constraint(slot1, &constraint1_id, &constraint1).unwrap();

		let constraint2_id = B256::from_slice(&[2; 32]);
		let constraint2 = create_test_constraint(2);
		database.store_constraint(slot1, &constraint2_id, &constraint2).unwrap();

		let constraint3_id = B256::from_slice(&[3; 32]);
		let constraint3 = create_test_constraint(3);
		database.store_constraint(slot2, &constraint3_id, &constraint3).unwrap();

		// Mark one as sent
		database.mark_constraint_sent(&constraint2_id).unwrap();

		// get_all_constraints should return all 3 regardless of status
		let all_constraints = database.get_all_constraints().unwrap();
		assert_eq!(all_constraints.len(), 3);

		// Verify all constraint IDs are present
		let all_ids: Vec<B256> = all_constraints.iter().map(|(id, _)| *id).collect();
		assert!(all_ids.contains(&constraint1_id));
		assert!(all_ids.contains(&constraint2_id));
		assert!(all_ids.contains(&constraint3_id));
	}

	/// Test delete_commitment with slot-based keys
	#[tokio::test]
	async fn test_delete_commitment_slot_based() {
		let (_temp_dir, database) = create_test_db();

		let slot = 12345;
		let request_hash = B256::from_slice(&[0xAB; 32]);
		let commitment = create_test_commitment(1);

		// Store commitment
		database.store_commitment(slot, &request_hash, &commitment).unwrap();

		// Verify it exists
		let retrieved = database.get_commitment(&request_hash).unwrap();
		assert!(retrieved.is_some());

		// Delete commitment
		database.delete_commitment(&request_hash).unwrap();

		// Verify it's gone
		let retrieved = database.get_commitment(&request_hash).unwrap();
		assert!(retrieved.is_none());

		// Verify slot-based retrieval also returns empty
		let slot_commitments = database.get_commitments_for_slot(slot).unwrap();
		assert_eq!(slot_commitments.len(), 0);
	}

	/// Test delete_constraint with slot-based keys
	#[tokio::test]
	async fn test_delete_constraint_slot_based() {
		let (_temp_dir, database) = create_test_db();

		let slot = 12345;
		let constraint_id = B256::from_slice(&[0xCD; 32]);
		let constraint = create_test_constraint(1);

		// Store constraint
		database.store_constraint(slot, &constraint_id, &constraint).unwrap();

		// Verify it exists
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_some());

		// Delete constraint
		database.delete_constraint(&constraint_id).unwrap();

		// Verify it's gone
		let retrieved = database.get_constraint(&constraint_id).unwrap();
		assert!(retrieved.is_none());

		// Verify slot-based retrieval also returns empty
		let slot_constraints = database.get_constraints_for_slot(slot).unwrap();
		assert_eq!(slot_constraints.len(), 0);
	}

	/// Test mixed slot operations
	#[tokio::test]
	async fn test_mixed_slot_operations() {
		let (_temp_dir, database) = create_test_db();

		let slot1 = 11111;
		let slot2 = 22222;

		// Store commitments and constraints for different slots
		let commitment1_hash = B256::from_slice(&[1; 32]);
		let commitment1 = create_test_commitment(1);
		database.store_commitment(slot1, &commitment1_hash, &commitment1).unwrap();

		let commitment2_hash = B256::from_slice(&[2; 32]);
		let commitment2 = create_test_commitment(2);
		database.store_commitment(slot2, &commitment2_hash, &commitment2).unwrap();

		let constraint1_id = B256::from_slice(&[3; 32]);
		let constraint1 = create_test_constraint(1);
		database.store_constraint(slot1, &constraint1_id, &constraint1).unwrap();

		let constraint2_id = B256::from_slice(&[4; 32]);
		let constraint2 = create_test_constraint(2);
		database.store_constraint(slot2, &constraint2_id, &constraint2).unwrap();

		// Test slot-specific retrieval
		let slot1_commitments = database.get_commitments_for_slot(slot1).unwrap();
		assert_eq!(slot1_commitments.len(), 1);
		assert_eq!(slot1_commitments[0].0, commitment1_hash);

		let slot2_commitments = database.get_commitments_for_slot(slot2).unwrap();
		assert_eq!(slot2_commitments.len(), 1);
		assert_eq!(slot2_commitments[0].0, commitment2_hash);

		let slot1_constraints = database.get_constraints_for_slot(slot1).unwrap();
		assert_eq!(slot1_constraints.len(), 1);
		assert_eq!(slot1_constraints[0].0, constraint1_id);

		let slot2_constraints = database.get_constraints_for_slot(slot2).unwrap();
		assert_eq!(slot2_constraints.len(), 1);
		assert_eq!(slot2_constraints[0].0, constraint2_id);

		// Test cross-slot isolation
		let slot1_constraints = database.get_constraints_for_slot(slot1).unwrap();
		assert!(!slot1_constraints.iter().any(|(id, _)| *id == constraint2_id));

		let slot2_constraints = database.get_constraints_for_slot(slot2).unwrap();
		assert!(!slot2_constraints.iter().any(|(id, _)| *id == constraint1_id));
	}

	/// Test edge cases with slot-based operations
	#[tokio::test]
	async fn test_slot_based_edge_cases() {
		let (_temp_dir, database) = create_test_db();

		// Test with slot 0
		let slot_zero = 0;
		let request_hash = B256::from_slice(&[0x00; 32]);
		let commitment = create_test_commitment(1);
		database.store_commitment(slot_zero, &request_hash, &commitment).unwrap();

		let retrieved = database.get_commitments_for_slot(slot_zero).unwrap();
		assert_eq!(retrieved.len(), 1);

		// Test with very large slot number
		let large_slot = u64::MAX;
		let constraint_id = B256::from_slice(&[0xFF; 32]);
		let constraint = create_test_constraint(1);
		database.store_constraint(large_slot, &constraint_id, &constraint).unwrap();

		let retrieved = database.get_constraints_for_slot(large_slot).unwrap();
		assert_eq!(retrieved.len(), 1);

		// Test with same request_hash/constraint_id across different slots
		let slot1 = 100;
		let slot2 = 200;
		let same_hash = B256::from_slice(&[0xAA; 32]);
		let same_constraint_id = B256::from_slice(&[0xBB; 32]);

		let commitment1 = create_test_commitment(1);
		let commitment2 = create_test_commitment(2);
		database.store_commitment(slot1, &same_hash, &commitment1).unwrap();
		database.store_commitment(slot2, &same_hash, &commitment2).unwrap();

		let constraint1 = create_test_constraint(1);
		let constraint2 = create_test_constraint(2);
		database.store_constraint(slot1, &same_constraint_id, &constraint1).unwrap();
		database.store_constraint(slot2, &same_constraint_id, &constraint2).unwrap();

		// Both slots should have their respective items
		let slot1_commitments = database.get_commitments_for_slot(slot1).unwrap();
		assert_eq!(slot1_commitments.len(), 1);
		assert_eq!(slot1_commitments[0].1.commitment.commitment_type, 1);

		let slot2_commitments = database.get_commitments_for_slot(slot2).unwrap();
		assert_eq!(slot2_commitments.len(), 1);
		assert_eq!(slot2_commitments[0].1.commitment.commitment_type, 2);

		let slot1_constraints = database.get_constraints_for_slot(slot1).unwrap();
		assert_eq!(slot1_constraints.len(), 1);
		assert_eq!(slot1_constraints[0].1.constraint_type, 1);

		let slot2_constraints = database.get_constraints_for_slot(slot2).unwrap();
		assert_eq!(slot2_constraints.len(), 1);
		assert_eq!(slot2_constraints[0].1.constraint_type, 2);
	}

	// Helper functions for creating test data
	fn create_test_commitment(commitment_type: u64) -> SignedCommitment {
		let request_hash = B256::random();
		let commitment = Commitment {
			commitment_type,
			payload: Bytes::from(vec![commitment_type as u8]),
			request_hash,
			slasher: Address::random(),
		};
		SignedCommitment {
			commitment,
			nonce: commitment_type * 1000,
			signing_id: B256::random(),
			signature: Signature::from_bytes_and_parity(&[0u8; 64], true),
		}
	}

	fn create_test_constraint(constraint_type: u64) -> crate::types::Constraint {
		crate::types::Constraint { constraint_type, payload: Bytes::from(vec![constraint_type as u8]) }
	}

	// Delegation tests
	#[test]
	fn test_store_delegation() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;
		let delegation = create_test_delegation();

		// Store delegation
		db.store_delegation(slot, &delegation).unwrap();

		// Verify delegation was stored
		let stored_delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(stored_delegation.is_some());
		let stored_delegation = stored_delegation.unwrap();
		assert_eq!(stored_delegation.message.committer, delegation.message.committer);
		assert_eq!(stored_delegation.message.delegate, delegation.message.delegate);
	}

	#[test]
	fn test_store_multiple_delegations_same_slot() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;
		let delegation1 = create_test_delegation();
		let delegation2 = create_test_delegation_with_committer(Address::from([2u8; 20]));

		// Store first delegation
		db.store_delegation(slot, &delegation1).unwrap();

		// Verify first delegation was stored
		let stored_delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(stored_delegation.is_some());
		assert_eq!(stored_delegation.unwrap().message.committer, delegation1.message.committer);

		// Store second delegation (this should overwrite the first since we assume only one per slot)
		db.store_delegation(slot, &delegation2).unwrap();

		// Verify only the second delegation is stored
		let stored_delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(stored_delegation.is_some());
		assert_eq!(stored_delegation.unwrap().message.committer, delegation2.message.committer);
	}

	#[test]
	fn test_get_delegation_for_slot_empty() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;

		// Get delegation for empty slot
		let delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(delegation.is_none());
	}

	#[test]
	fn test_is_delegated_true() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;
		let delegation = create_test_delegation();

		// Initially no delegations
		assert!(!db.is_delegated(slot).unwrap());

		// Store delegation
		db.store_delegation(slot, &delegation).unwrap();

		// Now should have delegations
		assert!(db.is_delegated(slot).unwrap());
	}

	#[test]
	fn test_is_delegated_false() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;

		// No delegations stored
		assert!(!db.is_delegated(slot).unwrap());
	}

	#[test]
	fn test_delete_delegations_for_slot() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;
		let delegation1 = create_test_delegation();
		let delegation2 = create_test_delegation_with_committer(Address::from([2u8; 20]));

		// Store multiple delegations
		db.store_delegation(slot, &delegation1).unwrap();
		db.store_delegation(slot, &delegation2).unwrap();

		// Verify they exist
		assert!(db.is_delegated(slot).unwrap());
		let delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(delegation.is_some());

		// Delete all delegations for the slot
		db.delete_delegations_for_slot(slot).unwrap();

		// Verify they are gone
		assert!(!db.is_delegated(slot).unwrap());
		let delegation_after_delete = db.get_delegation_for_slot(slot).unwrap();
		assert!(delegation_after_delete.is_none());
	}

	#[test]
	fn test_delete_delegations_for_slot_partial() {
		let (_temp_dir, db) = create_test_db();
		let slot1 = 12345;
		let slot2 = 67890;
		let delegation1 = create_test_delegation();
		let delegation2 = create_test_delegation_with_committer(Address::from([2u8; 20]));

		// Store delegations for both slots
		db.store_delegation(slot1, &delegation1).unwrap();
		db.store_delegation(slot2, &delegation2).unwrap();

		// Verify both slots have delegations
		assert!(db.is_delegated(slot1).unwrap());
		assert!(db.is_delegated(slot2).unwrap());

		// Delete delegations for slot1 only
		db.delete_delegations_for_slot(slot1).unwrap();

		// Verify slot1 is empty but slot2 still has delegations
		assert!(!db.is_delegated(slot1).unwrap());
		assert!(db.is_delegated(slot2).unwrap());

		let slot1_delegation = db.get_delegation_for_slot(slot1).unwrap();
		let slot2_delegation = db.get_delegation_for_slot(slot2).unwrap();
		assert!(slot1_delegation.is_none());
		assert!(slot2_delegation.is_some());
	}

	#[test]
	fn test_delegation_key_format() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;
		let delegation = create_test_delegation();

		// Store delegation
		db.store_delegation(slot, &delegation).unwrap();

		// Manually check the key format by iterating through the database
		let iter = db.db.iterator(rocksdb::IteratorMode::Start);
		let mut found_delegation_key = false;
		let expected_key = format!("delegation:{}", slot);

		for item in iter {
			let (key, _value) = item.unwrap();
			let key_str = String::from_utf8_lossy(&key);

			if key_str == expected_key {
				found_delegation_key = true;
				// Verify the key format: delegation:slot
				assert_eq!(key_str, expected_key);
				break;
			}
		}

		assert!(found_delegation_key, "Delegation key not found in database");
	}

	#[test]
	fn test_delegation_serialization_roundtrip() {
		let (_temp_dir, db) = create_test_db();
		let slot = 12345;
		let original_delegation = create_test_delegation();

		// Store delegation
		db.store_delegation(slot, &original_delegation).unwrap();

		// Retrieve delegation
		let retrieved_delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(retrieved_delegation.is_some());
		let retrieved_delegation = retrieved_delegation.unwrap();

		// Verify all fields match
		assert_eq!(retrieved_delegation.message.committer, original_delegation.message.committer);
		assert_eq!(retrieved_delegation.message.delegate, original_delegation.message.delegate);
		assert_eq!(retrieved_delegation.message.slot, original_delegation.message.slot);
		assert_eq!(retrieved_delegation.signature, original_delegation.signature);
	}

	// Helper functions for delegation tests
	fn create_test_delegation() -> crate::types::constraints::SignedDelegation {
		use crate::types::constraints::{Delegation, SignedDelegation};
		use commit_boost::prelude::{BlsPublicKey, BlsSignature};

		let committer = Address::from([1u8; 20]);

		// Use the actual test data from integration tests
		let pubkey_bytes = [
			0x88, 0x38, 0x27, 0x19, 0x3f, 0x76, 0x27, 0xcd, 0x04, 0xe6, 0x21, 0xe1, 0xe8, 0xd5, 0x64, 0x98, 0x36, 0x2a,
			0x52, 0xb2, 0xa3, 0x0c, 0x9a, 0x1c, 0x72, 0x03, 0x6e, 0xb9, 0x35, 0xc4, 0x27, 0x8d, 0xee, 0x23, 0xd3, 0x8a,
			0x24, 0xd2, 0xf7, 0xdd, 0xa6, 0x26, 0x89, 0x88, 0x6f, 0x0c, 0x39, 0xf4,
		];
		let proposer = BlsPublicKey::deserialize(&pubkey_bytes).unwrap();
		let delegate = BlsPublicKey::deserialize(&pubkey_bytes).unwrap();

		let slot = 12345;
		let metadata = Bytes::from(vec![0x01, 0x02, 0x03]);

		// Create a mock signature - we'll use a simple approach for testing
		let signature_bytes = [0u8; 96]; // This will fail BLS validation but that's OK for database tests
		let signature = BlsSignature::deserialize(&signature_bytes).unwrap();

		let nonce = 12345;
		let signing_id = B256::from_slice(&[3u8; 32]);

		let delegation = Delegation { proposer, delegate, committer, slot, metadata };

		SignedDelegation { message: delegation, nonce, signing_id, signature }
	}

	fn create_test_delegation_with_committer(committer: Address) -> crate::types::constraints::SignedDelegation {
		use crate::types::constraints::{Delegation, SignedDelegation};
		use commit_boost::prelude::{BlsPublicKey, BlsSignature};

		// Use the actual test data from integration tests
		let pubkey_bytes = [
			0x88, 0x38, 0x27, 0x19, 0x3f, 0x76, 0x27, 0xcd, 0x04, 0xe6, 0x21, 0xe1, 0xe8, 0xd5, 0x64, 0x98, 0x36, 0x2a,
			0x52, 0xb2, 0xa3, 0x0c, 0x9a, 0x1c, 0x72, 0x03, 0x6e, 0xb9, 0x35, 0xc4, 0x27, 0x8d, 0xee, 0x23, 0xd3, 0x8a,
			0x24, 0xd2, 0xf7, 0xdd, 0xa6, 0x26, 0x89, 0x88, 0x6f, 0x0c, 0x39, 0xf4,
		];
		let proposer = BlsPublicKey::deserialize(&pubkey_bytes).unwrap();
		let delegate = BlsPublicKey::deserialize(&pubkey_bytes).unwrap();

		let slot = 12345;
		let metadata = Bytes::from(vec![0x04, 0x05, 0x06]);

		// Create a mock signature - we'll use a simple approach for testing
		let signature_bytes = [0u8; 96]; // This will fail BLS validation but that's OK for database tests
		let signature = BlsSignature::deserialize(&signature_bytes).unwrap();

		let nonce = 67890;
		let signing_id = B256::from_slice(&[5u8; 32]);

		let delegation = Delegation { proposer, delegate, committer, slot, metadata };

		SignedDelegation { message: delegation, nonce, signing_id, signature }
	}
}
