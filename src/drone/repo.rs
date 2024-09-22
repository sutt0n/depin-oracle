use sqlx::{Pool, Postgres};

use crate::drone::entity::DroneDto;
use crate::drone::error::DroneError;
use crate::primitives::MachineId;

use uuid::Uuid;

#[derive(Clone)]
pub struct Drones {
    pool: Pool<Postgres>,
}

impl Drones {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(
        &self,
        drone: DroneDto,
        machine_id: MachineId,
    ) -> anyhow::Result<(), DroneError> {
        let mut tx = self.pool.begin().await?;

        let machine_uuid: Uuid = machine_id.into();

        let _ = sqlx::query!(
            r#"INSERT INTO drone (
            machine_id,
            serial_number, 
            latitude,
            longitude,
            altitude,
            x_speed,
            y_speed,
            yaw,
            pilot_latitude,
            pilot_longitude,
            home_latitude,
            home_longitude
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
            machine_uuid,
            drone.serial_number,
            drone.latitude,
            drone.longitude,
            drone.altitude,
            drone.x_speed,
            drone.y_speed,
            drone.yaw,
            drone.pilot_latitude,
            drone.pilot_longitude,
            drone.home_latitude,
            drone.home_longitude,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
