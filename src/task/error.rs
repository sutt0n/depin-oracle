use fang::AsyncQueueError;
use fang::CronError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("TaskError - CronError: {0}")]
    CronError(#[from] CronError),
    #[error("TaskError - AsyncQueueError: {0}")]
    AsyncQueueError(#[from] AsyncQueueError),
    #[error("TaskError - SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("TaskError - SolanaParsePubKeyError: {0}")]
    SolanaParsePubKeyError(#[from] solana_sdk::pubkey::ParsePubkeyError),
    #[error("TaskError - SolanaProgramError: {0}")]
    SolanaProgramError(#[from] solana_sdk::program_error::ProgramError),
    #[error("TaskError - SolanaRPCClientError: {0}")]
    SolanaRPCClientError(#[from] solana_client::client_error::ClientError),
}
