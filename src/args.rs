
use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {

    /// Install the workflow
    Install,


    /// Configure local settings
    Config {
        #[clap(subcommand)]
        method: ConfigMethod,
    },

    /// Refresh the cache
    Refresh,

    /// List repos for the current user
    Repos {
        #[clap(long, default_value = "false")]
        no_cache: bool,
    },

    /// List pull requests for a repo
    Pulls {
        repo: crate::FullName,
    },

    /// Search issues and pull requests
    SearchIssues {
        #[clap(subcommand)]
        query: SearchQuery
    },

    /// Copy repo info to the clipboard using mdcopy
    Copy,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SearchQuery {
    Reviews,
    Pulls,
    Custom {
        query: String,
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum ConfigMethod {
    /// Open the configuration in the default macos editor
    Open,

    /// Open in the $EDITOR or $VISUAL environment variable, in a terminal
    Edit,
}

impl Args {
    pub fn new() -> Result<Self> {
        Ok(Self::parse())
    }
}
