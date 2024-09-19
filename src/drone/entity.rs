use ::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DronePayload {
    pub id: u64,
    pub serial_number: String,
    pub created: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub x_speed: f64,
    pub y_speed: f64,
    pub yaw: f64,
    pub pilot_latitude: f64,
    pub pilot_longitude: f64,
    pub home_latitude: f64,
    pub home_longitude: f64,
}
