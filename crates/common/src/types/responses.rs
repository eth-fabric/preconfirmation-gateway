use serde::{Deserialize, Serialize};

use crate::types::commitments::SlotInfo;

/// Response containing slot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfoResponse {
	pub slots: Vec<SlotInfo>,
}
