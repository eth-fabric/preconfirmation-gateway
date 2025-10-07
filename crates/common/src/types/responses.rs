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
