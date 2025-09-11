pub mod context;

pub use context::DatabaseContext;

use std::env;

use anyhow::{Context, Result};
use postgres::{Client, NoTls};

/// Initialize a PostgreSQL database connection
pub fn create_connection() -> Result<Client> {
	let database_url =
		env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/preconfirmation_gateway".to_string());

	let client = Client::connect(&database_url, NoTls)
		.with_context(|| format!("Failed to connect to database: {}", database_url))?;

	Ok(client)
}

/// Test the database connection
pub fn test_connection(client: &mut Client) -> Result<()> {
	client.simple_query("SELECT 1").with_context(|| "Failed to execute test query")?;

	tracing::info!("Database connection test successful");
	Ok(())
}
