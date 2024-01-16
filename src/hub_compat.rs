use std::{collections::HashMap, fs::read_to_string};

use eyre::{Context, ContextCompat, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HubConfig {
    pub user: String,
    pub oauth_token: String,
    pub protocol: Option<String>,
}

// even on macos this is a path under ~/.config, not ~/Library/Application Support
// so we can't use the dirs::config_dir() function.
// This file appears to be yaml, so we can use serde to parse it.
impl HubConfig {
    pub fn new() -> Result<HubConfig> {
        let config_path = dirs::home_dir()
            .wrap_err("missing $HOME")?
            .join(".config/hub");
        let config = read_to_string(config_path).wrap_err("failed to read hub config")?;
        let mut config = serde_yaml::from_str::<HashMap<String, Vec<HubConfig>>>(&config)?;
        let config = config
            .remove("github.com")
            .wrap_err("missing github.com config")?
            .pop()
            .wrap_err("expected github.com to have at least one item")?;

        Ok(config)
    }
}
