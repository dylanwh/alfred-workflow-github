mod actions;
mod alfred;
mod args;
mod hub_compat;

use std::{str::FromStr, fmt::{Display, Formatter}};

use args::{Action, Args};
use eyre::Result;
use hub_compat::HubConfig;


#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    let args = Args::new()?;
    configure_octocrab().await?;

    match args.action {
        Action::Fetch => actions::fetch::run().await?,
        Action::Pulls { repo } => actions::pulls::run(repo, args.limit).await?,
        Action::SearchIssues { query } => actions::search_issues::run(query, args.limit).await?,
    }

    Ok(())
}

async fn configure_octocrab() -> Result<()> {
    dotenv::dotenv().ok();
    let hc = HubConfig::new().await?;
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(hc.oauth_token)
        .build()?;
    octocrab::initialise(octocrab);

    Ok(())
}


#[derive(Clone, Debug)]
pub struct FullName {
    pub owner: String,
    pub name: String,
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl FromStr for FullName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('/') {
            Some((owner, name)) => Ok(Self {
                owner: owner.to_string(),
                name: name.to_string(),
            }),
            None => Err("not a github repository full-name".to_string()),
        }
    }
}
