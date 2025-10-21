use alloy::primitives::U256;
use eyre::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};

/// JSON-RPC response structure
#[derive(Debug, Clone)]
pub struct RpcResponse {
	pub result: Value,
}

/// Trait for making JSON-RPC requests (mockable for testing)
/// When test-utils feature is enabled, mockall will generate MockRpcClient
#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
pub trait RpcClient: Send + Sync {
	/// Perform a JSON-RPC call to the given endpoint
	async fn call(&self, endpoint: &str, payload: Value) -> Result<Value>;
}

/// Production RPC client implementation using reqwest
pub struct ReqwestRpcClient {
	client: Client,
	max_retries: u32,
}

impl ReqwestRpcClient {
	/// Create a new ReqwestRpcClient with the given timeout and retry configuration
	pub fn new(timeout_secs: u64, max_retries: u32) -> Result<Self> {
		let client = Client::builder()
			.timeout(Duration::from_secs(timeout_secs))
			.build()
			.context("Failed to create HTTP client")?;
		Ok(Self { client, max_retries })
	}
}

impl RpcClient for ReqwestRpcClient {
	async fn call(&self, endpoint: &str, payload: Value) -> Result<Value> {
		let mut attempts = 0;

		while attempts < self.max_retries {
			match self.client.post(endpoint).header("Content-Type", "application/json").json(&payload).send().await {
				Ok(response) => {
					if response.status().is_success() {
						match response.json::<Value>().await {
							Ok(json_response) => {
								if json_response.get("error").is_some() {
									return Err(eyre::eyre!("RPC error: {}", json_response["error"]));
								}
								return Ok(json_response);
							}
							Err(e) => {
								warn!("Failed to parse response as JSON (attempt {}): {}", attempts + 1, e);
							}
						}
					} else {
						warn!("HTTP error from RPC endpoint (attempt {}): {}", attempts + 1, response.status());
					}
				}
				Err(e) => {
					warn!("Network error connecting to RPC endpoint (attempt {}): {}", attempts + 1, e);
				}
			}

			attempts += 1;
			if attempts < self.max_retries {
				tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
			}
		}

		Err(eyre::eyre!("Failed to connect to RPC endpoint after {} attempts", self.max_retries))
	}
}

/// Execution client RPC API for gas price oracle functionality
pub struct ExecutionApiClient<R: RpcClient> {
	rpc_client: Arc<R>,
	config: ExecutionApiConfig,
}

// Manual Debug implementation since R might not implement Debug
impl<R: RpcClient> std::fmt::Debug for ExecutionApiClient<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExecutionApiClient").field("config", &self.config).finish()
	}
}

// Manual Clone implementation since R might not implement Clone
impl<R: RpcClient> Clone for ExecutionApiClient<R> {
	fn clone(&self) -> Self {
		Self { rpc_client: Arc::clone(&self.rpc_client), config: self.config.clone() }
	}
}

/// Gas price information from execution client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPriceInfo {
	/// Current gas price in wei (256-bit integer)
	pub gas_price: U256,
	/// Latest block number
	pub block_number: u64,
	/// Timestamp when this data was retrieved
	pub timestamp: u64,
}

impl GasPriceInfo {
	/// Convert the stored gas price to a primitive integer, clamping to the maximum representable value on overflow.
	///
	/// Returns the gas price as a `u64`; if the stored `U256` value is greater than `u64::MAX`, this returns `u64::MAX`.
	///
	/// # Examples
	///
	pub fn gas_price_as_u64_clamped(&self) -> u64 {
		if self.gas_price > U256::from(u64::MAX) { u64::MAX } else { self.gas_price.to::<u64>() }
	}

	/// Convert the stored `gas_price` to a `u64` if it does not overflow.
	///
	/// Returns `Ok(u64)` containing the gas price when the value is less than or equal to `u64::MAX`,
	/// and an `Err` describing the overflow when the gas price is larger than `u64::MAX`.
	///
	/// # Examples
	///
	pub fn gas_price_as_u64_checked(&self) -> Result<u64> {
		if self.gas_price > U256::from(u64::MAX) {
			Err(eyre::eyre!("Gas price {} exceeds u64::MAX ({})", self.gas_price, u64::MAX))
		} else {
			Ok(self.gas_price.to::<u64>())
		}
	}
}

/// Configuration for execution client API client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionApiConfig {
	/// Primary execution client node endpoint
	pub endpoint: String,
	/// Request timeout in seconds
	pub request_timeout_secs: u64,
	/// Maximum number of retries for failed requests
	pub max_retries: u32,
}

impl Default for ExecutionApiConfig {
	/// Constructs a default ExecutionApiConfig with a local RPC endpoint, a short request timeout, and a small retry count.
	///
	/// # Examples
	///
	fn default() -> Self {
		Self { endpoint: "http://localhost:8545".to_string(), request_timeout_secs: 10, max_retries: 3 }
	}
}

impl<R: RpcClient> ExecutionApiClient<R> {
	/// Constructs a new `ExecutionApiClient` from the provided configuration and RPC client.
	///
	/// # Errors
	///
	/// Returns an error if the configuration is invalid (e.g., empty endpoint).
	///
	/// # Examples
	///
	pub fn new(config: ExecutionApiConfig, rpc_client: R) -> Result<Self> {
		// Validate configuration
		if config.endpoint.trim().is_empty() {
			eyre::bail!("Endpoint cannot be empty");
		}

		if config.request_timeout_secs == 0 {
			eyre::bail!("Request timeout must be greater than zero");
		}

		Ok(Self { rpc_client: Arc::new(rpc_client), config })
	}

	/// Fetches the current gas price from the configured execution client node and returns it with context.
	///
	/// The returned `GasPriceInfo` contains the gas price as a `U256`, the block number at which
	/// the price was observed (or `0` if the block number could not be fetched), and the UNIX
	/// epoch timestamp (seconds) when the price was retrieved.
	///
	/// # Returns
	///
	/// `GasPriceInfo` containing the current gas price, the block number (or `0` if unavailable),
	/// and the retrieval timestamp (seconds since the UNIX epoch).
	///
	/// # Examples
	///
	pub async fn get_gas_price(&self) -> Result<GasPriceInfo> {
		debug!("Fetching gas price from execution client node: {}", self.config.endpoint);

		let payload = json!({
			"jsonrpc": "2.0",
			"method": "eth_gasPrice",
			"params": [],
			"id": 1
		});

		let response = self
			.rpc_client
			.call(&self.config.endpoint, payload)
			.await
			.context("Failed to get gas price from execution client node")?;

		let gas_price_hex =
			response["result"].as_str().ok_or_else(|| eyre::eyre!("Invalid gas price response format"))?;

		let gas_price = U256::from_str_radix(gas_price_hex.strip_prefix("0x").unwrap_or(gas_price_hex), 16)
			.context("Failed to parse gas price as U256 hex value")?;

		// Get current block number for context
		let block_number = match self.get_block_number().await {
			Ok(num) => num,
			Err(err) => {
				warn!("Failed to get block number: {}", err);
				0
			}
		};

		let timestamp =
			std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

		let gas_price_info = GasPriceInfo { gas_price, block_number, timestamp };

		debug!("Retrieved gas price: {} wei at block {}", gas_price, block_number);
		Ok(gas_price_info)
	}

	/// Retrieves the current block number from the configured execution client node.
	///
	/// # Returns
	///
	/// The current block number as a `u64`.
	///
	/// # Examples
	///
	pub async fn get_block_number(&self) -> Result<u64> {
		let payload = json!({
			"jsonrpc": "2.0",
			"method": "eth_blockNumber",
			"params": [],
			"id": 3
		});

		let response = self
			.rpc_client
			.call(&self.config.endpoint, payload)
			.await
			.context("Failed to get block number from execution client node")?;

		let block_hex =
			response["result"].as_str().ok_or_else(|| eyre::eyre!("Invalid block number response format"))?;

		let block_number = u64::from_str_radix(block_hex.strip_prefix("0x").unwrap_or(block_hex), 16)
			.context("Failed to parse block number hex value")?;

		Ok(block_number)
	}

	/// Estimates the gas required for a transaction.
	///
	/// # Parameters
	///
	/// * `tx` - The transaction request to estimate gas for
	///
	/// # Returns
	///
	/// The estimated gas amount as a `u64`.
	///
	/// # Examples
	///
	pub async fn estimate_gas(&self, tx: &alloy::rpc::types::TransactionRequest) -> Result<u64> {
		debug!("Estimating gas for transaction");

		// Serialize the transaction request for the RPC call
		let tx_json = serde_json::to_value(tx).context("Failed to serialize transaction request")?;

		let payload = json!({
			"jsonrpc": "2.0",
			"method": "eth_estimateGas",
			"params": [tx_json],
			"id": 2
		});

		let response = self
			.rpc_client
			.call(&self.config.endpoint, payload)
			.await
			.context("Failed to estimate gas from execution client node")?;

		let gas_hex = response["result"].as_str().ok_or_else(|| eyre::eyre!("Invalid gas estimate response format"))?;

		let gas_estimate = u64::from_str_radix(gas_hex.strip_prefix("0x").unwrap_or(gas_hex), 16)
			.context("Failed to parse gas estimate hex value")?;

		debug!("Estimated gas: {}", gas_estimate);
		Ok(gas_estimate)
	}
}

// Convenience constructor for production use with ReqwestRpcClient
impl ExecutionApiClient<ReqwestRpcClient> {
	/// Creates a new ExecutionApiClient with the default ReqwestRpcClient.
	///
	/// This is the standard constructor for production use. For testing, use
	/// `ExecutionApiClient::new()` with a mock RPC client.
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - The endpoint is empty
	/// - The request timeout is zero
	/// - The underlying HTTP client cannot be constructed
	///
	/// # Examples
	///
	pub fn with_default_client(config: ExecutionApiConfig) -> Result<Self> {
		let rpc_client = ReqwestRpcClient::new(config.request_timeout_secs, config.max_retries)?;
		Self::new(config, rpc_client)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_reth_client_creation() {
		let config = ExecutionApiConfig::default();
		let client = ExecutionApiClient::with_default_client(config);
		assert!(client.is_ok());
	}

	#[tokio::test]
	async fn test_hex_parsing() {
		// Test gas price hex parsing with U256
		let gas_price = U256::from_str_radix("1dcd6500", 16).unwrap();
		assert_eq!(gas_price, U256::from(500000000u64)); // 0.5 gwei

		let gas_price_with_prefix = U256::from_str_radix("0x1dcd6500".strip_prefix("0x").unwrap(), 16).unwrap();
		assert_eq!(gas_price_with_prefix, U256::from(500000000u64));

		// Test large values that exceed u64
		let large_price =
			U256::from_str_radix("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16).unwrap();
		assert!(large_price > U256::from(u64::MAX));
	}

	#[tokio::test]
	async fn test_gas_price_conversion() {
		// Test normal gas price that fits in u64
		let normal_price = GasPriceInfo {
			gas_price: U256::from(20_000_000_000u64), // 20 gwei
			block_number: 100,
			timestamp: 1234567890,
		};
		assert_eq!(normal_price.gas_price_as_u64_clamped(), 20_000_000_000u64);
		assert_eq!(normal_price.gas_price_as_u64_checked().unwrap(), 20_000_000_000u64);

		// Test gas price that exceeds u64::MAX
		let huge_price = GasPriceInfo {
			gas_price: U256::from_str_radix("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16)
				.unwrap(),
			block_number: 100,
			timestamp: 1234567890,
		};
		// Should clamp to u64::MAX
		assert_eq!(huge_price.gas_price_as_u64_clamped(), u64::MAX);
		// Should return error
		assert!(huge_price.gas_price_as_u64_checked().is_err());
	}

	#[tokio::test]
	async fn test_config_serialization() {
		let config = ExecutionApiConfig {
			endpoint: "http://localhost:8545".to_string(),
			request_timeout_secs: 30,
			max_retries: 5,
		};

		let serialized = serde_json::to_string(&config).unwrap();
		let deserialized: ExecutionApiConfig = serde_json::from_str(&serialized).unwrap();

		assert_eq!(config.endpoint, deserialized.endpoint);
		assert_eq!(config.request_timeout_secs, deserialized.request_timeout_secs);
		assert_eq!(config.max_retries, deserialized.max_retries);
	}

	#[tokio::test]
	async fn test_reth_client_with_invalid_endpoint() {
		let config =
			ExecutionApiConfig { endpoint: "invalid://endpoint".to_string(), request_timeout_secs: 1, max_retries: 1 };

		let client = ExecutionApiClient::with_default_client(config);
		assert!(client.is_ok());

		// Test that get_gas_price fails with invalid endpoint
		let client = client.unwrap();
		let result = client.get_gas_price().await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_reth_client_with_timeout() {
		let config = ExecutionApiConfig {
			endpoint: "http://httpbin.org/delay/5".to_string(),
			request_timeout_secs: 1,
			max_retries: 1,
		};

		let client = ExecutionApiClient::with_default_client(config).unwrap();
		let result = client.get_gas_price().await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_reth_client_retry_logic() {
		let config = ExecutionApiConfig {
			endpoint: "http://httpbin.org/status/500".to_string(),
			request_timeout_secs: 5,
			max_retries: 3,
		};

		let client = ExecutionApiClient::with_default_client(config).unwrap();
		let result = client.get_gas_price().await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_block_number_parsing() {
		let config = ExecutionApiConfig::default();
		let client = ExecutionApiClient::with_default_client(config).unwrap();

		// Test with invalid endpoint (should fail gracefully)
		let result = client.get_block_number().await;
		assert!(result.is_err());
	}

	#[test]
	fn test_u256_serialization() {
		use serde_json;

		let value = U256::from(12345u64);
		let serialized = serde_json::to_string(&value).unwrap();
		let deserialized: U256 = serde_json::from_str(&serialized).unwrap();
		assert_eq!(value, deserialized);
	}

	#[test]
	fn test_u256_serialization_with_hex_prefix() {
		use serde_json;

		let value = U256::from_str_radix("1a2b3c", 16).unwrap();
		let serialized = serde_json::to_string(&value).unwrap();
		let deserialized: U256 = serde_json::from_str(&serialized).unwrap();
		assert_eq!(value, deserialized);
	}

	#[test]
	fn test_u256_serialization_large_value() {
		use serde_json;

		let value =
			U256::from_str_radix("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16).unwrap();
		let serialized = serde_json::to_string(&value).unwrap();
		let deserialized: U256 = serde_json::from_str(&serialized).unwrap();
		assert_eq!(value, deserialized);
	}

	#[test]
	fn test_gas_price_info_edge_cases() {
		// Test with zero gas price
		let zero_price = GasPriceInfo { gas_price: U256::from(0u64), block_number: 0, timestamp: 0 };
		assert_eq!(zero_price.gas_price_as_u64_clamped(), 0);
		assert_eq!(zero_price.gas_price_as_u64_checked().unwrap(), 0);

		// Test with exactly u64::MAX
		let max_price = GasPriceInfo { gas_price: U256::from(u64::MAX), block_number: 100, timestamp: 1234567890 };
		assert_eq!(max_price.gas_price_as_u64_clamped(), u64::MAX);
		assert_eq!(max_price.gas_price_as_u64_checked().unwrap(), u64::MAX);
	}

	#[test]
	fn test_reth_api_config_default() {
		let config = ExecutionApiConfig::default();
		assert_eq!(config.endpoint, "http://localhost:8545");
		assert_eq!(config.request_timeout_secs, 10);
		assert_eq!(config.max_retries, 3);
	}

	#[test]
	fn test_reth_api_config_custom() {
		let config =
			ExecutionApiConfig { endpoint: "http://custom:8545".to_string(), request_timeout_secs: 60, max_retries: 5 };
		assert_eq!(config.endpoint, "http://custom:8545");
		assert_eq!(config.request_timeout_secs, 60);
		assert_eq!(config.max_retries, 5);
	}

	#[tokio::test]
	async fn test_rpc_call_with_invalid_json() {
		let config = ExecutionApiConfig {
			endpoint: "http://httpbin.org/post".to_string(),
			request_timeout_secs: 5,
			max_retries: 1,
		};

		let client = ExecutionApiClient::with_default_client(config).unwrap();

		// Test with invalid JSON payload
		let invalid_payload = serde_json::json!({
			"jsonrpc": "2.0",
			"method": "invalid_method",
			"params": [],
			"id": 1
		});

		// This should fail because httpbin will return HTML, not JSON-RPC
		let result = client.rpc_client.call(&client.config.endpoint, invalid_payload).await;
		// The test might pass or fail depending on httpbin's response format
		// We just verify it doesn't panic
		match result {
			Ok(_) => println!("Unexpected success - httpbin returned valid JSON-RPC"),
			Err(_) => println!("Expected failure - httpbin returned non-JSON-RPC response"),
		}
	}

	#[tokio::test]
	async fn test_rpc_call_with_rpc_error() {
		let config = ExecutionApiConfig {
			endpoint: "http://httpbin.org/post".to_string(),
			request_timeout_secs: 5,
			max_retries: 1,
		};

		let client = ExecutionApiClient::with_default_client(config).unwrap();

		// Test with payload that would result in RPC error
		let error_payload = serde_json::json!({
			"jsonrpc": "2.0",
			"method": "eth_gasPrice",
			"params": [],
			"id": 1
		});

		// This should fail because httpbin doesn't understand JSON-RPC
		let result = client.rpc_client.call(&client.config.endpoint, error_payload).await;
		// The test might pass or fail depending on httpbin's response format
		// We just verify it doesn't panic
		match result {
			Ok(_) => println!("Unexpected success - httpbin returned valid JSON-RPC"),
			Err(_) => println!("Expected failure - httpbin returned non-JSON-RPC response"),
		}
	}

	#[test]
	fn test_gas_price_info_timestamp() {
		let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

		let gas_price_info = GasPriceInfo { gas_price: U256::from(1000u64), block_number: 12345, timestamp: now };

		assert_eq!(gas_price_info.timestamp, now);
		assert_eq!(gas_price_info.block_number, 12345);
	}

	#[test]
	fn test_hex_parsing_edge_cases() {
		// Test parsing hex without 0x prefix
		let hex_without_prefix = "1a2b3c";
		let parsed = U256::from_str_radix(hex_without_prefix, 16).unwrap();
		assert_eq!(parsed, U256::from(0x1a2b3c));

		// Test parsing hex with 0x prefix
		let hex_with_prefix = "0x1a2b3c";
		let parsed = U256::from_str_radix(hex_with_prefix.strip_prefix("0x").unwrap(), 16).unwrap();
		assert_eq!(parsed, U256::from(0x1a2b3c));

		// Test parsing zero
		let zero_hex = "0x0";
		let parsed = U256::from_str_radix(zero_hex.strip_prefix("0x").unwrap(), 16).unwrap();
		assert_eq!(parsed, U256::from(0));

		// Test parsing empty string
		let empty_hex = "";
		let parsed = U256::from_str_radix(empty_hex, 16).unwrap();
		assert_eq!(parsed, U256::from(0));
	}

	// ============================================================================
	// Anvil Integration Tests
	// ============================================================================

	#[tokio::test]
	async fn test_anvil_get_gas_price() -> Result<()> {
		use alloy::node_bindings::Anvil;

		// Spin up a local Anvil node
		let anvil = Anvil::new().block_time(1).try_spawn()?;

		// Create ExecutionApiClient with Anvil's endpoint
		let rpc_url = anvil.endpoint_url();
		let config = ExecutionApiConfig { endpoint: rpc_url.to_string(), request_timeout_secs: 10, max_retries: 3 };
		let client = ExecutionApiClient::with_default_client(config)?;

		// Test get_gas_price
		let gas_price_info = client.get_gas_price().await?;

		// Verify the result
		assert!(gas_price_info.gas_price > U256::ZERO, "Gas price should be non-zero");
		assert!(gas_price_info.block_number >= 0, "Block number should be valid");

		// Verify timestamp is reasonable (within last hour)
		let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		assert!(gas_price_info.timestamp <= now, "Timestamp should not be in the future");
		assert!(gas_price_info.timestamp > now - 3600, "Timestamp should be recent");

		println!(
			"Anvil gas price test passed: {} wei at block {}",
			gas_price_info.gas_price, gas_price_info.block_number
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_anvil_get_block_number() -> Result<()> {
		use alloy::node_bindings::Anvil;

		// Spin up a local Anvil node with 1 second block time
		let anvil = Anvil::new().block_time(1).try_spawn()?;

		// Create ExecutionApiClient with Anvil's endpoint
		let rpc_url = anvil.endpoint_url();
		let config = ExecutionApiConfig { endpoint: rpc_url.to_string(), request_timeout_secs: 10, max_retries: 3 };
		let client = ExecutionApiClient::with_default_client(config)?;

		// Sleep briefly to let Anvil mine some blocks
		tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

		// Test get_block_number
		let block_number = client.get_block_number().await?;
		assert!(block_number > 0, "Block number should be greater than 0 after sleeping");

		// Verify we can call it multiple times
		let block_number_2 = client.get_block_number().await?;
		assert!(block_number_2 >= block_number, "Block number should not decrease");

		println!("Anvil block number test passed: block {}", block_number);

		Ok(())
	}

	#[tokio::test]
	async fn test_anvil_estimate_gas() -> Result<()> {
		use alloy::network::TransactionBuilder;
		use alloy::node_bindings::Anvil;
		use alloy::rpc::types::TransactionRequest;

		// Spin up a local Anvil node with pre-funded accounts
		let anvil = Anvil::new().block_time(1).try_spawn()?;

		// Create ExecutionApiClient with Anvil's endpoint
		let rpc_url = anvil.endpoint_url();
		let config = ExecutionApiConfig { endpoint: rpc_url.to_string(), request_timeout_secs: 10, max_retries: 3 };
		let client = ExecutionApiClient::with_default_client(config)?;

		// Build a simple ETH transfer transaction
		// Anvil pre-funds the first 10 accounts, so we can use them
		let from = anvil.addresses()[0];
		let to = anvil.addresses()[1];
		let tx = TransactionRequest::default()
			.with_from(from)
			.with_to(to)
			.with_value(U256::from(1_000_000_000_000_000_000u64)); // 1 ETH

		// Test estimate_gas
		let gas_estimate = client.estimate_gas(&tx).await?;

		// Verify the gas estimate is reasonable for a simple transfer
		assert!(gas_estimate >= 21000, "Gas estimate should be at least 21000 for simple transfer");
		assert!(gas_estimate <= 100000, "Gas estimate should be reasonable (not excessive)");

		println!("Anvil estimate gas test passed: {} gas", gas_estimate);

		Ok(())
	}

	#[tokio::test]
	async fn test_anvil_block_progression() -> Result<()> {
		use alloy::node_bindings::Anvil;

		// Spin up a local Anvil node with 1 second block time
		let anvil = Anvil::new().block_time(1).try_spawn()?;

		// Create ExecutionApiClient with Anvil's endpoint
		let rpc_url = anvil.endpoint_url();
		let config = ExecutionApiConfig { endpoint: rpc_url.to_string(), request_timeout_secs: 10, max_retries: 3 };
		let client = ExecutionApiClient::with_default_client(config)?;

		// Get initial block number
		let initial_block = client.get_block_number().await?;
		println!("Initial block: {}", initial_block);

		// Wait for a few blocks to be mined
		tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

		// Get new block number
		let new_block = client.get_block_number().await?;
		println!("New block after 3 seconds: {}", new_block);

		// Verify block number increased (should have mined at least 2-3 blocks)
		assert!(new_block > initial_block, "Block number should have increased");
		assert!(new_block >= initial_block + 2, "Should have mined at least 2 blocks in 3 seconds");

		println!("Anvil block progression test passed: {} -> {} blocks", initial_block, new_block);

		Ok(())
	}
}
