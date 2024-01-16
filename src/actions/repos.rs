use std::str::FromStr;

use eyre::{ContextCompat, Result};
use itertools::Itertools;
use octocrab::{models::Repository, Page};
use serde_json::json;
use tokio::fs;

use crate::{
    alfred::{tokenize, AuthorIcon, Item, Items, Modifier, Modifiers, ALFRED_WORKFLOW_DATA},
    OCTOCRAB, github_util,
};

pub async fn run(no_cache: bool) -> Result<()> {
    fs::create_dir_all(ALFRED_WORKFLOW_DATA.as_ref()?).await?;

    let all_repos_path = ALFRED_WORKFLOW_DATA.as_ref()?.join("repos.json");
    if !no_cache
        && all_repos_path.exists()
        && all_repos_path.metadata()?.modified()?.elapsed()?.as_secs() < 86400
    {
        let all_repos = fs::read_to_string(all_repos_path).await?;
        println!("{}", all_repos);
        return Ok(());
    }

    let octocrab = OCTOCRAB.clone();
    let repos = octocrab.all_pages(user_repos().await?).await?;
    let starred_repos = octocrab.all_pages(user_starred_repos().await?).await?;
    let items: Items = repos
        .into_iter()
        .chain(starred_repos.into_iter())
        .unique_by(|r| r.full_name.clone())
        .sorted_by(|a, b| a.full_name.cmp(&b.full_name))
        .filter(|r| r.full_name.is_some() && r.archived == Some(false))
        .map(repository_to_item)
        .collect::<Result<_>>()?;

    github_util::fetch_avatars(&items).await?;

    let json = serde_json::to_string(&items)?;
    println!("{}", json);
    fs::write(all_repos_path, json).await?;

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


async fn user_repos() -> Result<Page<Repository>> {
    let repos = OCTOCRAB
        .clone()
        .current()
        .list_repos_for_authenticated_user()
        .per_page(100u8)
        .send()
        .await?;
    Ok(repos)
}

async fn user_starred_repos() -> Result<Page<Repository>> {
    let repos = OCTOCRAB
        .clone()
        .current()
        .list_repos_starred_by_authenticated_user()
        .per_page(100u8)
        .send()
        .await?;
    Ok(repos)
}
