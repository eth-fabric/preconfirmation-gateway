use eyre::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, error, info};

use common::constants::routes;
use common::types::{SignedConstraints, SignedDelegation};

/// HTTP client for communicating with constraints relay
pub struct ConstraintsClient {
	client: Client,
	base_url: String,
	api_key: Option<String>,
}

/// Response from posting constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostConstraintsResponse {
	pub success: bool,
	pub message: String,
}

/// Response from getting delegations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDelegationsResponse {
	pub delegations: Vec<SignedDelegation>,
}

impl ConstraintsClient {
	/// Create a new constraints client
	pub fn new(base_url: String, api_key: Option<String>) -> Self {
		let client = Client::builder().timeout(Duration::from_secs(30)).build().expect("Failed to create HTTP client");

		Self { client, base_url, api_key }
	}

	/// POST signed constraints to relay
	pub async fn post_constraints(&self, signed_constraints: &SignedConstraints) -> Result<()> {
		let url = format!("{}{}", self.base_url, routes::relay::CONSTRAINTS);

		debug!("Posting constraints to: {}", url);

		let mut request = self.client.post(&url).json(signed_constraints);

		// Add API key if provided
		if let Some(api_key) = &self.api_key {
			request = request.header("Authorization", format!("Bearer {}", api_key));
		}

		let response = request.send().await?;

		if response.status().is_success() {
			info!("Successfully posted constraints (status: {})", response.status());
			Ok(())
		} else {
			let status = response.status();
			let error_text = response.text().await.unwrap_or_default();
			error!("Failed to post constraints: {} - {}", status, error_text);
			Err(eyre::eyre!("Failed to post constraints (status {}): {}", status, error_text))
		}
	}

	/// GET delegations from relay for a specific slot
	pub async fn get_delegations_for_slot(&self, slot: u64) -> Result<Vec<SignedDelegation>> {
		let url = format!("{}{}", self.base_url, routes::relay::DELEGATIONS_SLOT.replace("{slot}", &slot.to_string()));

		debug!("Getting delegations for slot {} from: {}", slot, url);

		let mut request = self.client.get(&url);

		// Add API key if provided
		if let Some(api_key) = &self.api_key {
			request = request.header("Authorization", format!("Bearer {}", api_key));
		}

		let response = request.send().await?;

		if response.status().is_success() {
			let result: GetDelegationsResponse = response.json().await?;
			info!("Successfully retrieved {} delegations for slot {}", result.delegations.len(), slot);
			Ok(result.delegations)
		} else {
			let status = response.status();
			let error_text = response.text().await.unwrap_or_default();
			error!("Failed to get delegations for slot {}: {} - {}", slot, status, error_text);
			Err(eyre::eyre!("Failed to get delegations for slot {}: {} - {}", slot, status, error_text))
		}
	}

	/// Health check for the relay
	pub async fn health_check(&self) -> Result<bool> {
		let url = format!("{}{}", self.base_url, routes::relay::HEALTH);

		debug!("Checking relay health at: {}", url);

		let response = self.client.get(&url).timeout(Duration::from_secs(5)).send().await?;

		Ok(response.status().is_success())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use common::constants::defaults;

	#[test]
	fn test_client_creation() {
		let client = ConstraintsClient::new(defaults::RELAY_URL.to_string(), Some("test-api-key".to_string()));

		assert_eq!(client.base_url, defaults::RELAY_URL);
		assert_eq!(client.api_key, Some("test-api-key".to_string()));
	}

	#[test]
	fn test_client_without_api_key() {
		let client = ConstraintsClient::new(defaults::RELAY_URL.to_string(), None);

		assert_eq!(client.base_url, defaults::RELAY_URL);
		assert_eq!(client.api_key, None);
	}
}
