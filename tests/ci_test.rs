use eyre::{ensure, eyre, WrapErr};
use forgejo_api::{structs::*, Forgejo};

#[tokio::test]
async fn ci() -> eyre::Result<()> {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL")?)?;
    let token = std::env::var("FORGEJO_API_CI_TOKEN")?;
    let api = Forgejo::new(forgejo_api::Auth::Token(&token), url)?;

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

async fn user(api: &Forgejo) -> eyre::Result<()> {
    let myself = api.user_get_current().await?;
    ensure!(myself.is_admin.unwrap(), "user should be admin");
    ensure!(
        myself.login.as_ref().unwrap() == "TestingAdmin",
        "user should be named \"TestingAdmin\""
    );

    let myself_indirect = api.user_get("TestingAdmin").await?;
    ensure!(
        myself == myself_indirect,
        "result of `myself` does not match result of `get_user`"
    );

    let query = UserListFollowingQuery {
        page: None,
        limit: None,
    };
    let following = api.user_list_following("TestingAdmin", query).await?;
    ensure!(following == Vec::new(), "following list not empty");

    let query = UserListFollowersQuery {
        page: None,
        limit: None,
    };
    let followers = api.user_list_followers("TestingAdmin", query).await?;
    ensure!(followers == Vec::new(), "follower list not empty");

    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL")?)?;
    let password_api = Forgejo::new(
        forgejo_api::Auth::Password {
            username: "TestingAdmin",
            password: "password",
            mfa: None,
        },
        url,
    )
    .wrap_err("failed to log in using username and password")?;

    ensure!(
        api.user_get_current().await? == password_api.user_get_current().await?,
        "users not equal comparing token-auth and pass-auth"
    );

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

    let repo_opt = CreateRepoOption {
        auto_init: Some(false),
        default_branch: Some("main".into()),
        description: Some("Test Repo".into()),
        gitignores: Some("".into()),
        issue_labels: Some("".into()),
        license: Some("".into()),
        name: "test".into(),
        private: Some(false),
        readme: None,
        template: Some(false),
        trust_model: Some("default".into()),
    };
    let remote_repo = api.create_current_user_repo(repo_opt).await?;
    ensure!(
        remote_repo.has_pull_requests.unwrap(),
        "repo does not accept pull requests"
    );
    ensure!(
        remote_repo.owner.as_ref().unwrap().login.as_ref().unwrap() == "TestingAdmin",
        "repo owner is not \"TestingAdmin\""
    );
    ensure!(
        remote_repo.name.as_ref().unwrap() == "test",
        "repo owner is not \"test\""
    );
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let mut remote_url = remote_repo.clone_url.clone().unwrap();
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

    let pr_opt = CreatePullRequestOption {
        assignee: None,
        assignees: Some(vec!["TestingAdmin".into()]),
        base: Some("main".into()),
        body: Some("This is a test PR".into()),
        due_date: None,
        head: Some("test".into()),
        labels: None,
        milestone: None,
        title: Some("test pr".into()),
    };
    let pr = api
        .repo_create_pull_request("TestingAdmin", "test", pr_opt)
        .await
        .wrap_err("couldn't create pr")?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "test", pr.number.unwrap())
        .await
        .is_ok();
    ensure!(!is_merged, "pr should not yet be merged");
    let merge_opt = MergePullRequestOption {
        r#do: "merge".into(),
        merge_commit_id: None,
        merge_message_field: None,
        merge_title_field: None,
        delete_branch_after_merge: Some(true),
        force_merge: None,
        head_commit_id: None,
        merge_when_checks_succeed: None,
    };
    api.repo_merge_pull_request("TestingAdmin", "test", pr.number.unwrap(), merge_opt)
        .await
        .wrap_err_with(|| eyre!("couldn't merge pr {}", pr.number.unwrap()))?;
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "test", pr.number.unwrap())
        .await
        .is_ok();
    ensure!(is_merged, "pr should be merged");
    let _ = git().args(["fetch"]).status()?;
    let _ = git().args(["pull"]).status()?;

    let query = RepoListReleasesQuery {
        draft: None,
        pre_release: None,
        per_page: None,
        page: None,
        limit: None,
    };
    ensure!(
        api.repo_list_releases("TestingAdmin", "test", query)
            .await
            .wrap_err("releases list not found")?
            .is_empty(),
        "there should be no releases yet"
    );

    let tag_opt = CreateTagOption {
        message: Some("This is a tag!".into()),
        tag_name: "v1.0".into(),
        target: None,
    };
    api.repo_create_tag("TestingAdmin", "test", tag_opt)
        .await
        .wrap_err("failed to create tag")?;

    let release_opt = CreateReleaseOption {
        body: Some("This is a release!".into()),
        draft: Some(true),
        name: Some("v1.0".into()),
        prerelease: Some(false),
        tag_name: "v1.0".into(),
        target_commitish: None,
    };
    let release = api
        .repo_create_release("TestingAdmin", "test", release_opt)
        .await
        .wrap_err("failed to create release")?;
    let edit_release = EditReleaseOption {
        body: None,
        draft: Some(false),
        name: None,
        prerelease: None,
        tag_name: None,
        target_commitish: None,
    };
    api.repo_edit_release("TestingAdmin", "test", release.id.unwrap(), edit_release)
        .await
        .wrap_err("failed to edit release")?;

    let release_by_tag = api
        .repo_get_release_by_tag("TestingAdmin", "test", "v1.0")
        .await
        .wrap_err("failed to find release")?;
    let release_latest = api
        .repo_get_latest_release("TestingAdmin", "test")
        .await
        .wrap_err("failed to find latest release")?;
    ensure!(release_by_tag == release_latest, "releases not equal");

    let attachment = api
        .repo_create_release_attachment(
            "TestingAdmin",
            "test",
            release.id.unwrap(),
            b"This is a file!".to_vec(),
            RepoCreateReleaseAttachmentQuery {
                name: Some("test.txt".into()),
            },
        )
        .await
        .wrap_err("failed to create release attachment")?;
    ensure!(
        &*api
            .download_release_attachment(
                "TestingAdmin",
                "test",
                release.id.unwrap(),
                attachment.id.unwrap()
            )
            .await?
            == b"This is a file!",
        "couldn't download attachment"
    );
    let _zip_archive = api
        .repo_get_archive("TestingAdmin", "test", "v1.0.zip")
        .await
        .unwrap();
    let _tar_archive = api
        .repo_get_archive("TestingAdmin", "test", "v1.0.tar.gz")
        .await
        .unwrap();
    // check these contents when their return value is fixed

    api.repo_delete_release_attachment(
        "TestingAdmin",
        "test",
        release.id.unwrap(),
        attachment.id.unwrap(),
    )
    .await
    .wrap_err("failed to deleted attachment")?;

    api.repo_delete_release("TestingAdmin", "test", release.id.unwrap())
        .await
        .wrap_err("failed to delete release")?;

    api.repo_delete_tag("TestingAdmin", "test", "v1.0")
        .await
        .wrap_err("failed to delete release")?;

    Ok(())
}

async fn admin(api: &forgejo_api::Forgejo) -> eyre::Result<()> {
    let user_opt = CreateUserOption {
        created_at: None,
        email: "user@noreply.example.org".into(),
        full_name: None,
        login_name: None,
        must_change_password: None,
        password: Some("userpass".into()),
        restricted: Some(false),
        send_notify: Some(true),
        source_id: None,
        username: "Pipis".into(),
        visibility: Some("public".into()),
    };
    let _ = api
        .admin_create_user(user_opt)
        .await
        .wrap_err("failed to create user")?;

    let query = AdminSearchUsersQuery {
        source_id: None,
        login_name: None,
        page: None,
        limit: None,
    };
    let users = api
        .admin_search_users(query)
        .await
        .wrap_err("failed to search users")?;
    ensure!(
        users
            .iter()
            .find(|u| u.login.as_ref().unwrap() == "Pipis")
            .is_some(),
        "could not find new user"
    );
    let query = AdminGetAllEmailsQuery {
        page: None,
        limit: None,
    };
    let users = api
        .admin_get_all_emails(query)
        .await
        .wrap_err("failed to search emails")?;
    ensure!(
        users
            .iter()
            .find(|u| u.email.as_ref().unwrap() == "user@noreply.example.org")
            .is_some(),
        "could not find new user"
    );

    let org_opt = CreateOrgOption {
        description: None,
        email: None,
        full_name: None,
        location: None,
        repo_admin_change_team_access: None,
        username: "test-org".into(),
        visibility: Some("public".into()),
        website: None,
    };
    let _ = api
        .admin_create_org("Pipis", org_opt)
        .await
        .wrap_err("failed to create org")?;
    let query = AdminGetAllOrgsQuery {
        page: None,
        limit: None,
    };
    ensure!(
        !api.admin_get_all_orgs(query).await?.is_empty(),
        "org list empty"
    );

    let key_opt = CreateKeyOption {
        key: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIN68ehQAsbGEwlXPa2AxbAh1QxFQrtRel2jeC0hRlPc1 user@noreply.example.org".into(),
        read_only: None,
        title: "Example Key".into(),
    };
    let key = api
        .admin_create_public_key("Pipis", key_opt)
        .await
        .wrap_err("failed to create key")?;
    api.admin_delete_user_public_key("Pipis", key.id.unwrap())
        .await
        .wrap_err("failed to delete key")?;

    let rename_opt = RenameUserOption {
        new_username: "Bepis".into(),
    };
    api.admin_rename_user("Pipis", rename_opt)
        .await
        .wrap_err("failed to rename user")?;
    let query = AdminDeleteUserQuery { purge: Some(true) };
    api.admin_delete_user("Bepis", query)
        .await
        .wrap_err("failed to delete user")?;
    let query = AdminDeleteUserQuery { purge: Some(true) };
    ensure!(
        api.admin_delete_user("Ghost", query).await.is_err(),
        "deleting fake user should fail"
    );

    let query = AdminCronListQuery {
        page: None,
        limit: None,
    };
    let crons = api
        .admin_cron_list(query)
        .await
        .wrap_err("failed to get crons list")?;
    api.admin_cron_run(
        &crons
            .get(0)
            .ok_or_else(|| eyre!("no crons"))?
            .name
            .as_ref()
            .unwrap(),
    )
    .await
    .wrap_err("failed to run cron")?;

    let hook_opt = CreateHookOption {
        active: None,
        authorization_header: None,
        branch_filter: None,
        config: CreateHookOptionConfig {
            // content_type: "json".into(),
            // url: url::Url::parse("http://test.local/").unwrap(),
            additional: [
                ("content_type".into(), "json".into()),
                ("url".into(), "http://test.local/".into()),
            ]
            .into(),
        },
        events: Some(Vec::new()),
        r#type: "forgejo".into(),
    };
    // yarr har har me matey this is me hook
    let hook = api
        .admin_create_hook(hook_opt)
        .await
        .wrap_err("failed to create hook")?;
    let edit_hook = EditHookOption {
        active: Some(true),
        authorization_header: None,
        branch_filter: None,
        config: None,
        events: None,
    };
    api.admin_edit_hook(hook.id.unwrap(), edit_hook)
        .await
        .wrap_err("failed to edit hook")?;
    api.admin_delete_hook(hook.id.unwrap())
        .await
        .wrap_err("failed to delete hook")?;

    Ok(())
}
