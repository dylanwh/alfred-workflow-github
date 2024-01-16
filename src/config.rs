use std::path::PathBuf;

use eyre::{ContextCompat, Result};
use serde::{Deserialize, Serialize};

use crate::actions::{search_issues::SearchIssuesConfig, copy::CopyConfig};

// TODO: later this could function as app config for the workflow
// I like to customize this per-machine, so it's stored in the workflow data dir
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub search_issues: SearchIssuesConfig,

    #[serde(default)]
    pub copy: CopyConfig,
}

impl Config {
    pub fn file() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .wrap_err("unable to find config dir")?
            .join(env!("CARGO_PKG_NAME"));
        let config_file = config_dir.join("config.toml");
        Ok(config_file)
    }

    pub async fn load() -> Result<Self> {
        let config_file = Self::file()?;
        if !config_file.exists() {
            let config = Config::default();
            let config = toml::to_string_pretty(&config)?;
            if let Some(parent) = config_file.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            tokio::fs::write(&config_file, config).await?;
        }
        let config = tokio::fs::read_to_string(&config_file).await?;
        Ok(toml::from_str(&config)?)
    }
}
