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
        users.iter().any(|u| u.login.as_ref().unwrap() == "Pipis"),
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
            .any(|u| u.email.as_ref().unwrap() == "pipis@noreply.example.org"),
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
    api.admin_delete_user_public_key("KeyHolder", key.id.unwrap() as u64)
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
    api.admin_cron_run(crons.first().expect("no crons").name.as_ref().unwrap())
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
    api.admin_edit_hook(hook.id.unwrap() as u64, edit_hook)
        .await
        .expect("failed to edit hook");
    api.admin_delete_hook(hook.id.unwrap() as u64)
        .await
        .expect("failed to delete hook");
}

#[tokio::test]
async fn quota_group() {
    let api = common::login();

    let user_opts = CreateUserOption {
        created_at: None,
        email: "1997@example.com".into(),
        full_name: None,
        login_name: None,
        must_change_password: None,
        password: Some("dialtone".into()),
        restricted: None,
        send_notify: None,
        source_id: None,
        username: "salesman".into(),
        visibility: None,
    };
    api.admin_create_user(user_opts)
        .await
        .expect("failed to create user");

    let group = CreateQuotaGroupOptions {
        name: Some("no doing anything".into()),
        rules: Some(vec![CreateQuotaRuleOptions {
            limit: Some(0),
            name: Some("blah".into()),
            subjects: Some(vec![CreateQuotaRuleOptionsSubjects::SizeAll]),
        }]),
    };
    let quota_group = api
        .admin_create_quota_group(group)
        .await
        .expect("failed to create quota group");

    api.admin_add_user_to_quota_group("no doing anything", "salesman")
        .await
        .expect("failed to add user to quota group");

    assert!(quota_group
        .name
        .as_ref()
        .is_some_and(|name| name == "no doing anything"));
    assert!(quota_group
        .rules
        .as_ref()
        .is_some_and(|rules| rules.len() == 1));

    let quota_groups = api
        .admin_list_quota_groups()
        .await
        .expect("failed to list quota groups");
    assert_eq!(quota_groups.len(), 1);
    assert_eq!(&quota_groups[0], &quota_group);

    let quota_info = api
        .admin_get_user_quota("salesman")
        .await
        .expect("failed to get user quota");
    let usage = quota_info
        .used
        .expect("quota info missing usage info")
        .size
        .expect("quota info missing size info");
    assert!(usage
        .git
        .is_some_and(|git| git.lfs.is_some_and(|lfs| lfs == 0)));
    assert!(usage
        .repos
        .as_ref()
        .is_some_and(|repos| repos.public.is_some_and(|lfs| lfs == 0)));
    assert!(usage
        .repos
        .is_some_and(|repos| repos.private.is_some_and(|lfs| lfs == 0)));
    assert!(usage
        .assets
        .is_some_and(|assets| assets.artifacts.is_some_and(|lfs| lfs == 0)));

    api.admin_remove_rule_from_quota_group("no doing anything", "blah")
        .await
        .expect("failed to delete rule from quota group");
    api.admin_remove_user_from_quota_group("no doing anything", "salesman")
        .await
        .expect("failed to remove user from quota group");
    api.admin_delete_quota_group("no doing anything")
        .await
        .expect("failed to delete quota group");
}
