
use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    Refresh,
    Repos {
        #[clap(long, default_value = "false")]
        no_cache: bool,
    },
    Pulls {
        repo: crate::FullName,
    },

    /// Search issues and pull requests
    SearchIssues {
        #[clap(subcommand)]
        query: SearchQuery
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum SearchQuery {
    Reviews,
    Pulls,
    Custom {
        query: String,
    }
}

impl Args {
    pub fn new() -> Result<Self> {
        Ok(Self::parse())
    }
}
