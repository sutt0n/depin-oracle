use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SolanaConfig {
    #[serde(default)]
    pub keypair: String,
    #[serde(default)]
    pub mint_address: String,
}
