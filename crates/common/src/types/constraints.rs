use alloy::consensus::TxEnvelope;
use alloy::primitives::{Address, B256, Bytes, keccak256};
use alloy::rlp::Decodable;
use alloy::rpc::types::beacon::relay::SubmitBlockRequest as AlloySubmitBlockRequest;
use alloy::sol_types::SolValue;
use eyre::Result;
use eyre::eyre;
use serde::{Deserialize, Serialize};
use tracing::error;

use ::urc::registry::BLS::G1Point;
use commit_boost::prelude::BlsPublicKey;
use urc::registry::ISlasher::Delegation as SolDelegation;

use super::{MessageType, convert_pubkey_to_g1_point};

/// A constraint with its type and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
	pub constraint_type: u64,
	pub payload: Bytes,
}
/// A delegation message from proposer to gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
	pub proposer: BlsPublicKey,
	pub delegate: BlsPublicKey,
	pub committer: Address,
	pub slot: u64,
	pub metadata: Bytes,
}

impl Delegation {
	/// ABI-encodes the ConstraintsMessage struct
	pub fn to_message_hash(&self) -> Result<B256> {
		// Convert the pubkeys to G1 points
		let proposer = convert_pubkey_to_g1_point(&self.proposer).map_err(|e| {
			error!("Error converting proposer pubkey {} to G1 point: {e:?}", self.proposer.as_hex_string());
			e
		})?;
		let delegate = convert_pubkey_to_g1_point(&self.delegate).map_err(|e| {
			error!("Error converting delegate pubkey {} to G1 point: {e:?}", self.delegate.as_hex_string());
			e
		})?;

		// Convert the delegation to EVM format
		let delegation_evm = SolDelegation {
			proposer,
			delegate,
			committer: Address(*self.committer),
			slot: self.slot,
			metadata: self.metadata.clone(),
		};

		// Get the object root to sign
		let message_type = MessageType::Delegation.to_uint256();
		let encoded = (message_type, delegation_evm).abi_encode_params(); // Rust equivalent of abi.encode(message_type, delegation) in Solidity
		let object_root = keccak256(encoded);
		Ok(object_root)
	}
}

/// A signed delegation with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedDelegation {
	pub message: Delegation,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: commit_boost::prelude::BlsSignature,
}

/// A constraints message containing multiple constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintsMessage {
	pub proposer: BlsPublicKey,
	pub delegate: BlsPublicKey,
	pub slot: u64,
	pub constraints: Vec<Constraint>,
	pub receivers: Vec<BlsPublicKey>,
}

impl ConstraintsMessage {
	/// ABI-encodes the ConstraintsMessage struct
	pub fn to_message_hash(&self) -> Result<B256> {
		alloy::sol! {
			struct SolConstraint {
				uint64 constraintType;
				bytes payload;
			}

			struct SolConstraintsMessage {
				G1Point proposer;
				G1Point delegate;
				uint64 slot;
				SolConstraint[] constraints;
				G1Point[] receivers;
			}
		}

		// Convert the pubkeys to G1 points
		let proposer = convert_pubkey_to_g1_point(&self.proposer).map_err(|e| {
			error!("Error converting proposer pubkey {} to G1 point: {e:?}", self.proposer.as_hex_string());
			e
		})?;
		let delegate = convert_pubkey_to_g1_point(&self.delegate).map_err(|e| {
			error!("Error converting delegate pubkey {} to G1 point: {e:?}", self.delegate.as_hex_string());
			e
		})?;

		// Convert the ConstraintsMessage to EVM format
		let constraints_message_evm = SolConstraintsMessage {
			proposer,
			delegate,
			slot: self.slot,
			constraints: self
				.constraints
				.iter()
				.map(|c| SolConstraint { constraintType: c.constraint_type, payload: c.payload.clone() })
				.collect(),
			receivers: self.receivers.iter().map(convert_pubkey_to_g1_point).collect::<Result<Vec<_>, _>>()?,
		};

		// Get the object root to sign
		let message_type = MessageType::Constraints.to_uint256();
		let encoded = (message_type, constraints_message_evm).abi_encode_params(); // Rust equivalent of abi.encode(message_type, delegation) in Solidity
		let object_root = keccak256(encoded);
		Ok(object_root)
	}
}

/// A signed constraints message with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedConstraints {
	pub message: ConstraintsMessage,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: commit_boost::prelude::BlsSignature,
}

/// Constraint capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintCapabilities {
	pub constraint_types: Vec<u64>,
}

/// Proofs of constraint validity for a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintProofs {
	#[serde(rename = "constraintTypes")]
	pub constraint_types: Vec<u64>,
	pub payloads: Vec<Bytes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBlockRequestWithProofs {
	#[serde(flatten)]
	pub message: AlloySubmitBlockRequest,
	pub proofs: ConstraintProofs,
}

impl SubmitBlockRequestWithProofs {
	pub fn slot(&self) -> u64 {
		self.message.bid_trace().slot
	}

	pub fn into_block_request(self) -> AlloySubmitBlockRequest {
		self.message
	}

	pub fn transactions(&self) -> Result<Vec<TxEnvelope>> {
		// Extract transaction bytes from the appropriate variant
		let tx_bytes_list = match &self.message {
			AlloySubmitBlockRequest::Electra(request) => {
				&request.execution_payload.payload_inner.payload_inner.transactions
			}
			AlloySubmitBlockRequest::Fulu(request) => {
				&request.execution_payload.payload_inner.payload_inner.transactions
			}
			AlloySubmitBlockRequest::Deneb(request) => {
				&request.execution_payload.payload_inner.payload_inner.transactions
			}
			AlloySubmitBlockRequest::Capella(request) => &request.execution_payload.payload_inner.transactions,
		};

		// Decode transactions
		let mut transactions = Vec::new();

		for tx_bytes in tx_bytes_list {
			let tx =
				TxEnvelope::decode(&mut tx_bytes.as_ref()).map_err(|e| eyre!("Failed to decode transaction: {}", e))?;
			transactions.push(tx);
		}

		if transactions.is_empty() {
			return Err(eyre!("No transactions in execution payload"));
		}

		Ok(transactions)
	}
}

// URC-related types and conversion helpers moved to `types::urc` and `types::mod`

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::bls_pubkey_from_hex;
	use alloy::primitives::{Bytes, U256, hex};

	#[test]
	fn test_message_type_to_uint256() {
		assert_eq!(MessageType::Reserved.to_uint256(), U256::from(0));
		assert_eq!(MessageType::Registration.to_uint256(), U256::from(1));
		assert_eq!(MessageType::Delegation.to_uint256(), U256::from(2));
		assert_eq!(MessageType::Commitment.to_uint256(), U256::from(3));
		assert_eq!(MessageType::Constraints.to_uint256(), U256::from(4));
	}

	#[test]
	fn test_delegation_to_message_hash() {
		let proposer = bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let delegate = bls_pubkey_from_hex(
			"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
		)
		.unwrap();

		let delegation = Delegation {
			proposer,
			delegate,
			committer: hex!("0x1111111111111111111111111111111111111111").into(),
			slot: 5,
			metadata: Bytes::from("some-metadata-here"),
		};
		let message_hash = delegation.to_message_hash().unwrap();
		let expected_hash = hex!("0xcd9aca062121f6f50df1bfd7e74e2b023a5a0d9e1387447568a2119db5022e1b");
		assert_eq!(message_hash, expected_hash);
	}

	#[test]
	fn test_constraints_message_to_message_hash() {
		let proposer = bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();
		let delegate = bls_pubkey_from_hex(
			"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
		)
		.unwrap();

		// Create test BLS public keys
		let receivers = vec![
			bls_pubkey_from_hex(
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
			)
			.unwrap(),
		];

		let constraints_message = ConstraintsMessage {
			proposer,
			delegate,
			slot: 67890,
			constraints: vec![
				Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02]) },
				Constraint { constraint_type: 2, payload: Bytes::from(vec![0x03, 0x04]) },
			],
			receivers,
		};

		let message_hash = constraints_message.to_message_hash().unwrap();
		let expected_hash = hex!("b27bb26406c8fe6cf9e5bb1723d7dd2b06e4d32efc0cb0419dc57cc6c4b0ca87");
		assert_eq!(message_hash, expected_hash);
	}

	#[test]
	fn test_constraint_capabilities() {
		let capabilities = ConstraintCapabilities { constraint_types: vec![1, 2, 3, 4, 5] };

		assert_eq!(capabilities.constraint_types.len(), 5);
		assert_eq!(capabilities.constraint_types[0], 1);
		assert_eq!(capabilities.constraint_types[4], 5);
	}

	/// Test serialization and deserialization of constraint types
	#[test]
	fn test_constraint_serialization() {
		let constraint = Constraint { constraint_type: 1, payload: Bytes::from(vec![1, 2, 3, 4, 5, 6, 7, 8]) };

		// Test JSON serialization
		let json = serde_json::to_string(&constraint).unwrap();
		let deserialized: Constraint = serde_json::from_str(&json).unwrap();

		assert_eq!(constraint.constraint_type, deserialized.constraint_type);
		assert_eq!(constraint.payload, deserialized.payload);
	}
}
