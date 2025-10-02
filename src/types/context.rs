use crate::types::database::DatabaseContext;
use commit_boost::prelude::StartCommitModuleConfig;
use alloy::primitives::Address;
use std::sync::Arc;
use tokio::sync::Mutex;

/// RPC context that provides access to shared resources for all RPC handlers
#[derive(Clone, Debug)]
pub struct RpcContext<T = ()> {
	/// Database context for PostgreSQL operations
	pub database: DatabaseContext,
	/// Commit module configuration for commit-boost operations (Arc<Mutex> for thread safety)
	pub commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	/// Committer address for signing operations
	pub committer_address: Address,
}

impl<T> RpcContext<T> {
	/// Create a new RPC context with the given database context, commit config, and committer address
	pub fn new(database: DatabaseContext, commit_config: StartCommitModuleConfig<T>, committer_address: Address) -> Self {
		Self { 
			database, 
			commit_config: Arc::new(Mutex::new(commit_config)),
			committer_address,
		}
	}

	/// Convenience method for database operations
	/// This delegates to the underlying DatabaseContext's with_client method
	pub async fn with_database<F, Fut, R>(&self, f: F) -> anyhow::Result<R>
	where
		F: FnOnce(deadpool_postgres::Client) -> Fut,
		Fut: std::future::Future<Output = anyhow::Result<R>>,
	{
		self.database.with_client(f).await
	}

	/// Get a database client from the connection pool
	pub async fn database_client(&self) -> anyhow::Result<deadpool_postgres::Client> {
		self.database.client().await
	}
}
