use sqlx::{Pool, Postgres};

use crate::primitives::MachineId;

use uuid::Uuid;

use super::{MachinePayoutsError, NewMachinePayout};

#[derive(Clone)]
pub struct MachinePayouts {
    pool: Pool<Postgres>,
}

impl MachinePayouts {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(
        &self,
        machine_payout: NewMachinePayout,
    ) -> anyhow::Result<(), MachinePayoutsError> {
        let mut tx = self.pool.begin().await?;

        let machine_id: Uuid = machine_payout.machine_id.into();

        let _ = sqlx::query!(
            r#"INSERT INTO machine_payout (
            machine_id,
            amount,
            wallet_destination
        ) VALUES ($1, $2, $3)"#,
            machine_id,
            machine_payout.amount,
            machine_payout.wallet_destination
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
