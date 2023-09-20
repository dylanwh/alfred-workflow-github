use eyre::Result;
use regex::Regex;
use serde_json::json;

use crate::alfred::{AuthorIcon, Item, Items};

pub async fn run(query: String, limit: u8) -> Result<()> {
    let full_name_re = Regex::new(r"^https://github.com/(?<full_name>[^/]+/[^/]+)")?;

    let items = octocrab::instance()
        .search()
        .issues_and_pull_requests(&query)
        .page(1u8)
        .per_page(limit)
        .sort("updated")
        .send()
        .await?
        .into_iter()
        .flat_map(|issue| {
            let html_url = issue.html_url.clone().to_string();
            let caps = full_name_re.captures(&html_url)?;
            let full_name = format!(
                "{full_name}#{number}",
                full_name = &caps["full_name"],
                number = issue.number
            );

            Some(
                Item::builder()
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
                    .build(),
            )
        })
        .collect::<Items>();

    let json = serde_json::to_string(&items)?;
    println!("{}", json);

    Ok(())
}
