use forgejo_api::Forgejo;
use eyre::{eyre, ensure, WrapErr};

#[tokio::test]
async fn ci() -> eyre::Result<()> {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL")?)?;
    let token = std::env::var("FORGEJO_API_CI_TOKEN")?;
    let api = Forgejo::new(&token, url)?;

    let mut results = Vec::new();

    results.push(user(&api).await.wrap_err("user error"));
    results.push(repo(&api).await.wrap_err("repo error"));

    let mut errors = 0;
    for report in results.into_iter().filter_map(Result::err) {
        errors += 1;
        for (i, err) in report.chain().enumerate() {
            println!("{i}. {err}");
        }
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
    let tree = local_repo.find_tree(index.write_tree().unwrap())?;
    let author = git2::Signature::now("TestingAdmin", "admin@noreply.example.org").unwrap();
    let commit_oid = local_repo.commit(None, &author, &author, "bibblybeebly", &tree, &[])?;
    let branch = local_repo.branch("main", &local_repo.find_commit(commit_oid).unwrap(), true)?;
    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().ok_or_else(|| eyre!("branch name not found"))?;
    local_repo.set_head(branch_ref_name)?;

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
    let mut origin = local_repo.remote("origin", remote_repo.clone_url.as_str())?;
    origin.push(&[dbg!(branch_ref_name)], Some(&mut push_options))?;

    tokio::fs::write("/test_repo/example.rs", "fn add_one(x: u32) -> u32 { x + 1 }").await?;
    let mut index = local_repo.index()?;
    index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree = local_repo.find_tree(index.write_tree().unwrap())?;
    let commit_oid = local_repo.commit(None, &author, &author, "egg", &tree, &[])?;
    let branch = local_repo.branch("test", &local_repo.find_commit(commit_oid).unwrap(), true)?;
    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().ok_or_else(|| eyre!("branch name not found"))?;
    local_repo.set_head(branch_ref_name)?;

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        git2::Cred::userpass_plaintext("TestingAdmin", "password")
    });
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    origin.push(&[dbg!(branch_ref_name)], Some(&mut push_options)).wrap_err("failed to push branch")?;

    let pr_opt = forgejo_api::CreatePullRequestOption {
        assignee: None,
        assignees: vec!["TestingAdmin".into()],
        base: "main".into(),
        body: "This is a test PR".into(),
        due_date: None,
        head: "test".into(),
        labels: Vec::new(),
        milestone: None,
        title: "test pr".into(),
    };
    let pr = api.create_pr("TestingAdmin", "test", pr_opt).await.wrap_err("couldn't create pr")?;
    let is_merged = api.is_merged("TestingAdmin", "test", pr.number).await.wrap_err_with(|| eyre!("couldn't find unmerged pr {}", pr.number))?;
    ensure!(!is_merged, "pr should not yet be merged");
    let merge_opt = forgejo_api::MergePullRequestOption {
        act: forgejo_api::MergePrAction::Merge,
        merge_commit_id: None,
        merge_message_field: None,
        merge_title_field: None,
        delete_branch_after_merge: Some(true),
        force_merge: None,
        head_commit_id: None,
        merge_when_checks_succeed: None,
    };
    api.merge_pr("TestingAdmin", "test", pr.number, merge_opt).await.wrap_err_with(|| eyre!("couldn't merge pr {}", pr.number))?;
    let is_merged = api.is_merged("TestingAdmin", "test", pr.number).await.wrap_err_with(|| eyre!("couldn't find merged pr {}", pr.number))?;
    ensure!(is_merged, "pr should be merged");

    Ok(())
}
