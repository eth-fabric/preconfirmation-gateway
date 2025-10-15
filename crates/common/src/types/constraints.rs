use alloy::primitives::{Address, B256, Bytes};
use alloy::sol_types::SolValue;
use alloy_primitives::keccak256;
use eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::error;

use blst::*;
use commit_boost::prelude::{BlsPublicKey, BlsSignature};
use urc::i_registry::BLS::{G1Point, G2Point};
use urc::i_registry::ISlasher::Delegation as SolDelegation;

use super::MessageType;

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
	pub signature: BlsSignature,
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
			receivers: self.receivers.iter().map(|r| convert_pubkey_to_g1_point(r)).collect::<Result<Vec<_>, _>>()?,
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
	pub signature: BlsSignature,
}

/// Constraint capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintCapabilities {
	pub constraint_types: Vec<u64>,
}

/// Converts a pubkey to its corresponding affine G1 point form for EVM
/// precompile usage
pub fn convert_pubkey_to_g1_point(pubkey: &BlsPublicKey) -> eyre::Result<G1Point> {
	// Convert pubkey to bytes
	let pubkey_byes = pubkey.serialize();

	// Uncompress the bytes to an affine point
	let mut pubkey_affine = blst_p1_affine::default();
	let uncompress_result = unsafe { blst_p1_uncompress(&mut pubkey_affine, pubkey_byes.as_ptr()) };
	match uncompress_result {
		BLST_ERROR::BLST_SUCCESS => Ok(()),
		_ => Err(eyre::eyre!("Error converting pubkey to affine point: {uncompress_result:?}")),
	}?;

	// Convert the coordinates to big-endian byte arrays
	let (x_a, x_b) = convert_fp_to_uint256_pair(&pubkey_affine.x);
	let (y_a, y_b) = convert_fp_to_uint256_pair(&pubkey_affine.y);

	// Return the G1Point
	Ok(G1Point { x_a, x_b, y_a, y_b })
}

/// Converts a signature to its corresponding affine G2 point form for EVM
/// precompile usage
pub fn convert_signature_to_g2_point(signature: &BlsSignature) -> eyre::Result<G2Point> {
	// Convert signature to bytes
	let signature_bytes = signature.serialize();

	// Uncompress the bytes to an affine point
	let mut signature_affine = blst_p2_affine::default();
	let uncompress_result = unsafe { blst_p2_uncompress(&mut signature_affine, signature_bytes.as_ptr()) };
	match uncompress_result {
		BLST_ERROR::BLST_SUCCESS => Ok(()),
		_ => Err(eyre::eyre!("Error converting signature to affine point: {uncompress_result:?}")),
	}?;

	// Convert the coordinates to big-endian byte arrays
	let (x_c0_a, x_c0_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[0]);
	let (x_c1_a, x_c1_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[1]);
	let (y_c0_a, y_c0_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[0]);
	let (y_c1_a, y_c1_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[1]);

	// Return the G2Point
	Ok(G2Point { x_c0_a, x_c0_b, x_c1_a, x_c1_b, y_c0_a, y_c0_b, y_c1_a, y_c1_b })
}

/// Converts a blst_fp to a pair of B256, as used in G1Point
pub fn convert_fp_to_uint256_pair(fp: &blst_fp) -> (B256, B256) {
	let mut fp_bytes = [0u8; 48];
	unsafe {
		blst_bendian_from_fp(fp_bytes.as_mut_ptr(), fp);
	}

	// The first one is the high 16 bytes, padded on the left with zeros
	let mut high_bytes = [0u8; 32];
	high_bytes[16..].copy_from_slice(&fp_bytes[0..16]);
	let high = B256::from(high_bytes);

	// The second one is the low 32 bytes
	let mut low_bytes = [0u8; 32];
	low_bytes.copy_from_slice(&fp_bytes[16..48]);
	let low = B256::from(low_bytes);

	// Return the pair
	(high, low)
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::{Bytes, U256, hex};

	// Local implementation to avoid cb-common dependency
	fn bls_pubkey_from_hex(hex_str: &str) -> BlsPublicKey {
		let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
		let bytes = hex::decode(hex_str).expect("Could not decode BLS hex string");
		if bytes.len() != 48 {
			panic!("Invalid BLS public key length: expected 48 bytes, got {}", bytes.len());
		}
		BlsPublicKey::deserialize(&bytes).expect("Failed to deserialize BLS public key")
	}

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
		);

		let delegate = bls_pubkey_from_hex(
			"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
		);

		let delegation = Delegation {
			proposer: proposer,
			delegate: delegate,
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
		);
		let delegate = bls_pubkey_from_hex(
			"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
		);

		// Create test BLS public keys
		let receivers = vec![bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)];

		let constraints_message = ConstraintsMessage {
			proposer: proposer,
			delegate: delegate,
			slot: 67890,
			constraints: vec![
				Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02]) },
				Constraint { constraint_type: 2, payload: Bytes::from(vec![0x03, 0x04]) },
			],
			receivers: receivers,
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
