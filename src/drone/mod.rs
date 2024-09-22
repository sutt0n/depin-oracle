mod entity;
pub mod error;
pub mod repo;

pub use entity::*;
pub use error::*;

//pub(crate) async fn calculate_reward(drone: DronePayload, machine: Machine) -> anyhow::Result<u64> {
//    let mut base_reward = 0.5;
//
//    if drone.home_latitude > 0.0 || drone.home_longitude > 0.0 {
//        base_reward += 0.5;
//    }
//
//    if drone.pilot_latitude > 0.0 || drone.pilot_longitude > 0.0 {
//        base_reward += 0.5;
//    }
//}
