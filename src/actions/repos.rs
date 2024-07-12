use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use chrono::{DateTime, Duration, Utc};
use eyre::{ContextCompat, Result};
use futures::Future;
use itertools::Itertools;
use octocrab::{models::Repository, Octocrab};
use serde_json::json;
use tokio::fs;

use crate::{
    alfred::{tokenize, AuthorIcon, Item, Items, Modifier, Modifiers, ALFRED_WORKFLOW_DATA},
    github_util, OCTOCRAB,
};

pub async fn run(no_cache: bool) -> Result<()> {
    fs::create_dir_all(ALFRED_WORKFLOW_DATA.as_ref()?).await?;

    let both_cache = ALFRED_WORKFLOW_DATA.as_ref()?.join("both.json");
    let repos_cache = ALFRED_WORKFLOW_DATA.as_ref()?.join("repos.json");
    let stars_cache = ALFRED_WORKFLOW_DATA.as_ref()?.join("stars.json");
    if !no_cache
        && both_cache.exists()
        && both_cache.metadata()?.modified()?.elapsed()?.as_secs() < 86400
    {
        let all_repos = fs::read_to_string(both_cache).await?;
        println!("{}", all_repos);
        return Ok(());
    }

    let octocrab = OCTOCRAB.clone();
    let repos =
        cache_repos_smart(&repos_cache, |since| user_repos(octocrab.clone(), since)).await?;
    let stars = cache_repos(stars_cache, Duration::seconds(3600), || {
        user_starred_repos(octocrab.clone())
    })
    .await?;

    let items: Items = repos
        .into_iter()
        .chain(stars.into_iter())
        .unique_by(|r| r.full_name.clone())
        .sorted_by(|a, b| a.full_name.cmp(&b.full_name))
        .filter(|r| r.full_name.is_some() && r.archived == Some(false))
        .map(repository_to_item)
        .collect::<Result<_>>()?;

    github_util::fetch_avatars(&items).await?;

    let json = serde_json::to_string(&items)?;
    println!("{}", json);
    fs::write(both_cache, json).await?;

    Ok(())
}

fn repository_to_item(r: Repository) -> Result<Item> {
    let html_url = r.html_url.clone().wrap_err("html_url is None")?.to_string();
    let full_name = r.full_name.clone().wrap_err("full_name is None")?;
    let name = r.name.clone();
    let owner = r.owner.clone().wrap_err("owner is None")?.login;
    let item = Item::builder()
        .title(&full_name)
        .subtitle(r.description.clone())
        .uid(&html_url)
        .arg(&html_url)
        .matches(tokenize(&full_name)?.join(" "))
        .icon(AuthorIcon::from_str(&owner)?)
        .variables(json!(
           {
               "full_name": &full_name,
               "name": &name,
               "owner": &owner,
               "html_url": &html_url,
           }
        ))
        .mods(
            Modifiers::builder()
                .alt(
                    Modifier::builder()
                        .subtitle("View GitHub Wiki")
                        .arg(format!("{html_url}/wiki"))
                        .build(),
                )
                .cmd(
                    Modifier::builder()
                        .subtitle("View Pull Requests")
                        .variables(json!({
                            "action": "pulls",
                            "full_name": &full_name,
                            "html_url": &html_url,
                        }))
                        .build(),
                )
                .build(),
        )
        .build();

    Ok(item)
}

async fn user_repos(crab: Arc<Octocrab>, since: Option<Since>) -> Result<Vec<Repository>> {
    let mut repos = crab
        .current()
        .list_repos_for_authenticated_user()
        .per_page(100u8);
    if let Some(since) = since {
        repos = repos.since(since);
    }

    Ok(crab.all_pages(repos.send().await?).await?)
}

async fn user_starred_repos(crab: Arc<Octocrab>) -> Result<Vec<Repository>> {
    let repos = crab
        .current()
        .list_repos_starred_by_authenticated_user()
        .per_page(100u8)
        .send()
        .await?;
    Ok(crab.all_pages(repos).await?)
}

type Since = DateTime<Utc>;

async fn cache_repos_smart<P, F, T>(file: P, fetch: F) -> Result<Vec<Repository>>
where
    P: AsRef<Path>,
    F: FnOnce(Option<Since>) -> T,
    T: Future<Output = Result<Vec<Repository>>>,
{
    let repos = if let Ok((since, repos)) = cached(file.as_ref()).await {
        let new_repos = fetch(Some(since)).await?;
        let repos = repos
            .into_iter()
            .chain(new_repos.into_iter())
            .filter(|r| r.full_name.is_some() && r.archived == Some(false))
            .collect::<Vec<_>>();
        fs::write(file.as_ref(), serde_json::to_string(&repos)?).await?;
        repos
    } else {
        fetch(None).await?
    };

    Ok(repos)
}

async fn cache_repos<P, F, T>(file: P, expires: Duration, fetch: F) -> Result<Vec<Repository>>
where
    P: AsRef<Path>,
    F: FnOnce() -> T,
    T: Future<Output = Result<Vec<Repository>>>,
{
    match cached(file.as_ref()).await {
        Ok((since, repos)) if Utc::now() - since < expires => Ok(repos),
        _ => {
            let repos = fetch().await?;
            fs::write(file.as_ref(), serde_json::to_string(&repos)?).await?;

            Ok(repos)
        }
    }
}

async fn cached<P>(file: P) -> Result<(Since, Vec<Repository>)>
where
    P: AsRef<Path>,
{
    let repos = fs::read_to_string(file.as_ref()).await?;
    let repos: Vec<Repository> = serde_json::from_str(&repos)?;
    let since = tokio::fs::metadata(file.as_ref()).await?.modified()?;

    Ok((DateTime::from(since), repos))
}
