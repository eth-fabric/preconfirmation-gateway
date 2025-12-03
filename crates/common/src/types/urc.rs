use alloy::primitives::{Address, B256, Bytes, keccak256};
use alloy::sol_types::{SolCall, SolValue};
use commit_boost::prelude::{BlsPublicKey, BlsSignature};
use eyre::Result;

use urc::registry::IRegistry::SignedRegistration as SolSignedRegistration;
use urc::registry::Registry::registerCall as SolRegisterCall;

use super::{MessageType, convert_pubkey_to_g1_point, convert_signature_to_g2_point};

/// URC registration message
pub struct Registration {
	pub owner: Address,
}

impl Registration {
	/// Generate the object root to be signed for a registration.
	pub fn to_message_hash(&self) -> Result<B256> {
		let message_type = MessageType::Registration.to_uint256();
		let encoded = (message_type, self.owner).abi_encode_params();
		let message_hash = keccak256(encoded);
		Ok(message_hash)
	}
}

/// Signed registration used for URC.register
pub struct SignedRegistration {
	pub pubkey: BlsPublicKey,
	pub signature: BlsSignature,
	pub nonce: u64,
}

impl SignedRegistration {
	pub fn as_sol_type(&self) -> Result<SolSignedRegistration> {
		let pubkey = convert_pubkey_to_g1_point(&self.pubkey)?;
		let signature = convert_signature_to_g2_point(&self.signature)?;
		let registration = SolSignedRegistration { pubkey, signature, nonce: self.nonce };
		Ok(registration)
	}
}

/// Container for URC register() call parameters
pub struct URCRegisterInputs {
	pub registrations: Vec<SignedRegistration>,
	pub owner: Address,
	pub signing_id: B256,
}

impl URCRegisterInputs {
	/// ABI encode for URC register() call
	/// Signature: register(SignedRegistration[] calldata registrations, address owner, bytes32 signingId)
	pub fn abi_encode_with_selector(&self) -> Result<Bytes> {
		let sol_registrations = self.registrations.iter().map(|r| r.as_sol_type()).collect::<Result<Vec<_>, _>>()?;

		let register_call =
			SolRegisterCall { registrations: sol_registrations, owner: self.owner, signingId: self.signing_id };

		let encoded = register_call.abi_encode();
		Ok(Bytes::from(encoded))
	}
}
