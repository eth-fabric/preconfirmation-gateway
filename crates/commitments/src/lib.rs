pub mod handlers;
pub mod methods;
pub mod server;
pub mod state;
pub mod utils;

// Re-export commonly used functions for easier access
pub use handlers::{commitment_request_handler, commitment_result_handler, fee_handler, slots_handler};
pub use methods::setup_commitment_methods;
pub use server::run_server;
pub use state::CommitmentsServerState;
