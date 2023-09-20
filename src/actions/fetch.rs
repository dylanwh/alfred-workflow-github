use eyre::Result;
use itertools::Itertools;
use octocrab::{models::Repository, Page};
use serde_json::json;

use crate::alfred::{AuthorIcon, Item, Items, ALFRED_WORKFLOW_DATA};

pub async fn run() -> Result<()> {
    dotenv::dotenv().ok();

    #[allow(unused)]
    tokio::fs::create_dir_all(ALFRED_WORKFLOW_DATA.as_path()).await?;

    let all_repos_path = ALFRED_WORKFLOW_DATA.join("repos.json");
    if all_repos_path.exists()
        && all_repos_path.metadata()?.modified()?.elapsed()?.as_secs() < 86400
    {
        let all_repos = tokio::fs::read_to_string(all_repos_path).await?;
        println!("{}", all_repos);
        return Ok(());
    }

    let repos = user_repos().await?;
    let starred_repos = user_starred_repos().await?;
    let items: Items = repos
        .into_iter()
        .chain(starred_repos.into_iter())
        .unique_by(|r| r.full_name.clone())
        .sorted_by(|a, b| a.full_name.cmp(&b.full_name))
        .filter(|r| r.full_name.is_some() && r.archived == Some(false))
        .flat_map(|r| {
            let html_url = r.html_url.clone()?.to_string();

            Some(
                Item::builder()
                    .title(r.full_name.clone()?)
                    .subtitle(r.description.clone()?)
                    .uid(&html_url)
                    .arg(&html_url)
                    .matches(vec![r.full_name.clone()?, r.name.clone()].join(" "))
                    .icon(AuthorIcon::from(r.owner.clone()))
                    .variables({
                        Some(json!(
                           {
                               "full_name": r.full_name.as_ref()?,
                               "name": r.name.clone(),
                               "owner": r.owner.clone()?.login,
                               "html_url": r.html_url.clone()?.to_string()
                           }
                        ))
                    })
                    .build(),
            )
        })
        .collect();

    let http_client = reqwest::Client::new();

    let fetches = items
        .owners()
        .map(|ref owner| fetch_github_user_avatar(&http_client, owner.clone()));
    futures::future::join_all(fetches).await;

    let json = serde_json::to_string(&items)?;
    println!("{}", json);
    tokio::fs::write(all_repos_path, json).await?;

    Ok(())
}

async fn fetch_github_user_avatar(client: &reqwest::Client, owner: String) -> Result<()> {
    let icon_url = format!("https://github.com/{owner}.png");
    let icon_path = ALFRED_WORKFLOW_DATA.join(format!("{owner}.png"));
    if icon_path.exists() {
        return Ok(());
    }
    let res = client.get(&icon_url).send().await?;
    tokio::fs::write(icon_path, res.bytes().await?).await?;

    Ok(())
}

async fn user_repos() -> Result<Page<Repository>> {
    let repos = octocrab::instance()
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await?;
    Ok(repos)
}

async fn user_starred_repos() -> Result<Page<Repository>> {
    let repos = octocrab::instance()
        .current()
        .list_repos_starred_by_authenticated_user()
        .send()
        .await?;
    Ok(repos)
}
