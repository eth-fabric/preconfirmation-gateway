pub mod context;

pub use context::DatabaseContext;

use std::env;

use anyhow::{Context, Result};
use tokio_postgres::{Client, NoTls};

/// Initialize a PostgreSQL database connection
pub async fn create_connection() -> Result<Client> {
	let database_url =
		env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/preconfirmation_gateway".to_string());

	let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
		.await
		.with_context(|| format!("Failed to connect to database: {}", database_url))?;

	// Spawn the connection task to handle the connection in the background
	tokio::spawn(async move {
		if let Err(e) = connection.await {
			tracing::error!("Database connection error: {}", e);
		}
	});

	Ok(client)
}

/// Test the database connection
pub async fn test_connection(client: &Client) -> Result<()> {
	client.simple_query("SELECT 1").await.with_context(|| "Failed to execute test query")?;

	tracing::info!("Database connection test successful");
	Ok(())
}
