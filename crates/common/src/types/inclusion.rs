use alloy::primitives::{Address, B256, Bytes, Signature};
use alloy::sol_types::SolValue;
use alloy_primitives::keccak256;
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

use super::MessageType;

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

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::keccak256;

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
}
