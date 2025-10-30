use axum::{Json, Router, extract::Path, routing::get};
use clap::Parser;
use common::types::beacon::{BeaconTiming, ProposerDutiesResponse, ValidatorDuty};
use eyre::Result;
use tracing::info;

/// Mock beacon node that returns alternating proposer duties
#[derive(Parser, Debug)]
#[command(name = "beacon-mock")]
#[command(about = "Mock beacon node for testing")]
struct Args {
	/// Proposer BLS public key (hex format with 0x prefix)
	proposer_bls_key: String,
}

/// Handler for proposer duties endpoint
async fn get_proposer_duties_handler(
	Path(epoch): Path<u64>,
	axum::extract::State(proposer_key): axum::extract::State<String>,
) -> Json<ProposerDutiesResponse> {
	info!("Getting proposer duties for epoch {}", epoch);
	// Calculate slot range for epoch (32 slots per epoch)
	let start_slot = BeaconTiming::epoch_to_first_slot(epoch);
	let end_slot = BeaconTiming::epoch_to_last_slot(epoch);

	// Generate alternating duties
	// Even slots: use provided proposer key
	// Odd slots: use default zero key
	let duties: Vec<ValidatorDuty> = (start_slot..=end_slot)
		.map(|slot| {
			let is_even = slot % 2 == 0;
			let pubkey = if is_even {
				proposer_key.clone()
			} else {
				// Default zero BLS public key (96 hex chars = 48 bytes)
				"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
					.to_string()
			};

			ValidatorDuty { validator_index: slot.to_string(), pubkey, slot: slot.to_string() }
		})
		.collect();

	Json(ProposerDutiesResponse { execution_optimistic: false, finalized: true, data: duties })
}

#[tokio::main]
async fn main() -> Result<()> {
	common::logging::setup("info".into())?;
	// Parse command line arguments
	let args = Args::parse();

	info!("Mock Beacon Node Server");
	info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	info!("Listening on: http://0.0.0.0:5052");
	info!("Proposer key: {}", args.proposer_bls_key);
	info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	info!("Endpoint: GET /eth/v1/validator/duties/proposer/{{epoch}}");
	info!("Pattern: Even slots = proposer key, Odd slots = zero key");

	// Build router with proposer key as shared state
	let app = Router::new()
		.route("/eth/v1/validator/duties/proposer/{epoch}", get(get_proposer_duties_handler))
		.with_state(args.proposer_bls_key);

	// Bind to port 5052 (standard beacon node port)
	let listener = tokio::net::TcpListener::bind("0.0.0.0:5052").await?;

	info!("✓ Server ready");

	// Start server
	axum::serve(listener, app).await?;

	Ok(())
}
