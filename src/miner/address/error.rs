use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineAddressError {
    #[error("MachineAddressError - SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("MachineAddressError - NotFound")]
    NotFound,
}
