use axum::{
	routing::{get, post},
	Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
	cors::{Any, CorsLayer},
	trace::TraceLayer,
};
use tracing::info;

use crate::config::RelayConfig;
use crate::database::RelayDatabase;
use crate::handlers::{
	capabilities_handler, get_constraints_for_slot_handler, get_delegations_handler, health_handler,
	store_constraints_handler, store_delegation_handler, RelayState,
};
use common::constants::routes::{constraints, relay};

/// Setup logging for the relay server
pub fn setup_logging(log_level: &str) -> eyre::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

	tracing_subscriber::fmt()
		.with_env_filter(filter)
		.with_target(false)
		.with_thread_ids(true)
		.with_thread_names(true)
		.init();

	Ok(())
}

/// Run the relay server
pub async fn run_relay_server(config: RelayConfig) -> eyre::Result<()> {
	info!("Starting relay server on port {}", config.relay.port);

	// Create database
	let database = Arc::new(RelayDatabase::new(&config.relay.database_path)?);
	info!("Database initialized at {:?}", config.relay.database_path);

	// Create shared state
	let state = RelayState { database, config: config.clone() };

	// Build router
	let app = Router::new()
		.route(relay::DELEGATION, post(store_delegation_handler))
		.route(relay::DELEGATIONS_SLOT, get(get_delegations_handler))
		.route(relay::CONSTRAINTS, post(store_constraints_handler))
		.route(relay::CONSTRAINTS_SLOT, get(get_constraints_for_slot_handler))
		.route(constraints::BUILDER_CAPABILITIES, get(capabilities_handler))
		.route(relay::HEALTH, get(health_handler))
		.layer(
			ServiceBuilder::new()
				.layer(TraceLayer::new_for_http())
				.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)),
		)
		.with_state(state);

	// Start server
	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.relay.port)).await?;
	info!("Relay server listening on port {}", config.relay.port);

	axum::serve(listener, app).await?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::TempDir;

	fn create_test_config() -> RelayConfig {
		let temp_dir = TempDir::new().unwrap();
		RelayConfig {
			relay: crate::config::RelayServerConfig {
				port: 0, // Use 0 for random port in tests
				database_path: temp_dir.path().join("test_db"),
				log_level: "debug".to_string(),
				constraint_capabilities: vec![1],
			},
			storage: crate::config::StorageConfig { max_delegations_per_slot: 100, max_constraints_per_slot: 1000 },
		}
	}

	#[test]
	fn test_setup_logging() {
		let result = setup_logging("debug");
		assert!(result.is_ok());
	}

	#[test]
	fn test_config_creation() {
		let config = create_test_config();
		assert_eq!(config.relay.log_level, "debug");
		assert_eq!(config.storage.max_delegations_per_slot, 100);
	}
}
