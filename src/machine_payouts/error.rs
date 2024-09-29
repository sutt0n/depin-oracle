use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachinePayoutsError {
    #[error("MachinePayoutsError - SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),
}
