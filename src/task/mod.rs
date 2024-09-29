//mod config;
pub mod error;

//pub use config::*;
mod create_token_account;
mod process_payouts;

use fang::asynk::async_queue::AsyncQueue;
use fang::AsyncQueueable;
use fang::AsyncRunnable;
use fang::AsyncWorkerPool;
use fang::NoTls;

use create_token_account::CreateTokenAccountsTask;

use error::TaskError;
use solana_sdk::pubkey::Pubkey;

use self::process_payouts::ProcessPayoutsTask;

#[derive(Clone)]
pub struct Task {
    pool: AsyncWorkerPool<AsyncQueue<NoTls>>,
    queue: AsyncQueue<NoTls>,
}

impl Task {
    pub async fn init(
        db_uri: String,
        token_mint_pubkey: String,
        keypair: String,
    ) -> anyhow::Result<Self, TaskError> {
        let mut queue = AsyncQueue::builder()
            .uri(db_uri)
            .max_pool_size(10_u32)
            .build();

        queue.connect(NoTls).await?;

        let mut pool = AsyncWorkerPool::builder()
            .number_of_workers(10_u32)
            .queue(queue.clone())
            .build();

        let token_mint_pubkey: Pubkey = token_mint_pubkey.parse()?;

        let create_token_accounts_task = CreateTokenAccountsTask {
            token_mint_pubkey,
            keypair: keypair.clone(),
        };

        queue
            .schedule_task(&create_token_accounts_task as &dyn AsyncRunnable)
            .await?;

        let process_payouts_task = ProcessPayoutsTask {
            token_mint_pubkey,
            keypair: keypair.clone(),
        };

        queue
            .schedule_task(&process_payouts_task as &dyn AsyncRunnable)
            .await?;

        pool.start().await;

        Ok(Self { pool, queue })
    }
}
