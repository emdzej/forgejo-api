use forgejo_api::structs::*;
use forgejo_api::Forgejo;

pub fn login() -> Forgejo {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let token = std::env::var("FORGEJO_API_CI_TOKEN").unwrap();
    Forgejo::new(forgejo_api::Auth::Token(&token), url).unwrap()
}

#[cfg(feature = "sync")]
pub fn sync_login() -> sync::Forgejo {
    let url = url::Url::parse(&std::env::var("FORGEJO_API_CI_INSTANCE_URL").unwrap()).unwrap();
    let token = std::env::var("FORGEJO_API_CI_TOKEN").unwrap();
    sync::Forgejo::new(forgejo_api::Auth::Token(&token), url).unwrap()
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

pub struct Git {
    dir: &'static std::path::Path,
}

impl Git {
    pub fn new<T: AsRef<std::path::Path> + ?Sized>(path: &'static T) -> Self {
        let dir = path.as_ref();
        std::fs::create_dir_all(dir).unwrap();
        Self { dir }
    }

    pub fn run(&self, args: &[impl AsRef<std::ffi::OsStr>]) {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(self.dir);
        let _ = cmd.args(args).status().unwrap();
    }
}

pub async fn setup_local_repo(git: &Git) {
    git.run(&["config", "--global", "init.defaultBranch", "main"]);
    git.run(&["init"]);
    git.run(&["config", "user.name", "TestingAdmin"]);
    git.run(&["config", "user.email", "admin@noreply.example.org"]);
    tokio::fs::write(&git.dir.join("README.md"), "# Test\nThis is a test repo")
        .await
        .unwrap();
    git.run(&["add", "."]);
    git.run(&["commit", "-m", "initial commit"]);
}

pub async fn basic_repo(api: &forgejo_api::Forgejo, git: &Git, name: &str) -> Repository {
    setup_local_repo(git).await;
    let repo_opt = CreateRepoOption {
        auto_init: Some(false),
        default_branch: Some("main".into()),
        description: Some("Test Repo".into()),
        gitignores: Some("".into()),
        issue_labels: Some("".into()),
        license: Some("".into()),
        name: name.into(),
        object_format_name: None,
        private: Some(false),
        readme: None,
        template: Some(false),
        trust_model: Some(CreateRepoOptionTrustModel::Default),
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
        remote_repo.name.as_ref().unwrap() == name,
        "repo name is not \"{name}\""
    );

    let mut remote_url = remote_repo.clone_url.clone().unwrap();
    remote_url.set_username("TestingAdmin").unwrap();
    remote_url.set_password(Some("password")).unwrap();
    git.run(&["remote", "add", "origin", remote_url.as_str()]);
    git.run(&["push", "-u", "origin", "main"]);

    remote_repo
}

pub async fn basic_org_repo(
    api: &forgejo_api::Forgejo,
    git: &Git,
    org: &str,
    name: &str,
) -> Repository {
    setup_local_repo(git).await;

    let repo_opt = CreateRepoOption {
        auto_init: Some(false),
        default_branch: Some("main".into()),
        description: Some("Test Repo".into()),
        gitignores: Some("".into()),
        issue_labels: Some("".into()),
        license: Some("".into()),
        name: name.into(),
        object_format_name: None,
        private: Some(false),
        readme: None,
        template: Some(false),
        trust_model: Some(CreateRepoOptionTrustModel::Default),
    };
    let remote_repo = api.create_org_repo(org, repo_opt).await.unwrap();
    assert!(
        remote_repo.has_pull_requests.unwrap(),
        "repo does not accept pull requests"
    );
    assert!(
        remote_repo.owner.as_ref().unwrap().login.as_ref().unwrap() == org,
        "repo owner is not \"TestingAdmin\""
    );
    assert!(
        remote_repo.name.as_ref().unwrap() == name,
        "repo name is not \"{name}\""
    );

    let mut remote_url = remote_repo.clone_url.clone().unwrap();
    remote_url.set_username("TestingAdmin").unwrap();
    remote_url.set_password(Some("password")).unwrap();
    git.run(&["remote", "add", "origin", remote_url.as_str()]);
    git.run(&["push", "-u", "origin", "main"]);

    remote_repo
}
