use crate::slot_timer::SlotTimer;
use crate::types::database::DatabaseContext;
use commit_boost::prelude::{BlsPublicKey, StartCommitModuleConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

/// RPC context that provides access to shared resources for all RPC handlers
#[derive(Clone, Debug)]
pub struct RpcContext<T = ()> {
	/// Database context for PostgreSQL operations
	pub database: DatabaseContext,
	/// Pricing database context for price storage
	pub pricing_database: DatabaseContext,
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
}

impl<T> RpcContext<T> {
	/// Create a new RPC context with the given database context and commit config
	pub fn new(
		database: DatabaseContext,
		pricing_database: DatabaseContext,
		commit_config: StartCommitModuleConfig<T>,
		bls_public_key: BlsPublicKey,
		relay_url: String,
		api_key: Option<String>,
		slot_timer: SlotTimer,
	) -> Self {
		Self {
			database,
			pricing_database,
			commit_config: Arc::new(Mutex::new(commit_config)),
			bls_public_key,
			relay_url,
			api_key,
			slot_timer,
		}
	}

	/// Get a reference to the database
	pub fn database(&self) -> &DatabaseContext {
		&self.database
	}

	/// Get a reference to the pricing database
	pub fn pricing_database(&self) -> &DatabaseContext {
		&self.pricing_database
	}

	/// Get a reference to the slot timer
	pub fn slot_timer(&self) -> &SlotTimer {
		&self.slot_timer
	}
}
