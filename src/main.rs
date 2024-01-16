mod actions;
mod alfred;
mod args;
mod config;
mod hub_compat;
mod github_util;

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    sync::Arc,
};

use args::{Action, Args};
use config::Config;
use eyre::Result;
use hub_compat::HubConfig;
use octocrab::Octocrab;
use once_cell::sync::Lazy;

/// A global instance of Octocrab.
///
/// this is used instead of octocrab::instance() because we need to set the
/// personal token from the hub config, and that costs 500ms when using octocrab::initialize().
static OCTOCRAB: Lazy<Arc<Octocrab>> = Lazy::new(|| {
    let hc = HubConfig::new().expect("failed to read hub config");
    let octocrab = octocrab::OctocrabBuilder::default()
        .personal_token(hc.oauth_token)
        .build()
        .expect("failed to build octocrab");

    Arc::new(octocrab)
});

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::new()?;


    match args.action {
        Action::Install  => actions::install::run()?,
        Action::Refresh => actions::refresh::run().await?,
        Action::Config { method } => actions::config::run(method).await?,
        Action::Repos { no_cache } => actions::repos::run(no_cache).await?,
        Action::Pulls { repo } => actions::pulls::run(repo).await?,
        Action::SearchIssues { query } => {
            let config = Config::load().await?;
            actions::search_issues::run(&config.search_issues, query).await?
        },
        Action::Copy => {
            let config = Config::load().await?;
            actions::copy::run(&config.copy).await?
        },
    }

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
