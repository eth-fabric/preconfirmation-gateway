use alloy::primitives::{Address, B256, Bytes, Signature};
use alloy::sol_types::SolValue;
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

/// A constraint with its type and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
	pub constraint_type: u64,
	pub payload: Bytes,
}

impl Constraint {
	/// ABI-encodes the Constraint struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolConstraint {
				uint64 constraintType;
				bytes payload;
			}
		}

		Ok(Bytes::from(SolConstraint::abi_encode(&SolConstraint {
			constraintType: self.constraint_type,
			payload: self.payload.clone(),
		})))
	}

	/// ABI-decodes a Constraint from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolConstraint {
				uint64 constraintType;
				bytes payload;
			}
		}

		let decoded = SolConstraint::abi_decode(data).wrap_err("Failed to decode Constraint")?;

		Ok(Constraint { constraint_type: decoded.constraintType, payload: decoded.payload })
	}

	/// Hashes the constraint using ABI encoding and keccak256
	pub fn hash(&self) -> Result<B256> {
		let encoded = self.abi_encode()?;
		Ok(alloy::primitives::keccak256(&encoded))
	}
}

/// A delegation message from proposer to gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
	pub proposer: Bytes, // BLS public key
	pub delegate: Bytes, // BLS public key
	pub committer: Address,
	pub slot: u64,
	pub metadata: Bytes,
}

impl Delegation {
	/// ABI-encodes the Delegation struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolDelegation {
				bytes proposer;
				bytes delegate;
				address committer;
				uint64 slot;
				bytes metadata;
			}
		}

		Ok(Bytes::from(SolDelegation::abi_encode(&SolDelegation {
			proposer: self.proposer.clone(),
			delegate: self.delegate.clone(),
			committer: self.committer,
			slot: self.slot,
			metadata: self.metadata.clone(),
		})))
	}

	/// ABI-decodes a Delegation from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolDelegation {
				bytes proposer;
				bytes delegate;
				address committer;
				uint64 slot;
				bytes metadata;
			}
		}

		let decoded = SolDelegation::abi_decode(data).wrap_err("Failed to decode Delegation")?;

		Ok(Delegation {
			proposer: decoded.proposer,
			delegate: decoded.delegate,
			committer: decoded.committer,
			slot: decoded.slot,
			metadata: decoded.metadata,
		})
	}

	/// Hashes the delegation using ABI encoding and keccak256
	pub fn hash(&self) -> Result<B256> {
		let encoded = self.abi_encode()?;
		Ok(alloy::primitives::keccak256(&encoded))
	}
}

/// A constraints message containing multiple constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintsMessage {
	pub proposer: Bytes, // BLS public key
	pub delegate: Bytes, // BLS public key
	pub slot: u64,
	pub constraints: Vec<Constraint>,
	pub receivers: Vec<Bytes>, // BLS public keys
}

impl ConstraintsMessage {
	/// ABI-encodes the ConstraintsMessage struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolConstraint {
				uint64 constraintType;
				bytes payload;
			}

			struct SolConstraintsMessage {
				bytes proposer;
				bytes delegate;
				uint64 slot;
				SolConstraint[] constraints;
				bytes[] receivers;
			}
		}

		// Convert constraints to SolConstraint format
		let sol_constraints: Vec<SolConstraint> = self
			.constraints
			.iter()
			.map(|c| SolConstraint { constraintType: c.constraint_type, payload: c.payload.clone() })
			.collect();

		Ok(Bytes::from(SolConstraintsMessage::abi_encode(&SolConstraintsMessage {
			proposer: self.proposer.clone(),
			delegate: self.delegate.clone(),
			slot: self.slot,
			constraints: sol_constraints,
			receivers: self.receivers.clone(),
		})))
	}

	/// ABI-decodes a ConstraintsMessage from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolConstraint {
				uint64 constraintType;
				bytes payload;
			}

			struct SolConstraintsMessage {
				bytes proposer;
				bytes delegate;
				uint64 slot;
				SolConstraint[] constraints;
				bytes[] receivers;
			}
		}

		let decoded = SolConstraintsMessage::abi_decode(data).wrap_err("Failed to decode ConstraintsMessage")?;

		// Convert SolConstraint back to Constraint
		let constraints: Vec<Constraint> = decoded
			.constraints
			.into_iter()
			.map(|c| Constraint { constraint_type: c.constraintType, payload: c.payload })
			.collect();

		Ok(ConstraintsMessage {
			proposer: decoded.proposer,
			delegate: decoded.delegate,
			slot: decoded.slot,
			constraints,
			receivers: decoded.receivers,
		})
	}

	/// Hashes the constraints message using ABI encoding and keccak256
	pub fn hash(&self) -> Result<B256> {
		let encoded = self.abi_encode()?;
		Ok(alloy::primitives::keccak256(&encoded))
	}
}

/// A signed delegation with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedDelegation {
	pub message: Delegation,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: Signature,
}

/// A signed constraints message with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedConstraints {
	pub message: ConstraintsMessage,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: Signature,
}

/// Constraint capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintCapabilities {
	pub constraint_types: Vec<u64>,
}

/// Constraint proofs for block submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintProofs {
	pub constraint_types: Vec<u64>,
	pub payloads: Vec<Bytes>,
}

impl ConstraintProofs {
	/// Validates that constraint_types and payloads have the same length
	pub fn validate(&self) -> Result<()> {
		if self.constraint_types.len() != self.payloads.len() {
			return Err(eyre::eyre!(
				"Constraint types and payloads length mismatch: {} vs {}",
				self.constraint_types.len(),
				self.payloads.len()
			));
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_abi_encode_constraint() -> Result<()> {
		let constraint = Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02, 0x03]) };

		let encoded = constraint.abi_encode()?;
		assert!(!encoded.is_empty());

		// Test round-trip encoding/decoding
		let decoded = Constraint::abi_decode(&encoded)?;
		assert_eq!(decoded.constraint_type, constraint.constraint_type);
		assert_eq!(decoded.payload, constraint.payload);

		Ok(())
	}

	#[test]
	fn test_abi_encode_delegation() -> Result<()> {
		let delegation = Delegation {
			proposer: Bytes::from(vec![0x11; 48]), // BLS public key size
			delegate: Bytes::from(vec![0x22; 48]),
			committer: Address::from([0x33; 20]),
			slot: 12345,
			metadata: Bytes::from(vec![0x44, 0x55, 0x66]),
		};

		let encoded = delegation.abi_encode()?;
		assert!(!encoded.is_empty());

		// Test round-trip encoding/decoding
		let decoded = Delegation::abi_decode(&encoded)?;
		assert_eq!(decoded.proposer, delegation.proposer);
		assert_eq!(decoded.delegate, delegation.delegate);
		assert_eq!(decoded.committer, delegation.committer);
		assert_eq!(decoded.slot, delegation.slot);
		assert_eq!(decoded.metadata, delegation.metadata);

		Ok(())
	}

	#[test]
	fn test_abi_encode_constraints_message() -> Result<()> {
		let constraints = vec![
			Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02]) },
			Constraint { constraint_type: 2, payload: Bytes::from(vec![0x03, 0x04]) },
		];

		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 67890,
			constraints,
			receivers: vec![Bytes::from(vec![0x33; 48]), Bytes::from(vec![0x44; 48])],
		};

		let encoded = message.abi_encode()?;
		assert!(!encoded.is_empty());

		// Test round-trip encoding/decoding
		let decoded = ConstraintsMessage::abi_decode(&encoded)?;
		assert_eq!(decoded.proposer, message.proposer);
		assert_eq!(decoded.delegate, message.delegate);
		assert_eq!(decoded.slot, message.slot);
		assert_eq!(decoded.constraints.len(), message.constraints.len());
		assert_eq!(decoded.receivers.len(), message.receivers.len());

		Ok(())
	}

	#[test]
	fn test_constraint_hash() -> Result<()> {
		let constraint = Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02, 0x03]) };

		let hash = constraint.hash()?;
		assert_ne!(hash, B256::ZERO);

		// Hash should be deterministic
		let hash2 = constraint.hash()?;
		assert_eq!(hash, hash2);

		Ok(())
	}

	#[test]
	fn test_delegation_hash() -> Result<()> {
		let delegation = Delegation {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			committer: Address::from([0x33; 20]),
			slot: 12345,
			metadata: Bytes::from(vec![0x44, 0x55, 0x66]),
		};

		let hash = delegation.hash()?;
		assert_ne!(hash, B256::ZERO);

		// Hash should be deterministic
		let hash2 = delegation.hash()?;
		assert_eq!(hash, hash2);

		Ok(())
	}

	#[test]
	fn test_constraints_message_hash() -> Result<()> {
		let constraints = vec![Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02]) }];

		let message = ConstraintsMessage {
			proposer: Bytes::from(vec![0x11; 48]),
			delegate: Bytes::from(vec![0x22; 48]),
			slot: 67890,
			constraints,
			receivers: vec![Bytes::from(vec![0x33; 48])],
		};

		let hash = message.hash()?;
		assert_ne!(hash, B256::ZERO);

		// Hash should be deterministic
		let hash2 = message.hash()?;
		assert_eq!(hash, hash2);

		Ok(())
	}

	#[test]
	fn test_constraint_proofs_validation() -> Result<()> {
		// Valid case
		let valid_proofs = ConstraintProofs {
			constraint_types: vec![1, 2, 3],
			payloads: vec![Bytes::from(vec![0x01]), Bytes::from(vec![0x02]), Bytes::from(vec![0x03])],
		};
		assert!(valid_proofs.validate().is_ok());

		// Invalid case - length mismatch
		let invalid_proofs = ConstraintProofs { constraint_types: vec![1, 2], payloads: vec![Bytes::from(vec![0x01])] };
		assert!(invalid_proofs.validate().is_err());

		Ok(())
	}
}
