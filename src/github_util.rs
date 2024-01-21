use std::path::Path;

use eyre::Result;
use tokio::fs;

use crate::alfred::{Items, ALFRED_WORKFLOW_CACHE};

pub async fn fetch_avatars(items: &Items) -> Result<()> {
    let http_client = reqwest::Client::new();
    let cache_dir = ALFRED_WORKFLOW_CACHE.as_ref()?;

    let fetches = items
        .owners()
        .map(|ref owner| fetch_github_user_avatar(&http_client, cache_dir, owner.clone()));
    futures::future::join_all(fetches).await;

    Ok(())
}

pub async fn fetch_github_user_avatar<P>(
    client: &reqwest::Client,
    cache_dir: P,
    owner: String,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let icon_url = format!("https://github.com/{owner}.png");
    let cache_dir = cache_dir.as_ref();
    let icon_path = cache_dir.join(format!("{owner}.png"));
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir).await?;
    }
    if icon_path.exists() {
        return Ok(());
    }
    let res = client.get(&icon_url).send().await?;
    fs::write(icon_path, res.bytes().await?).await?;

    Ok(())
}
