use thiserror::Error;

use crate::solana::error::SolanaError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    //#[error("{0}")]
    //BrokerError(#[from] crate::broker::BrokerError),
    //#[error("{0}")]
    //AccountError(#[from] crate::account::AccountError),
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("{0}")]
    DroneError(#[from] crate::drone::DroneError),
    #[error("{0}")]
    DeserializationError(String),
    #[error("{0}")]
    SolanaError(#[from] SolanaError),
    #[error("{0}")]
    MachineError(#[from] crate::miner::MachineError),
}
