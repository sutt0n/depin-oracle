mod config;
pub mod error;

pub use config::*;

use error::SolanaError;

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::instruction::mint_to_checked;

use std::sync::Arc;

#[derive(Clone)]
pub struct SolanaClient {
    inner: Arc<RpcClient>,
    keypair: Arc<Keypair>,
    mint_address: solana_sdk::pubkey::Pubkey,
}

const SOLANA_DEVNET: &str = "https://api.devnet.solana.com";
const INITIAL_REWARD: f64 = 50.0;
const TOKEN_CAP: u64 = 1_000_000_000;
const TOKEN_DECIMALS: u8 = 9;
// 10 minute halving interval
const HALVING_INTERVAL: u64 = 10;
//const HALVING_INTERVAL: u64 = 15_552_000;

impl SolanaClient {
    pub async fn init(config: SolanaConfig) -> anyhow::Result<Self, SolanaError> {
        let oracle_keypair: Keypair = Keypair::read_from_file(config.keypair).unwrap();
        let inner = solana_client::rpc_client::RpcClient::new(SOLANA_DEVNET.to_string());
        let mint_address: Pubkey = config.mint_address.as_str().parse()?;

        let inner = Arc::new(inner);
        let keypair = Arc::new(oracle_keypair);

        Ok(Self {
            inner,
            keypair,
            mint_address,
        })
    }

    pub async fn calculate_reward(&self, rank: f64) -> anyhow::Result<f64, SolanaError> {
        let reward = INITIAL_REWARD / 2_f64.powf((rank / HALVING_INTERVAL as f64).floor());
        Ok(reward)
    }

    pub async fn submit_payout(
        &self,
        destination_addr: String,
        amount: f64,
    ) -> anyhow::Result<(), SolanaError> {
        let token_mint_pubkey: Pubkey = self.mint_address;
        let destination_wallet_pubkey: Pubkey = destination_addr.parse()?;

        println!("Token Mint Pubkey: {:?}", token_mint_pubkey);

        // Get recent blockhash
        let recent_blockhash = self.inner.get_latest_blockhash()?;

        println!("Recent Blockhash: {:?}", recent_blockhash);

        let sender_token_account =
            get_associated_token_address(&self.keypair.pubkey(), &token_mint_pubkey);
        let recipient_token_account =
            get_associated_token_address(&destination_wallet_pubkey, &token_mint_pubkey);

        let mut instructions = vec![];

        let sender_token_balance = self
            .inner
            .get_token_account_balance(&sender_token_account)?;
        println!(
            "Sender's token balance: {} (decimals: {})",
            sender_token_balance.ui_amount_string, sender_token_balance.decimals
        );

        match self.inner.get_account(&recipient_token_account) {
            Ok(account) => {
                if account.owner != spl_token::id() {
                    eprintln!("Recipient's associated token account exists but is not owned by the SPL Token Program. Cannot proceed.");
                }
                println!("Recipient's associated token account exists and is owned by the SPL Token Program.");
                // Account exists and is valid; proceed
            }
            Err(_) => {
                // Account doesn't exist; create the associated token account
                println!("Recipient's associated token account does not exist. Creating it now.");
                let create_ata_ix = create_associated_token_account(
                    &self.keypair.pubkey(),     // Payer
                    &destination_wallet_pubkey, // Payer
                    &token_mint_pubkey,         // Owner of the associated token account
                    &spl_token::id(),           // Token mint
                );
                instructions.push(create_ata_ix);
            }
        }

        let amount = (amount * 10_f64.powi(TOKEN_DECIMALS as i32)) as u64;

        println!("Amount: {:?}", amount);

        let mint_ix = mint_to_checked(
            &spl_token::id(),
            &token_mint_pubkey,
            &recipient_token_account,
            &self.keypair.pubkey(),
            &[],
            amount,
            9,
        )?;
        instructions.push(mint_ix);

        println!("Instructions: {:?}", instructions);

        // 50_000_000_000

        // Build and send transaction
        let recent_blockhash = self.inner.get_latest_blockhash()?;

        println!("Recent Blockhash: {:?}", recent_blockhash);

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash,
        );

        println!("Transaction: {:?}", tx);

        // do this in a task, so we can continue on

        tokio::spawn({
            let inner = self.inner.clone();
            async move {
                let signature = inner.send_and_confirm_transaction(&tx);

                println!("Signature: {:?}", signature);

                match signature {
                    Ok(signature) => {
                        println!("Signature: {:?}", signature);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
        });

        Ok(())
    }
}
