use alloy::primitives::{Address, B256, Bytes, Signature};
use alloy::sol_types::SolValue;
use alloy_primitives::keccak256;
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

use super::MessageType;

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

	pub fn request_hash(&self) -> Result<B256> {
		let encoded_request = self.abi_encode()?;
		Ok(alloy::primitives::keccak256(&encoded_request))
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

	pub fn to_message_hash(&self) -> Result<B256> {
		alloy::sol! {
			struct SolCommitment {
				uint64 commitment_type;
				bytes payload;
				bytes32 request_hash;
				address slasher;
			}
		}

		let commitment_evm = SolCommitment {
			commitment_type: self.commitment_type,
			payload: self.payload.clone(),
			request_hash: self.request_hash,
			slasher: self.slasher,
		};

		let message_type = MessageType::Commitment.to_uint256();
		let encoded = (message_type, commitment_evm).abi_encode_params(); // Rust equivalent of abi.encode(message_type, commitment) in Solidity
		let object_root = keccak256(encoded);
		Ok(object_root)
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

/// Fee payload for an inclusion preconf request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeePayload {
	pub request_hash: B256,
	pub price_gwei: u64,
}

impl FeePayload {
	/// ABI-encodes the FeePayload struct
	pub fn abi_encode(&self) -> Result<Bytes> {
		alloy::sol! {
			struct SolFeePayload {
				bytes32 request_hash;
				uint64 price_gwei;
			}
		}
		Ok(Bytes::from(SolFeePayload::abi_encode(&SolFeePayload {
			request_hash: self.request_hash,
			price_gwei: self.price_gwei,
		})))
	}

	/// ABI-decodes a FeePayload from bytes
	pub fn abi_decode(data: &Bytes) -> Result<Self> {
		alloy::sol! {
			struct SolFeePayload {
				bytes32 request_hash;
				uint64 price_gwei;
			}
		}

		let decoded = SolFeePayload::abi_decode(data).wrap_err("Failed to decode FeePayload")?;

		Ok(FeePayload { request_hash: decoded.request_hash, price_gwei: decoded.price_gwei })
	}
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
		assert!(!encoded_bytes.is_empty());

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
		assert!(!encoded_bytes.is_empty());

		assert_eq!(
			keccak256(&encoded).to_string(),
			"0x53c39bc1f3870634c4f9a8b63b2e4e3a914acc9da71f4d0093df91eabd9247a9"
		);
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

	#[test]
	fn test_abi_encode_fee_payload() -> Result<()> {
		let fee_payload = FeePayload { request_hash: B256::ZERO, price_gwei: 1 };
		fee_payload.abi_encode().unwrap();
		Ok(())
	}

	#[test]
	fn test_abi_decode_fee_payload() -> Result<()> {
		let original_fee_payload = FeePayload { request_hash: B256::ZERO, price_gwei: 1 };
		let encoded = original_fee_payload.abi_encode().unwrap();
		FeePayload::abi_decode(&encoded).unwrap();
		Ok(())
	}
}
