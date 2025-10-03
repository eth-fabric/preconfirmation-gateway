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
	pub nonce: u64,
	pub signing_id: B256,
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

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::keccak256;

	#[test]
	fn test_abi_encode_commitment_request() -> Result<()> {
		let commitment_request =
			CommitmentRequest { commitment_type: 1, payload: Bytes::new(), slasher: Address::ZERO };

		let encoded = commitment_request.abi_encode()?;

		// Verify the encoded data is not empty
		assert!(!encoded.is_empty());

		// Verify the structure contains our data
		let encoded_bytes = encoded.as_ref();
		assert!(encoded_bytes.len() > 0);

		assert_eq!(
			keccak256(&encoded).to_string(),
			"0xf61a6130b6ebfffcb3738e03fe820e4b883b623ec3ab7657ffbf385b2e94edba"
		);
		Ok(())
	}

	#[test]
	fn test_abi_encode_commitment() -> Result<()> {
		let commitment =
			Commitment { commitment_type: 1, payload: Bytes::new(), request_hash: B256::ZERO, slasher: Address::ZERO };

		let encoded = commitment.abi_encode()?;

		// Verify the encoded data is not empty
		assert!(!encoded.is_empty());

		// Verify the structure contains our data
		let encoded_bytes = encoded.as_ref();
		assert!(encoded_bytes.len() > 0);

		assert_eq!(
			keccak256(&encoded).to_string(),
			"0x53c39bc1f3870634c4f9a8b63b2e4e3a914acc9da71f4d0093df91eabd9247a9"
		);
		Ok(())
	}

	#[test]
	fn test_abi_encode_inclusion_payload() -> Result<()> {
		let payload = InclusionPayload { slot: 12345, signed_tx: Bytes::new() };

		let encoded = payload.abi_encode()?;

		// Verify the encoded data is not empty
		assert!(!encoded.is_empty());

		assert_eq!(
			keccak256(&encoded).to_string(),
			"0x0ccee42883ae08bce4f8725891b2e9cc65a260876b55b23d4b7d94d60fad8211"
		);
		Ok(())
	}

	#[test]
	fn test_abi_decode_inclusion_payload() -> Result<()> {
		let original_payload =
			InclusionPayload { slot: 67890, signed_tx: Bytes::from(vec![0x05, 0x06, 0x07, 0x08, 0x09]) };

		// Encode first
		let encoded = original_payload.abi_encode()?;

		// Then decode
		let decoded = InclusionPayload::abi_decode(&encoded)?;

		// Verify they match
		assert_eq!(decoded.slot, original_payload.slot);
		assert_eq!(decoded.signed_tx, original_payload.signed_tx);

		println!("Original: {:?}", original_payload);
		println!("Decoded: {:?}", decoded);
		Ok(())
	}

	#[test]
	fn test_abi_decode_commitment_request() -> Result<()> {
		let original_request = CommitmentRequest {
			commitment_type: 2,
			payload: Bytes::from(vec![0xaa, 0xbb, 0xcc]),
			slasher: "0x1234567890123456789012345678901234567890".parse()?,
		};

		// Encode first
		let encoded = original_request.abi_encode()?;

		// Then decode
		let decoded = CommitmentRequest::abi_decode(&encoded)?;

		// Verify they match
		assert_eq!(decoded.commitment_type, original_request.commitment_type);
		assert_eq!(decoded.payload, original_request.payload);
		assert_eq!(decoded.slasher, original_request.slasher);

		println!("Original: {:?}", original_request);
		println!("Decoded: {:?}", decoded);
		Ok(())
	}

	#[test]
	fn test_abi_decode_commitment() -> Result<()> {
		let original_commitment = Commitment {
			commitment_type: 3,
			payload: Bytes::from(vec![0xdd, 0xee, 0xff]),
			request_hash: B256::from_slice(&[0x11; 32]),
			slasher: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".parse()?,
		};

		// Encode first
		let encoded = original_commitment.abi_encode()?;

		// Then decode
		let decoded = Commitment::abi_decode(&encoded)?;

		// Verify they match
		assert_eq!(decoded.commitment_type, original_commitment.commitment_type);
		assert_eq!(decoded.payload, original_commitment.payload);
		assert_eq!(decoded.request_hash, original_commitment.request_hash);
		assert_eq!(decoded.slasher, original_commitment.slasher);

		println!("Original: {:?}", original_commitment);
		println!("Decoded: {:?}", decoded);
		Ok(())
	}
}
