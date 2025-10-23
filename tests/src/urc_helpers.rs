use alloy::node_bindings::AnvilInstance;
use alloy::primitives::{Address, B256};
use commit_boost::prelude::Chain;
use eyre::{Context, Result};
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tracing::info;

/// Deploy URC Registry contract to Anvil using the existing Forge script
/// This uses the deployment script from urc/script/Deploy.s.sol
pub async fn deploy_urc_to_anvil(anvil: &AnvilInstance) -> Result<Address> {
	info!("Deploying URC Registry contract to Anvil using Forge script...");

	// Get Anvil RPC URL
	let rpc_url = anvil.endpoint_url();

	info!("Running forge script to deploy URC...");

	// Spawn forge script with piped stdout/stderr so we can stream output
	let mut child = Command::new("forge")
		.current_dir("../urc")
		.args([
			"script",
			"script/Deploy.s.sol:DeployScript",
			"--sig",
			"deploy()",
			"--rpc-url",
			&rpc_url.to_string(),
			"--broadcast",
			"--private-key",
			"0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", // Anvil key 0
		])
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()?;

	// Read stdout line by line
	let stdout = child.stdout.take().ok_or_else(|| eyre::eyre!("No stdout from forge"))?;
	let reader = BufReader::new(stdout);

	let mut deployed_address: Option<Address> = None;
	let mut all_output = String::new();

	for line in reader.lines() {
		let line = line?;
		all_output.push_str(&line);
		all_output.push('\n');

		// Stream output for visibility
		info!("forge: {}", line);

		// Look for the specific pattern: "Registry deployed to: 0x..."
		if line.contains("Registry deployed to:") {
			if let Some(addr) = extract_registry_address(&line) {
				info!("Found Registry deployed address: {:?}", addr);
				deployed_address = Some(addr);
				// Keep reading to capture all output but we found our address
			}
		}
	}

	// Wait for the child process to complete
	let status = child.wait().context("Failed to wait for forge script")?;

	if !status.success() {
		// Read stderr for diagnostics
		let stderr = if let Some(mut stderr) = child.stderr {
			use std::io::Read;
			let mut buf = String::new();
			stderr.read_to_string(&mut buf).ok();
			buf
		} else {
			String::new()
		};

		return Err(eyre::eyre!(
			"Forge script failed with exit code: {:?}\nOutput:\n{}\nStderr:\n{}",
			status.code(),
			all_output,
			stderr
		));
	}

	// Return the deployed address
	let address =
		deployed_address.ok_or_else(|| eyre::eyre!("Could not find deployed contract address in forge output"))?;

	info!("URC Registry successfully deployed at: {:?}", address);

	Ok(address)
}

/// Compute the Commit-Boost signing_domain for the given chain and write it to urc/config/registry.json
pub fn prepare_urc_registry_config_for_chain(chain: Chain) -> Result<B256> {
	// Compute signing domain = DOMAIN_TYPE(4 bytes) || fork_data_root[0..28]
	// ForkData = { fork_version: [u8;4], genesis_validators_root: B256 }
	let mut domain_bytes = [0u8; 32];
	// DOMAIN_TYPE
	domain_bytes[..4].copy_from_slice(&common::constants::COMMIT_BOOST_DOMAIN);
	// fork_data_root = H(tree_hash(ForkData))
	let fork_version = chain.genesis_fork_version();
	// SSZ tree hash for ForkData's two 32-byte chunks using SHA256
	let mut leaf0 = [0u8; 32];
	leaf0[..4].copy_from_slice(&fork_version);
	let leaf1 = common::constants::GENESIS_VALIDATORS_ROOT;
	use sha2::{Digest, Sha256};
	let mut hasher = Sha256::new();
	hasher.update(&leaf0);
	hasher.update(&leaf1);
	let fork_data_root = hasher.finalize();

	domain_bytes[4..].copy_from_slice(&fork_data_root[..28]);
	let domain = B256::from_slice(&domain_bytes);

	// Resolve repo-relative path to urc/config/registry.json
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
	let mut config_path = PathBuf::from(manifest_dir);
	config_path.pop(); // tests -> repo root
	config_path.push("urc/config/registry.json");

	// Read and patch JSON
	let contents = std::fs::read_to_string(&config_path)
		.with_context(|| format!("Failed to read URC registry config at {}", config_path.display()))?;
	let mut json: Value = serde_json::from_str(&contents)
		.with_context(|| format!("Failed to parse JSON at {}", config_path.display()))?;

	// Write signingDomain as 0x-prefixed hex string
	let domain_hex = format!("0x{}", hex::encode(domain.as_slice()));
	json["signingDomain"] = Value::String(domain_hex);

	// Persist
	let updated = serde_json::to_string_pretty(&json)?;
	std::fs::write(&config_path, updated)
		.with_context(|| format!("Failed to write updated JSON to {}", config_path.display()))?;

	Ok(domain)
}

/// Deploy helper that first patches the signingDomain in config to match the provided chain
pub async fn deploy_urc_to_anvil_with_chain(anvil: &AnvilInstance, chain: Chain) -> Result<Address> {
	let _ = prepare_urc_registry_config_for_chain(chain)?;
	deploy_urc_to_anvil(anvil).await
}

/// Extract Registry address from the deployment log line
/// Looks for pattern: "Registry deployed to: 0x..."
fn extract_registry_address(line: &str) -> Option<Address> {
	// Find "Registry deployed to:" and extract address after it
	if let Some(pos) = line.find("Registry deployed to:") {
		let after_marker = &line[pos + "Registry deployed to:".len()..];
		// Skip whitespace and get the address
		let addr_str = after_marker.trim();

		// Extract exactly 42 characters (0x + 40 hex chars)
		if addr_str.len() >= 42 && addr_str.starts_with("0x") {
			let addr_candidate = &addr_str[..42];
			// Try to parse as Address
			if let Ok(addr) = addr_candidate.parse::<Address>() {
				return Some(addr);
			}
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::node_bindings::Anvil;

	#[test]
	fn test_extract_registry_address() {
		// Test with exact pattern from forge output
		let line = "Registry deployed to: 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0";
		let addr = extract_registry_address(line).unwrap();
		assert_eq!(addr, "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0".parse::<Address>().unwrap());

		// Test with extra whitespace
		let line = "Registry deployed to:   0x5FbDB2315678afecb367f032d93F642f64180aa3";
		let addr = extract_registry_address(line).unwrap();
		assert_eq!(addr, "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse::<Address>().unwrap());

		// Test with no match
		let line = "Some other output";
		let addr = extract_registry_address(line);
		assert!(addr.is_none());

		// Test with wrong pattern
		let line = "Contract deployed at: 0x5FbDB2315678afecb367f032d93F642f64180aa3";
		let addr = extract_registry_address(line);
		assert!(addr.is_none());
	}

	#[tokio::test]
	async fn test_deploy_urc_to_anvil() {
		let anvil = Anvil::new().spawn();
		let urc_address = deploy_urc_to_anvil(&anvil).await.unwrap();
		println!("URC address: {:?}", urc_address);
		assert_ne!(urc_address, Address::repeat_byte(0x42));
	}
}
