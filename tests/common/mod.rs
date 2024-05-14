use forgejo_api::Forgejo;

pub fn get_api() -> Forgejo {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let token = std::env::var("FORGEJO_API_CI_TOKEN").unwrap();
    Forgejo::new(forgejo_api::Auth::Token(&token), url).unwrap()
}
