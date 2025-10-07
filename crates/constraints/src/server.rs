use axum::{
	Router,
	routing::{get, post},
};
use eyre::Result;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

use super::client::ConstraintsClient;
use super::handlers;
use commit_boost::prelude::StartCommitModuleConfig;
use common::config::InclusionPreconfConfig;
use common::constants::{defaults, routes};
use common::types::RpcContext;

/// Run the constraints REST API server
pub async fn run_constraints_server<T: Send + Sync + Clone + 'static>(
	rpc_context: RpcContext<T>,
	port: u16,
) -> Result<()> {
	let app = Router::new()
		.route(routes::constraints::PROCESS, post(handlers::process_constraints_handler))
		.route(routes::HEALTH, get(handlers::health_handler))
		.route(routes::constraints::CAPABILITIES, get(handlers::capabilities_handler))
		.with_state(rpc_context);

	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	info!("Starting constraints server on {}", addr);

	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app).await?;

	Ok(())
}

/// Setup logging for the constraints server
pub fn setup_logging(commit_config: &StartCommitModuleConfig<InclusionPreconfConfig>) -> Result<()> {
	let app_config = &commit_config.extra;
	let mut filter = tracing_subscriber::EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&app_config.log_level));

	if app_config.enable_method_tracing {
		for method in &app_config.traced_methods {
			let directive = format!("constraints[method_call{{name = \"{}\"}}]=trace", method);
			filter = filter.add_directive(directive.parse()?);
		}
	}

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;
	Ok(())
}

/// Create a constraints client for relay communication
pub fn create_constraints_client(relay_url: String, api_key: Option<String>) -> ConstraintsClient {
	ConstraintsClient::new(relay_url, api_key)
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ConstraintsServerConfig {
	pub port: u16,
	pub relay_url: String,
	pub api_key: Option<String>,
	pub timeout_seconds: u64,
}

impl Default for ConstraintsServerConfig {
	fn default() -> Self {
		Self {
			port: defaults::CONSTRAINTS_SERVER_PORT,
			relay_url: defaults::RELAY_URL.to_string(),
			api_key: None,
			timeout_seconds: defaults::TIMEOUT_SECONDS,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_server_config_default() {
		let config = ConstraintsServerConfig::default();
		assert_eq!(config.port, defaults::CONSTRAINTS_SERVER_PORT);
		assert_eq!(config.relay_url, defaults::RELAY_URL);
		assert_eq!(config.api_key, None);
		assert_eq!(config.timeout_seconds, defaults::TIMEOUT_SECONDS);
	}

	#[test]
	fn test_server_config_custom() {
		let config = ConstraintsServerConfig {
			port: 9090,
			relay_url: "https://custom-relay.com".to_string(),
			api_key: Some("custom-key".to_string()),
			timeout_seconds: 60,
		};

		assert_eq!(config.port, 9090);
		assert_eq!(config.relay_url, "https://custom-relay.com");
		assert_eq!(config.api_key, Some("custom-key".to_string()));
		assert_eq!(config.timeout_seconds, 60);
	}
}
