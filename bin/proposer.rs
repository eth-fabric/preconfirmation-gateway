use alloy::primitives::{Address, B256, U256};
use clap::{Parser, Subcommand};
use commit_boost::prelude::*;
use common::beacon::BeaconApiClient;
use common::config::BeaconApiConfig;
use common::slot_timer::SlotTimer;
use constraints::ConstraintsClient;
use eyre::{Context, Result};
use proposer::{ProposerConfig, process_lookahead, send_registration_transaction, sign_registrations};
use tokio::time::{Duration, interval};
use tracing::{debug, error, info};

#[derive(Parser)]
#[command(name = "proposer")]
#[command(about = "Proposer service for preconfirmations", long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Run the proposer service daemon
	Run,

	/// Register BLS keys with the URC contract
	Register {
		/// URC contract address
		#[arg(long)]
		urc_address: Address,

		/// Path to keystore file
		#[arg(long)]
		keystore: String,

		/// Keystore password (will prompt if not provided)
		#[arg(long)]
		password: Option<String>,

		/// Dry run - sign registrations but don't send transaction
		#[arg(long)]
		dry_run: bool,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt::init();

	let cli = Cli::parse();

	match cli.command.unwrap_or(Commands::Run) {
		Commands::Run => run_daemon().await,
		Commands::Register { urc_address, keystore, password, dry_run } => {
			handle_register_command(urc_address, keystore, password, dry_run).await
		}
	}
}

async fn run_daemon() -> Result<()> {
	info!("Starting proposer service");

	// Load configuration using commit-boost's config loader
	let mut commit_config = load_commit_module_config::<ProposerConfig>()
		.map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

	let config = commit_config.extra.clone();

	info!("Proposer configuration:");
	info!("  Delegate BLS key: {}", config.delegate_bls_public_key);
	info!("  Committer address: {}", config.committer_address);
	info!("  Relay URL: {}", config.relay_url);
	info!("  Beacon API URL: {}", config.beacon_api_url);
	info!("  Poll interval: {} seconds", config.poll_interval_seconds);

	// Create beacon API client
	let beacon_config = BeaconApiConfig {
		primary_endpoint: config.beacon_api_url.clone(),
		fallback_endpoints: vec![],
		request_timeout_secs: 30,
		genesis_time: config.beacon_genesis_timestamp,
	};

	let beacon_client = BeaconApiClient::with_default_client(beacon_config)
		.map_err(|e| eyre::eyre!("Failed to create beacon API client: {}", e))?;

	info!("Beacon API client initialized");

	// Create constraints client for posting delegations to relay
	let constraints_client = ConstraintsClient::new(config.relay_url.clone(), config.relay_api_key.clone());

	// Test relay connectivity
	match constraints_client.health_check().await {
		Ok(true) => info!("Relay health check passed"),
		Ok(false) => error!("Relay health check failed"),
		Err(e) => error!("Failed to check relay health: {}", e),
	}

	// Create slot timer for tracking current slot
	let slot_timer = SlotTimer::new(config.beacon_genesis_timestamp);

	info!("Starting proposer duty polling loop");

	// Set up polling interval
	let mut poll_interval = interval(Duration::from_secs(config.poll_interval_seconds));

	loop {
		poll_interval.tick().await;

		let current_slot = slot_timer.get_current_slot();
		debug!("Checking proposer duties for current slot: {}", current_slot);

		// Process lookahead to find and post delegations
		match process_lookahead(&beacon_client, &constraints_client, &mut commit_config, current_slot).await {
			Ok(_) => {
				debug!("Lookahead processed successfully for slot {}", current_slot);
			}
			Err(e) => {
				error!("Error processing lookahead for slot {}: {}", current_slot, e);
			}
		}
	}
}

async fn handle_register_command(
	urc_address: Address,
	keystore_path: String,
	password: Option<String>,
	dry_run: bool,
) -> Result<()> {
	info!("Starting URC registration process");

	// Load configuration
	let mut commit_config =
		load_commit_module_config::<ProposerConfig>().context("Failed to load commit module config")?;

	// Clone config fields we'll need later to avoid borrow checker issues
	let owner: Address = commit_config.extra.urc_owner.parse().context("Invalid URC owner address")?;
	let module_signing_id: B256 = commit_config.extra.module_signing_id.parse().context("Invalid module signing ID")?;
	let execution_rpc_url = commit_config.extra.execution_rpc_url.clone();
	let collateral: U256 =
		commit_config.extra.registration_collateral_wei.parse().context("Invalid collateral amount")?;

	info!("URC Registration Parameters:");
	info!("  URC Contract: {:?}", urc_address);
	info!("  Owner: {:?}", owner);
	info!("  Execution RPC: {}", execution_rpc_url);
	info!("  Collateral: {} wei", collateral);

	// Sign registrations for all keys
	info!("Signing registrations for all consensus keys...");
	let inputs = sign_registrations(&mut commit_config, owner, &module_signing_id)
		.await
		.context("Failed to sign registrations")?;

	info!("Successfully signed {} registrations", inputs.registrations.len());
	for (i, reg) in inputs.registrations.iter().enumerate() {
		info!("  [{}] Pubkey: {}", i + 1, reg.pubkey.as_hex_string());
		info!("       Nonce:  {}", reg.nonce);
	}

	if dry_run {
		info!("Dry run mode - not sending transaction");
		info!("Would register {} keys to URC at {:?}", inputs.registrations.len(), urc_address);
		return Ok(());
	}

	// Get password
	let password = match password {
		Some(p) => p,
		None => {
			info!("Enter keystore password:");
			rpassword::read_password().context("Failed to read password")?
		}
	};

	// Send registration transaction
	info!("Sending URC registration transaction...");
	let tx_hash =
		send_registration_transaction(&inputs, urc_address, &execution_rpc_url, &keystore_path, &password, collateral)
			.await
			.context("Failed to send registration transaction")?;

	info!("✅ Registration successful!");
	info!("Transaction hash: {:?}", tx_hash);
	info!("Owner: {:?}", owner);
	info!("Number of keys registered: {}", inputs.registrations.len());

	Ok(())
}
