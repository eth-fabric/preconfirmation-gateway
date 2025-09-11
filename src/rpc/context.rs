use crate::db::DatabaseContext;

/// RPC context that provides access to shared resources for all RPC handlers
#[derive(Clone, Debug)]
pub struct RpcContext {
	/// Database context for PostgreSQL operations
	pub database: DatabaseContext,
}

impl RpcContext {
	/// Create a new RPC context with the given database context
	pub fn new(database: DatabaseContext) -> Self {
		Self { database }
	}

	/// Convenience method for database operations
	/// This delegates to the underlying DatabaseContext's with_client method
	pub async fn with_database<F, R>(&self, f: F) -> anyhow::Result<R>
	where
		F: FnOnce(&tokio_postgres::Client) -> R,
	{
		self.database.with_client(f).await
	}

	/// Get a reference to the database client
	pub fn database_client(&self) -> &tokio_postgres::Client {
		self.database.client()
	}
}
