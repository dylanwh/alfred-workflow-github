use eyre::Result;

use crate::alfred::ALFRED_WORKFLOW_DATA;

pub async fn run() -> Result<()> {
    let repos_json = ALFRED_WORKFLOW_DATA.as_ref()?.join("repos.json");
    if repos_json.exists() {
        println!("removed {}", repos_json.to_string_lossy());
        tokio::fs::remove_file(repos_json).await?;
    }

    Ok(())
}
