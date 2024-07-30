use forgejo_api::structs::*;

mod common;

#[tokio::test]
async fn myself() {
    let api = common::login();

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
}

#[tokio::test]
async fn follow() {
    let api = common::login();

    let query = UserListFollowingQuery::default();
    let following = api
        .user_list_following("TestingAdmin", query)
        .await
        .unwrap();
    assert!(following.is_empty(), "following list not empty");

    let query = UserListFollowersQuery::default();
    let followers = api
        .user_list_followers("TestingAdmin", query)
        .await
        .unwrap();
    assert!(followers.is_empty(), "follower list not empty");

    let option = CreateUserOption {
        created_at: None,
        email: "follower@no-reply.example.org".into(),
        full_name: None,
        login_name: None,
        must_change_password: Some(false),
        password: Some("password".into()),
        restricted: None,
        send_notify: None,
        source_id: None,
        username: "Follower".into(),
        visibility: None,
    };
    let _ = api.admin_create_user(option).await.unwrap();
    let new_user = common::login_pass("Follower", "password");

    new_user
        .user_current_put_follow("TestingAdmin")
        .await
        .unwrap();
    api.user_current_put_follow("Follower").await.unwrap();

    let query = UserListFollowingQuery::default();
    let following = api
        .user_list_following("TestingAdmin", query)
        .await
        .unwrap();
    assert!(!following.is_empty(), "following list empty");

    let query = UserListFollowersQuery::default();
    let followers = api
        .user_list_followers("TestingAdmin", query)
        .await
        .unwrap();
    assert!(!followers.is_empty(), "follower list empty");
}

#[tokio::test]
async fn password_login() {
    let api = common::login();
    let password_api = common::login_pass("TestingAdmin", "password");

    assert!(
        api.user_get_current().await.unwrap() == password_api.user_get_current().await.unwrap(),
        "users not equal comparing token-auth and pass-auth"
    );
}

#[tokio::test]
async fn oauth2_login() {
    let api = common::login();
    let opt = forgejo_api::structs::CreateOAuth2ApplicationOptions {
        confidential_client: Some(true),
        name: Some("Test Application".into()),
        redirect_uris: Some(vec!["http://127.0.0.1:48879/".into()]),
    };
    let app = api.user_create_oauth2_application(opt).await.unwrap();
    let client_id = app.client_id.unwrap();
    let client_secret = app.client_secret.unwrap();

    let base_url = &std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap();

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Log in via the web interface
    let _ = client
        .post(&format!("{base_url}user/login"))
        .form(&[("user_name", "TestingAdmin"), ("password", "password")])
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Load the authorization page
    let response = client
        .get(&format!(
            "{base_url}login/oauth/authorize\
             ?client_id={client_id}\
             &redirect_uri=http%3A%2F%2F127.0.0.1%3A48879%2F\
             &response_type=code\
             &state=theyve"
        ))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    let csrf = response.cookies().find(|x| x.name() == "_csrf").unwrap();

    // Authorize the new application via the web interface
    let response = client
        .post(&format!("{base_url}login/oauth/grant"))
        .form(&[
            ("_csrf", csrf.value()),
            ("client_id", &client_id),
            ("state", "theyve"),
            ("scope", ""),
            ("nonce", ""),
            ("redirect_uri", "http://127.0.0.1:48879/"),
        ])
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Extract the code from the redirect url
    let location = response.headers().get(reqwest::header::LOCATION).unwrap();
    let location = url::Url::parse(dbg!(location.to_str().unwrap())).unwrap();
    let mut code = None;
    for (key, value) in location.query_pairs() {
        if key == "code" {
            code = Some(value.into_owned());
        } else if key == "error_description" {
            panic!("{value}");
        }
    }
    let code = code.unwrap();

    // Redeem the code and check it works
    let url = url::Url::parse(&base_url).unwrap();
    let api = forgejo_api::Forgejo::new(forgejo_api::Auth::None, url.clone()).unwrap();

    let request = forgejo_api::structs::OAuthTokenRequest::Confidential {
        client_id: &client_id,
        client_secret: &client_secret,
        code: &code,
        redirect_uri: url::Url::parse("http://127.0.0.1:48879/").unwrap(),
    };
    let token = api.oauth_get_access_token(request).await.unwrap();
    let token_api =
        forgejo_api::Forgejo::new(forgejo_api::Auth::OAuth2(&token.access_token), url.clone())
            .unwrap();
    let myself = token_api.user_get_current().await.unwrap();
    assert_eq!(myself.login.as_deref(), Some("TestingAdmin"));

    let request = forgejo_api::structs::OAuthTokenRequest::Refresh {
        refresh_token: &token.refresh_token,
        client_id: &client_id,
        client_secret: &client_secret,
    };
    let token = token_api.oauth_get_access_token(request).await.unwrap();
    let token_api =
        forgejo_api::Forgejo::new(forgejo_api::Auth::OAuth2(&token.access_token), url).unwrap();
    let myself = token_api.user_get_current().await.unwrap();
    assert_eq!(myself.login.as_deref(), Some("TestingAdmin"));
}

#[tokio::test]
async fn user_vars() {
    let api = common::login();

    let query = GetUserVariablesListQuery::default();
    let var_list = api
        .get_user_variables_list(query)
        .await
        .expect("failed to list user vars");
    assert!(var_list.is_empty());

    let opt = CreateVariableOption {
        value: "false".into(),
    };
    api.create_user_variable("likes_dogs", opt)
        .await
        .expect("failed to create user var");

    let new_var = api
        .get_user_variable("likes_dogs")
        .await
        .expect("failed to get user var");
    assert_eq!(new_var.data.as_deref(), Some("false"));

    // what??? totally wrong. I love dogs!
    let opt = UpdateVariableOption {
        name: Some("loves_dogs".into()),
        value: "true".into(),
    };
    api.update_user_variable("likes_dogs", opt)
        .await
        .expect("failed to update user variable");

    let new_var = api
        .get_user_variable("loves_dogs")
        .await
        .expect("failed to get user var");
    assert_eq!(new_var.data.as_deref(), Some("true"));

    api.delete_user_variable("loves_dogs")
        .await
        .expect("failed to delete user var");
}
