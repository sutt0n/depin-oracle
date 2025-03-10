use ::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::primitives::MachineId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachinePayload {
    pub id: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub wallet_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMiner {
    pub id: MachineId,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miner {
    pub id: MachineId,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}
