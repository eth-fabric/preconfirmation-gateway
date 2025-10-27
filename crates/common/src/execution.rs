use alloy::network::Ethereum;
use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

/// Build a Provider<Ethereum> using Alloy's ProviderBuilder.
///
/// This replaces the custom execution client and should be used for all JSON-RPC
/// interactions (gas price, gas estimation, block queries, etc.).
pub fn build_eth_provider(endpoint: &str) -> Result<impl Provider<Ethereum>> {
	Ok(ProviderBuilder::new().network::<Ethereum>().connect_http(endpoint.parse()?))
}
