use serde::{Deserialize, Serialize};

use crate::types::rpc::SlotInfo;

/// Response containing slot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfoResponse {
	pub slots: Vec<SlotInfo>,
}
