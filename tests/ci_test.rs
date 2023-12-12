use eyre::{ensure, eyre, WrapErr};
use forgejo_api::Forgejo;

#[tokio::test]
async fn ci() -> eyre::Result<()> {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL")?)?;
    let token = std::env::var("FORGEJO_API_CI_TOKEN")?;
    let api = Forgejo::new(&token, url)?;

    let mut results = Vec::new();

    results.push(user(&api).await.wrap_err("user error"));
    results.push(repo(&api).await.wrap_err("repo error"));
    results.push(admin(&api).await.wrap_err("admin error"));

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
    ensure!(
        myself.login == "TestingAdmin",
        "user should be named \"TestingAdmin\""
    );

    let myself_indirect = api
        .get_user("TestingAdmin")
        .await?
        .ok_or_else(|| eyre!("\"TestingAdmin\" not found, but should have been."))?;
    ensure!(
        myself == myself_indirect,
        "result of `myself` does not match result of `get_user`"
    );

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
    let _ = git()
        .args(["config", "--global", "init.defaultBranch", "main"])
        .status()?;
    let _ = git().args(["init"]).status()?;
    let _ = git()
        .args(["config", "user.name", "TestingAdmin"])
        .status()?;
    let _ = git()
        .args(["config", "user.email", "admin@noreply.example.org"])
        .status()?;
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
    ensure!(
        remote_repo.has_pull_requests,
        "repo does not accept pull requests"
    );
    ensure!(
        remote_repo.owner.login == "TestingAdmin",
        "repo owner is not \"TestingAdmin\""
    );
    ensure!(remote_repo.name == "test", "repo owner is not \"test\"");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let mut remote_url = remote_repo.clone_url.clone();
    remote_url.set_username("TestingAdmin").unwrap();
    remote_url.set_password(Some("password")).unwrap();
    let _ = git()
        .args(["remote", "add", "origin", remote_url.as_str()])
        .status()?;
    let _ = git().args(["push", "-u", "origin", "main"]).status()?;

    let _ = git().args(["switch", "-c", "test"]).status()?;
    tokio::fs::write(
        "/test_repo/example.rs",
        "fn add_one(x: u32) -> u32 { x + 1 }",
    )
    .await?;
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
    let pr = api
        .create_pr("TestingAdmin", "test", pr_opt)
        .await
        .wrap_err("couldn't create pr")?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let is_merged = api
        .is_merged("TestingAdmin", "test", pr.number)
        .await
        .wrap_err_with(|| eyre!("couldn't find unmerged pr {}", pr.number))?;
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
    api.merge_pr("TestingAdmin", "test", pr.number, merge_opt)
        .await
        .wrap_err_with(|| eyre!("couldn't merge pr {}", pr.number))?;
    let is_merged = api
        .is_merged("TestingAdmin", "test", pr.number)
        .await
        .wrap_err_with(|| eyre!("couldn't find merged pr {}", pr.number))?;
    ensure!(is_merged, "pr should be merged");
    let _ = git().args(["fetch"]).status()?;
    let _ = git().args(["pull"]).status()?;

    ensure!(
        api.get_releases("TestingAdmin", "test", forgejo_api::ReleaseQuery::default())
            .await
            .wrap_err("releases list not found")?
            .is_empty(),
        "there should be no releases yet"
    );

    let tag_opt = forgejo_api::CreateTagOption {
        message: Some("This is a tag!".into()),
        tag_name: "v1.0".into(),
        target: None,
    };
    api.create_tag("TestingAdmin", "test", tag_opt)
        .await
        .wrap_err("failed to create tag")?;

    let release_opt = forgejo_api::CreateReleaseOption {
        body: "This is a release!".into(),
        draft: true,
        name: "v1.0".into(),
        prerelease: false,
        tag_name: "v1.0".into(),
        target_commitish: None,
    };
    let release = api
        .create_release("TestingAdmin", "test", release_opt)
        .await
        .wrap_err("failed to create release")?;
    let edit_release = forgejo_api::EditReleaseOption {
        draft: Some(false),
        ..Default::default()
    };
    api.edit_release("TestingAdmin", "test", release.id, edit_release)
        .await
        .wrap_err("failed to edit release")?;

    let release_by_tag = api
        .get_release_by_tag("TestingAdmin", "test", "v1.0")
        .await
        .wrap_err("failed to find release")?;
    let release_latest = api
        .latest_release("TestingAdmin", "test")
        .await
        .wrap_err("failed to find latest release")?;
    ensure!(release_by_tag == release_latest, "releases not equal");

    let attachment = api
        .create_release_attachment(
            "TestingAdmin",
            "test",
            release.id,
            "test.txt",
            b"This is a file!".to_vec(),
        )
        .await
        .wrap_err("failed to create release attachment")?;
    api.delete_release_attachment("TestingAdmin", "test", release.id, attachment.id)
        .await
        .wrap_err("failed to deleted attachment")?;

    api.delete_release("TestingAdmin", "test", release.id)
        .await
        .wrap_err("failed to delete release")?;

    api.delete_tag("TestingAdmin", "test", "v1.0")
        .await
        .wrap_err("failed to delete release")?;

    Ok(())
}

async fn admin(api: &forgejo_api::Forgejo) -> eyre::Result<()> {
    let user_opt = forgejo_api::CreateUserOption {
        created_at: None,
        email: "user@noreply.example.org".into(),
        full_name: None,
        login_name: None,
        must_change_password: false,
        password: "userpass".into(),
        restricted: false,
        send_notify: true,
        source_id: None,
        username: "Pipis".into(),
        visibility: "public".into(),
    };
    let _ = api
        .admin_create_user(user_opt)
        .await
        .wrap_err("failed to create user")?;

    let users = api
        .admin_users(forgejo_api::AdminUserQuery::default())
        .await
        .wrap_err("failed to search users")?;
    ensure!(
        users.iter().find(|u| u.login == "Pipis").is_some(),
        "could not find new user"
    );
    let users = api
        .admin_get_emails(forgejo_api::EmailListQuery::default())
        .await
        .wrap_err("failed to search emails")?;
    ensure!(
        users
            .iter()
            .find(|u| u.email == "user@noreply.example.org")
            .is_some(),
        "could not find new user"
    );

    let org_opt = forgejo_api::CreateOrgOption {
        description: None,
        full_name: None,
        location: None,
        repo_admin_change_team_access: None,
        username: "test-org".into(),
        visibility: forgejo_api::OrgVisibility::Public,
        website: None,
    };
    let _ = api
        .admin_create_org("Pipis", org_opt)
        .await
        .wrap_err("failed to create org")?;
    ensure!(
        !api.admin_get_orgs(forgejo_api::AdminOrganizationQuery::default())
            .await?
            .is_empty(),
        "org list empty"
    );

    let key_opt = forgejo_api::CreateKeyOption {
        key: "ssh-ed25519 00000000000000000000000000000000000000000000000000000000000000000000 user@noreply.example.org".into(),
        read_only: None,
        title: "Example Key".into(),
    };
    let key = api.admin_add_key("Pipis", key_opt).await.wrap_err("failed to create key")?;
    api.admin_delete_key("Pipis", key.id).await.wrap_err("failed to delete key")?;

    let rename_opt = forgejo_api::RenameUserOption {
        new_username: "Bepis".into(),
    };
    api.admin_rename_user("Pipis", rename_opt)
        .await
        .wrap_err("failed to rename user")?;
    api.admin_delete_user("Bepis", true)
        .await
        .wrap_err("failed to delete user")?;
    ensure!(
        api.admin_delete_user("Ghost", true).await.is_err(),
        "deleting fake user should fail"
    );

    let crons = api
        .admin_get_crons(forgejo_api::CronQuery::default())
        .await
        .wrap_err("failed to get crons list")?;
    api.admin_run_cron(&crons.get(0).ok_or_else(|| eyre!("no crons"))?.name)
        .await
        .wrap_err("failed to run cron")?;

    let hook_opt = forgejo_api::CreateHookOption {
        active: None,
        authorization_header: None,
        branch_filter: None,
        config: forgejo_api::CreateHookOptionConfig {
            content_type: "text/plain".into(),
            url: url::Url::parse("http://test.local/").unwrap(),
            other: Default::default(),
        },
        events: Vec::new(),
        _type: forgejo_api::HookType::Forgejo,
    };
    // yarr har har me matey this is me hook
    let hook = api
        .admin_create_hook(hook_opt)
        .await
        .wrap_err("failed to create hook")?;
    let edit_hook = forgejo_api::EditHookOption {
        active: Some(true),
        ..Default::default()
    };
    api.admin_edit_hook(hook.id, edit_hook)
        .await
        .wrap_err("failed to edit hook")?;
    api.admin_delete_hook(hook.id)
        .await
        .wrap_err("failed to delete hook")?;

    Ok(())
}
