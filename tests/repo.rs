use forgejo_api::structs::*;

mod common;

#[tokio::test]
async fn repo() {
    let api = common::get_api();

    tokio::fs::create_dir("./test_repo").await.unwrap();
    let git = || {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir("./test_repo");
        cmd
    };
    let _ = git()
        .args(["config", "--global", "init.defaultBranch", "main"])
        .status()
        .unwrap();
    let _ = git().args(["init"]).status().unwrap();
    let _ = git()
        .args(["config", "user.name", "TestingAdmin"])
        .status()
        .unwrap();
    let _ = git()
        .args(["config", "user.email", "admin@noreply.example.org"])
        .status()
        .unwrap();
    tokio::fs::write("./test_repo/README.md", "# Test\nThis is a test repo")
        .await
        .unwrap();
    let _ = git().args(["add", "."]).status().unwrap();
    let _ = git()
        .args(["commit", "-m", "initial commit"])
        .status()
        .unwrap();

    let repo_opt = CreateRepoOption {
        auto_init: Some(false),
        default_branch: Some("main".into()),
        description: Some("Test Repo".into()),
        gitignores: Some("".into()),
        issue_labels: Some("".into()),
        license: Some("".into()),
        name: "test".into(),
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
        remote_repo.name.as_ref().unwrap() == "test",
        "repo owner is not \"test\""
    );
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let mut remote_url = remote_repo.clone_url.clone().unwrap();
    remote_url.set_username("TestingAdmin").unwrap();
    remote_url.set_password(Some("password")).unwrap();
    let _ = git()
        .args(["remote", "add", "origin", remote_url.as_str()])
        .status()
        .unwrap();
    let _ = git()
        .args(["push", "-u", "origin", "main"])
        .status()
        .unwrap();

    let _ = git().args(["switch", "-c", "test"]).status().unwrap();
    tokio::fs::write(
        "./test_repo/example.rs",
        "fn add_one(x: u32) -> u32 { x + 1 }",
    )
    .await
    .unwrap();
    let _ = git().args(["add", "."]).status().unwrap();
    let _ = git().args(["commit", "-m", "egg"]).status().unwrap();
    let _ = git()
        .args(["push", "-u", "origin", "test"])
        .status()
        .unwrap();

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
    let pr = api
        .repo_create_pull_request("TestingAdmin", "test", pr_opt)
        .await
        .expect("couldn't create pr");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "test", pr.number.unwrap())
        .await
        .is_ok();
    assert!(!is_merged, "pr should not yet be merged");
    let pr_files_query = RepoGetPullRequestFilesQuery::default();
    let (_, _) = api
        .repo_get_pull_request_files("TestingAdmin", "test", pr.number.unwrap(), pr_files_query)
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
    api.repo_merge_pull_request("TestingAdmin", "test", pr.number.unwrap(), merge_opt)
        .await
        .expect("couldn't merge pr");
    let is_merged = api
        .repo_pull_request_is_merged("TestingAdmin", "test", pr.number.unwrap())
        .await
        .is_ok();
    assert!(is_merged, "pr should be merged");
    let _ = git().args(["fetch"]).status().unwrap();
    let _ = git().args(["pull"]).status().unwrap();

    let query = RepoListReleasesQuery::default();
    assert!(
        api.repo_list_releases("TestingAdmin", "test", query)
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
    api.repo_create_tag("TestingAdmin", "test", tag_opt)
        .await
        .expect("failed to create tag");

    let release_opt = CreateReleaseOption {
        body: Some("This is a release!".into()),
        draft: Some(true),
        name: Some("v1.0".into()),
        prerelease: Some(false),
        tag_name: "v1.0".into(),
        target_commitish: None,
    };
    let release = api
        .repo_create_release("TestingAdmin", "test", release_opt)
        .await
        .expect("failed to create release");
    let edit_release = EditReleaseOption {
        body: None,
        draft: Some(false),
        name: None,
        prerelease: None,
        tag_name: None,
        target_commitish: None,
    };
    api.repo_edit_release("TestingAdmin", "test", release.id.unwrap(), edit_release)
        .await
        .expect("failed to edit release");

    let release_by_tag = api
        .repo_get_release_by_tag("TestingAdmin", "test", "v1.0")
        .await
        .expect("failed to find release");
    let release_latest = api
        .repo_get_latest_release("TestingAdmin", "test")
        .await
        .expect("failed to find latest release");
    assert!(release_by_tag == release_latest, "releases not equal");

    let attachment = api
        .repo_create_release_attachment(
            "TestingAdmin",
            "test",
            release.id.unwrap(),
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
                "test",
                release.id.unwrap(),
                attachment.id.unwrap()
            )
            .await
            .unwrap()
            == b"This is a file!",
        "couldn't download attachment"
    );
    let _zip_archive = api
        .repo_get_archive("TestingAdmin", "test", "v1.0.zip")
        .await
        .unwrap();
    let _tar_archive = api
        .repo_get_archive("TestingAdmin", "test", "v1.0.tar.gz")
        .await
        .unwrap();
    // check these contents when their return value is fixed

    api.repo_delete_release_attachment(
        "TestingAdmin",
        "test",
        release.id.unwrap(),
        attachment.id.unwrap(),
    )
    .await
    .expect("failed to deleted attachment");

    api.repo_delete_release("TestingAdmin", "test", release.id.unwrap())
        .await
        .expect("failed to delete release");

    api.repo_delete_tag("TestingAdmin", "test", "v1.0")
        .await
        .expect("failed to delete release");
}
