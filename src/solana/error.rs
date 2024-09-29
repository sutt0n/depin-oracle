use solana_client::client_error::ClientError;
use solana_sdk::{program_error::ProgramError, pubkey::ParsePubkeyError};
use thiserror::Error;

use crate::machine_payouts::MachinePayoutsError;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("SolanaError - ParsePubkeyError: {0}")]
    ParsePubkeyError(#[from] ParsePubkeyError),
    #[error("SolanaError - ClientError: {0}")]
    ClientError(#[from] ClientError),
    #[error("SolanaError - ProgramError: {0}")]
    ProgramError(#[from] ProgramError),
    #[error("SolanaError - IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("SolanaError - MachinePayoutsError: {0}")]
    MachinePayoutsError(#[from] MachinePayoutsError),
}
