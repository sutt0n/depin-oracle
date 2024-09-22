use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerAddress {
    pub id: MinerAddressId,
    pub miner_id: MinerId,
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
