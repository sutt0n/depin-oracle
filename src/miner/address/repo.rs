use sqlx::{Pool, Postgres};

use crate::drone::error::DroneError;

use uuid::Uuid;

use crate::miner::address::entity::MinerAddress;

#[derive(Clone)]
pub struct MinerAddresses {
    pool: Pool<Postgres>,
}

impl MinerAddresses {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, miner_address: MinerAddress) -> anyhow::Result<(), DroneError> {
        let mut tx = self.pool.begin().await?;

        let miner_id = Uuid::from(miner_address.miner_id);

        let _ = sqlx::query!(
            r#"INSERT INTO miner_address (
            miner_id,
            address
        ) VALUES ($1, $2)"#,
            miner_id,
            miner_address.address,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
