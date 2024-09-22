use crate::primitives::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerAddress {
    pub id: MinerAddressId,
    pub machine_id: MachineId,
    pub address: String,
    pub status: MinerAddressStatus,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMinerAddress {
    pub machine_id: MachineId,
    pub address: String,
    pub status: MinerAddressStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MinerAddressStatus {
    #[default]
    Active,
    Inactive,
}

impl From<String> for MinerAddressStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "active" => MinerAddressStatus::Active,
            "inactive" => MinerAddressStatus::Inactive,
            _ => panic!("Invalid miner address status"),
        }
    }
}

impl From<&str> for MinerAddressStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => MinerAddressStatus::Active,
            "inactive" => MinerAddressStatus::Inactive,
            _ => panic!("Invalid miner address status"),
        }
    }
}

impl From<MinerAddressStatus> for String {
    fn from(s: MinerAddressStatus) -> Self {
        match s {
            MinerAddressStatus::Active => "active".to_string(),
            MinerAddressStatus::Inactive => "inactive".to_string(),
        }
    }
}
