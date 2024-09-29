mod config;
pub mod error;

pub use config::*;

use error::SolanaError;

use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account as SolanaAccount;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::instruction::mint_to_checked;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as TokenAccount;

use std::sync::{Arc, OnceLock};

use crate::machine_payouts::repo::MachinePayouts;
use crate::machine_payouts::{self, MachinePayoutStatus, NewMachinePayout};
use crate::primitives::MachineId;

#[derive(Clone)]
pub struct SolanaClient {
    inner: Arc<RpcClient>,
    keypair: Arc<Keypair>,
    mint_address: solana_sdk::pubkey::Pubkey,
    payouts: Vec<Transaction>,
}

const SOLANA_DEVNET: &str = "https://api.devnet.solana.com";
const SOLANA_LOCALNET: &str = "http://127.0.0.1:8899";
const INITIAL_REWARD: f64 = 50.0;
const TOKEN_CAP: u64 = 1_000_000_000;
pub const TOKEN_DECIMALS: u8 = 9;
// 10 minute halving interval
//const HALVING_INTERVAL: u64 = 10;
const HALVING_INTERVAL: u64 = 15_552_000;

static RPC_CLIENT: OnceLock<Arc<RpcClient>> = OnceLock::new();

impl SolanaClient {
    pub async fn init(config: SolanaConfig) -> anyhow::Result<Self, SolanaError> {
        let oracle_keypair: Keypair = Keypair::read_from_file(config.keypair).unwrap();
        let inner = RpcClient::new(SOLANA_DEVNET.to_string());
        let mint_address: Pubkey = config.mint_address.as_str().parse()?;

        //RPC_CLIENT
        //    .set(RpcClient::new(SOLANA_DEVNET.to_string()))
        //    .unwrap();

        let inner = Arc::new(inner);
        let keypair = Arc::new(oracle_keypair);

        RPC_CLIENT.get_or_init(|| Arc::clone(&inner));

        Ok(Self {
            inner,
            keypair,
            mint_address,
            payouts: vec![],
        })
    }

    pub async fn calculate_reward(&self, rank: f64) -> anyhow::Result<f64, SolanaError> {
        let reward = INITIAL_REWARD / 2_f64.powf((rank / HALVING_INTERVAL as f64).floor());
        Ok(reward)
    }

    pub async fn submit_payout(
        &self,
        destination_addr: String,
        amount: i64,
        machine_payouts: &MachinePayouts,
        machine_id: MachineId,
    ) -> anyhow::Result<(), SolanaError> {
        let machine_payout = NewMachinePayout {
            wallet_destination: destination_addr,
            machine_id,
            amount,
            status: MachinePayoutStatus::Pending,
            token_account: None,
        };

        machine_payouts.create(machine_payout).await?;

        Ok(())
    }
}

pub async fn get_solana_client() -> Arc<RpcClient> {
    RPC_CLIENT
        .get_or_init(|| {
            let inner = RpcClient::new(SOLANA_DEVNET.to_string());
            Arc::new(inner)
        })
        .clone()
}
