use common::types::{SignedConstraints, SignedDelegation};
use eyre::Result;
use rocksdb::{Options, DB};
use std::path::Path;

/// Database context for the relay server
/// Uses separate RocksDB instance to avoid conflicts with gateway
pub struct RelayDatabase {
	db: DB,
}

impl RelayDatabase {
	/// Create a new database instance
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		let mut opts = Options::default();
		opts.create_if_missing(true);

		let db = DB::open(&opts, path)?;
		Ok(Self { db })
	}

	/// Store a verified delegation (assumes only one delegation per slot)
	pub fn store_delegation(&self, slot: u64, delegation_id: &str, delegation: &SignedDelegation) -> Result<()> {
		// Use simple key format: "delegation:{slot}" (ignore delegation_id since we assume one per slot)
		let key = format!("delegation:{}", slot);
		let value = serde_json::to_vec(delegation)?;
		self.db.put(key.as_bytes(), &value)?;
		Ok(())
	}

	/// Get the delegation for a slot (assumes only one delegation per slot)
	pub fn get_delegation_for_slot(&self, slot: u64) -> Result<Option<SignedDelegation>> {
		// Use direct key lookup instead of iteration
		let key = format!("delegation:{}", slot);

		match self.db.get(key.as_bytes())? {
			Some(value) => {
				let delegation: SignedDelegation = serde_json::from_slice(&value)?;
				Ok(Some(delegation))
			}
			None => Ok(None),
		}
	}

	/// Store a verified constraint
	pub fn store_constraint(&self, slot: u64, constraint_id: &str, constraint: &SignedConstraints) -> Result<()> {
		let key = format!("constraint:{}:{}", slot, constraint_id);
		let value = serde_json::to_vec(constraint)?;
		self.db.put(key.as_bytes(), &value)?;
		Ok(())
	}

	/// Get all constraints for a slot
	pub fn get_constraints_for_slot(&self, slot: u64) -> Result<Vec<SignedConstraints>> {
		let mut constraints = Vec::new();
		let prefix = format!("constraint:{}:", slot);

		let iter = self.db.prefix_iterator(prefix.as_bytes());
		for item in iter {
			let (_, value) = item?;
			let constraint: SignedConstraints = serde_json::from_slice(&value)?;
			constraints.push(constraint);
		}

		Ok(constraints)
	}

	/// Get health status
	pub fn get_health_status(&self) -> Result<RelayHealthStatus> {
		let mut total_delegations = 0;
		let mut total_constraints = 0;

		// Count delegations
		let iter = self.db.prefix_iterator(b"delegation:");
		for item in iter {
			let _ = item?;
			total_delegations += 1;
		}

		// Count constraints
		let iter = self.db.prefix_iterator(b"constraint:");
		for item in iter {
			let _ = item?;
			total_constraints += 1;
		}

		Ok(RelayHealthStatus { total_delegations, total_constraints, database_healthy: true })
	}

	/// Delete all data (for testing)
	pub fn clear_all(&self) -> Result<()> {
		let iter = self.db.iterator(rocksdb::IteratorMode::Start);
		for item in iter {
			let (key, _) = item?;
			self.db.delete(&key)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct RelayHealthStatus {
	pub total_delegations: usize,
	pub total_constraints: usize,
	pub database_healthy: bool,
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::{Bytes, B256};
	use commit_boost::prelude::{BlsPublicKey, BlsSignature};
	use common::types::constraints::{Delegation, SignedDelegation};
	use hex;
	use tempfile::TempDir;

	fn create_test_delegation() -> SignedDelegation {
		// Use a valid BLS public key from the test data
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		SignedDelegation {
			message: Delegation {
				proposer: BlsPublicKey::deserialize(&valid_bls_key).expect("Failed to deserialize BLS public key"),
				delegate: BlsPublicKey::deserialize(&valid_bls_key).expect("Failed to deserialize BLS public key"),
				committer: "0x1234567890123456789012345678901234567890".parse().unwrap(),
				slot: 12345,
				metadata: Bytes::from(vec![0x01, 0x02]),
			},
			signature: BlsSignature::empty(),
			nonce: 1,
			signing_id: B256::from_slice(&[0x01; 32]),
		}
	}

	#[test]
	fn test_database_creation() {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let db = RelayDatabase::new(&db_path).unwrap();
		assert!(db.get_health_status().unwrap().database_healthy);
	}

	#[test]
	fn test_delegation_storage_and_retrieval() {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let db = RelayDatabase::new(&db_path).unwrap();

		let delegation = create_test_delegation();
		let slot = 12345;
		let delegation_id = "test_delegation_1";

		// Store delegation
		db.store_delegation(slot, delegation_id, &delegation).unwrap();

		// Retrieve delegation for slot
		let retrieved_delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(retrieved_delegation.is_some());

		// Verify content matches
		assert_eq!(retrieved_delegation.unwrap().message.committer, delegation.message.committer);
	}

	#[test]
	fn test_multiple_delegations_same_slot() {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let db = RelayDatabase::new(&db_path).unwrap();

		let slot = 12345;

		// Store multiple delegations
		for i in 0..3 {
			let delegation = create_test_delegation();
			db.store_delegation(slot, &format!("delegation_{}", i), &delegation).unwrap();
		}

		// Retrieve delegation for slot (should be the last one stored)
		let delegation = db.get_delegation_for_slot(slot).unwrap();
		assert!(delegation.is_some());
	}

	#[test]
	fn test_health_status() {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");
		let db = RelayDatabase::new(&db_path).unwrap();

		// Initially empty
		let health = db.get_health_status().unwrap();
		assert_eq!(health.total_delegations, 0);
		assert_eq!(health.total_constraints, 0);
		assert!(health.database_healthy);

		// Add some data
		let delegation = create_test_delegation();
		db.store_delegation(12345, "test", &delegation).unwrap();

		let health = db.get_health_status().unwrap();
		assert_eq!(health.total_delegations, 1);
	}
}
