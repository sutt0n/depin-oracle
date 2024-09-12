use serde::{Deserialize, Serialize};

use crate::broker::BrokerConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub broker: BrokerConfig,
    //#[serde(default)]
    //pub account: AccountConfig,
}
