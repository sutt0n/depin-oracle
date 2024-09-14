use serde::{Deserialize, Serialize};

use crate::broker::BrokerConfig;
use crate::solana::SolanaConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub broker: BrokerConfig,
    #[serde(default)]
    pub solana: SolanaConfig,
}
