

use eyre::Result;

use serde_json::json;

use crate::{
    alfred::{
        models::{Item, Items},
        AuthorIcon,
    },
    FullName,
};

pub async fn run(repo: FullName, limit: u8) -> Result<()> {
    let pulls = octocrab::instance()
        .pulls(repo.owner, repo.name)
        .list()
        .page(1u8)
        .per_page(limit)
        .send()
        .await?;
    let items = pulls
        .into_iter()
        .flat_map(|pull| {
            let html_url = pull.html_url.clone()?.to_string();
            Some(
                Item::builder()
                    .title(pull.title.clone()?)
                    .subtitle(format!(
                        "#{number} opened by {login}",
                        number = pull.number,
                        login = pull.user.as_ref().map(|u| u.login.as_str()).unwrap_or("")
                    ))
                    .arg(html_url)
                    .icon(AuthorIcon::from(&pull.user))
                    .variables({
                        let pull = &pull;
                        let full_name = pull.base.clone().repo?.full_name?;
                        let number = pull.number;

                        Some(json! {
                            {
                                "created_at": pull.created_at,
                                "updated_at": pull.updated_at,
                                "full_name": format!("{full_name}#{number}")
                            }
                        })
                    })
                    .build(),
            )
        })
        .collect::<Items>();

    let json = serde_json::to_string(&items)?;
    println!("{}", json);

    Ok(())
}