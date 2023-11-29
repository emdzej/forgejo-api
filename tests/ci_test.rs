use forgejo_api::Forgejo;
use eyre::{eyre, ensure};

#[tokio::test]
async fn ci() -> eyre::Result<()> {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL")?)?;
    let token = std::env::var("FORGEJO_API_CI_TOKEN")?;
    let api = Forgejo::new(&token, url)?;

    let mut results = Vec::new();

    results.push(user(&api).await.map_err(|e| eyre!("user error: {e}")));
    results.push(repo(&api).await.map_err(|e| eyre!("repo error: {e}")));

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

async fn user(api: &forgejo_api::Forgejo) -> eyre::Result<()> {
    let myself = api.myself().await?;
    ensure!(myself.is_admin, "user should be admin");
    ensure!(myself.login == "TestingAdmin", "user should be named \"TestingAdmin\"");

    let myself_indirect = api
        .get_user("TestingAdmin")
        .await?
        .ok_or_else(|| eyre!("\"TestingAdmin\" not found, but should have been."))?;
    ensure!(myself == myself_indirect, "result of `myself` does not match result of `get_user`");

    let following = api.get_following("TestingAdmin").await?;
    ensure!(following == Some(Vec::new()), "following list not empty");
    let followers = api.get_followers("TestingAdmin").await?;
    ensure!(followers == Some(Vec::new()), "follower list not empty");

    Ok(())
}

async fn repo(api: &forgejo_api::Forgejo) -> eyre::Result<()> {
    tokio::fs::create_dir("/test_repo").await?;
    let local_repo = git2::Repository::init("/test_repo")?;
    tokio::fs::write("/test_repo/README.md", "# Test\nThis is a test repo").await?;
    let mut index = local_repo.index()?;
    index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree = local_repo
        .find_tree(index.write_tree().unwrap())
        .unwrap();
    let author = git2::Signature::now("TestingAdmin", "admin@noreply.example.org").unwrap();
    let commit_oid = local_repo
        .commit(None, &author, &author, "bibblybeebly", &tree, &[])
        .unwrap();
    let branch = local_repo
        .branch("main", &local_repo.find_commit(commit_oid).unwrap(), true)
        .unwrap();
    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().unwrap();
    local_repo.set_head(branch_ref_name).unwrap();

    let repo_opt = forgejo_api::CreateRepoOption {
        auto_init: false,
        default_branch: "main".into(),
        description: Some("Test Repo".into()),
        gitignores: "".into(),
        issue_labels: "".into(),
        license: "".into(),
        name: "test".into(),
        private: false,
        readme: "".into(),
        template: false,
        trust_model: forgejo_api::TrustModel::Default,
    };
    let remote_repo = api.create_repo(repo_opt).await?;
    ensure!(remote_repo.owner.login == "TestingAdmin", "repo owner is not \"TestingAdmin\"");
    ensure!(remote_repo.name == "test", "repo owner is not \"test\"");

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        git2::Cred::userpass_plaintext("TestingAdmin", "password")
    });
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    let mut origin = local_repo.remote("origin", remote_repo.clone_url.as_str()).unwrap();
    origin.push(&[branch_ref_name], Some(&mut push_options)).unwrap();

    Ok(())
}
