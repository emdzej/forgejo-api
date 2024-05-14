use forgejo_api::Forgejo;

pub fn login() -> Forgejo {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let token = std::env::var("FORGEJO_API_CI_TOKEN").unwrap();
    Forgejo::new(forgejo_api::Auth::Token(&token), url).unwrap()
}

pub fn login_pass(username: &str, password: &str) -> Forgejo {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let auth = forgejo_api::Auth::Password {
        username,
        password,
        mfa: None,
    };
    Forgejo::new(auth, url).unwrap()
}
