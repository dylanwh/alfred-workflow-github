use eyre::Result;
use tokio::fs::read_to_string;

use crate::{args::ConfigMethod, config::Config};

pub async fn run(method: ConfigMethod) -> Result<()> {
    let config_file = Config::file()?;
    match method {
        ConfigMethod::Open => open::that(&config_file)?,
        ConfigMethod::Edit => {
            let _ = Config::load().await?;
            let config = read_to_string(&config_file).await?;
            let changes = edit::edit(&config)?;
            if !changes.is_empty() && changes != config {
                tokio::fs::write(&config_file, changes).await?;
            }
        }
    };
    Ok(())
}
