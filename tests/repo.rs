use forgejo_api::structs::*;

mod common;

struct Git {
    dir: &'static std::path::Path,
}

impl Git {
    fn new<T: AsRef<std::path::Path> + ?Sized>(path: &'static T) -> Self {
        let dir = path.as_ref();
        std::fs::create_dir_all(dir).unwrap();
        Self { dir }
    }

    fn run(&self, args: &[impl AsRef<std::ffi::OsStr>]) {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(self.dir);
        let _ = cmd.args(args).status().unwrap();
    }
}

async fn setup_local_repo(git: &Git) {
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

async fn basic_repo(api: &forgejo_api::Forgejo, git: &Git, name: &str) -> Repository {
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

async fn basic_org_repo(
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

#[tokio::test]
async fn pull_request() {
    let api = common::login();

    let git = Git::new("./test_repos/pr");
    let _ = basic_repo(&api, &git, "pr-test").await;
    git.run(&["switch", "-c", "test"]);

    tokio::fs::write(
        "./test_repos/pr/example.rs",
        "fn add_one(x: u32) -> u32 { x + 1 }",
    )
    .await
    .unwrap();
    git.run(&["add", "."]);
    git.run(&["commit", "-m", "egg"]);
    git.run(&["push", "-u", "origin", "test"]);

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

    // Wait for... something to happen, or else creating a PR will return 404
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let pr = api
        .repo_create_pull_request("TestingAdmin", "pr-test", pr_opt)
        .await
        .expect("couldn't create pr");

    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "pr-test", pr.number.unwrap() as u64)
        .await
        .is_ok();
    assert!(!is_merged, "pr should not yet be merged");

    let pr_files_query = RepoGetPullRequestFilesQuery::default();
    let (_, _) = api
        .repo_get_pull_request_files(
            "TestingAdmin",
            "pr-test",
            pr.number.unwrap() as u64,
            pr_files_query,
        )
        .await
        .unwrap();

    let merge_opt = MergePullRequestOption {
        r#do: MergePullRequestOptionDo::Merge,
        merge_commit_id: None,
        merge_message_field: None,
        merge_title_field: None,
        delete_branch_after_merge: Some(true),
        force_merge: None,
        head_commit_id: None,
        merge_when_checks_succeed: None,
    };

    api.repo_merge_pull_request(
        "TestingAdmin",
        "pr-test",
        pr.number.unwrap() as u64,
        merge_opt,
    )
    .await
    .expect("couldn't merge pr");
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "pr-test", pr.number.unwrap() as u64)
        .await
        .is_ok();
    assert!(is_merged, "pr should be merged");
}

#[tokio::test]
async fn release() {
    let api = common::login();

    let git = Git::new("./test_repos/release");
    let _ = basic_repo(&api, &git, "release-test").await;

    // Wait for the repo to be finished being populated, or else creating a
    // release will return "422 repo is empty"
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let query = RepoListReleasesQuery::default();
    assert!(
        api.repo_list_releases("TestingAdmin", "release-test", query)
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
    api.repo_create_tag("TestingAdmin", "release-test", tag_opt)
        .await
        .expect("failed to create tag");

    let release_opt = CreateReleaseOption {
        body: Some("This is a release!".into()),
        draft: Some(true),
        name: Some("v1.0".into()),
        prerelease: Some(false),
        tag_name: "v1.0".into(),
        target_commitish: None,
        hide_archive_links: None,
    };
    let release = api
        .repo_create_release("TestingAdmin", "release-test", release_opt)
        .await
        .expect("failed to create release");
    let edit_release = EditReleaseOption {
        body: None,
        draft: Some(false),
        name: None,
        prerelease: None,
        tag_name: None,
        target_commitish: None,
        hide_archive_links: None,
    };
    api.repo_edit_release(
        "TestingAdmin",
        "release-test",
        release.id.unwrap() as u64,
        edit_release,
    )
    .await
    .expect("failed to edit release");

    let release_by_tag = api
        .repo_get_release_by_tag("TestingAdmin", "release-test", "v1.0")
        .await
        .expect("failed to find release");
    let release_latest = api
        .repo_get_latest_release("TestingAdmin", "release-test")
        .await
        .expect("failed to find latest release");
    assert!(release_by_tag == release_latest, "releases not equal");

    let attachment = api
        .repo_create_release_attachment(
            "TestingAdmin",
            "release-test",
            release.id.unwrap() as u64,
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
                "release-test",
                release.id.unwrap() as u64,
                attachment.id.unwrap() as u64
            )
            .await
            .unwrap()
            == b"This is a file!",
        "couldn't download attachment"
    );
    let _zip_archive = api
        .repo_get_archive("TestingAdmin", "release-test", "v1.0.zip")
        .await
        .unwrap();
    let _tar_archive = api
        .repo_get_archive("TestingAdmin", "release-test", "v1.0.tar.gz")
        .await
        .unwrap();
    // check these contents when their return value is fixed

    api.repo_delete_release_attachment(
        "TestingAdmin",
        "release-test",
        release.id.unwrap() as u64,
        attachment.id.unwrap() as u64,
    )
    .await
    .expect("failed to deleted attachment");

    api.repo_delete_release("TestingAdmin", "release-test", release.id.unwrap() as u64)
        .await
        .expect("failed to delete release");

    api.repo_delete_tag("TestingAdmin", "release-test", "v1.0")
        .await
        .expect("failed to delete release");
}

#[tokio::test]
async fn delete_repo() {
    let api = common::login();
    let git = Git::new("./test_repos/delete");
    let _ = basic_repo(&api, &git, "delete-test").await;

    api.repo_delete("TestingAdmin", "delete-test")
        .await
        .expect("failed to delete repo");
}

#[tokio::test]
async fn team_pr_review_request() {
    let api = common::login();

    let org_opt = CreateOrgOption {
        description: Some("Testing team review requests".into()),
        email: None,
        full_name: None,
        location: None,
        repo_admin_change_team_access: None,
        username: "team-review-org".into(),
        visibility: None,
        website: None,
    };
    api.org_create(org_opt).await.unwrap();

    let git = Git::new("./test_repos/team-pr-review");
    let _ = basic_org_repo(&api, &git, "team-review-org", "team-pr-review").await;

    git.run(&["switch", "-c", "test"]);
    tokio::fs::write(
        "./test_repos/team-pr-review/example.rs",
        "fn add_one(x: u32) -> u32 { x + 1 }",
    )
    .await
    .unwrap();
    git.run(&["add", "."]);
    git.run(&["commit", "-m", "egg"]);
    git.run(&["push", "-u", "origin", "test"]);

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

    // Wait for... something to happen, or else creating a PR will return 404
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    api.repo_create_pull_request("team-review-org", "team-pr-review", pr_opt)
        .await
        .expect("couldn't create pr");

    let review_opt = PullReviewRequestOptions {
        reviewers: None,
        team_reviewers: Some(vec!["Owners".into()]),
    };
    api.repo_create_pull_review_requests("team-review-org", "team-pr-review", 1, review_opt)
        .await
        .expect("couldn't request review");

    let pr = api
        .repo_get_pull_request("team-review-org", "team-pr-review", 1)
        .await
        .expect("couldn't get pr");
    assert_eq!(pr.requested_reviewers, Some(Vec::new()));
}
