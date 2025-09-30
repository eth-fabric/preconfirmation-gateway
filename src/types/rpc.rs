use alloy::primitives::{Address, B256, Bytes, Signature};
use alloy::sol_types::SolValue;
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

/// Payload for commitments/constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionPayload {
	pub slot: u64,
	pub signed_tx: Bytes,
}

impl InclusionPayload {
	/// ABI-encodes the InclusionPayload struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolInclusionPayload {
				uint64 slot;
				bytes signed_tx;
			}
		}

		Ok(Bytes::from(SolInclusionPayload::abi_encode(&SolInclusionPayload {
			slot: self.slot,
			signed_tx: self.signed_tx.clone(),
		})))
	}

	/// ABI-decodes an InclusionPayload from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolInclusionPayload {
				uint64 slot;
				bytes signed_tx;
			}
		}

		let decoded = SolInclusionPayload::abi_decode(data).wrap_err("Failed to decode InclusionPayload")?;

		Ok(InclusionPayload { slot: decoded.slot, signed_tx: decoded.signed_tx })
	}
}

/// Request for a new SignedCommitment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentRequest {
	pub commitment_type: u64,
	pub payload: Bytes,
	pub slasher: Address,
}

impl CommitmentRequest {
	/// ABI-encodes the CommitmentRequest struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolCommitmentRequest {
				uint64 commitment_type;
				bytes payload;
				address slasher;
			}
		}

		Ok(Bytes::from(SolCommitmentRequest::abi_encode(&SolCommitmentRequest {
			commitment_type: self.commitment_type,
			payload: self.payload.clone(),
			slasher: self.slasher,
		})))
	}

	/// ABI-decodes a CommitmentRequest from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolCommitmentRequest {
				uint64 commitment_type;
				bytes payload;
				address slasher;
			}
		}

		let decoded = SolCommitmentRequest::abi_decode(data).wrap_err("Failed to decode CommitmentRequest")?;

		Ok(CommitmentRequest {
			commitment_type: decoded.commitment_type,
			payload: decoded.payload,
			slasher: decoded.slasher,
		})
	}
}

/// Core commitment data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
	pub commitment_type: u64,
	pub payload: Bytes,
	pub request_hash: B256,
	pub slasher: Address,
}

impl Commitment {
	/// ABI-encodes the Commitment struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolCommitment {
				uint64 commitment_type;
				bytes payload;
				bytes32 request_hash;
				address slasher;
			}
		}

		Ok(Bytes::from(SolCommitment::abi_encode(&SolCommitment {
			commitment_type: self.commitment_type,
			payload: self.payload.clone(),
			request_hash: self.request_hash,
			slasher: self.slasher,
		})))
	}

	/// ABI-decodes a Commitment from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolCommitment {
				uint64 commitment_type;
				bytes payload;
				bytes32 request_hash;
				address slasher;
			}
		}

		let decoded = SolCommitment::abi_decode(data).wrap_err("Failed to decode Commitment")?;

		Ok(Commitment {
			commitment_type: decoded.commitment_type,
			payload: decoded.payload,
			request_hash: decoded.request_hash,
			slasher: decoded.slasher,
		})
	}
}

/// A commitment with its ECDSA signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCommitment {
	pub commitment: Commitment,
	pub signature: Signature,
}

/// Information about offerings for a specific chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offering {
	pub chain_id: u64,
	pub commitment_types: Vec<u64>,
}

/// Information about a specific slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfo {
	pub slot: u64,
	pub offerings: Vec<Offering>,
}

/// Fee information for a commitment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeInfo {
	pub fee_payload: Bytes, // opaque fee payload
	pub commitment_type: u64,
}
