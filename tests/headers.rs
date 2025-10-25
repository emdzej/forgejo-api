mod common;
use common::Git;
use forgejo_api::structs::*;

macro_rules! test_header {
    ($e:expr, $header:ident) => {
        let (headers, _) = $e.await.expect("api called failed");
        assert!(headers.$header.is_some());
    };
}

#[tokio::test]
async fn total_count() {
    // password login needed for user_get_tokens
    let api = common::login_pass("TestingAdmin", "password");

    test_header!(api.user_get_tokens("TestingAdmin"), x_total_count);
    test_header!(
        api.user_list_activity_feeds("TestingAdmin", UserListActivityFeedsQuery::default()),
        x_total_count
    );
    test_header!(api.user_current_list_gpg_keys(), x_total_count);
    test_header!(api.user_get_oauth2_applications(), x_total_count);
    test_header!(api.user_get_oauth2_applications(), x_total_count);
    test_header!(
        api.user_current_list_keys(UserCurrentListKeysQuery::default()),
        x_total_count
    );
    test_header!(api.user_list_blocked_users(), x_total_count);
    test_header!(
        api.user_current_list_repos(UserCurrentListReposQuery::default()),
        x_total_count
    );
    test_header!(api.user_get_stop_watches(), x_total_count);
    test_header!(
        api.user_current_tracked_times(UserCurrentTrackedTimesQuery::default()),
        x_total_count
    );
    test_header!(api.user_list_followers("TestingAdmin"), x_total_count);
    test_header!(
        api.list_packages("TestingAdmin", ListPackagesQuery::default()),
        x_total_count
    );
    test_header!(
        api.notify_get_list(NotifyGetListQuery::default()),
        x_total_count
    );

    let git = Git::new("./test_repos/header_test");
    common::basic_repo(&api, &git, "header_test").await;
    let body = CreateIssueOption {
        assignee: None,
        assignees: None,
        body: None,
        closed: None,
        due_date: None,
        labels: None,
        milestone: None,
        r#ref: None,
        title: "Test".into(),
    };
    api.issue_create_issue("TestingAdmin", "header_test", body)
        .await
        .unwrap();
    let body = EditReactionOption {
        content: Some("+1".into()),
    };
    api.issue_post_issue_reaction("TestingAdmin", "header_test", 1, body)
        .await
        .unwrap();
    test_header!(
        api.issue_get_issue_reactions("TestingAdmin", "header_test", 1,),
        x_total_count
    );
    test_header!(
        api.issue_list_issues(
            "TestingAdmin",
            "header_test",
            IssueListIssuesQuery::default()
        ),
        x_total_count
    );
    test_header!(
        api.issue_list_labels(
            "TestingAdmin",
            "header_test",
            IssueListLabelsQuery::default()
        ),
        x_total_count
    );
    test_header!(
        api.issue_get_milestones_list(
            "TestingAdmin",
            "header_test",
            IssueGetMilestonesListQuery::default()
        ),
        x_total_count
    );

    git.run(&["switch", "-c", "pluey"]);
    tokio::fs::write("./test_repos/header_test/pluey.txt", "wahoo")
        .await
        .unwrap();
    git.run(&["add", "."]);
    git.run(&["commit", "-m", "\"We implemented pluey\""]);
    git.run(&["push"]);
    let body = CreatePullRequestOption {
        assignee: None,
        assignees: None,
        base: Some("main".into()),
        body: None,
        due_date: None,
        head: Some("pluey".into()),
        labels: None,
        milestone: None,
        title: Some("pluey".into()),
    };
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let pr = api
        .repo_create_pull_request("TestingAdmin", "header_test", body)
        .await
        .unwrap();
    test_header!(
        api.repo_get_pull_request_files(
            "TestingAdmin",
            "header_test",
            2,
            RepoGetPullRequestFilesQuery::default()
        ),
        x_total_count
    );
    test_header!(
        api.issue_get_comments(
            "TestingAdmin",
            "header_test",
            1,
            IssueGetCommentsQuery::default()
        ),
        x_total_count
    );
    test_header!(
        api.repo_get_pull_request_commits(
            "TestingAdmin",
            "header_test",
            2,
            RepoGetPullRequestCommitsQuery::default()
        ),
        x_total_count
    );
    let sha = pr.head.unwrap().sha.unwrap();
    test_header!(
        api.repo_list_pull_requests(
            "TestingAdmin",
            "header_test",
            RepoListPullRequestsQuery::default()
        ),
        x_total_count
    );
    let body = CreatePullReviewOptions {
        body: Some("woah".into()),
        comments: None,
        commit_id: None,
        event: None,
    };
    api.repo_create_pull_review("TestingAdmin", "header_test", 2, body)
        .await
        .unwrap();
    test_header!(
        api.repo_list_pull_reviews("TestingAdmin", "header_test", 2,),
        x_total_count
    );
    test_header!(
        api.issue_get_comments_and_timeline(
            "TestingAdmin",
            "header_test",
            2,
            IssueGetCommentsAndTimelineQuery::default()
        ),
        x_total_count
    );

    test_header!(
        api.repo_list_push_mirrors("TestingAdmin", "header_test",),
        x_total_count
    );
    //tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    test_header!(
        api.repo_list_branches("TestingAdmin", "header_test",),
        x_total_count
    );
    test_header!(
        api.repo_list_branches("TestingAdmin", "header_test",),
        x_total_count
    );
    test_header!(
        api.repo_list_statuses(
            "TestingAdmin",
            "header_test",
            &sha,
            RepoListStatusesQuery::default()
        ),
        x_total_count
    );
    test_header!(
        api.repo_list_keys("TestingAdmin", "header_test", RepoListKeysQuery::default()),
        x_total_count
    );
    test_header!(
        api.repo_list_hooks("TestingAdmin", "header_test"),
        x_total_count
    );
    test_header!(
        api.repo_list_releases(
            "TestingAdmin",
            "header_test",
            RepoListReleasesQuery::default()
        ),
        x_total_count
    );
    test_header!(
        api.repo_list_actions_secrets("TestingAdmin", "header_test",),
        x_total_count
    );
    test_header!(
        api.repo_list_tags("TestingAdmin", "header_test"),
        x_total_count
    );
    test_header!(
        api.get_repo_variables_list("TestingAdmin", "header_test",),
        x_total_count
    );
    let body = CreateWikiPageOptions {
        content_base64: Some(
            "WW91IGRlY29kZWQgdGhpcyB0byBzZWUgd2hhdCBpdCBzYXlzPyBZb3UncmUgcXVpdGUgYSBuZXJk".into(),
        ),
        message: Some("wahoo".into()),
        title: Some("Home".into()),
    };
    api.repo_create_wiki_page("TestingAdmin", "header_test", body)
        .await
        .unwrap();
    test_header!(
        api.repo_get_wiki_page_revisions("TestingAdmin", "header_test", "Home",),
        x_total_count
    );
    test_header!(
        api.repo_get_wiki_pages("TestingAdmin", "header_test",),
        x_total_count
    );

    test_header!(api.admin_cron_list(), x_total_count);
    //test_header!(api.admin_list_quota_groups(), x_total_count);
    //test_header!(api.admin_list_quota_rules(), x_total_count);
    //test_header!(api.user_list_quota_artifacts(UserListQuotaArtifactsQuery::default()), x_total_count);
    //test_header!(api.user_list_quota_attachments(UserListQuotaAttachmentsQuery::default()), x_total_count);
    //test_header!(api.user_list_quota_packages(UserListQuotaPackagesQuery::default()), x_total_count);
    let body = CreateOrgOption {
        description: None,
        email: None,
        full_name: None,
        location: None,
        repo_admin_change_team_access: None,
        username: "header-test-org".into(),
        visibility: None,
        website: None,
    };
    api.org_create(body).await.unwrap();
    test_header!(api.org_list_teams("header-test-org"), x_total_count);
    test_header!(api.org_get_all(), x_total_count);
}

#[tokio::test]
async fn has_more() {
    let api = common::login_pass("TestingAdmin", "password");

    let git = Git::new("./test_repos/header_more_test");
    common::basic_repo(&api, &git, "header_more_test").await;

    git.run(&["switch", "-c", "pluey"]);
    tokio::fs::write("./test_repos/header_more_test/pluey.txt", "wahoo")
        .await
        .unwrap();
    git.run(&["add", "."]);
    git.run(&["commit", "-m", "\"We implemented pluey\""]);
    git.run(&["push"]);
    let body = CreatePullRequestOption {
        assignee: None,
        assignees: None,
        base: Some("main".into()),
        body: None,
        due_date: None,
        head: Some("pluey".into()),
        labels: None,
        milestone: None,
        title: Some("pluey".into()),
    };
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    api.repo_create_pull_request("TestingAdmin", "header_more_test", body)
        .await
        .unwrap();
    test_header!(
        api.repo_get_pull_request_files(
            "TestingAdmin",
            "header_more_test",
            1,
            RepoGetPullRequestFilesQuery::default()
        ),
        x_has_more
    );
    test_header!(
        api.repo_get_pull_request_commits(
            "TestingAdmin",
            "header_more_test",
            1,
            RepoGetPullRequestCommitsQuery::default()
        ),
        x_has_more
    );
}
