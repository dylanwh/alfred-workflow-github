use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    Fetch,
    Pulls { repo: String },
}

impl Args {
    pub fn new() -> Result<Self> {
        Ok(Self::parse())
    }
}
