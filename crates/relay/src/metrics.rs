use axum::response::IntoResponse;
use lazy_static::lazy_static;
use prometheus::{
	register_histogram_vec_with_registry, register_int_counter_vec_with_registry, Encoder, HistogramVec, IntCounterVec,
	Registry, TextEncoder,
};

pub const REGISTRY_NAME: &str = "inclusion-preconf-relay";

lazy_static! {
	pub static ref RELAY_METRICS_REGISTRY: Registry =
		Registry::new_custom(Some(REGISTRY_NAME.to_string()), None).unwrap();
	pub static ref REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"http_requests_total",
		"Total number of HTTP requests",
		&["endpoint", "method"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();
	pub static ref RESPONSES_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"http_responses_total",
		"Total number of HTTP responses by status",
		&["endpoint", "method", "status"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();
	pub static ref REQUEST_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"http_request_duration_seconds",
		"Request latency in seconds",
		&["endpoint", "method"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();

	// Proposer lookahead task metrics
	pub static ref PROPOSER_LOOKAHEAD_UPDATES_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"proposer_lookahead_updates_total",
		"Total number of proposer lookahead updates by result",
		&["result"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref PROPOSER_LOOKAHEAD_UPDATE_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"proposer_lookahead_update_latency_seconds",
		"Latency of proposer lookahead updates in seconds",
		&["result"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref PROPOSER_DUTIES_FETCHED_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"proposer_duties_fetched_total",
		"Total number of proposer duties fetched by result",
		&["result"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref PROPOSER_LOOKAHEAD_DB_OPERATIONS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"proposer_lookahead_db_operations_total",
		"Total database operations for proposer lookahead by operation type and result",
		&["operation", "result"],
		RELAY_METRICS_REGISTRY
	)
	.unwrap();
}

pub async fn metrics_handler() -> axum::response::Response {
	let metric_families = RELAY_METRICS_REGISTRY.gather();
	let mut buffer = Vec::new();
	let encoder = TextEncoder::new();
	if encoder.encode(&metric_families, &mut buffer).is_err() {
		return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
	}
	axum::response::Response::builder()
		.status(axum::http::StatusCode::OK)
		.header(axum::http::header::CONTENT_TYPE, encoder.format_type())
		.body(axum::body::Body::from(buffer))
		.unwrap()
}
