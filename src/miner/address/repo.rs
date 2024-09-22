use sqlx::{Pool, Postgres};

use crate::{drone::error::DroneError, primitives::MachineId};

use uuid::Uuid;

use crate::miner::address::entity::{MinerAddress, NewMinerAddress};
use crate::miner::address::error::MachineAddressError;

#[derive(Clone)]
pub struct MinerAddresses {
    pool: Pool<Postgres>,
}

impl MinerAddresses {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, machine_address: NewMinerAddress) -> anyhow::Result<(), DroneError> {
        let mut tx = self.pool.begin().await?;

        let machine_id = Uuid::from(machine_address.machine_id);

        let _ = sqlx::query!(
            r#"INSERT INTO machine_address (
            machine_id,
            address
        ) VALUES ($1, $2)"#,
            machine_id,
            machine_address.address,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_latest_by_machine_id(
        &self,
        machine_id: MachineId,
    ) -> anyhow::Result<MinerAddress, MachineAddressError> {
        let machine_id = Uuid::from(machine_id);

        let machine_address = sqlx::query_as!(
            MinerAddress,
            r#"SELECT * FROM machine_address WHERE machine_id = $1 ORDER BY created_at DESC LIMIT 1"#,
            machine_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match machine_address {
            Some(machine_address) => Ok(machine_address),
            None => Err(MachineAddressError::NotFound),
        }
    }
}
