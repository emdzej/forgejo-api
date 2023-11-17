use super::*;

/// Repository operations.
impl Forgejo {
    /// Gets info about the specified repository.
    pub async fn get_repo(&self, user: &str, repo: &str) -> Result<Option<Repo>, ForgejoError> {
        self.get_opt(&format!("repos/{user}/{repo}/")).await
    }

    /// Creates a repository.
    pub async fn create_repo(&self, repo: CreateRepoOption) -> Result<Repo, ForgejoError> {
        self.post("user/repos", &repo).await
    }

    pub async fn get_pulls(&self, owner: &str, repo: &str, query: PullQuery) -> Result<Vec<PullRequest>, ForgejoError> {
        self.get(&query.to_string(owner, repo)).await
    }

    pub async fn create_pr(&self, owner: &str, repo: &str, opts: CreatePullRequestOption) -> Result<PullRequest, ForgejoError> {
        self.post(&format!("repos/{owner}/{repo}/pulls"), &opts).await
    }

    pub async fn is_merged(&self, owner: &str, repo: &str, pr: u64) -> Result<bool, ForgejoError> {
        self.get_opt::<()>(&format!("repos/{owner}/{repo}/pulls/{pr}/merge")).await.map(|o| o.is_some())
    }

    pub async fn merge_pr(&self, owner: &str, repo: &str, pr: u64, opts: MergePullRequestOption) -> Result<(), ForgejoError> {
        self.post(&format!("repos/{owner}/{repo}/pulls/{pr}/merge"), &opts).await
    }

    pub async fn cancel_merge(&self, owner: &str, repo: &str, pr: u64) -> Result<(), ForgejoError> {
        self.delete(&format!("repos/{owner}/{repo}/pulls/{pr}/merge")).await
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Repo {
    pub clone_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub default_branch: String,
    pub description: String,
    pub fork: bool,
    pub forks_count: u64,
    pub full_name: String,

    pub owner: User,
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
    #[serde(with = "time::serde::rfc3339")]
    pub closed_at: time::OffsetDateTime,
    pub closed_issues: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub description: String,
    #[serde(with = "time::serde::rfc3339")]
    pub due_on: time::OffsetDateTime,
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
    #[serde(with = "time::serde::rfc3339")]
    pub closed_at: time::OffsetDateTime,
    pub comments: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub diff_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub due_date: time::OffsetDateTime,
    pub head: PrBranchInfo,
    pub html_url: Url,
    pub id: u64,
    pub is_locked: bool,
    pub labels: Vec<Label>,
    pub merge_base: String,
    pub merge_commit_sha: String,
    pub mergeable: bool,
    pub merged: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub merged_at: time::OffsetDateTime,
    pub merged_by: User,
    pub milestone: Milestone,
    pub number: u64,
    pub patch_url: Url,
    pub pin_order: u64,
    pub requested_reviewers: Vec<User>,
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
    pub repo: Repo,
    pub repo_id: u64,
    pub sha: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PullRequestMeta {
    pub merged: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub merged_at: time::OffsetDateTime,
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
            s.write_fmt(format_args!("{milestone}")).expect("writing to a string never fails");
            s.push('&');
        }
        for label in &self.labels {
            s.push_str("labels=");
            s.write_fmt(format_args!("{label}")).expect("writing to a string never fails");
            s.push('&');
        }
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to a string never fails");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to a string never fails");
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

