use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("SolanaError - ParsePubkeyError: {0}")]
    ParsePubkeyError(#[from] solana_sdk::pubkey::ParsePubkeyError),
    #[error("SolanaError - ClientError: {0}")]
    ClientError(#[from] solana_client::client_error::ClientError),
    #[error("SolanaError - ProgramError: {0}")]
    ProgramError(#[from] solana_sdk::program_error::ProgramError),
}
