use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::primitives::{MachineId, MachinePayoutId};

pub type MachinePayoutAmount = i64;

pub struct NewMachinePayout {
    pub machine_id: MachineId,
    pub amount: i64,
    pub wallet_destination: String,
    pub token_account: Option<String>,
    pub status: MachinePayoutStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachinePayout {
    pub id: MachinePayoutId,
    pub machine_id: MachineId,
    pub amount: i64,
    pub wallet_destination: String,
    pub token_account: Option<String>,
    pub status: MachinePayoutStatus,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MachinePayoutStatus {
    Pending,
    Complete,
    Failed,
}

impl From<MachinePayoutStatus> for String {
    fn from(status: MachinePayoutStatus) -> Self {
        match status {
            MachinePayoutStatus::Pending => "pending".to_string(),
            MachinePayoutStatus::Complete => "complete".to_string(),
            MachinePayoutStatus::Failed => "failed".to_string(),
        }
    }
}

impl From<String> for MachinePayoutStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "pending" => MachinePayoutStatus::Pending,
            "complete" => MachinePayoutStatus::Complete,
            "failed" => MachinePayoutStatus::Failed,
            _ => panic!("Invalid MachinePayoutStatus"),
        }
    }
}

impl From<&str> for MachinePayoutStatus {
    fn from(status: &str) -> Self {
        match status {
            "pending" => MachinePayoutStatus::Pending,
            "complete" => MachinePayoutStatus::Complete,
            "failed" => MachinePayoutStatus::Failed,
            _ => panic!("Invalid MachinePayoutStatus"),
        }
    }
}
