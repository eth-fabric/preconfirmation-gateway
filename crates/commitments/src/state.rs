use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use common::execution::{ExecutionApiClient, ReqwestRpcClient};
use common::slot_timer::SlotTimer;
use common::types::database::DatabaseContext;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Server state that provides access to shared resources for all RPC handlers
/// This holds runtime resources (database connections, RPC clients, timers) needed by the commitments service
#[derive(Clone)]
pub struct CommitmentsServerState<T> {
	/// Database context for RocksDB operations
	pub database: DatabaseContext,
	/// Commit module configuration for commit-boost operations (Arc<Mutex> for thread safety)
	pub commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	/// BLS public key for signing constraints
	pub bls_public_key: BlsPublicKey,
	/// Relay URL for constraints communication
	pub relay_url: String,
	/// API key for relay authentication
	pub api_key: Option<String>,
	/// Slot timer for Ethereum slot timing
	pub slot_timer: SlotTimer,
	/// Execution RPC client for gas price and estimation
	pub execution_client: Arc<ExecutionApiClient<ReqwestRpcClient>>,
}

// Manual Debug implementation since ExecutionApiClient might not derive Debug properly
impl<T> std::fmt::Debug for CommitmentsServerState<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CommitmentsServerState")
			.field("database", &self.database)
			.field("bls_public_key", &self.bls_public_key)
			.field("relay_url", &self.relay_url)
			.field("slot_timer", &self.slot_timer)
			.field("execution_client", &self.execution_client)
			.finish()
	}
}

impl<T> CommitmentsServerState<T> {
	/// Create a new commitments server state with the given database context and commit config
	pub fn new(
		database: DatabaseContext,
		commit_config: StartCommitModuleConfig<T>,
		bls_public_key: BlsPublicKey,
		relay_url: String,
		api_key: Option<String>,
		slot_timer: SlotTimer,
		execution_client: Arc<ExecutionApiClient<ReqwestRpcClient>>,
	) -> Self {
		Self {
			database,
			commit_config: Arc::new(Mutex::new(commit_config)),
			bls_public_key,
			relay_url,
			api_key,
			slot_timer,
			execution_client,
		}
	}

	/// Get a reference to the database
	pub fn database(&self) -> &DatabaseContext {
		&self.database
	}

	/// Get a reference to the slot timer
	pub fn slot_timer(&self) -> &SlotTimer {
		&self.slot_timer
	}

	/// Get a reference to the execution RPC client
	pub fn execution_client(&self) -> &ExecutionApiClient<ReqwestRpcClient> {
		&self.execution_client
	}
}
