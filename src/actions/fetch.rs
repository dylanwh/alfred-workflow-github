use std::{env, path::PathBuf, collections::HashMap};

use eyre::{Result, Context};
use itertools::Itertools;
use octocrab::{models::Repository, Page};

use crate::alfred_models::{Item, Icon};

pub async fn run() -> Result<()> {
    dotenv::dotenv().ok();

    #[allow(unused)]
    let alfred_workflow_data =
        env::var("alfred_workflow_data").wrap_err("alfred_workflow_data not defined")?;
    let alfred_workflow_data_dir = PathBuf::from(&alfred_workflow_data);
    tokio::fs::create_dir_all(&alfred_workflow_data_dir).await?;

    let all_repos_path = alfred_workflow_data_dir.join("repos.json");
    if all_repos_path.exists()
        && all_repos_path.metadata()?.modified()?.elapsed()?.as_secs() < 86400
    {
        let all_repos = tokio::fs::read_to_string(all_repos_path).await?;
        println!("{}", all_repos);
        return Ok(());
    }

    let repos = user_repos().await?;
    let starred_repos = user_starred_repos().await?;
    let all_repos = repos
        .into_iter()
        .chain(starred_repos.into_iter())
        .unique_by(|r| r.full_name.clone())
        .sorted_by(|a, b| a.full_name.cmp(&b.full_name))
        .filter(|r| r.full_name.is_some() && r.archived == Some(false))
        .map(|r| {
            let vars = extract_variables(&r);
            Item::builder()
                .title(r.full_name.clone())
                .subtitle(r.description)
                .arg(r.html_url.map(|u| u.to_string()))
                .matches(vec![r.full_name.clone().unwrap_or_default(), r.name].join(" "))
                .icon(
                    Icon::builder()
                        .path(format!(
                            "{}/{}.png",
                            alfred_workflow_data,
                            r.owner.map(|o| o.login).unwrap_or_default()
                        ))
                        .build(),
                )
                .variables(vars)
                .build()
        })
        .collect::<Vec<_>>();

    let http_client = reqwest::Client::new();

    let fetches = all_repos
        .iter()
        .flat_map(|item| item.owner())
        .unique()
        .map(|ref owner| {
            fetch_github_user_avatar(&http_client, &alfred_workflow_data, owner.clone())
        });
    futures::future::join_all(fetches).await;

    let json = serde_json::to_string(&all_repos)?;
    println!("{}", json);
    tokio::fs::write(all_repos_path, json).await?;

    Ok(())
}

async fn fetch_github_user_avatar(
    client: &reqwest::Client,
    alfred_workflow_data: &str,
    owner: String,
) -> Result<()> {
    let icon_url = format!("https://github.com/{owner}.png");
    let icon_path = PathBuf::from(format!("{alfred_workflow_data}/{owner}.png"));
    if icon_path.exists() {
        return Ok(());
    }
    let res = client.get(&icon_url).send().await?;
    tokio::fs::write(icon_path, res.bytes().await?).await?;

    Ok(())
}

fn extract_variables(r: &Repository) -> HashMap<String, String> {
    let mut variables = HashMap::new();
    if let Some(ref full_name) = r.full_name {
        variables.insert("full_name".into(), full_name.clone());
    }
    variables.insert("name".into(), r.name.clone());
    if let Some(ref owner) = r.owner {
        variables.insert("owner".into(), owner.login.clone());
    }
    if let Some(ref html_url) = r.html_url {
        variables.insert("html_url".into(), html_url.to_string());
    }

    variables
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
