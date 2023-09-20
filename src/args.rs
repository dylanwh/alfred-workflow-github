use clap::{Parser, Subcommand};
use eyre::Result;


#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,

    #[arg(long, short, default_value = "10")]
    pub limit: u8,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    Fetch,
    Pulls { repo: crate::FullName },
    SearchIssues { query: String },
}

impl Args {
    pub fn new() -> Result<Self> {
        Ok(Self::parse())
    }
}
