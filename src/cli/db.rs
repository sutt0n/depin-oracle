use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    #[serde(default)]
    pub pg_con: String,
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
}

static POOL: OnceLock<sqlx::PgPool> = OnceLock::new();

pub async fn init_pool(config: &DbConfig) -> anyhow::Result<sqlx::PgPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.pool_size)
        .connect(&config.pg_con)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    POOL.set(pool.clone()).unwrap();

    Ok(pool)
}

pub fn get_pool() -> &'static sqlx::PgPool {
    POOL.get().unwrap()
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            pg_con: "".to_string(),
            pool_size: default_pool_size(),
        }
    }
}

fn default_pool_size() -> u32 {
    20
}
