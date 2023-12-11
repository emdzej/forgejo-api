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
            if let Some(err) = err.downcast_ref::<forgejo_api::ForgejoError>() {
                if let forgejo_api::ForgejoError::BadStructure(_, body) = err {
                    println!("BODY: {body}");
                }
            }
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
    let git = || {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir("/test_repo");
        cmd
    };
    let _ = git().args(["config", "--global", "init.defaultBranch", "main"]).status()?;
    let _ = git().args(["init"]).status()?;
    let _ = git().args(["config", "user.name", "TestingAdmin"]).status()?;
    let _ = git().args(["config", "user.email", "admin@noreply.example.org"]).status()?;
    tokio::fs::write("/test_repo/README.md", "# Test\nThis is a test repo").await?;
    let _ = git().args(["add", "."]).status()?;
    let _ = git().args(["commit", "-m", "initial commit"]).status()?;


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
    ensure!(remote_repo.has_pull_requests, "repo does not accept pull requests");
    ensure!(remote_repo.owner.login == "TestingAdmin", "repo owner is not \"TestingAdmin\"");
    ensure!(remote_repo.name == "test", "repo owner is not \"test\"");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let mut remote_url = remote_repo.clone_url.clone();
    remote_url.set_username("TestingAdmin").unwrap();
    remote_url.set_password(Some("password")).unwrap();
    let _ = git().args(["remote", "add", "origin", remote_url.as_str()]).status()?;
    let _ = git().args(["push", "-u", "origin", "main"]).status()?;

    let _ = git().args(["switch", "-c", "test"]).status()?;
    tokio::fs::write("/test_repo/example.rs", "fn add_one(x: u32) -> u32 { x + 1 }").await?;
    let _ = git().args(["add", "."]).status()?;
    let _ = git().args(["commit", "-m", "egg"]).status()?;
    let _ = git().args(["push", "-u", "origin", "test"]).status()?;

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
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
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
