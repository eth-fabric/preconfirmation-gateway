pub mod config;
pub mod handlers;
pub mod server;
pub mod utils;

// Re-export commonly used types and functions for easier access
pub use config::RelayConfig;
pub use server::{run_relay_server, setup_logging};
