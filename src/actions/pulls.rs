use eyre::Result;

pub async fn run(repo: String) -> Result<()> {
    match repo.split_once('/') {
        Some((owner, repo)) => octocrab::instance().pulls(repo).list().send().await?;
        None => todo!(),
    }

    Ok(())
}