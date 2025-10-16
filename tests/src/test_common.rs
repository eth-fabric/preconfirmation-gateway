use alloy::primitives::{Address, B256, Bytes, Signature, b256, hex};
use cb_common::{
	commit::client::SignerClient,
	config::load_module_signing_configs,
	types::{Jwt, ModuleId},
};
use commit_boost::prelude::StartCommitModuleConfig;
use common::config::InclusionPreconfConfig;
use common::slot_timer::SlotTimer;
use common::types::commitments::InclusionPayload;
use common::types::{Commitment, CommitmentRequest, DatabaseContext, RpcContext, SignedCommitment};
use eyre::Result;
use rand::Rng;
use rocksdb::{DB, Options};
use std::{collections::HashMap, sync::Arc};
use tempfile::TempDir;
use tokio::sync::oneshot;

// Test constants
pub const MODULE_ID: &str = "inclusion-preconf-module";
pub const SIGNING_ID: B256 = b256!("0x1111111111111111111111111111111111111111111111111111111111111111");
pub const PUBKEY: [u8; 48] =
	hex!("883827193f7627cd04e621e1e8d56498362a52b2a30c9a1c72036eb935c4278dee23d38a24d2f7dda62689886f0c39f4");

/// Starts a local signer server for testing and reconstructs a StartCommitModuleConfig
/// This function allows unit tests to start a local signer service and get a properly configured
/// StartCommitModuleConfig that can be used to test signing functionality.
pub async fn start_local_signer_server(
	module_id: &str,
	signing_id: B256,
	admin_secret: &str,
	port: u16,
) -> Result<StartCommitModuleConfig<()>> {
	use cb_tests::{signer_service, utils};

	utils::setup_test_env();

	let mut cfg = utils::get_commit_boost_config(utils::get_pbs_static_config(utils::get_pbs_config(0)));

	let module_id = ModuleId(module_id.to_string());

	cfg.modules = Some(vec![utils::create_module_config(module_id.clone(), signing_id)]);

	let jwts = HashMap::from([(module_id.clone(), admin_secret.to_string())]);

	let mod_cfgs = load_module_signing_configs(&cfg, &jwts)?;

	let start_config = signer_service::start_server(port, &mod_cfgs, admin_secret.to_string(), false).await?;
	let jwt_config = mod_cfgs.get(&module_id).expect("JWT config for test module not found");

	// Reconstruct StartCommitModuleConfig using the same URL and JWT secret as the local signer
	let signer_url = format!("http://{}", start_config.endpoint)
		.parse()
		.map_err(|e| eyre::eyre!("Failed to parse signer URL: {}", e))?;

	let module_jwt = Jwt(jwt_config.jwt_secret.clone());

	// Create SignerClient with the same parameters as the local signer
	let signer_client = SignerClient::new(signer_url, None, module_jwt, module_id.clone())?;

	// Use the chain from the config
	let chain = cfg.chain;

	Ok(StartCommitModuleConfig { id: module_id, chain, signer_client, extra: () })
}

/// Starts a local signer server for testing with custom configuration
pub async fn start_local_signer_server_with_config(
	module_id: &str,
	signing_id: B256,
	admin_secret: &str,
	port: u16,
	app_config: InclusionPreconfConfig,
) -> Result<StartCommitModuleConfig<InclusionPreconfConfig>> {
	use cb_tests::{signer_service, utils};

	utils::setup_test_env();

	let mut cfg = utils::get_commit_boost_config(utils::get_pbs_static_config(utils::get_pbs_config(0)));

	let module_id = ModuleId(module_id.to_string());

	cfg.modules = Some(vec![utils::create_module_config(module_id.clone(), signing_id)]);

	let jwts = HashMap::from([(module_id.clone(), admin_secret.to_string())]);

	let mod_cfgs = load_module_signing_configs(&cfg, &jwts)?;

	let start_config = signer_service::start_server(port, &mod_cfgs, admin_secret.to_string(), false).await?;
	let jwt_config = mod_cfgs.get(&module_id).expect("JWT config for test module not found");

	// Reconstruct StartCommitModuleConfig using the same URL and JWT secret as the local signer
	let signer_url = format!("http://{}", start_config.endpoint)
		.parse()
		.map_err(|e| eyre::eyre!("Failed to parse signer URL: {}", e))?;

	let module_jwt = Jwt(jwt_config.jwt_secret.clone());

	// Create SignerClient with the same parameters as the local signer
	let signer_client = SignerClient::new(signer_url, None, module_jwt, module_id.clone())?;

	// Use the chain from the config
	let chain = cfg.chain;

	Ok(StartCommitModuleConfig { id: module_id, chain, signer_client, extra: app_config })
}

/// Unified test harness builder for all test scenarios
pub struct TestHarnessBuilder {
	signer_port: Option<u16>,
	// Option<Option<u16>>: None = don't launch, Some(None) = auto-assign, Some(Some(port)) = specific port
	commitments_port: Option<Option<u16>>,
	relay_port: Option<Option<u16>>,
	module_id: String,
	signing_id: B256,
	admin_secret: String,
}

impl Default for TestHarnessBuilder {
	fn default() -> Self {
		Self {
			signer_port: None,
			commitments_port: None,
			relay_port: None,
			module_id: MODULE_ID.to_string(),
			signing_id: SIGNING_ID,
			admin_secret: "test-admin-secret".to_string(),
		}
	}
}

impl TestHarnessBuilder {
	/// Set an explicit signer port
	pub fn with_signer_port(mut self, port: u16) -> Self {
		self.signer_port = Some(port);
		self
	}

	/// Launch commitments service with optional port (None = auto-assign, Some(port) = specific port)
	pub fn with_commitments_port(mut self, port: Option<u16>) -> Self {
		self.commitments_port = Some(port);
		self
	}

	/// Launch relay service with optional port (None = auto-assign, Some(port) = specific port)
	pub fn with_relay_port(mut self, port: Option<u16>) -> Self {
		self.relay_port = Some(port);
		self
	}

	/// Setup all databases needed for testing
	fn setup_databases(
		&self,
		temp_dir: &TempDir,
	) -> Result<(DatabaseContext, DatabaseContext, DatabaseContext, DatabaseContext)> {
		let mut opts = Options::default();
		opts.create_if_missing(true);

		// Commitments database
		let db_path = temp_dir.path().join("test_commitments_db");
		let db = DB::open(&opts, &db_path)?;
		let commitments_database = DatabaseContext::new(Arc::new(db));

		// Pricing database
		let pricing_db_path = temp_dir.path().join("test_pricing_db");
		let pricing_db = DB::open(&opts, &pricing_db_path)?;
		let pricing_database = DatabaseContext::new(Arc::new(pricing_db));

		// Delegations database
		let delegations_db_path = temp_dir.path().join("test_delegations_db");
		let delegations_db = DB::open(&opts, &delegations_db_path)?;
		let delegations_database = DatabaseContext::new(Arc::new(delegations_db));

		// Constraints/Relay database
		let relay_db_path = temp_dir.path().join("test_relay_db");
		let relay_db = DB::open(&opts, &relay_db_path)?;
		let relay_database = DatabaseContext::new(Arc::new(relay_db));

		Ok((commitments_database, pricing_database, delegations_database, relay_database))
	}

	/// Build the test harness
	pub async fn build(self) -> Result<TestHarness> {
		// Auto-generate ports if not specified
		let mut rng = rand::thread_rng();
		let signer_port = self.signer_port.unwrap_or_else(|| rng.gen_range(20000..65535));

		// Handle commitments port: if Some(_), we want to launch; the inner Option determines auto vs specific
		let commitments_port =
			self.commitments_port.map(|opt_port| opt_port.unwrap_or_else(|| rng.gen_range(20000..65535)));

		// Handle relay port: if Some(_), we want to launch; the inner Option determines auto vs specific
		let relay_port = self.relay_port.map(|opt_port| opt_port.unwrap_or_else(|| rng.gen_range(20000..65535)));

		// Create temporary directory for all databases
		let temp_dir = TempDir::new()?;

		// Setup all databases
		let (database, pricing_database, _delegations_database, _relay_database) = self.setup_databases(&temp_dir)?;

		// Create test InclusionPreconfConfig
		let app_config = InclusionPreconfConfig {
			commitments_server_host: "127.0.0.1".to_string(),
			commitments_server_port: commitments_port.unwrap_or(18545), // Default port if not launching service
			commitments_database_url: temp_dir.path().join("test_commitments_db").to_string_lossy().to_string(),
			constraints_database_url: temp_dir.path().join("test_constraints_db").to_string_lossy().to_string(),
			delegations_database_url: temp_dir.path().join("test_delegations_db").to_string_lossy().to_string(),
			pricing_database_url: temp_dir.path().join("test_pricing_db").to_string_lossy().to_string(),
			log_level: "info".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: None,
			constraints_bls_public_key: hex::encode(PUBKEY),
			constraints_delegate_public_key: hex::encode(PUBKEY),
			eth_genesis_timestamp: 1606824023,
			constraints_receivers: vec![hex::encode(PUBKEY)],
		};

		// Start local signer server with config
		let commit_config = start_local_signer_server_with_config(
			&self.module_id,
			self.signing_id,
			&self.admin_secret,
			signer_port,
			app_config,
		)
		.await?;

		// Use consensus BLS public key (this is registered with the signer service)
		let consensus_bls_public_key = cb_common::types::BlsPublicKey::deserialize(&PUBKEY)
			.map_err(|e| eyre::eyre!("Failed to deserialize consensus BLS public key: {:?}", e))?;

		// Generate proxy keys for proposer
		let proposer_bls_public_key = {
			let mut commit_config_guard = commit_config.signer_client.clone();
			commit_config_guard
				.generate_proxy_key_bls(consensus_bls_public_key.clone())
				.await
				.map_err(|e| eyre::eyre!("Failed to generate proposer proxy BLS key: {}", e))?
				.message
				.proxy
		};

		// Generate TWO gateway proxy keys (for positive/negative tests)
		let gateway_bls_one = {
			let mut commit_config_guard = commit_config.signer_client.clone();
			commit_config_guard
				.generate_proxy_key_bls(consensus_bls_public_key.clone())
				.await
				.map_err(|e| eyre::eyre!("Failed to generate gateway_one proxy BLS key: {}", e))?
				.message
				.proxy
		};

		let gateway_bls_two = {
			let mut commit_config_guard = commit_config.signer_client.clone();
			commit_config_guard
				.generate_proxy_key_bls(consensus_bls_public_key.clone())
				.await
				.map_err(|e| eyre::eyre!("Failed to generate gateway_two proxy BLS key: {}", e))?
				.message
				.proxy
		};

		// Generate TWO committer addresses (for positive/negative tests)
		let committer_one = {
			let mut commit_config_guard = commit_config.signer_client.clone();
			commit_config_guard
				.generate_proxy_key_ecdsa(consensus_bls_public_key.clone())
				.await
				.map_err(|e| eyre::eyre!("Failed to generate committer_one ECDSA proxy key: {}", e))?
				.message
				.proxy
		};

		let committer_two = {
			let mut commit_config_guard = commit_config.signer_client.clone();
			commit_config_guard
				.generate_proxy_key_ecdsa(consensus_bls_public_key.clone())
				.await
				.map_err(|e| eyre::eyre!("Failed to generate committer_two ECDSA proxy key: {}", e))?
				.message
				.proxy
		};

		// Construct relay URL based on relay port if specified
		let relay_url = if let Some(port) = relay_port {
			format!("http://127.0.0.1:{}", port)
		} else {
			"http://127.0.0.1:3001".to_string() // Default test relay URL
		};
		let api_key = None::<String>;

		// Create slot timer with test genesis timestamp
		let slot_timer = SlotTimer::new(1606824023);

		// Create RPC context - use gateway_one as the primary gateway
		let context = RpcContext {
			database,
			pricing_database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			bls_public_key: gateway_bls_one.clone(),
			relay_url: relay_url.clone(),
			api_key,
			slot_timer,
		};

		// Launch relay service if relay port is specified
		let relay_service =
			if let Some(port) = relay_port { Some(Self::launch_relay_service(port, &temp_dir).await?) } else { None };

		// Launch commitments service if commitments port was explicitly specified (not auto-generated)
		let commitments_service = if self.commitments_port.is_some() {
			Some(Self::launch_commitments_service(context.clone()).await?)
		} else {
			None
		};

		Ok(TestHarness {
			context,
			proposer_bls_public_key,
			gateway_bls_one,
			gateway_bls_two,
			committer_one,
			committer_two,
			commitments_service,
			relay_service,
			_temp_dir: temp_dir,
			_signer_port: signer_port,
		})
	}

	/// Launch the commitments RPC server
	async fn launch_commitments_service(rpc_context: RpcContext<InclusionPreconfConfig>) -> Result<ServiceHandle> {
		let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

		// Get the port from the commit config
		let commit_config = rpc_context.commit_config.lock().await;
		let app_config: InclusionPreconfConfig = commit_config.extra.clone();
		let port = app_config.commitments_server_port;
		let url = format!("http://127.0.0.1:{}", port);
		drop(commit_config); // Release the lock

		// Spawn the server task
		let handle = tokio::spawn(async move {
			tokio::select! {
				result = commitments::server::run_server(rpc_context) => {
					result
				}
				_ = shutdown_rx => {
					tracing::info!("Commitments service shutting down gracefully");
					Ok(())
				}
			}
		});

		// Wait a bit for the server to start
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;

		Ok(ServiceHandle { handle: Some(handle), shutdown_tx: Some(shutdown_tx), url })
	}

	/// Launch the relay HTTP server
	async fn launch_relay_service(port: u16, temp_dir: &TempDir) -> Result<ServiceHandle> {
		let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

		// Create relay config with test database path
		let database_path = temp_dir.path().join("relay_db");
		let config = relay::config::RelayConfig {
			relay: relay::config::RelayServerConfig {
				port,
				database_path,
				log_level: "info".to_string(),
				constraint_capabilities: vec![1],
				chain: commit_boost::prelude::Chain::Hoodi,
			},
			storage: relay::config::StorageConfig { max_delegations_per_slot: 100, max_constraints_per_slot: 1000 },
		};

		let url = format!("http://127.0.0.1:{}", port);

		// Spawn the server task
		let handle = tokio::spawn(async move {
			tokio::select! {
				result = relay::server::run_relay_server(config) => {
					result
				}
				_ = shutdown_rx => {
					tracing::info!("Relay service shutting down gracefully");
					Ok(())
				}
			}
		});

		// Wait a bit longer for the relay server to start (includes database init)
		tokio::time::sleep(std::time::Duration::from_millis(500)).await;

		Ok(ServiceHandle { handle: Some(handle), shutdown_tx: Some(shutdown_tx), url })
	}
}

/// Unified test harness for all test scenarios  
/// Holds setup state and provides helpers to create properly signed objects
pub struct TestHarness {
	pub context: RpcContext<InclusionPreconfConfig>,
	pub proposer_bls_public_key: cb_common::types::BlsPublicKey,
	pub gateway_bls_one: cb_common::types::BlsPublicKey,
	pub gateway_bls_two: cb_common::types::BlsPublicKey,
	pub committer_one: Address,
	pub committer_two: Address,
	pub commitments_service: Option<ServiceHandle>,
	pub relay_service: Option<ServiceHandle>,
	_temp_dir: TempDir,
	_signer_port: u16,
}

impl TestHarness {
	/// Create a new builder for the test harness
	pub fn builder() -> TestHarnessBuilder {
		TestHarnessBuilder::default()
	}

	// ===== Helper methods to create Delegation inputs =====

	/// Create a Delegation message (unsigned) with custom delegate and committer
	pub fn create_delegation(
		&self,
		slot: u64,
		delegate: cb_common::types::BlsPublicKey,
		committer: Address,
	) -> common::types::constraints::Delegation {
		common::types::constraints::Delegation {
			proposer: self.proposer_bls_public_key.clone(),
			delegate,
			committer,
			slot,
			metadata: Bytes::new(),
		}
	}

	/// Create a properly signed delegation using the BLS signer
	/// The signer_bls_public_key parameter allows testing with different signers (e.g., invalid signer for negative tests)
	pub async fn create_signed_delegation(
		&self,
		delegation: &common::types::constraints::Delegation,
		signer_bls_public_key: cb_common::types::BlsPublicKey,
	) -> Result<common::types::constraints::SignedDelegation> {
		// Get the delegation message hash
		let message_hash = delegation.to_message_hash()?;

		// Sign using the BLS proxy signer with the specified key
		let response = {
			let mut config = self.context.commit_config.lock().await;
			common::signer::call_proxy_bls_signer(&mut *config, message_hash, signer_bls_public_key).await?
		};

		// Create the signed delegation
		Ok(common::types::constraints::SignedDelegation {
			message: delegation.clone(),
			nonce: response.nonce,
			signing_id: response.module_signing_id,
			signature: response.signature,
		})
	}

	// ===== Helper methods to create CommitmentRequest =====

	/// Create a valid signed transaction for testing
	/// Uses the verified utility function from commitments::utils that passes signature verification
	pub fn create_signed_tx(&self) -> Vec<u8> {
		commitments::utils::create_valid_signed_transaction().to_vec()
	}

	/// Create a valid signed transaction with custom parameters for testing
	/// Note: This uses a mock signature and may not pass recover_signer() verification
	/// For tests that need signature verification, use create_signed_tx() instead
	pub fn create_signed_tx_custom(&self, nonce: u64, to: Address, value: u128) -> Result<Vec<u8>> {
		use alloy::consensus::{Signed, TxEip1559, TxEnvelope};
		use alloy::primitives::{Signature, TxKind, U256};
		use alloy::rlp::Encodable;

		// Create an EIP-1559 transaction
		let tx = TxEip1559 {
			chain_id: 1, // Mainnet chain ID
			nonce,
			gas_limit: 21_000,
			max_fee_per_gas: 20_000_000_000u128,
			max_priority_fee_per_gas: 2_000_000_000u128,
			to: TxKind::Call(to),
			value: U256::from(value),
			input: alloy::primitives::Bytes::new(),
			access_list: Default::default(),
		};

		// Create a mock signature for testing
		let mock_signature = Signature::new(U256::from(1u64), U256::from(2u64), false);
		let signed_tx = Signed::new_unhashed(tx, mock_signature);
		let tx_envelope = TxEnvelope::Eip1559(signed_tx);

		// RLP encode the transaction
		let mut encoded_tx = Vec::new();
		tx_envelope.encode(&mut encoded_tx);

		Ok(encoded_tx)
	}

	/// Create a valid CommitmentRequest for testing
	pub fn create_commitment_request(
		&self,
		slot: u64,
		signed_tx: Vec<u8>,
		slasher: Address,
	) -> Result<common::types::CommitmentRequest> {
		use common::types::commitments::InclusionPayload;

		let inclusion_payload = InclusionPayload { slot, signed_tx: signed_tx.into() };
		let payload = inclusion_payload.abi_encode()?;

		Ok(common::types::CommitmentRequest { commitment_type: common::constants::COMMITMENT_TYPE, payload, slasher })
	}

	// ===== Helper methods for headers =====

	/// Create test headers with BLS authentication using the specified proxy signer
	pub async fn create_headers_with_valid_signature(
		&self,
		slot: u64,
		bls_public_key: cb_common::types::BlsPublicKey,
	) -> axum::http::HeaderMap {
		use axum::http::{HeaderMap, HeaderValue};

		// Get the commit config from context
		let mut commit_config = self.context.commit_config.lock().await;

		// Compute slot hash for signing
		let slot_hash = alloy::primitives::keccak256(slot.to_string().as_bytes());

		// Call the proxy BLS signer to sign the slot hash
		let bls_response =
			common::signer::call_proxy_bls_signer(&mut *commit_config, slot_hash, bls_public_key.clone())
				.await
				.expect("Failed to sign slot hash with proxy BLS signer");

		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Receiver-PublicKey",
			HeaderValue::from_str(&format!("0x{}", hex::encode(bls_public_key.serialize()))).unwrap(),
		);
		headers.insert(
			"X-Receiver-Signature",
			HeaderValue::from_str(&format!("0x{}", hex::encode(bls_response.signature.serialize()))).unwrap(),
		);
		headers.insert("X-Receiver-Nonce", HeaderValue::from_str(&bls_response.nonce.to_string()).unwrap());
		headers.insert(
			"X-Receiver-SigningId",
			HeaderValue::from_str(&format!("0x{}", hex::encode(bls_response.module_signing_id.as_slice()))).unwrap(),
		);
		headers
	}

	// ===== Utility methods =====

	/// Create a RelayState for relay testing
	pub fn create_relay_state(&self) -> relay::handlers::RelayState {
		relay::handlers::RelayState {
			database: Arc::new(self.context.database.clone()),
			config: relay::config::RelayConfig::default(),
		}
	}

	/// Create a ClientHarness for making RPC/HTTP calls to running services
	pub fn create_client_harness(&self) -> ClientHarness {
		let mut client = ClientHarness::new();

		// Add commitments URL if service is running
		if let Some(service) = &self.commitments_service {
			client = client.with_commitments_url(service.url.clone());
		}

		// Add relay URL if service is running
		if let Some(service) = &self.relay_service {
			client = client.with_relay_url(service.url.clone());
		}

		client
	}

	/// Process delegations from the relay
	/// This is a convenience wrapper around constraints::service::process_delegations
	pub async fn process_delegations(&self, slot: u64) -> Result<common::types::ProcessDelegationsResponse> {
		let relay_url =
			self.relay_service.as_ref().ok_or_else(|| eyre::eyre!("Relay service not running"))?.url.clone();

		constraints::service::process_delegations(
			slot,
			self.gateway_bls_one.clone(),
			&self.context.database,
			relay_url,
			None,
		)
		.await
	}

	/// Process constraints and post them to the relay
	/// This is a convenience wrapper around constraints::service::process_constraints
	pub async fn process_constraints(
		&self,
		slot: u64,
		receivers: Vec<cb_common::types::BlsPublicKey>,
	) -> Result<common::types::ProcessConstraintsResponse> {
		let relay_url =
			self.relay_service.as_ref().ok_or_else(|| eyre::eyre!("Relay service not running"))?.url.clone();

		constraints::service::process_constraints(
			slot,
			self.gateway_bls_one.clone(),
			self.proposer_bls_public_key.clone(),
			receivers,
			&self.context.database,
			self.context.commit_config.clone(),
			relay_url,
			None,
		)
		.await
	}
}

/// Handle for a running service in integration tests
pub struct ServiceHandle {
	handle: Option<tokio::task::JoinHandle<Result<()>>>,
	shutdown_tx: Option<oneshot::Sender<()>>,
	pub url: String,
}

impl ServiceHandle {
	/// Get the URL of the running service
	pub fn url(&self) -> &str {
		&self.url
	}

	/// Shutdown the service gracefully
	pub async fn shutdown(mut self) -> Result<()> {
		// Send shutdown signal
		if let Some(tx) = self.shutdown_tx.take() {
			let _ = tx.send(());
		}

		// Wait for the service to finish (with timeout)
		if let Some(handle) = self.handle.take() {
			match tokio::time::timeout(std::time::Duration::from_secs(5), handle).await {
				Ok(result) => result?,
				Err(_) => {
					tracing::warn!("Service shutdown timed out after 5 seconds");
					Ok(())
				}
			}
		} else {
			Ok(())
		}
	}
}

impl Drop for ServiceHandle {
	fn drop(&mut self) {
		// Abort the task if shutdown() wasn't called explicitly
		if let Some(handle) = self.handle.take() {
			handle.abort();
		}
	}
}

/// Client harness for making HTTP/RPC calls to running services
#[derive(Clone)]
pub struct ClientHarness {
	commitments_url: Option<String>,
	relay_url: Option<String>,
	http_client: reqwest::Client,
}

impl ClientHarness {
	/// Create a new client harness
	pub fn new() -> Self {
		Self { commitments_url: None, relay_url: None, http_client: reqwest::Client::new() }
	}

	/// Set the commitments service URL
	pub fn with_commitments_url(mut self, url: String) -> Self {
		self.commitments_url = Some(url);
		self
	}

	/// Set the relay service URL  
	pub fn with_relay_url(mut self, url: String) -> Self {
		self.relay_url = Some(url);
		self
	}

	/// Post a delegation to the relay service
	pub async fn post_delegation(&self, delegation: &common::types::constraints::SignedDelegation) -> Result<()> {
		let url = self.relay_url.as_ref().ok_or_else(|| eyre::eyre!("Relay URL not set"))?;

		let response = self.http_client.post(format!("{}/delegation", url)).json(delegation).send().await?;

		if !response.status().is_success() {
			let status = response.status();
			let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
			return Err(eyre::eyre!("Failed to send delegation (status {}): {}", status, error_text));
		}

		Ok(())
	}

	/// Post constraints to the relay service
	pub async fn post_constraints(
		&self,
		constraints: &common::types::constraints::SignedConstraints,
		headers: axum::http::HeaderMap,
	) -> Result<()> {
		let url = self.relay_url.as_ref().ok_or_else(|| eyre::eyre!("Relay URL not set"))?;

		let mut request = self.http_client.post(format!("{}/constraints", url)).json(constraints);

		// Add headers
		for (key, value) in headers.iter() {
			request = request.header(key.as_str(), value.to_str()?);
		}

		let response = request.send().await?;

		if !response.status().is_success() {
			let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
			return Err(eyre::eyre!("Failed to send constraints: {}", error_text));
		}

		Ok(())
	}

	/// Get constraints from the relay service (requires authentication headers)
	pub async fn get_constraints(
		&self,
		slot: u64,
		headers: axum::http::HeaderMap,
	) -> Result<Vec<common::types::constraints::SignedConstraints>> {
		let url = self.relay_url.as_ref().ok_or_else(|| eyre::eyre!("Relay URL not set"))?;

		let mut request = self.http_client.get(format!("{}/constraints/v0/relay/constraints/{}", url, slot));

		// Add headers
		for (key, value) in headers.iter() {
			request = request.header(key.as_str(), value.to_str()?);
		}

		let response = request.send().await?;

		if !response.status().is_success() {
			let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
			return Err(eyre::eyre!("Failed to get constraints: {}", error_text));
		}

		let result = response.json().await?;
		Ok(result)
	}

	/// Make an RPC call to the commitments service
	pub async fn call_commitment_request(
		&self,
		request: &common::types::CommitmentRequest,
	) -> Result<common::types::SignedCommitment> {
		let url = self.commitments_url.as_ref().ok_or_else(|| eyre::eyre!("Commitments URL not set"))?;

		use jsonrpsee::core::client::ClientT;
		use jsonrpsee::http_client::HttpClientBuilder;

		let client = HttpClientBuilder::default().build(url)?;

		let result: common::types::SignedCommitment = client
			.request("commitmentRequest", vec![request])
			.await
			.map_err(|e| eyre::eyre!("RPC call failed: {}", e))?;

		Ok(result)
	}

	/// Get a commitment result from the commitments service
	pub async fn call_commitment_result(
		&self,
		request_hash: alloy::primitives::B256,
	) -> Result<common::types::SignedCommitment> {
		let url = self.commitments_url.as_ref().ok_or_else(|| eyre::eyre!("Commitments URL not set"))?;

		use jsonrpsee::core::client::ClientT;
		use jsonrpsee::http_client::HttpClientBuilder;

		let client = HttpClientBuilder::default().build(url)?;

		let result: common::types::SignedCommitment = client
			.request("commitmentResult", vec![request_hash])
			.await
			.map_err(|e| eyre::eyre!("RPC call failed: {}", e))?;

		Ok(result)
	}

	/// Alias for call_commitment_request
	pub async fn commitment_request(
		&self,
		request: &common::types::CommitmentRequest,
	) -> Result<common::types::SignedCommitment> {
		self.call_commitment_request(request).await
	}

	/// Alias for call_commitment_result
	pub async fn commitment_result(
		&self,
		request_hash: &alloy::primitives::B256,
	) -> Result<common::types::SignedCommitment> {
		self.call_commitment_result(*request_hash).await
	}

	/// Get fee information from the commitments service
	pub async fn fee(&self, request: &common::types::CommitmentRequest) -> Result<common::types::commitments::FeeInfo> {
		let url = self.commitments_url.as_ref().ok_or_else(|| eyre::eyre!("Commitments URL not set"))?;

		use jsonrpsee::core::client::ClientT;
		use jsonrpsee::core::params::ArrayParams;
		use jsonrpsee::http_client::HttpClientBuilder;

		let client = HttpClientBuilder::default().build(url)?;

		// Build params array manually to ensure proper serialization
		let mut params = ArrayParams::new();
		params.insert(request)?;

		let result: common::types::commitments::FeeInfo =
			client.request("fee", params).await.map_err(|e| eyre::eyre!("RPC call failed: {}", e))?;

		Ok(result)
	}
}

impl Default for ClientHarness {
	fn default() -> Self {
		Self::new()
	}
}

/// Consolidated test helpers for commitment testing
/// This module provides shared helper functions for both commitment_request and commitment_result tests
pub mod test_helpers {
	use super::*;
	use common::constants::COMMITMENT_TYPE;

	/// Creates a valid inclusion payload
	pub fn create_valid_inclusion_payload(slot: u64, signed_tx: Vec<u8>) -> eyre::Result<Bytes> {
		let inclusion_payload = InclusionPayload { slot, signed_tx: signed_tx.into() };
		inclusion_payload.abi_encode()
	}

	/// Creates a valid signed transaction (RLP-encoded)
	/// This creates a realistic EIP-1559 transaction for testing
	pub fn create_valid_signed_tx() -> Vec<u8> {
		// Use the function from utils.rs
		commitments::utils::create_valid_signed_transaction().to_vec()
	}

	/// Creates a valid signed transaction with a specific nonce for testing
	pub fn create_valid_signed_tx_with_nonce(nonce: u64) -> Vec<u8> {
		use alloy::consensus::{Signed, TxEip1559, TxEnvelope};
		use alloy::primitives::{Address, Bytes, Signature, TxKind, U256};
		use alloy::rlp::Encodable;

		let tx = TxEip1559 {
			chain_id: 1,
			nonce,
			gas_limit: 21000,
			max_fee_per_gas: 20_000_000_000u128,
			max_priority_fee_per_gas: 2_000_000_000u128,
			to: TxKind::Call(Address::from([0x01; 20])),
			value: U256::from(1_000_000_000_000_000_000u64),
			input: Bytes::new(),
			access_list: Default::default(),
		};

		// Create a mock signature (all zeros for testing)
		let signature = Signature::from_raw(&[0u8; 65]).unwrap();
		let signed_tx = Signed::new_unhashed(tx, signature);
		let envelope = TxEnvelope::Eip1559(signed_tx);

		// Encode to bytes
		let mut buf = Vec::new();
		envelope.encode(&mut buf);
		buf
	}

	/// Creates a valid commitment request
	pub fn create_valid_commitment_request(
		commitment_type: u64,
		payload: Bytes,
		slasher: Address,
	) -> CommitmentRequest {
		CommitmentRequest { commitment_type, payload, slasher }
	}

	/// Creates a valid slasher address
	pub fn create_valid_slasher() -> Address {
		Address::random()
	}

	pub fn create_valid_commitment_type() -> u64 {
		COMMITMENT_TYPE
	}

	/// Creates an invalid slasher address (zero address)
	pub fn create_invalid_slasher() -> Address {
		Address::ZERO
	}

	/// Creates an invalid commitment type
	pub fn create_invalid_commitment_type() -> u64 {
		COMMITMENT_TYPE + 1 // Invalid commitment type
	}

	/// Creates an empty payload
	pub fn create_empty_payload() -> Bytes {
		Bytes::new()
	}

	/// Creates an invalid payload (not ABI-encoded InclusionPayload)
	pub fn create_invalid_payload() -> Bytes {
		Bytes::from(vec![0x01, 0x02, 0x03]) // Not ABI-encoded
	}

	/// Creates a valid signed commitment for testing
	pub fn create_signed_commitment_with_mock_signature(
		commitment_type: u64,
		payload: Bytes,
		request_hash: B256,
		slasher: Address,
		nonce: u64,
		signing_id: B256,
	) -> SignedCommitment {
		let commitment = Commitment { commitment_type, payload, request_hash, slasher };

		SignedCommitment {
			commitment,
			nonce,
			signing_id,
			signature: Signature::from_bytes_and_parity(&[0x01; 64], true),
		}
	}

	/// Creates a valid request hash
	pub fn create_valid_request_hash() -> B256 {
		B256::from_slice(&[0x11; 32])
	}

	/// Creates another valid request hash
	pub fn create_another_valid_request_hash() -> B256 {
		B256::from_slice(&[0x22; 32])
	}

	/// Creates a non-existent request hash
	pub fn create_nonexistent_request_hash() -> B256 {
		B256::from_slice(&[0x99; 32])
	}

	/// Creates a valid signing ID
	pub fn create_valid_signing_id() -> B256 {
		B256::from_slice(&[0x33; 32])
	}

	/// Creates a valid payload
	pub fn create_valid_payload() -> Bytes {
		Bytes::from(vec![0x01, 0x02, 0x03, 0x04, 0x05])
	}

	/// Creates another valid payload
	pub fn create_another_valid_payload() -> Bytes {
		Bytes::from(vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE])
	}

	/// Creates a valid nonce
	pub fn create_valid_nonce() -> u64 {
		12345
	}

	/// Creates another valid nonce
	pub fn create_another_valid_nonce() -> u64 {
		67890
	}

	/// Creates a test RPC context with a temporary RocksDB database and local signer server
	/// This function provides a complete test environment for RPC handler tests
	/// The port is randomly generated to avoid conflicts between concurrent tests
	pub async fn create_test_context() -> Result<RpcContext> {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");

		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		// Generate a random port to avoid conflicts
		let mut rng = rand::thread_rng();
		let port = rng.gen_range(20000..65535);

		// Start local signer server to get proper config
		let commit_config = start_local_signer_server(MODULE_ID, SIGNING_ID, "test-admin-secret", port).await?;

		// TODO: Get these from configuration - using defaults for tests
		// Use a valid BLS public key for testing
		let bls_public_key = cb_common::utils::bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;
		let relay_url = "https://relay.example.com".to_string();
		let api_key = None::<String>;

		// Create slot timer with test genesis timestamp
		let slot_timer = SlotTimer::new(1606824023);

		// Create pricing database for testing
		let pricing_db = DB::open(&opts, &temp_dir.path().join("test_pricing_db")).unwrap();
		let pricing_database = DatabaseContext::new(Arc::new(pricing_db));

		Ok(RpcContext {
			database,
			pricing_database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			bls_public_key,
			relay_url,
			api_key,
			slot_timer,
		})
	}

	/// Creates a test RPC context with InclusionPreconfConfig for server tests
	/// This function provides a complete test environment for server tests that need the full config
	pub async fn create_test_context_with_config() -> Result<RpcContext<InclusionPreconfConfig>> {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test_db");

		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = DB::open(&opts, &db_path).unwrap();
		let database = DatabaseContext::new(Arc::new(db));

		// Generate a random port to avoid conflicts
		let mut rng = rand::thread_rng();
		let rpc_port = rng.gen_range(3000..65535);
		let _constraints_port = rpc_port + 1;

		// Create test config
		let app_config = InclusionPreconfConfig {
			commitments_server_host: "127.0.0.1".to_string(),
			commitments_server_port: rpc_port,
			commitments_database_url: "./test_db".to_string(),
			constraints_database_url: "./test_constraints_db".to_string(),
			delegations_database_url: "./test_delegations_db".to_string(),
			pricing_database_url: "./test_pricing_db".to_string(),
			log_level: "info".to_string(),
			enable_method_tracing: false,
			traced_methods: vec![],
			constraints_relay_url: "https://relay.example.com".to_string(),
			constraints_api_key: None,
			constraints_bls_public_key:
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
			constraints_delegate_public_key:
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
			eth_genesis_timestamp: 1606824023,
			constraints_receivers: vec![
				"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6"
					.to_string(),
			],
		};

		// Start local signer server with config
		let commit_config = start_local_signer_server_with_config(
			MODULE_ID,
			SIGNING_ID,
			"test-admin-secret",
			rpc_port,
			app_config.clone(),
		)
		.await?;

		// Use a valid BLS public key for testing
		let bls_public_key = cb_common::utils::bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.map_err(|e| eyre::eyre!("Failed to create BLS public key: {}", e))?;
		let relay_url = "https://relay.example.com".to_string();
		let api_key = None::<String>;

		// Create slot timer with test genesis timestamp
		let slot_timer = SlotTimer::new(1606824023);

		// Create pricing database for testing
		let pricing_db = DB::open(&opts, &temp_dir.path().join("test_pricing_db")).unwrap();
		let pricing_database = DatabaseContext::new(Arc::new(pricing_db));

		Ok(RpcContext {
			database,
			pricing_database,
			commit_config: Arc::new(tokio::sync::Mutex::new(commit_config)),
			bls_public_key,
			relay_url,
			api_key,
			slot_timer,
		})
	}
}
