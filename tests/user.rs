use forgejo_api::{structs::*, Forgejo};

mod common;

#[tokio::test]
async fn user() {
    let api = common::get_api();

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

    let query = UserListFollowingQuery::default();
    let following = api
        .user_list_following("TestingAdmin", query)
        .await
        .unwrap();
    assert_eq!(following, Vec::new(), "following list not empty");

    let query = UserListFollowersQuery::default();
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
async fn oauth2_login() {
    let api = common::get_api();
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
    let api = Forgejo::new(forgejo_api::Auth::None, url.clone()).unwrap();

    let request = forgejo_api::structs::OAuthTokenRequest::Confidential {
        client_id: &client_id,
        client_secret: &client_secret,
        code: &code,
        redirect_uri: url::Url::parse("http://127.0.0.1:48879/").unwrap(),
    };
    let token = api.oauth_get_access_token(request).await.unwrap();
    let token_api =
        Forgejo::new(forgejo_api::Auth::OAuth2(&token.access_token), url.clone()).unwrap();
    let myself = token_api.user_get_current().await.unwrap();
    assert_eq!(myself.login.as_deref(), Some("TestingAdmin"));

    let request = forgejo_api::structs::OAuthTokenRequest::Refresh {
        refresh_token: &token.refresh_token,
        client_id: &client_id,
        client_secret: &client_secret,
    };
    let token = token_api.oauth_get_access_token(request).await.unwrap();
    let token_api = Forgejo::new(forgejo_api::Auth::OAuth2(&token.access_token), url).unwrap();
    let myself = token_api.user_get_current().await.unwrap();
    assert_eq!(myself.login.as_deref(), Some("TestingAdmin"));
}
