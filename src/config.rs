use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub surrealdb: SurrealDbConfig,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SurrealDbConfig {
    pub host: Url,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn load(file: impl Into<Option<String>>) -> Result<Self> {
        let s = fs::read_to_string(file.into().unwrap_or("config.dhall".to_string()))?;
        let config: Config = serde_dhall::from_str(&s).parse()?;
        Ok(config)
    }
}
