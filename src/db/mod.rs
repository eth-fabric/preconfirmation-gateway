
use std::env;

use anyhow::{Context, Result};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

/// Create a PostgreSQL connection pool
pub async fn create_pool() -> Result<Pool> {
	let database_url =
		env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/preconfirmation_gateway".to_string());

	let mut cfg = Config::new();
	cfg.url = Some(database_url.clone());
	
	let pool = cfg
		.create_pool(Some(Runtime::Tokio1), NoTls)
		.with_context(|| format!("Failed to create connection pool for database: {}", database_url))?;

	Ok(pool)
}

/// Test the database connection pool
pub async fn test_connection(pool: &Pool) -> Result<()> {
	let client = pool.get().await.with_context(|| "Failed to get connection from pool")?;
	
	client.simple_query("SELECT 1").await.with_context(|| "Failed to execute test query")?;

	tracing::info!("Database connection test successful");
	Ok(())
}
