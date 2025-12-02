/// The commitment type for inclusion commitments
pub const INCLUSION_COMMITMENT_TYPE: u64 = 1;

/// The constraint type for inclusion constraints
pub const INCLUSION_CONSTRAINT_TYPE: u64 = 1;

/// Maximum number of constraints per slot
pub const MAX_CONSTRAINTS_PER_SLOT: usize = 256;

// Commit-Boost signing constants
pub const APPLICATION_BUILDER_DOMAIN: [u8; 4] = [0, 0, 0, 1];
pub const GENESIS_VALIDATORS_ROOT: [u8; 32] = [0; 32];
pub const COMMIT_BOOST_DOMAIN: [u8; 4] = [109, 109, 111, 67];
pub const COMMIT_BOOST_VERSION: &str = env!("CARGO_PKG_VERSION");
// Prefer option_env! to avoid unexpected cfg values
pub const COMMIT_BOOST_COMMIT: &str = match option_env!("GIT_HASH") {
	Some(v) => v,
	None => "unknown",
};
pub const SIGNER_JWT_EXPIRATION: u64 = 300; // 5 minutes

/// Shared route constants for the preconfirmation gateway
pub mod routes {
	/// Health check endpoint
	pub const HEALTH: &str = "/health";

	/// Constraints-related routes (for external API use)
	pub mod constraints {
		/// Constraints capabilities endpoint
		pub const CAPABILITIES: &str = "/constraints/capabilities";

		/// API version prefix for constraints
		pub const API_V0: &str = "/constraints/v0";

		/// Builder constraints endpoint
		pub const BUILDER_CONSTRAINTS: &str = "/constraints/v0/builder/constraints";

		/// Builder capabilities endpoint
		pub const BUILDER_CAPABILITIES: &str = "/constraints/v0/builder/capabilities";

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
		pub const DELEGATIONS_SLOT: &str = "/delegations/{slot}";

		/// Store constraints endpoint
		pub const CONSTRAINTS: &str = "/constraints";

		/// Get constraints for a specific slot
		pub const CONSTRAINTS_SLOT: &str = "/constraints/v0/relay/constraints/{slot}";

		/// Submit block with proofs endpoint
		pub const BLOCKS_WITH_PROOFS: &str = "/constraints/v0/relay/blocks_with_proofs";

		/// Upstream builder API submit block endpoint
		pub const UPSTREAM_BUILDER_SUBMIT_BLOCK: &str = "/eth/v1/builder/blocks";
	}

	/// Beacon API routes
	pub mod beacon {
		/// Proposer duties endpoint prefix
		pub const PROPOSER_DUTIES: &str = "eth/v1/validator/duties/proposer";
	}
}

/// Default configuration values
pub mod defaults {
	/// Default relay URL for testing
	pub const RELAY_URL: &str = "https://relay.example.com";

	/// Default timeout in seconds
	pub const TIMEOUT_SECONDS: u64 = 30;
}
