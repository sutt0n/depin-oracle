mod config;
pub mod error;

pub use config::*;
use sqlx::{Pool, Postgres};

use crate::broker::Broker;

#[derive(Clone)]
pub struct OracleApp {
    _config: AppConfig,
    broker: Broker,
    _pool: Pool<Postgres>,
}

impl OracleApp {
    pub async fn init(
        pool: Pool<Postgres>,
        config: AppConfig,
        broker: Broker,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            broker,
            _config: config,
            _pool: pool,
        })
    }

    //pub async fn handle_drone_mqtt(&self, message: DronePayload) -> anyhow::Result<()> {
    //    println!("Handling drone mqtt message: {:?}", message);
    //
    //    Ok(())
    //}
}
