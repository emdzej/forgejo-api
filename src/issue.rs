use super::*;

impl Forgejo {
    pub async fn get_repo_issues(&self, owner: &str, repo: &str, query: IssueQuery) -> Result<Vec<Issue>, ForgejoError> {
        self.get(&query.to_string(owner, repo)).await
    }

    pub async fn create_issue(&self, owner: &str, repo: &str, opts: CreateIssueOption) -> Result<Issue, ForgejoError> {
        self.post(&format!("/repos/{owner}/{repo}/issues"), &opts).await
    }

    pub async fn get_issue(&self, owner: &str, repo: &str, id: u64) -> Result<Option<Issue>, ForgejoError> {
        self.get_opt(&format!("/repos/{owner}/{repo}/issues/{id}")).await
    }

    pub async fn delete_issue(&self, owner: &str, repo: &str, id: u64) -> Result<(), ForgejoError> {
        self.delete(&format!("/repos/{owner}/{repo}/issues/{id}")).await
    }

    pub async fn edit_issue(&self, owner: &str, repo: &str, id: u64, opts: EditIssueOption) -> Result<(), ForgejoError> {
        self.patch(&format!("/repos/{owner}/{repo}/issues/{id}"), &opts).await
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Issue {
    pub assets: Vec<Attachment>,
    pub assignee: User,
    pub assignees: Vec<User>,
    pub body: String,
    #[serde(with = "time::serde::rfc3339")]
    pub closed_at: time::OffsetDateTime,
    pub comments: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub due_date: time::OffsetDateTime,
    pub html_url: Url,
    pub id: u64,
    pub is_locked: bool,
    pub labels: Vec<Label>,
    pub milestone: Milestone,
    pub number: u64,
    pub original_author: String,
    pub original_author_id: u64,
    pub pin_order: u64,
    pub pull_request: PullRequestMeta,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repository: RepositoryMeta,
    pub state: State,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
    pub url: Url,
    pub user: User,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Label {
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    pub id: u64,
    pub name: String,
    pub url: Url,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Attachment {
    pub browser_download_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub download_count: u64,
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub uuid: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum State {
    Open,
    Closed,
}

impl State {
    fn as_str(&self) -> &'static str {
        match self {
            State::Open => "open",
            State::Closed => "closed",
        }
    }
}

#[derive(Default, Debug)]
pub struct IssueQuery {
    pub state: Option<State>,
    pub labels: Vec<String>,
    pub query: Option<String>,
    pub _type: Option<IssueQueryType>,
    pub milestones: Vec<String>,
    pub since: Option<time::OffsetDateTime>,
    pub before: Option<time::OffsetDateTime>,
    pub created_by: Option<String>,
    pub assigned_by: Option<String>,
    pub mentioned_by: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl IssueQuery {
    fn to_string(&self, owner: &str, repo: &str) -> String {
        format!("/repos/{owner}/{repo}/issues?state={}&labels={}&q={}&type={}&milestones={}&since={}&before={}&created_by={}&assigned_by={}&mentioned_by={}&page={}&limit={}",
            self.state.map(|s| s.as_str()).unwrap_or_default(),
            self.labels.join(","),
            self.query.as_deref().unwrap_or_default(),
            self._type.map(|t| t.as_str()).unwrap_or_default(),
            self.milestones.join(","),
            self.since.map(|t| t.format(&time::format_description::well_known::Rfc3339).unwrap()).unwrap_or_default(),
            self.before.map(|t| t.format(&time::format_description::well_known::Rfc3339).unwrap()).unwrap_or_default(),
            self.created_by.as_deref().unwrap_or_default(),
            self.assigned_by.as_deref().unwrap_or_default(),
            self.mentioned_by.as_deref().unwrap_or_default(),
            self.page.map(|page| page.to_string()).unwrap_or_default(),
            self.limit.map(|page| page.to_string()).unwrap_or_default(),
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IssueQueryType {
    Issues,
    Pulls,
}

impl IssueQueryType {
    fn as_str(&self) -> &'static str {
        match self {
            IssueQueryType::Issues => "issues",
            IssueQueryType::Pulls => "pulls",
        }
    }
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct CreateIssueOption {
    pub assignees: Vec<String>,
    pub body: Option<String>,
    pub closed: Option<bool>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_date: Option<time::OffsetDateTime>,
    pub labels: Vec<u64>,
    pub milestone: Option<u64>,
    pub _ref: Option<String>,
    pub title: String,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct EditIssueOption {
    pub assignees: Vec<String>,
    pub body: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_date: Option<time::OffsetDateTime>,
    pub labels: Vec<u64>,
    pub milestone: Option<u64>,
    pub _ref: Option<String>,
    pub state: Option<State>,
    pub title: Option<String>,
    pub unset_due_date: Option<bool>,
}
