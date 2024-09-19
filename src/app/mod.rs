mod config;
pub mod error;

pub use config::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::signers::Signers;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::instruction::{mint_to, mint_to_checked};
use spl_token::{instruction::transfer_checked, state::Mint};
use sqlx::{Pool, Postgres};

use crate::broker::Broker;
use crate::drone::repo::Drones;
use crate::drone::DronePayload;
use crate::solana::SolanaClient;

use self::error::ApplicationError;

#[derive(Clone)]
pub struct OracleApp {
    _config: AppConfig,
    broker: Broker,
    solana: SolanaClient,
    drones: Drones,
    _pool: Pool<Postgres>,
}

impl OracleApp {
    pub async fn init(
        pool: Pool<Postgres>,
        config: AppConfig,
        broker: Broker,
        drones: Drones,
        solana: SolanaClient,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _config: config,
            _pool: pool,
            broker,
            drones,
            solana,
        })
    }

    pub async fn handle_drone_mqtt(
        &self,
        message: Vec<u8>,
    ) -> anyhow::Result<(), ApplicationError> {
        println!("Handling drone mqtt message: {:?}", message);

        // decode drone or return error
        let drone = bincode::deserialize::<DronePayload>(&message).map_err(|e| {
            ApplicationError::DeserializationError(format!(
                "Failed to deserialize drone payload: {:?}",
                e
            ))
        })?;

        self.drones.create(drone).await?;

        // drone: insert drone into db
        // solana: calculate rewards
        // queue drone_payout job

        //self.submit_payout(drone).await?;
        //self.solana
        //    .submit_payout("5FnusLiFyNjZYVo96Mgf4rqsg34NC7LJ9qj9DpVCd2wi".to_string())
        //    .await?;
        Ok(())
    }
}
