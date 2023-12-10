use super::*;

/// Repository operations.
impl Forgejo {
    /// Gets info about the specified repository.
    pub async fn get_repo(&self, user: &str, repo: &str) -> Result<Option<Repository>, ForgejoError> {
        self.get_opt(&format!("repos/{user}/{repo}/")).await
    }

    /// Creates a repository.
    pub async fn create_repo(&self, repo: CreateRepoOption) -> Result<Repository, ForgejoError> {
        self.post("user/repos", &repo).await
    }

    pub async fn get_pulls(
        &self,
        owner: &str,
        repo: &str,
        query: PullQuery,
    ) -> Result<Vec<PullRequest>, ForgejoError> {
        self.get(&query.to_string(owner, repo)).await
    }

    pub async fn create_pr(
        &self,
        owner: &str,
        repo: &str,
        opts: CreatePullRequestOption,
    ) -> Result<PullRequest, ForgejoError> {
        self.post(&format!("repos/{owner}/{repo}/pulls"), &opts)
            .await
    }

    pub async fn is_merged(&self, owner: &str, repo: &str, pr: u64) -> Result<bool, ForgejoError> {
        self.get_opt::<()>(&format!("repos/{owner}/{repo}/pulls/{pr}/merge"))
            .await
            .map(|o| o.is_some())
    }

    pub async fn merge_pr(
        &self,
        owner: &str,
        repo: &str,
        pr: u64,
        opts: MergePullRequestOption,
    ) -> Result<(), ForgejoError> {
        self.post(&format!("repos/{owner}/{repo}/pulls/{pr}/merge"), &opts)
            .await
    }

    pub async fn cancel_merge(&self, owner: &str, repo: &str, pr: u64) -> Result<(), ForgejoError> {
        self.delete(&format!("repos/{owner}/{repo}/pulls/{pr}/merge"))
            .await
    }

    pub async fn get_releases(
        &self,
        owner: &str,
        repo: &str,
        query: ReleaseQuery,
    ) -> Result<Vec<Release>, ForgejoError> {
        self.get(&query.to_string(owner, repo)).await
    }

    pub async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Option<Release>, ForgejoError> {
        self.get_opt(&format!("repos/{owner}/{repo}/releases/{id}"))
            .await
    }

    pub async fn get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<Option<Release>, ForgejoError> {
        self.get_opt(&format!("repos/{owner}/{repo}/releases/tags/{tag}"))
            .await
    }

    pub async fn delete_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        self.delete(&format!("repos/{owner}/{repo}/releases/{id}"))
            .await
    }

    pub async fn delete_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<(), ForgejoError> {
        self.delete(&format!("repos/{owner}/{repo}/releases/tags/{tag}"))
            .await
    }

    pub async fn edit_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        opts: EditReleaseOption,
    ) -> Result<Release, ForgejoError> {
        self.patch(&format!("repos/{owner}/{repo}/releases/{id}"), &opts)
            .await
    }

    pub async fn get_release_attachments(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Vec<Attachment>, ForgejoError> {
        self.get(&format!("repos/{owner}/{repo}/releases/{id}/assets"))
            .await
    }

    pub async fn get_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        release_id: u64,
        attachment_id: u64,
    ) -> Result<Attachment, ForgejoError> {
        self.get(&format!(
            "repos/{owner}/{repo}/releases/{release_id}/assets/{attachment_id}"
        ))
        .await
    }

    pub async fn create_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        name: &str,
        file: &[u8],
    ) -> Result<Attachment, ForgejoError> {
        self.post_form(
            &format!("repos/{owner}/{repo}/releases/{id}/assets?name={name}"),
            &file,
        )
        .await
    }

    pub async fn delete_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        release_id: u64,
        attachment_id: u64,
    ) -> Result<(), ForgejoError> {
        self.delete(&format!(
            "repos/{owner}/{repo}/releases/{release_id}/assets/{attachment_id}"
        ))
        .await
    }

    pub async fn edit_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        release_id: u64,
        attachment_id: u64,
        opts: EditAttachmentOption,
    ) -> Result<Attachment, ForgejoError> {
        self.patch(
            &format!("repos/{owner}/{repo}/releases/{release_id}/assets/{attachment_id}"),
            &opts,
        )
        .await
    }

    pub async fn create_release(
        &self,
        owner: &str,
        repo: &str,
        opts: CreateReleaseOption,
    ) -> Result<Release, ForgejoError> {
        self.post(&format!("repos/{owner}/{repo}/releases"), &opts)
            .await
    }

    pub async fn latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<Release>, ForgejoError> {
        self.get_opt(&format!("repos/{owner}/{repo}/releases/latest"))
            .await
    }

    pub async fn get_tags(&self, owner: &str, repo: &str, query: TagQuery) -> Result<Vec<Tag>, ForgejoError> {
        self.get(&query.to_string(owner, repo)).await
    }

    pub async fn create_tag(&self, owner: &str, repo: &str, opts: CreateTagOption) -> Result<Tag, ForgejoError> {
        self.post(&format!("repos/{owner}/{repo}/tags"), &opts).await
    }

    pub async fn get_tag(&self, owner: &str, repo: &str, tag: &str) -> Result<Option<Tag>, ForgejoError> {
        self.get_opt(&format!("repos/{owner}/{repo}/tags/{tag}")).await
    }

    pub async fn delete_tag(&self, owner: &str, repo: &str, tag: &str) -> Result<(), ForgejoError> {
        self.delete(&format!("repos/{owner}/{repo}/tags/{tag}")).await
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Repository {
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_rebase_update: bool,
    pub allow_squash_merge: bool,
    pub archived: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub archived_at: Option<time::OffsetDateTime>,
    #[serde(deserialize_with = "crate::none_if_blank_url")]
    pub avatar_url: Option<Url>,
    pub clone_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub default_allow_maintainer_edit: bool,
    pub default_branch: String,
    pub default_delete_branch_after_merge: bool,
    pub default_merge_style: String,
    pub description: String,
    pub empty: bool,
    pub external_tracker: Option<ExternalTracker>,
    pub external_wiki: Option<ExternalWiki>,
    pub fork: bool,
    pub forks_count: u64,
    pub full_name: String,
    pub has_actions: bool,
    pub has_issues: bool,
    pub has_packages: bool,
    pub has_projects: bool,
    pub has_pull_requests: bool,
    pub has_releases: bool,
    pub has_wiki: bool,
    pub html_url: Url,
    pub id: u64,
    pub ignore_whitespace_conflicts: bool,
    pub internal: bool,
    pub internal_tracker: Option<InternalTracker>,
    pub language: String,
    pub languages_url: Url,
    pub link: String,
    pub mirror: bool,
    pub mirror_interval: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub mirror_updated: Option<time::OffsetDateTime>,
    pub name: String,
    pub open_issues_count: u64,
    pub open_pr_counter: u64,
    #[serde(deserialize_with = "crate::none_if_blank_url")]
    pub original_url: Option<Url>,
    pub owner: User,
    pub permissions: Permission,
    pub private: bool,
    pub release_counter: u64,
    pub repo_transfer: Option<RepoTransfer>,
    pub size: u64,
    pub ssh_url: String,
    pub stars_count: u64,
    pub template: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
    pub url: Url,
    pub watchers_count: u64,
    pub website: Option<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct RepositoryMeta {
    pub full_name: String,
    pub id: u64,
    pub name: String,
    pub owner: String,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateRepoOption {
    pub auto_init: bool,
    pub default_branch: String,
    pub description: Option<String>,
    pub gitignores: String,
    pub issue_labels: String,
    pub license: String,
    pub name: String,
    pub private: bool,
    pub readme: String,
    pub template: bool,
    pub trust_model: TrustModel,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub enum TrustModel {
    Default,
    Collaborator,
    Committer,
    #[serde(rename = "collaboratorcommiter")]
    CollaboratorCommitter,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Milestone {
    #[serde(with = "time::serde::rfc3339::option")]
    pub closed_at: Option<time::OffsetDateTime>,
    pub closed_issues: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub description: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_on: Option<time::OffsetDateTime>,
    pub id: u64,
    pub open_issues: u64,
    pub state: State,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PullRequest {
    pub allow_maintainer_edit: bool,
    pub assignee: User,
    pub assignees: Vec<User>,
    pub base: PrBranchInfo,
    pub body: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub closed_at: Option<time::OffsetDateTime>,
    pub comments: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub diff_url: Url,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_date: Option<time::OffsetDateTime>,
    pub head: PrBranchInfo,
    pub html_url: Url,
    pub id: u64,
    pub is_locked: bool,
    pub labels: Vec<Label>,
    pub merge_base: String,
    pub merge_commit_sha: Option<String>,
    pub mergeable: bool,
    pub merged: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub merged_at: Option<time::OffsetDateTime>,
    pub merged_by: Option<User>,
    pub milestone: Option<Milestone>,
    pub number: u64,
    pub patch_url: Url,
    pub pin_order: u64,
    pub requested_reviewers: Option<Vec<User>>,
    pub state: State,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
    pub url: Url,
    pub user: User,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PrBranchInfo {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub repo_id: u64,
    pub sha: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PullRequestMeta {
    pub merged: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub merged_at: Option<time::OffsetDateTime>,
}

#[derive(Debug)]
pub struct PullQuery {
    pub state: Option<State>,
    pub sort: Option<PullQuerySort>,
    pub milestone: Option<u64>,
    pub labels: Vec<u64>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl PullQuery {
    fn to_string(&self, owner: &str, repo: &str) -> String {
        use std::fmt::Write;
        // This is different to other query struct serialization because
        // `labels` is serialized so strangely
        let mut s = String::new();
        s.push_str("repos/");
        s.push_str(owner);
        s.push('/');
        s.push_str(repo);
        s.push_str("/pulls?");
        if let Some(state) = self.state {
            s.push_str("state=");
            s.push_str(state.as_str());
            s.push('&');
        }
        if let Some(sort) = self.sort {
            s.push_str("sort=");
            s.push_str(sort.as_str());
            s.push('&');
        }
        if let Some(milestone) = self.milestone {
            s.push_str("sort=");
            s.write_fmt(format_args!("{milestone}"))
                .expect("writing to a string never fails");
            s.push('&');
        }
        for label in &self.labels {
            s.push_str("labels=");
            s.write_fmt(format_args!("{label}"))
                .expect("writing to a string never fails");
            s.push('&');
        }
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}"))
                .expect("writing to a string never fails");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}"))
                .expect("writing to a string never fails");
            s.push('&');
        }
        s
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PullQuerySort {
    Oldest,
    RecentUpdate,
    LeastUpdate,
    MostComment,
    LeastComment,
    Priority,
}

impl PullQuerySort {
    fn as_str(&self) -> &'static str {
        match self {
            PullQuerySort::Oldest => "oldest",
            PullQuerySort::RecentUpdate => "recentupdate",
            PullQuerySort::LeastUpdate => "leastupdate",
            PullQuerySort::MostComment => "mostcomment",
            PullQuerySort::LeastComment => "leastcomment",
            PullQuerySort::Priority => "priority",
        }
    }
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct CreatePullRequestOption {
    pub assignee: Option<String>,
    pub assignees: Vec<String>,
    pub base: String,
    pub body: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_date: Option<time::OffsetDateTime>,
    pub head: String,
    pub labels: Vec<u64>,
    pub milestone: Option<u64>,
    pub title: String,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct MergePullRequestOption {
    #[serde(rename = "Do")]
    pub act: MergePrAction,
    #[serde(rename = "MergeCommitId")]
    pub merge_commit_id: Option<String>,
    #[serde(rename = "MergeMessageField")]
    pub merge_message_field: Option<String>,
    #[serde(rename = "MergeTitleField")]
    pub merge_title_field: Option<String>,
    pub delete_branch_after_merge: Option<bool>,
    pub force_merge: Option<bool>,
    pub head_commit_id: Option<String>,
    pub merge_when_checks_succeed: Option<bool>,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub enum MergePrAction {
    #[serde(rename = "merge")]
    #[default]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "rebase-merge")]
    RebaseMerge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "manually-merged")]
    ManuallyMerged,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Release {
    pub assets: Vec<Attachment>,
    pub author: User,
    pub body: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub draft: bool,
    pub html_url: Url,
    pub id: u64,
    pub name: String,
    pub prerelease: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub published_at: time::OffsetDateTime,
    pub tag_name: String,
    pub tarball_url: Url,
    pub target_commitish: String,
    pub url: Url,
    pub zipball_url: Url,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct CreateReleaseOption {
    pub body: String,
    pub draft: bool,
    pub name: String,
    pub prerelease: bool,
    pub tag_name: String,
    pub target_commitish: Option<String>,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct EditReleaseOption {
    pub body: Option<String>,
    pub draft: Option<bool>,
    pub name: Option<String>,
    pub prerelease: Option<bool>,
    pub tag_name: Option<String>,
    pub target_commitish: Option<String>,
}

#[derive(Default, Debug)]
pub struct ReleaseQuery {
    pub draft: Option<bool>,
    pub prerelease: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl ReleaseQuery {
    fn to_string(&self, owner: &str, repo: &str) -> String {
        format!(
            "repos/{owner}/{repo}/releases?draft={}&pre-release={}&page={}&limit={}",
            opt_bool_s(self.draft),
            opt_bool_s(self.prerelease),
            self.page.map(|page| page.to_string()).unwrap_or_default(),
            self.limit.map(|page| page.to_string()).unwrap_or_default(),
        )
    }
}

fn opt_bool_s(b: Option<bool>) -> &'static str {
    match b {
        Some(true) => "true",
        Some(false) => "false",
        None => "",
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Tag {
    pub commit: CommitMeta,
    pub id: String,
    pub message: String,
    pub name: String,
    pub tarball_url: Url,
    pub zipball_url: Url,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct CreateTagOption {
    pub message: Option<String>,
    pub tag_name: String,
    pub target: Option<String>,
}

#[derive(Default, Debug)]
pub struct TagQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl TagQuery {
    fn to_string(&self, owner: &str, repo: &str) -> String {
        format!("repos/{owner}/{repo}/tags?page={}&limit={}",
            self.page.map(|page| page.to_string()).unwrap_or_default(),
            self.limit.map(|page| page.to_string()).unwrap_or_default(),
        )
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct CommitMeta {
    #[serde(with = "time::serde::rfc3339")]
    pub created: time::OffsetDateTime,
    pub url: Url,
    pub sha: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ExternalTracker {
    #[serde(rename = "external_tracker_format")]
    pub format: String,
    #[serde(rename = "external_tracker_regexp_pattern")]
    pub regexp_pattern: String,
    #[serde(rename = "external_tracker_style")]
    pub style: String,
    #[serde(rename = "external_tracker_url")]
    pub url: Url,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct InternalTracker {
    pub allow_only_contributors_to_track_time: bool,
    pub enable_issue_dependencies: bool,
    pub enable_time_tracker: bool,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ExternalWiki {
    #[serde(rename = "external_wiki_url")]
    pub url: Url,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Permission {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct RepoTransfer {
    pub doer: User,
    pub recipient: User,
    pub teams: Vec<Team>,
}

