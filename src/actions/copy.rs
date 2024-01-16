use std::{env, path::PathBuf, process::Stdio};

use eyre::{Context, ContextCompat, Result};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyConfig {
    rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    focusedapp: String,
    mdcopy: String,
}

impl Default for CopyConfig {
    fn default() -> Self {
        Self {
            rules: vec![
                Rule {
                    focusedapp: "com.microsoft.edgemac".to_string(),
                    mdcopy: r#"-f markdown --trim -t -i "[$full_name]($html_url)""#.to_string(),
                },
                Rule {
                    focusedapp: "com.google.Chrome".to_string(),
                    mdcopy: r#"-f markdown --trim -t -i "[$full_name]($html_url)""#.to_string(),
                },
                Rule {
                    focusedapp: "com.apple.Safari".to_string(),
                    mdcopy: r#"-f markdown --trim -t -i "[$full_name]($html_url)""#.to_string(),
                },
                Rule {
                    focusedapp: "*".to_string(),
                    mdcopy: r#"-f text --trim -t -i "[$full_name]($html_url)""#.to_string(),
                },
            ],
        }
    }
}

pub async fn run(config: &CopyConfig) -> Result<()> {
    let mdcopy = PathBuf::from(env::var("mdcopy").wrap_err("missing mdcopy env")?);
    if !mdcopy.exists() {
        return Err(eyre::eyre!("mdcopy not found at {:?}", mdcopy));
    }
    let focusedapp = env::var("focusedapp").wrap_err("missing focusedapp env")?;
    // use wildmatch to match the focusedapp against the rules
    let rule = config
        .rules
        .iter()
        .find(|rule| wildmatch::WildMatch::new(&rule.focusedapp).matches(&focusedapp))
        .wrap_err(format!("no rule found for focusedapp {}", focusedapp))?;
    let args = &rule.mdcopy;
    let cmdline = format!("$mdcopy {args}");
    let sh = env::var("SHELL")
        .ok()
        .unwrap_or_else(|| "/bin/sh".to_string());
    let mut child = Command::new(sh)
        .arg("-c")
        .arg(cmdline)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    child.wait().await?;

    Ok(())
}
