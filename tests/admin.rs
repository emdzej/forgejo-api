use forgejo_api::structs::*;

mod common;

#[tokio::test]
async fn user() {
    let api = common::login();

    let user_opt = CreateUserOption {
        created_at: None,
        email: "pipis@noreply.example.org".into(),
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

    let query = AdminSearchUsersQuery::default();
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
    let query = AdminGetAllEmailsQuery::default();
    let users = api
        .admin_get_all_emails(query)
        .await
        .expect("failed to search emails");
    assert!(
        users
            .iter()
            .find(|u| u.email.as_ref().unwrap() == "pipis@noreply.example.org")
            .is_some(),
        "could not find new user"
    );
}

#[tokio::test]
async fn org() {
    let api = common::login();

    let user_opt = CreateUserOption {
        created_at: None,
        email: "org-owner@noreply.example.org".into(),
        full_name: None,
        login_name: None,
        must_change_password: None,
        password: Some("userpass".into()),
        restricted: Some(false),
        send_notify: Some(true),
        source_id: None,
        username: "OrgOwner".into(),
        visibility: Some("public".into()),
    };
    let _ = api
        .admin_create_user(user_opt)
        .await
        .expect("failed to create user");

    let org_opt = CreateOrgOption {
        description: None,
        email: None,
        full_name: None,
        location: None,
        repo_admin_change_team_access: None,
        username: "test-org".into(),
        visibility: Some(CreateOrgOptionVisibility::Public),
        website: None,
    };
    let _ = api
        .admin_create_org("OrgOwner", org_opt)
        .await
        .expect("failed to create org");
    let query = AdminGetAllOrgsQuery::default();
    assert!(
        !api.admin_get_all_orgs(query).await.unwrap().is_empty(),
        "org list empty"
    );
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
}

#[tokio::test]
async fn key() {
    let api = common::login();

    let user_opt = CreateUserOption {
        created_at: None,
        email: "key-holder@noreply.example.org".into(),
        full_name: None,
        login_name: None,
        must_change_password: None,
        password: Some("userpass".into()),
        restricted: Some(false),
        send_notify: Some(true),
        source_id: None,
        username: "KeyHolder".into(),
        visibility: Some("public".into()),
    };
    let _ = api
        .admin_create_user(user_opt)
        .await
        .expect("failed to create user");

    let key_opt = CreateKeyOption {
        key: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIN68ehQAsbGEwlXPa2AxbAh1QxFQrtRel2jeC0hRlPc1 user@noreply.example.org".into(),
        read_only: None,
        title: "Example Key".into(),
    };
    let key = api
        .admin_create_public_key("KeyHolder", key_opt)
        .await
        .expect("failed to create key");
    api.admin_delete_user_public_key("KeyHolder", key.id.unwrap())
        .await
        .expect("failed to delete key");
}

#[tokio::test]
async fn cron() {
    let api = common::login();

    let query = AdminCronListQuery::default();
    let crons = api
        .admin_cron_list(query)
        .await
        .expect("failed to get crons list");
    api.admin_cron_run(&crons.get(0).expect("no crons").name.as_ref().unwrap())
        .await
        .expect("failed to run cron");
}

#[tokio::test]
async fn hook() {
    let api = common::login();
    let hook_opt = CreateHookOption {
        active: None,
        authorization_header: None,
        branch_filter: None,
        config: CreateHookOptionConfig {
            content_type: "json".into(),
            url: url::Url::parse("http://test.local/").unwrap(),
            additional: Default::default(),
        },
        events: Some(Vec::new()),
        r#type: CreateHookOptionType::Gitea,
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
