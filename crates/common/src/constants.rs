use alloy::primitives::{B256, b256};
/// The signing ID for ECDSA operations
pub const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");

/// The commitment type for inclusion commitments
pub const COMMITMENT_TYPE: u64 = 1;
