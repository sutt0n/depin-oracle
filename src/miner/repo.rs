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
            r#"INSERT INTO machine (
            id,
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

    pub async fn get_rank(&self, machine_id: MachineId) -> anyhow::Result<f64, MachineError> {
        let base_rank = 50.0;

        // get number of machine_entry records
        let machine_uuid: Uuid = machine_id.into();

        let total_machine_entry_count = sqlx::query!(r#"SELECT COUNT(*) FROM machine_entry"#,)
            .fetch_one(&self.pool)
            .await?;

        let machine_entry_count = sqlx::query!(
            r#"SELECT COUNT(*) FROM machine_entry WHERE machine_id = $1"#,
            machine_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        let machine_entry_count: f64 = machine_entry_count.count.unwrap_or(0) as f64;
        let total_machine_entry_count: f64 = total_machine_entry_count.count.unwrap_or(0) as f64;

        let rank = base_rank + (total_machine_entry_count / machine_entry_count);

        Ok(rank)
    }

    pub async fn get_by_machine_id(
        &self,
        machine_id: String,
    ) -> anyhow::Result<Miner, MachineError> {
        let machine_id: MachineId = machine_id.into();
        let machine_uuid: Uuid = machine_id.into();

        println!("machine_uuid: {:?}", machine_uuid);

        let miner = sqlx::query_as!(
            Miner,
            r#"SELECT * FROM machine WHERE id = $1"#,
            machine_uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        match miner {
            None => return Err(MachineError::NotFound),
            Some(miner) => return Ok(miner),
        }
    }

    pub async fn update_last_seen(
        &self,
        machine_id: MachineId,
    ) -> anyhow::Result<(), MachineError> {
        let machine_uuid: Uuid = machine_id.into();

        let _ = sqlx::query!(
            r#"UPDATE machine SET last_seen = NOW() WHERE id = $1"#,
            machine_uuid
        )
        .execute(&self.pool)
        .await?;

        let _ = sqlx::query!(
            r#"INSERT INTO machine_entry (machine_id) VALUES ($1)"#,
            machine_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
