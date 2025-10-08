use serde::{Deserialize, Serialize};

use crate::types::commitments::SlotInfo;
use crate::types::constraints::SignedConstraints;
use crate::types::constraints::SignedDelegation;

/// Response containing slot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfoResponse {
	pub slots: Vec<SlotInfo>,
}

/// Response for processing constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConstraintsResponse {
	pub success: bool,
	pub slot: u64,
	pub processed_count: usize,
	pub signed_constraints: Option<SignedConstraints>,
	pub message: String,
}

/// Response for processing delegations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDelegationsResponse {
	pub success: bool,
	pub slot: u64,
	pub total_delegations: usize,
	pub matching_delegations: Vec<SignedDelegation>,
	pub message: String,
}

/// Response for health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
	pub status: String,
	pub timestamp: u64,
}

/// Response for posting a delegation
#[derive(Debug, Serialize)]
pub struct PostDelegationResponse {
	pub success: bool,
	pub message: String,
}

/// Response for getting delegations
#[derive(Debug, Serialize)]
pub struct GetDelegationsResponse {
	pub delegations: Vec<SignedDelegation>,
}

/// Response for posting constraints
#[derive(Debug, Serialize)]
pub struct PostConstraintsResponse {
	pub success: bool,
	pub message: String,
}
