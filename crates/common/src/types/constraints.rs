use alloy::primitives::{Address, B256, Bytes, Signature, U256};
use alloy::sol_types::SolValue;
use alloy_primitives::keccak256;
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use tracing::error;

use blst::*;
use commit_boost::prelude::{BlsPublicKey, BlsSignature};
use urc::i_registry::BLS::{G1Point, G2Point};
use urc::i_registry::ISlasher::Delegation as SolDelegation;

/// Binding of the MessageType enum, defined here:
/// https://github.com/eth-fabric/urc/blob/304e59f967dd8fdf4342c2f776f789e7c99b8ef9/src/IRegistry.sol#L99
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MessageType {
	Reserved = 0,
	Registration = 1,
	Delegation = 2,
	Commitment = 3,
	Constraints = 4,
}

impl MessageType {
	pub fn to_uint256(self) -> U256 {
		U256::from(self as u64)
	}
}

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
	pub proposer: BlsPublicKey,
	pub delegate: BlsPublicKey,
	pub committer: Address,
	pub slot: u64,
	pub metadata: Bytes,
}

impl Delegation {
	/// ABI-encodes the ConstraintsMessage struct
	pub fn to_object_root(&self) -> Result<B256> {
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
	pub fn to_object_root(&self) -> Result<B256> {
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
