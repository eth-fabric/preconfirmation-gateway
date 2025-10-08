use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use rocksdb::{DB, Options};

use crate::config::InclusionPreconfConfig;
use commit_boost::prelude::StartCommitModuleConfig;

/// Create a RocksDB database from commit-boost config
pub fn create_database(commit_config: &StartCommitModuleConfig<InclusionPreconfConfig>) -> Result<Arc<DB>> {
	let app_config = &commit_config.extra;
	// Environment variable takes precedence over config file
	let database_path = env::var("DATABASE_PATH").unwrap_or_else(|_| app_config.commitments_database_url.clone());

	// Create database directory if it doesn't exist
	std::fs::create_dir_all(&database_path)
		.with_context(|| format!("Failed to create database directory: {}", database_path))?;

	// Configure RocksDB options
	let mut opts = Options::default();
	opts.create_if_missing(true);
	opts.create_missing_column_families(true);

	// Open the database
	let db = DB::open(&opts, &database_path)
		.with_context(|| format!("Failed to open RocksDB database at: {}", database_path))?;

	tracing::info!("RocksDB database opened successfully at: {}", database_path);
	Ok(Arc::new(db))
}

/// Perform a health check on the database
pub async fn db_healthcheck(db: &Arc<DB>) -> Result<()> {
	// Test by putting and getting a test value
	let test_key = b"test_connection";
	let test_value = b"test_value";

	db.put(test_key, test_value).with_context(|| "Failed to put test value")?;

	let retrieved = db.get(test_key).with_context(|| "Failed to get test value")?;

	if let Some(value) = retrieved {
		if value == test_value {
			db.delete(test_key).with_context(|| "Failed to delete test value")?; // Clean up test data
			tracing::info!("Database connection test successful");
			Ok(())
		} else {
			Err(anyhow::anyhow!("Database test failed: value mismatch"))
		}
	} else {
		Err(anyhow::anyhow!("Database test failed: value not found"))
	}
}
