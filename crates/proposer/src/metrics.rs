use axum::response::IntoResponse;
use lazy_static::lazy_static;
use prometheus::{
	register_histogram_vec_with_registry, register_int_counter_vec_with_registry, Encoder, HistogramVec, IntCounterVec,
	Registry, TextEncoder,
};

pub const REGISTRY_NAME: &str = "inclusion-preconf-proposer";

lazy_static! {
	pub static ref PROPOSER_METRICS_REGISTRY: Registry =
		Registry::new_custom(Some(REGISTRY_NAME.to_string()), None).unwrap();
	pub static ref TASK_RUNS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"task_runs_total",
		"Total number of task executions",
		&["task", "result"],
		PROPOSER_METRICS_REGISTRY
	)
	.unwrap();
	pub static ref TASK_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"task_duration_seconds",
		"Task execution latency in seconds",
		&["task"],
		PROPOSER_METRICS_REGISTRY
	)
	.unwrap();
}

pub async fn metrics_handler() -> axum::response::Response {
	let metric_families = PROPOSER_METRICS_REGISTRY.gather();
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
