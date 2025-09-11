use anyhow::Result;
use deadpool_postgres::Pool;

/// Database context that provides access to the PostgreSQL connection pool
#[derive(Clone)]
pub struct DatabaseContext {
	pool: Pool,
}

impl DatabaseContext {
	/// Create a new DatabaseContext with the given pool
	pub fn new(pool: Pool) -> Self {
		Self { pool }
	}

	/// Execute a function with a database client from the pool
	pub async fn with_client<F, Fut, R>(&self, f: F) -> Result<R>
	where
		F: FnOnce(deadpool_postgres::Client) -> Fut,
		Fut: std::future::Future<Output = Result<R>>,
	{
		let client = self.pool.get().await?;
		f(client).await
	}

	/// Get a client from the connection pool
	pub async fn client(&self) -> Result<deadpool_postgres::Client> {
		Ok(self.pool.get().await?)
	}

	/// Test the database connection
	pub async fn test_connection(&self) -> Result<()> {
		let client = self.pool.get().await?;
		client.simple_query("SELECT 1").await?;
		Ok(())
	}
}

impl std::fmt::Debug for DatabaseContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("DatabaseContext").field("client", &"<PostgreSQL Client>").finish()
	}
}