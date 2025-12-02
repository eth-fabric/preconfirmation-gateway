use crate::metrics::metrics_handler;
use axum::{
	body::Body,
	extract::{Request, State},
	http::StatusCode,
	response::Response,
	routing::{get, post},
	Router,
};
use cb_common::pbs::ElectraSpec;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
	cors::{Any, CorsLayer},
	trace::TraceLayer,
};
use tracing::{error, info, warn};

use crate::config::RelayConfig;
use crate::handlers::{
	capabilities_handler, get_constraints_for_slot_handler, get_delegations_handler, health_handler,
	store_constraints_handler, store_delegation_handler, submit_blocks_with_proofs_handler, RelayState,
};
use common::constants::routes::{constraints, relay};
use common::types::database::DatabaseContext;

/// Setup logging for the relay server
pub fn setup_logging(log_level: &str) -> eyre::Result<()> {
	// Delegate to common logging setup for consistency across binaries
	common::logging::setup(log_level)
}

/// Proxy handler for forwarding unmatched requests to upstream relay
async fn proxy_handler(State(state): State<RelayState>, req: Request) -> Result<Response, StatusCode> {
	// If no upstream relay is configured, return 404
	let (Some(client), Some(upstream_url)) = (&state.proxy_client, &state.upstream_relay_url) else {
		warn!("Proxy handler called but no upstream relay configured");
		return Err(StatusCode::NOT_FOUND);
	};

	let method = req.method().clone();
	let uri = req.uri().clone();
	let path = uri.path();
	let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();

	// Build upstream URL
	let upstream_full_url = format!("{}{}{}", upstream_url, path, query);
	info!("Proxying {} {} to {}", method, path, upstream_full_url);

	// Extract headers and body
	let headers = req.headers().clone();
	let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
		Ok(bytes) => bytes,
		Err(e) => {
			error!("Failed to read request body: {}", e);
			return Err(StatusCode::BAD_REQUEST);
		}
	};

	// Build upstream request
	let mut upstream_req = client.request(method.clone(), &upstream_full_url);

	// Forward headers (excluding host and connection-related headers)
	for (key, value) in headers.iter() {
		let key_str = key.as_str();
		if key_str != "host" && key_str != "connection" && !key_str.starts_with("x-forwarded") {
			if let Ok(val) = value.to_str() {
				upstream_req = upstream_req.header(key_str, val);
			}
		}
	}

	// Add X-Forwarded-For header
	// Note: In production, you'd want to extract the actual client IP
	upstream_req = upstream_req.header("X-Forwarded-For", "relay-proxy");

	// Add body if present
	if !body_bytes.is_empty() {
		upstream_req = upstream_req.body(body_bytes.to_vec());
	}

	// Send request to upstream
	let upstream_response = match upstream_req.send().await {
		Ok(resp) => resp,
		Err(e) => {
			error!("Failed to proxy request to upstream: {}", e);
			return Err(StatusCode::BAD_GATEWAY);
		}
	};

	// Build response
	let status = upstream_response.status();
	let mut response_builder = Response::builder().status(status);

	// Copy response headers
	for (key, value) in upstream_response.headers() {
		response_builder = response_builder.header(key, value);
	}

	// Get response body
	let body_bytes = match upstream_response.bytes().await {
		Ok(bytes) => bytes,
		Err(e) => {
			error!("Failed to read upstream response body: {}", e);
			return Err(StatusCode::BAD_GATEWAY);
		}
	};

	// Build final response
	match response_builder.body(Body::from(body_bytes)) {
		Ok(response) => {
			info!("Proxy response: {} for {} {}", status, method, path);
			Ok(response)
		}
		Err(e) => {
			error!("Failed to build response: {}", e);
			Err(StatusCode::INTERNAL_SERVER_ERROR)
		}
	}
}

/// Run the relay server
pub async fn run_relay_server(config: RelayConfig, database: DatabaseContext) -> eyre::Result<()> {
	info!("Starting relay server on port {}", config.relay.port);

	// Use the provided database (already opened by the caller)
	let database = Arc::new(database);
	info!("Database initialized at {:?}", config.relay.database_path);

	// Create slot timer
	let slot_timer = common::slot_timer::SlotTimer::new(config.relay.genesis_timestamp);

	// Create proxy client if upstream relay URL is configured
	let (proxy_client, upstream_relay_url) = if let Some(ref url) = config.relay.upstream_relay_url {
		info!("Configuring proxy to upstream relay: {}", url);
		let client = reqwest::Client::builder()
			.timeout(std::time::Duration::from_secs(30))
			.build()
			.map_err(|e| eyre::eyre!("Failed to create proxy client: {}", e))?;
		(Some(client), Some(url.clone()))
	} else {
		info!("No upstream relay configured, proxy disabled");
		(None, None)
	};

	// Create shared state
	let state = RelayState { database, config: config.clone(), slot_timer, proxy_client, upstream_relay_url };

	// Build router
	let app = Router::new()
		.route(relay::DELEGATION, post(store_delegation_handler))
		.route(relay::DELEGATIONS_SLOT, get(get_delegations_handler))
		.route(relay::CONSTRAINTS, post(store_constraints_handler))
		.route(relay::CONSTRAINTS_SLOT, get(get_constraints_for_slot_handler))
		.route(relay::BLOCKS_WITH_PROOFS, post(submit_blocks_with_proofs_handler))
		.route(constraints::BUILDER_CAPABILITIES, get(capabilities_handler))
		.route(relay::HEALTH, get(health_handler))
		.route("/metrics", get(metrics_handler))
		.fallback(proxy_handler)
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
				upstream_relay_url: None,
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
