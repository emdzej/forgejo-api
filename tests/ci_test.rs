use forgejo_api::{structs::*, Forgejo};

fn get_api() -> Forgejo {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let token = std::env::var("FORGEJO_API_CI_TOKEN").unwrap();
    Forgejo::new(forgejo_api::Auth::Token(&token), url).unwrap()
}

#[tokio::test]
async fn user() {
    let api = get_api();

    let myself = api.user_get_current().await.unwrap();
    assert!(myself.is_admin.unwrap(), "user should be admin");
    assert_eq!(
        myself.login.as_ref().unwrap(),
        "TestingAdmin",
        "user should be named \"TestingAdmin\""
    );

    let myself_indirect = api.user_get("TestingAdmin").await.unwrap();
    assert_eq!(
        myself, myself_indirect,
        "result of `myself` does not match result of `get_user`"
    );

    let query = UserListFollowingQuery {
        page: None,
        limit: None,
    };
    let following = api
        .user_list_following("TestingAdmin", query)
        .await
        .unwrap();
    assert_eq!(following, Vec::new(), "following list not empty");

    let query = UserListFollowersQuery {
        page: None,
        limit: None,
    };
    let followers = api
        .user_list_followers("TestingAdmin", query)
        .await
        .unwrap();
    assert_eq!(followers, Vec::new(), "follower list not empty");

    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let password_api = Forgejo::new(
        forgejo_api::Auth::Password {
            username: "TestingAdmin",
            password: "password",
            mfa: None,
        },
        url,
    )
    .expect("failed to log in using username and password");

    assert!(
        api.user_get_current().await.unwrap() == password_api.user_get_current().await.unwrap(),
        "users not equal comparing token-auth and pass-auth"
    );
}

#[tokio::test]
async fn repo() {
    let api = get_api();

    tokio::fs::create_dir("/test_repo").await.unwrap();
    let git = || {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir("/test_repo");
        cmd
    };
    let _ = git()
        .args(["config", "--global", "init.defaultBranch", "main"])
        .status()
        .unwrap();
    let _ = git().args(["init"]).status().unwrap();
    let _ = git()
        .args(["config", "user.name", "TestingAdmin"])
        .status()
        .unwrap();
    let _ = git()
        .args(["config", "user.email", "admin@noreply.example.org"])
        .status()
        .unwrap();
    tokio::fs::write("/test_repo/README.md", "# Test\nThis is a test repo")
        .await
        .unwrap();
    let _ = git().args(["add", "."]).status().unwrap();
    let _ = git()
        .args(["commit", "-m", "initial commit"])
        .status()
        .unwrap();

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
    let remote_repo = api.create_current_user_repo(repo_opt).await.unwrap();
    assert!(
        remote_repo.has_pull_requests.unwrap(),
        "repo does not accept pull requests"
    );
    assert!(
        remote_repo.owner.as_ref().unwrap().login.as_ref().unwrap() == "TestingAdmin",
        "repo owner is not \"TestingAdmin\""
    );
    assert!(
        remote_repo.name.as_ref().unwrap() == "test",
        "repo owner is not \"test\""
    );
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let mut remote_url = remote_repo.clone_url.clone().unwrap();
    remote_url.set_username("TestingAdmin").unwrap();
    remote_url.set_password(Some("password")).unwrap();
    let _ = git()
        .args(["remote", "add", "origin", remote_url.as_str()])
        .status()
        .unwrap();
    let _ = git()
        .args(["push", "-u", "origin", "main"])
        .status()
        .unwrap();

    let _ = git().args(["switch", "-c", "test"]).status().unwrap();
    tokio::fs::write(
        "/test_repo/example.rs",
        "fn add_one(x: u32) -> u32 { x + 1 }",
    )
    .await
    .unwrap();
    let _ = git().args(["add", "."]).status().unwrap();
    let _ = git().args(["commit", "-m", "egg"]).status().unwrap();
    let _ = git()
        .args(["push", "-u", "origin", "test"])
        .status()
        .unwrap();

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
        .expect("couldn't create pr");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "test", pr.number.unwrap())
        .await
        .is_ok();
    assert!(!is_merged, "pr should not yet be merged");
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
        .expect("couldn't merge pr");
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "test", pr.number.unwrap())
        .await
        .is_ok();
    assert!(is_merged, "pr should be merged");
    let _ = git().args(["fetch"]).status().unwrap();
    let _ = git().args(["pull"]).status().unwrap();

    let query = RepoListReleasesQuery {
        draft: None,
        pre_release: None,
        per_page: None,
        page: None,
        limit: None,
    };
    assert!(
        api.repo_list_releases("TestingAdmin", "test", query)
            .await
            .unwrap()
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
        .expect("failed to create tag");

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
        .expect("failed to create release");
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
        .expect("failed to edit release");

    let release_by_tag = api
        .repo_get_release_by_tag("TestingAdmin", "test", "v1.0")
        .await
        .expect("failed to find release");
    let release_latest = api
        .repo_get_latest_release("TestingAdmin", "test")
        .await
        .expect("failed to find latest release");
    assert!(release_by_tag == release_latest, "releases not equal");

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
        .expect("failed to create release attachment");
    assert!(
        &*api
            .download_release_attachment(
                "TestingAdmin",
                "test",
                release.id.unwrap(),
                attachment.id.unwrap()
            )
            .await
            .unwrap()
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
    .expect("failed to deleted attachment");

    api.repo_delete_release("TestingAdmin", "test", release.id.unwrap())
        .await
        .expect("failed to delete release");

    api.repo_delete_tag("TestingAdmin", "test", "v1.0")
        .await
        .expect("failed to delete release");
}

#[tokio::test]
async fn admin() {
    let api = get_api();

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
        .expect("failed to create user");

    let query = AdminSearchUsersQuery {
        source_id: None,
        login_name: None,
        page: None,
        limit: None,
    };
    let users = api
        .admin_search_users(query)
        .await
        .expect("failed to search users");
    assert!(
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
        .expect("failed to search emails");
    assert!(
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
        .expect("failed to create org");
    let query = AdminGetAllOrgsQuery {
        page: None,
        limit: None,
    };
    assert!(
        !api.admin_get_all_orgs(query).await.unwrap().is_empty(),
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
        .expect("failed to create key");
    api.admin_delete_user_public_key("Pipis", key.id.unwrap())
        .await
        .expect("failed to delete key");

    let rename_opt = RenameUserOption {
        new_username: "Bepis".into(),
    };
    api.admin_rename_user("Pipis", rename_opt)
        .await
        .expect("failed to rename user");
    let query = AdminDeleteUserQuery { purge: Some(true) };
    api.admin_delete_user("Bepis", query)
        .await
        .expect("failed to delete user");
    let query = AdminDeleteUserQuery { purge: Some(true) };
    assert!(
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
        .expect("failed to get crons list");
    api.admin_cron_run(&crons.get(0).expect("no crons").name.as_ref().unwrap())
        .await
        .expect("failed to run cron");

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
        .expect("failed to create hook");
    let edit_hook = EditHookOption {
        active: Some(true),
        authorization_header: None,
        branch_filter: None,
        config: None,
        events: None,
    };
    api.admin_edit_hook(hook.id.unwrap(), edit_hook)
        .await
        .expect("failed to edit hook");
    api.admin_delete_hook(hook.id.unwrap())
        .await
        .expect("failed to delete hook");
}
