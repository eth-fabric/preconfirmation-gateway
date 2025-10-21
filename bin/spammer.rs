use common::constants::COMMITMENT_TYPE;
use common::types::CommitmentRequest;
use common::types::beacon::BeaconTiming;
use common::types::commitments::InclusionPayload;
use eyre::{Result, WrapErr};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use serde::Deserialize;
use std::env;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use alloy::consensus::{SignableTransaction, Signed, TxEip1559, TxEnvelope};
use alloy::primitives::{Address, Bytes, TxKind, U256};
use alloy::rlp::Encodable;
use alloy::signers::{SignerSync, local::PrivateKeySigner};

/// Configuration for the spammer
#[derive(Debug, Deserialize)]
struct SpammerConfig {
	/// Mode: "one-shot" or "continuous"
	mode: String,
	/// Gateway RPC endpoint
	gateway_url: String,
	/// Interval between requests in seconds (only used in continuous mode)
	interval_secs: u64,
	/// Sender private key (must have ETH balance for test transactions)
	sender_private_key: String,
	/// Slasher contract address (optional, random if not provided)
	slasher_address: Option<String>,
	/// Chain ID for transactions
	chain_id: u64,
	/// Genesis timestamp for slot calculation
	genesis_timestamp: u64,
	/// Transaction parameters
	transaction: TransactionConfig,
}

/// Transaction configuration
#[derive(Debug, Deserialize)]
struct TransactionConfig {
	gas_limit: u64,
	max_fee_per_gas: u128,
	max_priority_fee_per_gas: u128,
	value_wei: u128,
}

/// Generate a valid signed transaction
fn generate_signed_transaction(config: &SpammerConfig, signer: &PrivateKeySigner, nonce: u64) -> Result<Bytes> {
	// Create EIP-1559 transaction with random recipient
	let tx = TxEip1559 {
		chain_id: config.chain_id,
		nonce,
		gas_limit: config.transaction.gas_limit,
		max_fee_per_gas: config.transaction.max_fee_per_gas,
		max_priority_fee_per_gas: config.transaction.max_priority_fee_per_gas,
		to: TxKind::Call(Address::random()), // Random recipient address
		value: U256::from(config.transaction.value_wei),
		input: Bytes::new(),
		access_list: Default::default(),
	};

	// Sign the transaction
	let encoded_tx = tx.encoded_for_signing();
	let signature = signer.sign_message_sync(&encoded_tx).wrap_err("Failed to sign transaction")?;

	// Create signed transaction envelope
	let signed_tx = Signed::new_unhashed(tx, signature);
	let tx_envelope = TxEnvelope::Eip1559(signed_tx);

	// RLP encode
	let mut encoded = Vec::new();
	tx_envelope.encode(&mut encoded);

	Ok(Bytes::from(encoded))
}

/// Create a commitment request
fn create_commitment_request(config: &SpammerConfig, signed_tx: Bytes) -> Result<CommitmentRequest> {
	// Get current slot
	let current_slot = BeaconTiming::current_slot_estimate(config.genesis_timestamp);

	// Create inclusion payload
	let inclusion_payload = InclusionPayload { slot: current_slot, signed_tx };

	// ABI encode the payload
	let payload = inclusion_payload.abi_encode().wrap_err("Failed to encode inclusion payload")?;

	// Parse or generate slasher address
	let slasher = if let Some(addr_str) = &config.slasher_address {
		addr_str.parse::<Address>().wrap_err("Failed to parse slasher address")?
	} else {
		Address::random()
	};

	Ok(CommitmentRequest { commitment_type: COMMITMENT_TYPE, payload, slasher })
}

/// Send a commitment request via RPC
async fn send_commitment_request(
	gateway_url: &str,
	request: &CommitmentRequest,
) -> Result<common::types::SignedCommitment> {
	let client = HttpClientBuilder::default().build(gateway_url).wrap_err("Failed to build HTTP client")?;

	let result: common::types::SignedCommitment =
		client.request("commitmentRequest", vec![request]).await.wrap_err("RPC call failed")?;

	Ok(result)
}

/// Run in one-shot mode
async fn run_one_shot(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<()> {
	info!("Running in one-shot mode");

	// Generate transaction with nonce 0
	let signed_tx = generate_signed_transaction(config, signer, 0)?;
	info!("Generated signed transaction ({} bytes)", signed_tx.len());

	// Create commitment request
	let request = create_commitment_request(config, signed_tx)?;
	let request_hash = request.request_hash().wrap_err("Failed to compute request hash")?;
	info!("Created commitment request with hash: {:?}", request_hash);

	// Send request
	info!("Sending commitment request to {}", config.gateway_url);
	let response = send_commitment_request(&config.gateway_url, &request).await?;

	info!("✓ Commitment request successful!");
	info!("  Request hash: {:?}", response.commitment.request_hash);
	info!("  Commitment type: {}", response.commitment.commitment_type);
	info!("  Slasher: {:?}", response.commitment.slasher);

	Ok(())
}

/// Run in continuous mode
async fn run_continuous(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<()> {
	info!("Running in continuous mode (interval: {}s)", config.interval_secs);

	let mut interval = time::interval(Duration::from_secs(config.interval_secs));
	let mut nonce = 0u64;

	loop {
		interval.tick().await;

		info!("--- Sending commitment request #{} ---", nonce + 1);

		// Generate transaction
		match generate_signed_transaction(config, signer, nonce) {
			Ok(signed_tx) => {
				info!("Generated signed transaction ({} bytes)", signed_tx.len());

				// Create commitment request
				match create_commitment_request(config, signed_tx) {
					Ok(request) => {
						match request.request_hash() {
							Ok(request_hash) => {
								info!("Request hash: {:?}", request_hash);

								// Send request
								match send_commitment_request(&config.gateway_url, &request).await {
									Ok(response) => {
										info!("✓ Commitment request successful!");
										info!("  Commitment hash: {:?}", response.commitment.request_hash);
										nonce += 1;
									}
									Err(e) => {
										error!("✗ Failed to send commitment request: {}", e);
									}
								}
							}
							Err(e) => {
								error!("✗ Failed to compute request hash: {}", e);
							}
						}
					}
					Err(e) => {
						error!("✗ Failed to create commitment request: {}", e);
					}
				}
			}
			Err(e) => {
				error!("✗ Failed to generate signed transaction: {}", e);
			}
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt()
		.with_env_filter(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
		)
		.init();

	// Get config file path from args or use default
	let args: Vec<String> = env::args().collect();
	let config_path = if args.len() > 1 { &args[1] } else { "spammer-config.toml" };

	info!("Loading configuration from: {}", config_path);

	// Load configuration
	let config_content = std::fs::read_to_string(config_path).wrap_err("Failed to read config file")?;
	let config: SpammerConfig = toml::from_str(&config_content).wrap_err("Failed to parse config file")?;

	info!("Configuration loaded successfully");
	info!("  Mode: {}", config.mode);
	info!("  Gateway URL: {}", config.gateway_url);
	info!("  Chain ID: {}", config.chain_id);
	info!("  Genesis timestamp: {}", config.genesis_timestamp);

	// Parse sender private key
	let signer =
		config.sender_private_key.parse::<PrivateKeySigner>().wrap_err("Failed to parse sender private key")?;
	let sender_address = signer.address();
	info!("Sender address: {:?}", sender_address);

	// Run based on mode
	match config.mode.as_str() {
		"one-shot" => run_one_shot(&config, &signer).await?,
		"continuous" => run_continuous(&config, &signer).await?,
		_ => {
			return Err(eyre::eyre!("Invalid mode '{}'. Must be 'one-shot' or 'continuous'", config.mode));
		}
	}

	Ok(())
}
