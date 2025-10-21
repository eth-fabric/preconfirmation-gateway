pub mod client;
pub mod service;
pub mod utils;

// Re-export commonly used types and functions for easier access
pub use client::{ConstraintsClient, ConstraintsClientTrait};
pub use common::types::{ProcessConstraintsResponse, ProcessDelegationsResponse};
pub use service::{process_constraints, process_delegations};
pub use utils::{
	create_constraints_message, create_signed_constraints, parse_bls_public_key, parse_bls_public_keys,
	validate_constraints_message,
};

// Re-export mock for testing
#[cfg(test)]
pub use client::MockConstraintsClientTrait;
