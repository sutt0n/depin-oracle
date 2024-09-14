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
use crate::drone::Drone;
use crate::solana::SolanaClient;

use self::error::ApplicationError;

#[derive(Clone)]
pub struct OracleApp {
    _config: AppConfig,
    broker: Broker,
    solana: SolanaClient,
    _pool: Pool<Postgres>,
}

impl OracleApp {
    pub async fn init(
        pool: Pool<Postgres>,
        config: AppConfig,
        broker: Broker,
        solana: SolanaClient,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            broker,
            _config: config,
            solana,
            _pool: pool,
        })
    }

    pub async fn handle_drone_mqtt(&self, message: Vec<u8>) -> anyhow::Result<()> {
        println!("Handling drone mqtt message: {:?}", message);

        let _drone = bincode::deserialize::<Drone>(&message)?;

        // todo: calculate rewards

        //self.submit_payout(drone).await?;
        self.solana
            .submit_payout("5FnusLiFyNjZYVo96Mgf4rqsg34NC7LJ9qj9DpVCd2wi".to_string())
            .await?;
        Ok(())
    }
}
