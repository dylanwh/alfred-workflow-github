pub mod models;

use lazy_static::lazy_static;
pub use models::*;
use std::{env, path::PathBuf};

lazy_static! {
    pub static ref ALFRED_WORKFLOW_DATA: PathBuf =
        PathBuf::from(env::var("alfred_workflow_data").unwrap_or_default());
    pub static ref ALFRED_WORKFLOW_CACHE: PathBuf =
        PathBuf::from(env::var("alfred_workflow_cache").unwrap_or_default());
}
