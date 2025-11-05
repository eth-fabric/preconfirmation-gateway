use axum::response::IntoResponse;
use lazy_static::lazy_static;
use prometheus::{
	register_histogram_vec_with_registry, register_int_counter_vec_with_registry, Encoder, HistogramVec, IntCounterVec,
	Registry, TextEncoder,
};

pub const REGISTRY_NAME: &str = "inclusion-preconf-gateway";

lazy_static! {
	pub static ref GATEWAY_METRICS_REGISTRY: Registry =
		Registry::new_custom(Some(REGISTRY_NAME.to_string()), None).unwrap();

	// Delegation task metrics
	pub static ref DELEGATION_CHECKS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"delegation_checks_total",
		"Total number of delegation checks by result",
		&["result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref DELEGATION_CHECK_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"delegation_check_latency_seconds",
		"Latency of delegation checks in seconds",
		&["result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref DELEGATIONS_STORED_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"delegations_stored_total",
		"Total number of delegations stored by result",
		&["result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref DELEGATION_DB_OPERATIONS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"delegation_db_operations_total",
		"Total database operations for delegations by operation type and result",
		&["operation", "result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();

	// Constraints task metrics
	pub static ref CONSTRAINTS_POSTS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"constraints_posts_total",
		"Total number of constraint posts by result",
		&["result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref CONSTRAINTS_POST_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"constraints_post_latency_seconds",
		"Latency of constraint posts in seconds",
		&["result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();

	pub static ref CONSTRAINTS_DB_OPERATIONS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"constraints_db_operations_total",
		"Total database operations for constraints by operation type and result",
		&["operation", "result"],
		GATEWAY_METRICS_REGISTRY
	)
	.unwrap();
}

pub async fn metrics_handler() -> axum::response::Response {
	// Gather metrics from both the gateway registry and the constraints client registry
	let mut metric_families = GATEWAY_METRICS_REGISTRY.gather();
	let constraints_metrics = constraints::metrics::CONSTRAINTS_CLIENT_REGISTRY.gather();
	metric_families.extend(constraints_metrics);

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
