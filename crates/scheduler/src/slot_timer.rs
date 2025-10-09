use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Ethereum slot timing constants
const SLOT_TIME_SECONDS: u64 = 12;
const CONSTRAINT_WINDOW_OFFSET: u64 = 4; // 4 seconds into slot

/// Manages Ethereum slot timing and constraint windows
#[derive(Clone)]
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

	/// Get current slot timestamp
	pub fn get_current_slot_timestamp(&self) -> u64 {
		let slot = self.get_current_slot();
		self.genesis_timestamp + slot * SLOT_TIME_SECONDS
	}

	/// Check if we're in a constraint submission window
	pub fn is_in_constraint_window(&self) -> bool {
		let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		let slot_start = self.get_current_slot_timestamp();
		let window_start = slot_start + CONSTRAINT_WINDOW_OFFSET;
		let window_end = slot_start + SLOT_TIME_SECONDS;

		now >= window_start && now < window_end
	}

	/// Get time until next constraint window
	pub fn time_until_next_constraint_window(&self) -> Duration {
		let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		let current_slot = self.get_current_slot();
		let next_slot_start = self.genesis_timestamp + (current_slot + 1) * SLOT_TIME_SECONDS;
		let next_window_start = next_slot_start + CONSTRAINT_WINDOW_OFFSET;

		Duration::from_secs(next_window_start - now)
	}

	/// Get time remaining in current constraint window
	pub fn time_remaining_in_constraint_window(&self) -> Option<Duration> {
		if !self.is_in_constraint_window() {
			return None;
		}

		let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		let slot_start = self.get_current_slot_timestamp();
		let window_end = slot_start + SLOT_TIME_SECONDS;

		Some(Duration::from_secs(window_end - now))
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
