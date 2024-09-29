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
use spl_token::instruction::mint_to_checked;

use crate::solana::TOKEN_DECIMALS;
use crate::task::TaskError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "fang::serde")]
pub struct ProcessPayoutsTask {
    pub token_mint_pubkey: Pubkey,
    pub keypair: String,
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for ProcessPayoutsTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        let db_pool = crate::cli::db::get_pool();
        let rpc_client = crate::solana::get_solana_client().await;
        let oracle_keypair: Keypair = Keypair::read_from_file(self.keypair.clone()).unwrap();

        println!("Processing payouts");

        let pending_machine_payouts = sqlx::query!(
            r#"
        SELECT
            wallet_destination,
            amount
        FROM
            machine_payout
        WHERE
            status = 'pending'
        AND
            token_account IS NOT NULL
        "#,
        )
        .fetch_all(db_pool)
        .await
        .map_err(TaskError::from)?;

        // don't do anything until we have 10 pending payouts
        if pending_machine_payouts.len() < 10 {
            return Ok(());
        }

        let mut instructions = vec![];

        let recent_blockhash = rpc_client.get_latest_blockhash().map_err(TaskError::from)?;

        for pending_machine_payout in &pending_machine_payouts {
            let wallet_destination = &pending_machine_payout.wallet_destination;
            let wallet_destination: Pubkey = wallet_destination.parse().map_err(TaskError::from)?;
            let recipient_token_account =
                get_associated_token_address(&wallet_destination, &self.token_mint_pubkey);

            let mint_ix = mint_to_checked(
                &spl_token::id(),
                &self.token_mint_pubkey,
                &recipient_token_account,
                &oracle_keypair.pubkey(),
                &[],
                pending_machine_payout.amount.try_into().unwrap(),
                TOKEN_DECIMALS,
            )
            .map_err(TaskError::from)?;
            instructions.push(mint_ix);
        }

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&oracle_keypair.pubkey()),
            &[&oracle_keypair],
            recent_blockhash,
        );

        let signature = rpc_client.send_and_confirm_transaction(&tx);

        if signature.is_ok() {
            // update the database
            for pending_machine_payout in pending_machine_payouts {
                let wallet_destination_address = pending_machine_payout.wallet_destination;

                sqlx::query!(
                    r#"
                UPDATE machine_payout
                SET status = 'complete'
                WHERE wallet_destination = $1
                "#,
                    wallet_destination_address
                )
                .execute(db_pool)
                .await
                .map_err(TaskError::from)?;
            }
        }

        Ok(())
    }

    fn cron(&self) -> Option<Scheduled> {
        let expression = "0 * * * * * *";
        Some(Scheduled::CronPattern(expression.to_string()))
    }

    fn uniq(&self) -> bool {
        true
    }
}
