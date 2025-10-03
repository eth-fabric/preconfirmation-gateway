use std::env;

use anyhow::{Context, Result};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::config::InclusionPreconfConfig;
use commit_boost::prelude::StartCommitModuleConfig;

/// Create a PostgreSQL connection pool from commit-boost config
pub async fn create_pool(commit_config: &StartCommitModuleConfig<InclusionPreconfConfig>) -> Result<Pool> {
	let app_config = &commit_config.extra;
	// Environment variable takes precedence over config file
	let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| app_config.database_url.clone());

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
