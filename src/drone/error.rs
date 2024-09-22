use thiserror::Error;

#[derive(Error, Debug)]
pub enum DroneError {
    #[error("DroneError - SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("DroneError - UuidError: {0}")]
    UuidError(#[from] uuid::Error),
}
