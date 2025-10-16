pub mod commitments;
pub mod constraints;
pub mod context;
pub mod database;
pub mod requests;
pub mod responses;

// Re-export all types for easy access
pub use commitments::{Commitment, CommitmentRequest, FeeInfo, SignedCommitment, SlotInfo};
pub use constraints::{
	Constraint, ConstraintCapabilities, ConstraintsMessage, Delegation, SignedConstraints, SignedDelegation,
};
pub use context::RpcContext;
pub use database::DatabaseContext;
pub use requests::{ProcessConstraintsRequest, ProcessDelegationsRequest};
pub use responses::{
	GetDelegationsResponse, HealthResponse, ProcessConstraintsResponse, ProcessDelegationsResponse, SlotInfoResponse,
};

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
	pub fn to_uint256(self) -> alloy::primitives::U256 {
		alloy::primitives::U256::from(self as u64)
	}
}
