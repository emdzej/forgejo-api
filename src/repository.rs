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

// PR structs

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PullRequestMeta {
    pub merged: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub merged_at: time::OffsetDateTime,
}

