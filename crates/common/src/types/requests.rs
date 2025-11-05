use serde::{Deserialize, Serialize};

/// Request for processing constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConstraintsRequest {
	pub slot: u64,
	pub bls_public_key: String,
	pub proposer_public_key: String,
	pub receivers: Vec<String>,
}

/// Request for processing delegations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDelegationsRequest {
	pub slot: u64,
	pub delegate_bls_public_key: String,
}
