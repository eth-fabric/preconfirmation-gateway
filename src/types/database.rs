use std::sync::Arc;

use anyhow::Result;
use tokio_postgres::Client;

/// Database context that provides access to the PostgreSQL connection
/// tokio-postgres::Client is designed to be used concurrently via Arc
#[derive(Clone)]
pub struct DatabaseContext {
	client: Arc<Client>,
}

impl DatabaseContext {
	/// Create a new DatabaseContext with the given client
	pub fn new(client: Client) -> Self {
		Self { client: Arc::new(client) }
	}

	/// Execute a query with the database client
	/// This is a convenience method that provides access to the client
	pub async fn with_client<F, R>(&self, f: F) -> Result<R>
	where
		F: FnOnce(&Client) -> R,
	{
		Ok(f(&self.client))
	}

	/// Get a reference to the database client
	pub fn client(&self) -> &Client {
		&self.client
	}

	/// Test the database connection
	pub async fn test_connection(&self) -> Result<()> {
		self.client.simple_query("SELECT 1").await?;
		Ok(())
	}
}

impl std::fmt::Debug for DatabaseContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("DatabaseContext").field("client", &"<PostgreSQL Client>").finish()
	}
}