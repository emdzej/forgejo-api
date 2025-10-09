use forgejo_api::structs::*;

mod common;

#[tokio::test]
async fn org_vars() {
    let api = common::login();
    let org_opt = CreateOrgOption {
        description: Some("Testing organization variables".into()),
        email: None,
        full_name: Some("Org Variables".into()),
        location: Some("The Lab".into()),
        repo_admin_change_team_access: None,
        username: "org-vars".into(),
        visibility: None,
        website: None,
    };
    api.org_create(org_opt).await.expect("failed to create org");

    let (_, var_list) = api
        .get_org_variables_list("org-vars")
        .await
        .expect("failed to list org vars");
    assert!(var_list.is_empty());

    let opt = CreateVariableOption {
        value: "false".into(),
    };
    api.create_org_variable("org-vars", "want_pizza", opt)
        .await
        .expect("failed to create org var");

    let new_var = api
        .get_org_variable("org-vars", "want_pizza")
        .await
        .expect("failed to get org var");
    assert_eq!(new_var.data.as_deref(), Some("false"));

    // no no no we definitely do want pizza
    let opt = UpdateVariableOption {
        name: Some("really_want_pizza".into()),
        value: "true".into(),
    };
    api.update_org_variable("org-vars", "want_pizza", opt)
        .await
        .expect("failed to update org variable");

    let new_var = api
        .get_org_variable("org-vars", "really_want_pizza")
        .await
        .expect("failed to get org var");
    assert_eq!(new_var.data.as_deref(), Some("true"));

    api.delete_org_variable("org-vars", "really_want_pizza")
        .await
        .expect("failed to delete org var");
}
