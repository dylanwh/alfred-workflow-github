mod actions;
mod alfred_models;
mod args;
mod hub_compat;

use args::{Action, Args};
use eyre::Result;
use hub_compat::HubConfig;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    let args = Args::new()?;
    configure_octocrab().await?;

    match args.action {
        Action::Fetch => actions::fetch::run().await?,
        Action::Pulls { repo } => actions::pulls::run(repo).await?,
    }

    Ok(())
}

async fn configure_octocrab() -> Result<()> {
    let hc = HubConfig::new().await?;
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(hc.oauth_token)
        .build()?;
    octocrab::initialise(octocrab);

    Ok(())
}
