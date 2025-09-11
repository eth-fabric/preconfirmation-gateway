use std::sync::{Arc, Mutex};

use anyhow::Result;
use postgres::Client;

/// Database context that provides thread-safe access to the PostgreSQL connection
#[derive(Clone)]
pub struct DatabaseContext {
	client: Arc<Mutex<Client>>,
}

impl DatabaseContext {
	/// Create a new DatabaseContext with the given client
	pub fn new(client: Client) -> Self {
		Self { client: Arc::new(Mutex::new(client)) }
	}

	/// Execute a query with the database client
	/// This is a convenience method that handles locking the client
	pub fn with_client<F, R>(&self, f: F) -> Result<R>
	where
		F: FnOnce(&mut Client) -> Result<R>,
	{
		let mut client = self.client.lock().map_err(|_| anyhow::anyhow!("Failed to acquire database lock"))?;
		f(&mut client)
	}

	/// Test the database connection
	pub fn test_connection(&self) -> Result<()> {
		self.with_client(|client| {
			client.simple_query("SELECT 1")?;
			Ok(())
		})
	}
}

impl std::fmt::Debug for DatabaseContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("DatabaseContext").field("client", &"<PostgreSQL Client>").finish()
	}
}
