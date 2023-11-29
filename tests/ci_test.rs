use forgejo_api::{Forgejo, ForgejoError};

#[tokio::test]
async fn ci() -> eyre::Result<()> {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL")?)?;
    let token = std::env::var("FORGEJO_API_CI_TOKEN")?;
    let api = Forgejo::new(&token, url)?;

    let mut results = Vec::new();

    results.push(user(&api).await.map_err(|e| eyre::eyre!("user error: {e}")));

    let mut errors = 0;
    for res in results.into_iter().filter_map(Result::err) {
        errors += 1;
        println!("{res}");
    }
    if errors > 0 {
        eyre::bail!("test failed");
    }

    Ok(())
}

async fn user(api: &forgejo_api::Forgejo) -> Result<(), ForgejoError> {
    Ok(())
}
