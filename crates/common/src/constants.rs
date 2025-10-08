use alloy::primitives::{B256, b256};

/// The signing ID for ECDSA operations
pub const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");

/// The commitment type for inclusion commitments
pub const COMMITMENT_TYPE: u64 = 1;

/// The constraint type for inclusion constraints
pub const CONSTRAINT_TYPE: u64 = 1;

/// Shared route constants for the preconfirmation gateway
pub mod routes {
	/// Health check endpoint
	pub const HEALTH: &str = "/health";

	/// Constraints-related routes
	pub mod constraints {
		/// Process constraints endpoint (for scheduler)
		pub const PROCESS: &str = "/process-constraints";

		/// Process delegations endpoint (for scheduler)
		pub const PROCESS_DELEGATIONS: &str = "/process-delegations";

		/// Constraints capabilities endpoint
		pub const CAPABILITIES: &str = "/constraints/capabilities";

		/// API version prefix for constraints
		pub const API_V0: &str = "/constraints/v0";

		/// Builder constraints endpoint
		pub const BUILDER_CONSTRAINTS: &str = "/constraints/v0/builder/constraints";

		/// Relay delegations endpoint
		pub const RELAY_DELEGATIONS: &str = "/constraints/v0/relay/delegations";
	}

	/// Commitments-related routes (for future use)
	pub mod commitments {
		/// Commitment request endpoint
		pub const REQUEST: &str = "/commitment-request";

		/// Commitment result endpoint
		pub const RESULT: &str = "/commitment-result";

		/// Fee information endpoint
		pub const FEE: &str = "/fee";

		/// Slot information endpoint
		pub const SLOTS: &str = "/slots";
	}

	/// Relay-related routes
	pub mod relay {
		/// Health check endpoint
		pub const HEALTH: &str = "/health";

		/// Store delegation endpoint
		pub const DELEGATION: &str = "/delegation";

		/// Get delegations for a specific slot
		pub const DELEGATIONS_SLOT: &str = "/delegations/:slot";

		/// Store constraints endpoint
		pub const CONSTRAINTS: &str = "/constraints";

		/// Get constraints for a specific slot
		pub const CONSTRAINTS_SLOT: &str = "/constraints/v0/relay/constraints/:slot";
	}
}

/// Default configuration values
pub mod defaults {
	/// Default constraints server port
	pub const CONSTRAINTS_SERVER_PORT: u16 = 8081;

	/// Default relay URL for testing
	pub const RELAY_URL: &str = "https://relay.example.com";

	/// Default timeout in seconds
	pub const TIMEOUT_SECONDS: u64 = 30;
}
