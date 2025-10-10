use common::types::DatabaseContext;
use eyre::Result;
use tracing::{debug, info};

/// Pricer service that calculates and stores gas prices
pub struct PricerService {
	database: DatabaseContext,
}

impl PricerService {
	/// Create a new pricer service
	pub fn new(database: DatabaseContext) -> Self {
		Self { database }
	}

	/// Update the current price in the database
	pub async fn update_price(&self) -> Result<()> {
		debug!("Updating price in pricer service");

		// TODO: Query RPC provider for recent block base fees
		// TODO: Calculate average/median base fee from recent blocks
		// For now: placeholder calculation logic
		let calculated_price_gwei = self.calculate_price().await?;

		// Store the calculated price
		self.store_price(calculated_price_gwei)?;

		info!("Updated price to {} gwei", calculated_price_gwei);
		Ok(())
	}

	/// Get the latest price from the database
	pub fn get_latest_price(&self) -> Result<u64> {
		self.get_price()
	}

	/// Calculate price based on recent blockchain data
	async fn calculate_price(&self) -> Result<u64> {
		// TODO: Integrate with external RPC provider (Alloy)
		// TODO: Query recent block base fees
		// TODO: Calculate statistical price (median/average)

		// Placeholder: return a fixed price for now
		// This will be replaced with actual RPC queries
		Ok(1) // 1 gwei placeholder
	}

	/// Store a price in the database (overwrites previous)
	fn store_price(&self, price_gwei: u64) -> Result<()> {
		self.database.store_latest_price(price_gwei).map_err(|e| eyre::eyre!("Database error: {}", e))?;
		debug!("Stored price {} gwei in database", price_gwei);
		Ok(())
	}

	/// Retrieve the latest price from the database
	fn get_price(&self) -> Result<u64> {
		match self.database.get_latest_price().map_err(|e| eyre::eyre!("Database error: {}", e))? {
			Some(price) => {
				debug!("Retrieved price {} gwei from database", price);
				Ok(price)
			}
			None => Err(eyre::eyre!("No price found in database")),
		}
	}
}
