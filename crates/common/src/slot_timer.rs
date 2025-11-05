use std::time::{SystemTime, UNIX_EPOCH};

/// Ethereum slot timing constants
pub const SLOT_TIME_SECONDS: u64 = 12;
pub const CONSTRAINT_TRIGGER_OFFSET: u64 = 2; // 2 seconds before slot starts

/// Manages Ethereum slot timing and constraint windows
#[derive(Clone, Debug)]
pub struct SlotTimer {
	genesis_timestamp: u64,
}

impl SlotTimer {
	/// Create a new slot timer
	pub fn new(genesis_timestamp: u64) -> Self {
		Self { genesis_timestamp }
	}

	/// Get current slot number
	pub fn get_current_slot(&self) -> u64 {
		let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		(now - self.genesis_timestamp) / SLOT_TIME_SECONDS
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_slot_timer() {
		let slot_timer = SlotTimer::new(1606824023);
		println!("Current slot: {}", slot_timer.get_current_slot());
	}
}
