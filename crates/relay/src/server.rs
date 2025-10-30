use crate::metrics::metrics_handler;
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
use crate::handlers::{
	capabilities_handler, get_constraints_for_slot_handler, get_delegations_handler, health_handler,
	store_constraints_handler, store_delegation_handler, RelayState,
};
use common::constants::routes::{constraints, relay};
use common::types::database::DatabaseContext;

/// Setup logging for the relay server
pub fn setup_logging(log_level: &str) -> eyre::Result<()> {
	// Delegate to common logging setup for consistency across binaries
	common::logging::setup(log_level)
}

/// Run the relay server
pub async fn run_relay_server(config: RelayConfig, database: DatabaseContext) -> eyre::Result<()> {
	info!("Starting relay server on port {}", config.relay.port);

	// Use the provided database (already opened by the caller)
	let database = Arc::new(database);
	info!("Database initialized at {:?}", config.relay.database_path);

	// Create slot timer
	let slot_timer = common::slot_timer::SlotTimer::new(config.relay.genesis_timestamp);

	// Create shared state
	let state = RelayState { database, config: config.clone(), slot_timer };

	// Build router
	let app = Router::new()
		.route(relay::DELEGATION, post(store_delegation_handler))
		.route(relay::DELEGATIONS_SLOT, get(get_delegations_handler))
		.route(relay::CONSTRAINTS, post(store_constraints_handler))
		.route(relay::CONSTRAINTS_SLOT, get(get_constraints_for_slot_handler))
		.route(constraints::BUILDER_CAPABILITIES, get(capabilities_handler))
		.route(relay::HEALTH, get(health_handler))
		.route("/metrics", get(metrics_handler))
		.layer(
			ServiceBuilder::new()
				.layer(TraceLayer::new_for_http())
				.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)),
		)
		.with_state(state);

	// Start server
	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.relay.port)).await?;
	info!("Relay server listening on port {}", config.relay.port);

	axum::serve(listener, app)
		.with_graceful_shutdown(async {
			let _ = common::utils::wait_for_signal().await;
		})
		.await?;

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
				chain: commit_boost::prelude::Chain::Hoodi,
				genesis_timestamp: 1742213400,
				beacon_api_url: "http://localhost:5052".to_string(),
				lookahead_window: 64,
				lookahead_update_interval: 10,
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
