#[test]
fn ci() {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let token = std::env::var("FORGEJO_API_CI_TOKEN").unwrap();
    let api = forgejo_api::Forgejo::new(&token, url).unwrap();
}
