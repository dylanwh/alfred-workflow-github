pub mod models;

use eyre::Result;
use indexmap::IndexSet;
pub use models::*;
use once_cell::sync::Lazy;
use std::{env, path::PathBuf, str::FromStr};

use crate::FullName;

pub static ALFRED_WORKFLOW_DATA: Lazy<Result<PathBuf, env::VarError>> =
    Lazy::new(|| env::var("alfred_workflow_data").map(PathBuf::from));

pub static ALFRED_WORKFLOW_CACHE: Lazy<Result<PathBuf, env::VarError>> =
    Lazy::new(|| env::var("alfred_workflow_cache").map(PathBuf::from));

static TOKEN_SEPARATORS: &[char] = &['-', '.', '_', '/'];

pub fn tokenize(s: &str) -> Result<Vec<String>> {
    let mut tokens = IndexSet::new();
    tokens.insert(s.to_string());
    if let Ok(full_name) = FullName::from_str(s) {
        tokens.insert(full_name.owner);
        tokens.insert(full_name.name);
    }
    let parts = s.split(TOKEN_SEPARATORS);
    for part in parts {
        // don't re-add, as this would change the order
        if !tokens.contains(part) {
            tokens.insert(part.to_string());
        }
    }

    Ok(tokens.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("foo/bar").unwrap(),
            vec!["foo/bar".to_string(), "foo".to_string(), "bar".to_string()]
        );
        assert_eq!(
            tokenize("foo/bar-baz").unwrap(),
            vec![
                "foo/bar-baz".to_string(),
                "foo".to_string(),
                "bar-baz".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ]
        );
    }
}
