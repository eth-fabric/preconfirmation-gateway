use lazy_static::lazy_static;
use prometheus::{
	HistogramVec, IntCounterVec, Registry, register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
};

pub const REGISTRY_NAME: &str = "constraints-client";

lazy_static! {
	pub static ref CONSTRAINTS_CLIENT_REGISTRY: Registry =
		Registry::new_custom(Some(REGISTRY_NAME.to_string()), None).unwrap();

	// Constraints client metrics (for HTTP calls to relay)
	pub static ref CONSTRAINTS_CLIENT_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"constraints_client_requests_total",
		"Total HTTP requests to relay by endpoint and method",
		&["endpoint", "method"],
		CONSTRAINTS_CLIENT_REGISTRY
	)
	.unwrap();

	pub static ref CONSTRAINTS_CLIENT_RESPONSES_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"constraints_client_responses_total",
		"Total HTTP responses from relay by endpoint, method, and status",
		&["endpoint", "method", "status"],
		CONSTRAINTS_CLIENT_REGISTRY
	)
	.unwrap();

	pub static ref CONSTRAINTS_CLIENT_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"constraints_client_latency_seconds",
		"HTTP request latency to relay in seconds by endpoint and method",
		&["endpoint", "method"],
		CONSTRAINTS_CLIENT_REGISTRY
	)
	.unwrap();
}
