mod config;
pub mod error;

pub use config::*;
use sqlx::{Pool, Postgres};

use crate::broker::Broker;
use crate::drone::repo::Drones;
use crate::drone::MqttPayload;
use crate::miner::repo::Machines;
use crate::miner::{Miner, MinerAddresses};
use crate::solana::SolanaClient;

use self::error::ApplicationError;

#[derive(Clone)]
pub struct OracleApp {
    _config: AppConfig,
    broker: Broker,
    solana: SolanaClient,
    drones: Drones,
    machines: Machines,
    miner_addresses: MinerAddresses,
    _pool: Pool<Postgres>,
}

impl OracleApp {
    pub async fn init(
        pool: Pool<Postgres>,
        config: AppConfig,
        broker: Broker,
        drones: Drones,
        machines: Machines,
        miner_addresses: MinerAddresses,
        solana: SolanaClient,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _config: config,
            _pool: pool,
            broker,
            drones,
            machines,
            miner_addresses,
            solana,
        })
    }

    pub async fn handle_drone_mqtt(
        &self,
        message: Vec<u8>,
    ) -> anyhow::Result<(), ApplicationError> {
        println!("Handling drone mqtt message: {:?}", message);

        // decode drone or return error
        let payload = bincode::deserialize::<MqttPayload>(&message).map_err(|e| {
            ApplicationError::DeserializationError(format!(
                "Failed to deserialize drone payload: {:?}",
                e
            ))
        })?;

        let drone = payload.drone;
        let machine = payload.machine;

        println!("Drone: {:?}", drone);
        println!("Machine: {:?}", machine);

        // see if machine exists
        if let Some(existing_machine) = self.get_existing_machine(machine.clone().id).await? {
            println!("Existing machine: {:?}", existing_machine);
            self.machines
                .update_last_seen(existing_machine.machine_id)
                .await?;
        } else {
            println!("Creating new machine: {:?}", machine);
            self.machines.create_from_payload(machine).await?;
        }

        println!("Creating new drone: {:?}", drone);

        self.drones.create(drone).await?;

        // [done] drone: insert drone into db
        // solana: calculate rewards
        // todo: queue drone_payout job; for now, just payout

        //self.submit_payout(drone).await?;
        //self.solana
        //    .submit_payout("HShLUQnxQkcT2rZNxUAJdVeBBwSF6T7JT5XD1dUShKR6".to_string())
        //    .await?;
        Ok(())
    }

    async fn get_existing_machine(
        &self,
        machine_id: String,
    ) -> anyhow::Result<Option<Miner>, ApplicationError> {
        let machine = self.machines.get_by_machine_id(machine_id).await?;
        Ok(machine)
    }
}
