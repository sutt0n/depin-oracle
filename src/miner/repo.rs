use sqlx::{Pool, Postgres};

use crate::{drone::error::DroneError, primitives::MachineId};

use uuid::Uuid;

use super::{MachineError, MachinePayload, Miner, NewMiner};

#[derive(Clone)]
pub struct Machines {
    pool: Pool<Postgres>,
}

impl Machines {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create_from_payload(
        &self,
        machine: MachinePayload,
    ) -> anyhow::Result<(), DroneError> {
        let mut tx = self.pool.begin().await?;

        let miner: NewMiner = NewMiner {
            id: machine.id.into(),
            latitude: machine.latitude,
            longitude: machine.longtitude,
        };

        let miner_uuid: Uuid = miner.id.into();

        let _ = sqlx::query!(
            r#"INSERT INTO miner (
            machine_id,
            latitude,
            longitude
        ) VALUES ($1, $2, $3)"#,
            miner_uuid,
            miner.latitude,
            miner.longitude
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_by_machine_id(
        &self,
        machine_id: String,
    ) -> anyhow::Result<Option<Miner>, MachineError> {
        let machine_id: MachineId = machine_id.into();
        let machine_uuid: Uuid = machine_id.into();

        println!("machine_uuid: {:?}", machine_uuid);

        let miner = sqlx::query_as!(
            Miner,
            r#"SELECT * FROM miner WHERE machine_id = $1"#,
            machine_uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(miner)
    }

    pub async fn update_last_seen(
        &self,
        machine_id: MachineId,
    ) -> anyhow::Result<(), MachineError> {
        let machine_uuid: Uuid = machine_id.into();

        let _ = sqlx::query!(
            r#"UPDATE miner SET last_seen = NOW() WHERE machine_id = $1"#,
            machine_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
