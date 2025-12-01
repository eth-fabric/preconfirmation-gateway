use alloy_primitives::{B256, U256};
use eth_trie::{EthTrie, MemoryDB, Trie};
use ethereum_types::H256;
use eyre::{Context, Result, eyre};
use reth_primitives::TransactionSigned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents a Merkle inclusion proof for a constraint transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintProof {
	/// Transaction hash
	pub tx_hash: B256,
	/// Index of the transaction in the block
	pub tx_index: usize,
	/// Merkle proof nodes (hex-encoded)
	#[serde(with = "hex_vec")]
	pub proof: Vec<Vec<u8>>,
	/// Transaction root hash
	pub transactions_root: B256,
	/// Block number
	pub block_number: u64,
	/// Slot number
	pub slot: u64,
}

/// Serde helper for hex encoding Vec<Vec<u8>>
mod hex_vec {
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(data: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		use serde::ser::SerializeSeq;
		let mut seq = serializer.serialize_seq(Some(data.len()))?;
		for item in data {
			seq.serialize_element(&format!("0x{}", hex::encode(item)))?;
		}
		seq.end()
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let strings: Vec<String> = Vec::deserialize(deserializer)?;
		strings
			.into_iter()
			.map(|s| {
				let s = s.strip_prefix("0x").unwrap_or(&s);
				hex::decode(s).map_err(serde::de::Error::custom)
			})
			.collect()
	}
}

/// Builder for transaction Merkle Patricia Trie
pub struct TransactionTrieBuilder {
	trie: EthTrie<MemoryDB>,
	transactions: Vec<B256>,
}

impl TransactionTrieBuilder {
	/// Create a new trie builder
	pub fn new() -> Self {
		let memdb = Arc::new(MemoryDB::new(true));
		let trie = EthTrie::new(memdb);
		Self { trie, transactions: Vec::new() }
	}

	/// Build the transaction trie from a list of signed transactions
	pub fn build(transactions: &[TransactionSigned]) -> Result<Self> {
		let mut builder = Self::new();

		for (idx, tx) in transactions.iter().enumerate() {
			// Key is RLP-encoded index
			let key = alloy_rlp::encode(U256::from(idx));

			// Value is RLP-encoded signed transaction
			let tx_bytes = alloy_rlp::encode(tx);

			builder
				.trie
				.insert(key.as_slice(), &tx_bytes)
				.wrap_err_with(|| format!("Failed to insert transaction at index {idx} into trie"))?;
			builder.transactions.push(*tx.hash());
		}

		Ok(builder)
	}

	/// Get the root hash of the trie
	pub fn root(&mut self) -> Result<B256> {
		let root = self.trie.root_hash().wrap_err("Failed to compute trie root hash")?;
		Ok(B256::from_slice(root.as_bytes()))
	}

	/// Generate a proof for a transaction at the given index
	pub fn get_proof(&mut self, tx_index: usize) -> Result<Vec<Vec<u8>>> {
		if tx_index >= self.transactions.len() {
			return Err(eyre!("Transaction not found at index {tx_index}"));
		}

		let key = alloy_rlp::encode(U256::from(tx_index));
		let proof = self
			.trie
			.get_proof(key.as_slice())
			.wrap_err_with(|| format!("Failed to generate proof for transaction at index {tx_index}"))?;
		Ok(proof)
	}

	/// Find the index of a transaction by its hash
	pub fn find_tx_index(&self, tx_hash: &B256) -> Result<usize> {
		self.transactions
			.iter()
			.position(|hash| hash == tx_hash)
			.ok_or_else(|| eyre!("Transaction hash {tx_hash} not found in block"))
	}

	/// Verify a proof for a transaction at the given index
	pub fn verify_proof(&self, tx_index: usize, proof: &[Vec<u8>], root: &B256) -> Result<Vec<u8>> {
		let key = alloy_rlp::encode(U256::from(tx_index));
		let root_hash = H256::from_slice(root.as_slice());

		self.trie
			.verify_proof(root_hash, key.as_slice(), proof.to_vec())
			.wrap_err_with(|| format!("Failed to verify proof for transaction at index {tx_index}"))?
			.ok_or_else(|| eyre!("Invalid proof for transaction at index {tx_index}"))
	}

	/// Get the transaction hash at the given index
	pub fn get_tx_hash(&self, tx_index: usize) -> Result<B256> {
		self.transactions.get(tx_index).copied().ok_or_else(|| eyre!("Transaction not found at index {tx_index}"))
	}
}

impl Default for TransactionTrieBuilder {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::types::InclusionPayload;

	#[test]
	fn test_build_trie_and_generate_proof() {
		// Create some test transactions
		let payload1 = InclusionPayload::random();
		let tx1 = payload1.decode_transaction_signed().unwrap();
		let payload2 = InclusionPayload::random();
		let tx2 = payload2.decode_transaction_signed().unwrap();
		let transactions = vec![tx1, tx2];

		// // Build trie
		let mut builder = TransactionTrieBuilder::build(&transactions).unwrap();

		// Get root
		let root = builder.root().unwrap();
		assert_ne!(root, B256::ZERO);

		// Generate proof for first transaction
		let proof = builder.get_proof(0).unwrap();
		assert!(!proof.is_empty());

		// Find transaction index
		let tx1_hash = payload1.tx_hash().unwrap();
		let index = builder.find_tx_index(&tx1_hash).unwrap();
		assert_eq!(index, 0);

		// Verify proof
		let verified = builder.verify_proof(0, &proof, &root);
		assert!(verified.is_ok());

		// Generate proof for second transaction
		let proof2 = builder.get_proof(1).unwrap();
		assert!(!proof2.is_empty());

		// Find transaction index
		let tx2_hash = payload2.tx_hash().unwrap();
		let index = builder.find_tx_index(&tx2_hash).unwrap();
		assert_eq!(index, 1);

		// Verify proof
		let verified = builder.verify_proof(1, &proof2, &root);
		assert!(verified.is_ok());
	}
}
