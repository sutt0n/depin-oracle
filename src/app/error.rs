use thiserror::Error;

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
}
