use commit_boost::prelude::BlsPublicKey;
use eyre::Result;

/// Parse a BLS public key from a hexadecimal string.
///
/// Accepts strings with or without `0x` prefix. Returns an error if the
/// decoded byte length is not 48 or deserialization fails.
pub fn bls_pubkey_from_hex(hex_str: &str) -> Result<BlsPublicKey, eyre::Error> {
	let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
	let bytes = hex::decode(hex_str)?;
	if bytes.len() != 48 {
		return Err(eyre::eyre!("Invalid BLS public key length: expected 48 bytes, got {}", bytes.len()));
	}
	BlsPublicKey::deserialize(&bytes).map_err(|e| eyre::eyre!("Failed to deserialize BLS public key: {:?}", e))
}

#[cfg(unix)]
pub async fn wait_for_signal() -> eyre::Result<()> {
	use tokio::signal::unix::{signal, SignalKind};

	let mut sigint = signal(SignalKind::interrupt())?;
	let mut sigterm = signal(SignalKind::terminate())?;

	tokio::select! {
		_ = sigint.recv() => {}
		_ = sigterm.recv() => {}
	}

	Ok(())
}

#[cfg(windows)]
pub async fn wait_for_signal() -> eyre::Result<()> {
	tokio::signal::ctrl_c().await?;
	Ok(())
}
