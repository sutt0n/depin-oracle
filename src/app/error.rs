use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    //#[error("{0}")]
    //BrokerError(#[from] crate::broker::BrokerError),
    //#[error("{0}")]
    //AccountError(#[from] crate::account::AccountError),
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}
