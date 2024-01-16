
use eyre::{ContextCompat, Result};
use octocrab::models::issues::Issue;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    alfred::{AuthorIcon, Item, Items},
    args::SearchQuery,
    OCTOCRAB, github_util,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIssuesConfig {
    reviews: String,
    pulls: String,
}

impl Default for SearchIssuesConfig {
    fn default() -> Self {
        Self {
            reviews: "is:open is:pr user-review-requested:@me archived:false".to_string(),
            pulls: "is:open is:pr archived:false author:@me".to_string()
        }
    }
}

pub async fn run(config: &SearchIssuesConfig, query: SearchQuery) -> Result<()> {
    let full_name_re = Regex::new(r"^https://github.com/(?<full_name>[^/]+/[^/]+)")?;

    let query = match query {
        SearchQuery::Reviews => &config.reviews,
        SearchQuery::Pulls => &config.pulls,
        SearchQuery::Custom { ref query } => query,
    };

    let items: Items = OCTOCRAB
        .clone()
        .search()
        .issues_and_pull_requests(&query)
        .page(1u8)
        .per_page(15u8)
        .sort("updated")
        .send()
        .await?
        .into_iter()
        .map(|issue| issue_to_item(issue, &full_name_re))
        .collect::<Result<_>>()?;

    github_util::fetch_avatars(&items).await?;

    let json = serde_json::to_string(&items)?;
    println!("{}", json);

    Ok(())
}

fn issue_to_item(issue: Issue, full_name_re: &Regex) -> Result<Item> {
    let html_url = issue.html_url.clone().to_string();
    let caps = full_name_re
        .captures(&html_url)
        .wrap_err(format!("full_name_re failed to match {html_url}"))?;
    let full_name = format!(
        "{full_name}#{number}",
        full_name = &caps["full_name"],
        number = issue.number
    );

    let items = Item::builder()
        .title(issue.title.clone())
        .subtitle(format!(
            "{full_name} opened by {login}",
            full_name = full_name,
            login = issue.user.login
        ))
        .arg(&html_url)
        .icon(AuthorIcon::from(&issue.user))
        .variables(json!(
            {
                "created_at": issue.created_at,
                "updated_at": issue.updated_at,
                "full_name": full_name,
                "html_url": html_url,
            }
        ))
        .build();

    Ok(items)
}
