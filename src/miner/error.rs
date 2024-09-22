use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("MachineError - SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("MachineError - NotFound")]
    NotFound,
}
