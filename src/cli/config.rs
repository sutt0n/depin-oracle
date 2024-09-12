use anyhow::Context;
use serde::{Deserialize, Serialize};

use std::path::Path;

use crate::{app::AppConfig, broker::BrokerConfig};

use super::db::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub db: DbConfig,
    #[serde(default)]
    pub app: AppConfig,
}

pub struct EnvOverride {
    pub db_con: String,
}

impl Config {
    pub fn from_path(
        path: Option<impl AsRef<Path>>,
        EnvOverride { db_con }: EnvOverride,
    ) -> anyhow::Result<Self> {
        let mut config: Config = if let Some(path) = path {
            let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;

            println!("config_file: {:?}", config_file);

            serde_yaml::from_str(&config_file).context("Couldn't parse config file")?
        } else {
            Default::default()
        };

        println!("db_con: {:?}", config);

        config.db.pg_con = db_con;

        Ok(config)
    }
}
