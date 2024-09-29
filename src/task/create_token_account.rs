use std::collections::HashMap;

use fang::async_trait;
use fang::asynk::async_queue::AsyncQueueable;
use fang::serde::{Deserialize, Serialize};
use fang::typetag;
use fang::AsyncRunnable;
use fang::FangError;
use fang::Scheduled;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as TokenAccount;

use crate::task::TaskError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "fang::serde")]
pub struct CreateTokenAccountsTask {
    pub token_mint_pubkey: Pubkey,
    pub keypair: String,
}

impl From<TaskError> for FangError {
    fn from(err: TaskError) -> Self {
        let msg = format!("TaskError: {}", err);
        FangError { description: msg }
    }
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for CreateTokenAccountsTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        let db_pool = crate::cli::db::get_pool();
        let rpc_client = crate::solana::get_solana_client().await;
        let oracle_keypair: Keypair = Keypair::read_from_file(self.keypair.clone()).unwrap();

        let _ = sqlx::query!(
            r#"
        UPDATE machine_payout AS t1
            SET token_account = (
                SELECT t2.token_account
                FROM machine_payout AS t2
                WHERE t1.wallet_destination = t2.wallet_destination
                AND t2.token_account IS NOT NULL
                LIMIT 1
            )
        WHERE EXISTS (
            SELECT 1
            FROM machine_payout AS t3
            WHERE t1.wallet_destination = t3.wallet_destination
            AND t3.token_account IS NOT NULL
        );
        "#,
        )
        .execute(db_pool)
        .await
        .map_err(TaskError::from)?;

        let pending_machine_payouts = sqlx::query!(
            r#"
        SELECT
            wallet_destination
        FROM
            machine_payout
        WHERE
            status = 'pending'
        AND
            token_account IS NULL
        "#,
        )
        .fetch_all(db_pool)
        .await
        .map_err(TaskError::from)?;

        // don't do anything until we have 10 pending payouts
        if pending_machine_payouts.len() < 10 {
            return Ok(());
        }

        let mut accounts_to_check = vec![];
        let mut wallet_map: HashMap<String, Pubkey> = HashMap::new();

        for pending_machine_payout in pending_machine_payouts {
            let wallet_destination = pending_machine_payout.wallet_destination;
            let destination_wallet_pubkey: Pubkey =
                wallet_destination.parse().map_err(TaskError::from)?;
            let recipient_token_account =
                get_associated_token_address(&destination_wallet_pubkey, &self.token_mint_pubkey);

            wallet_map.insert(wallet_destination, recipient_token_account);
            accounts_to_check.push(recipient_token_account);
        }

        println!("Accounts to check: {:?}", accounts_to_check);
        println!("Wallet map: {:?}", wallet_map);

        if let Ok(accounts) = rpc_client.get_multiple_accounts(&accounts_to_check) {
            for account in accounts.into_iter().flatten() {
                let token_account: TokenAccount = TokenAccount::unpack(&account.data)
                    .map_err(TaskError::from)
                    .unwrap();

                let wallet_destination = token_account.owner.clone().to_string();

                println!("Updating wallet: {:?}", wallet_destination);

                if wallet_map.contains_key(&wallet_destination) {
                    // update the database
                    let _ = sqlx::query!(
                        r#"
                UPDATE machine_payout
                SET token_account = $1
                WHERE wallet_destination = $2
                "#,
                        wallet_map.get(&wallet_destination).unwrap().to_string(),
                        wallet_destination
                    )
                    .execute(db_pool)
                    .await
                    .map_err(TaskError::from)?;

                    wallet_map.remove(&wallet_destination);
                }
            }
        }

        println!("Wallets to create: {:?}", wallet_map);

        // leftover wallets in hashmap don't have a token account
        let mut instructions = vec![];

        let recent_blockhash = rpc_client.get_latest_blockhash().map_err(TaskError::from)?;

        for (wallet_destination, _recipient_token_account) in &wallet_map {
            let wallet_destination_pubkey: Pubkey =
                wallet_destination.parse().map_err(TaskError::from)?;
            let create_ata_ix = create_associated_token_account(
                &oracle_keypair.pubkey(),   // Payer
                &wallet_destination_pubkey, // Payer
                &self.token_mint_pubkey,    // Owner of the associated token account
                &spl_token::id(),           // Token mint
            );
            instructions.push(create_ata_ix.clone());
        }

        println!("Executing instruction");

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&oracle_keypair.pubkey()),
            &[&oracle_keypair],
            recent_blockhash,
        );

        let signature = rpc_client.send_and_confirm_transaction(&tx);

        match signature {
            Ok(_) => {
                // update the database
                for (wallet_destination, recipient_token_account) in wallet_map {
                    let wallet_destination_pubkey: Pubkey =
                        wallet_destination.parse().map_err(TaskError::from)?;
                    sqlx::query!(
                        r#"
                    UPDATE machine_payout
                    SET token_account = $1
                    WHERE wallet_destination = $2
                    "#,
                        recipient_token_account.to_string(),
                        wallet_destination_pubkey.to_string()
                    )
                    .execute(db_pool)
                    .await
                    .map_err(TaskError::from)?;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

        Ok(())
    }

    fn cron(&self) -> Option<Scheduled> {
        let expression = "30 * * * * * *";
        Some(Scheduled::CronPattern(expression.to_string()))
    }

    fn uniq(&self) -> bool {
        true
    }
}
