use super::*;

use std::fmt::Write;
use std::collections::BTreeMap;

impl Forgejo {
    pub async fn admin_get_crons(&self, query: CronQuery) -> Result<Vec<Cron>, ForgejoError> {
        self.get(&query.path()).await
    }

    pub async fn admin_run_cron(&self, name: &str) -> Result<(), ForgejoError> {
        self.post(&format!("admin/cron/{name}"), &()).await
    }

    pub async fn admin_get_emails(&self, query: EmailListQuery) -> Result<Vec<Email>, ForgejoError> {
        self.get(&query.path()).await
    }

    pub async fn admin_search_emails(&self, query: EmailSearchQuery) -> Result<Vec<Email>, ForgejoError> {
        self.get(&query.path()).await
    }

    pub async fn admin_get_hooks(&self, query: HookQuery) -> Result<Vec<Hook>, ForgejoError> {
        self.get(&query.path()).await
    }

    pub async fn admin_create_hook(&self, opt: CreateHookOption) -> Result<Hook, ForgejoError> {
        self.post("admin/hooks", &opt).await
    }

    pub async fn admin_get_hook(&self, id: u64) -> Result<Option<Hook>, ForgejoError> {
        self.get_opt(&format!("admin/hooks/{id}")).await
    }

    pub async fn admin_delete_hook(&self, id: u64) -> Result<(), ForgejoError> {
        self.delete(&format!("admin/hooks/{id}")).await
    }

    pub async fn admin_edit_hook(&self, id: u64, opt: EditHookOption) -> Result<Hook, ForgejoError> {
        self.patch(&format!("admin/hooks/{id}"), &opt).await
    }

    pub async fn admin_get_orgs(&self, query: AdminOrganizationQuery) -> Result<Vec<Organization>, ForgejoError> {
        self.get(&query.path()).await
    }

    pub async fn admin_unadopted_repos(&self, query: UnadoptedRepoQuery) -> Result<Vec<String>, ForgejoError> {
        self.get(&query.path()).await
    }
    
    pub async fn admin_adopt(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        self.post(&format!("admin/unadopted/{owner}/{repo}"), &()).await
    }

    pub async fn admin_delete_unadopted(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        self.delete(&format!("admin/unadopted/{owner}/{repo}")).await
    }

    pub async fn admin_users(&self, query: AdminUserQuery) -> Result<Vec<User>, ForgejoError> {
        self.get(&query.path()).await
    }

    pub async fn admin_create_user(&self, opt: CreateUserOption) -> Result<User, ForgejoError> {
        self.post("admin/users", &opt).await
    }

    pub async fn admin_delete_user(&self, user: &str, purge: bool) -> Result<(), ForgejoError> {
        self.delete(&format!("admin/users/{user}?purge={purge}")).await
    }

    pub async fn admin_edit_user(&self, user: &str, opt: CreateUserOption) -> Result<User, ForgejoError> {
        self.patch(&format!("admin/users/{user}"), &opt).await
    }

    pub async fn admin_add_key(&self, user: &str, opt: CreateKeyOption) -> Result<PublicKey, ForgejoError> {
        self.patch(&format!("admin/users/{user}/keys"), &opt).await
    }

    pub async fn admin_delete_key(&self, user: &str, id: u64) -> Result<(), ForgejoError> {
        self.delete(&format!("admin/users/{user}/keys/{id}")).await
    }

    pub async fn admin_create_org(&self, owner: &str, opt: CreateOrgOption) -> Result<Organization, ForgejoError> {
        self.post(&format!("admin/users/{owner}/orgs"), &opt).await
    }
    
    pub async fn admin_rename_user(&self, user: &str, opt: RenameUserOption) -> Result<(), ForgejoError> {
        self.post(&format!("admin/users/{user}/rename"), &opt).await
    }
    
    pub async fn admin_create_repo(&self, owner: &str, opt: CreateRepoOption) -> Result<Repository, ForgejoError> {
        self.post(&format!("admin/users/{owner}/repos"), &opt).await
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Cron {
    pub exec_times: u64,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub next: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub prev: time::OffsetDateTime,
    pub schedule: String,
}

#[derive(Default, Debug)]
pub struct CronQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl CronQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/cron?");
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        s
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Email {
    pub email: String,
    pub primary: bool,
    pub user_id: u64,
    pub username: String,
    pub verified: bool,
}

#[derive(Default, Debug)]
pub struct EmailListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl EmailListQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/emails?");
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        s
    }
}

#[derive(Default, Debug)]
pub struct EmailSearchQuery {
    pub query: String,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl EmailSearchQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/emails/search?");
        if !self.query.is_empty() {
            s.push_str("q=");
            s.push_str(&self.query);
            s.push('&');
        }
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        s
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Hook {
    pub active: bool,
    pub authorization_header: String,
    pub branch_filter: String,
    pub config: std::collections::BTreeMap<String, String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub events: Vec<String>,
    pub id: u64,
    #[serde(rename = "type")]
    pub _type: HookType,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum HookType {
    Forgejo, 
    Dingtalk, 
    Discord, 
    Gitea, 
    Gogs, 
    Msteams, 
    Slack, 
    Telegram, 
    Feishu, 
    Wechatwork, 
    Packagist
}

#[derive(Default, Debug)]
pub struct HookQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl HookQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/hooks?");
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        s
    }
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateHookOption {
    pub active: Option<bool>,
    pub authorization_header: Option<String>,
    pub branch_filter: Option<String>,
    pub config: CreateHookOptionConfig,
    pub events: Vec<String>,
    #[serde(rename = "type")]
    pub _type: HookType,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateHookOptionConfig {
    pub content_type: String,
    pub url: Url,
    #[serde(flatten)]
    pub other: BTreeMap<String, String>,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct EditHookOption {
    pub active: Option<bool>,
    pub authorization_header: Option<String>,
    pub branch_filter: Option<String>,
    pub config: Option<BTreeMap<String, String>>,
    pub events: Option<Vec<String>>,
}

#[derive(Default, Debug)]
pub struct AdminOrganizationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl AdminOrganizationQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/orgs?");
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        s
    }
}

#[derive(Default, Debug)]
pub struct UnadoptedRepoQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub pattern: String,
}

impl UnadoptedRepoQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/unadopted?");
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        if !self.pattern.is_empty() {
            s.push_str("pattern=");
            s.push_str(&self.pattern);
            s.push('&');
        }
        s
    }
}

#[derive(Default, Debug)]
pub struct AdminUserQuery {
    pub source_id: Option<u64>,
    pub login_name: String,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl AdminUserQuery {
    fn path(&self) -> String {
        let mut s = String::from("admin/users?");
        if let Some(source_id) = self.source_id {
            s.push_str("source_id=");
            s.write_fmt(format_args!("{source_id}")).expect("writing to string can't fail");
            s.push('&');
        }
        if !self.login_name.is_empty() {
            s.push_str("login_name=");
            s.push_str(&self.login_name);
            s.push('&');
        }
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}")).expect("writing to string can't fail");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}")).expect("writing to string can't fail");
            s.push('&');
        }
        s
    }
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateUserOption {
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    pub email: String,
    pub full_name: Option<String>,
    pub login_name: Option<String>,
    pub must_change_password: bool,
    pub password: String,
    pub restricted: bool,
    pub send_notify: bool,
    pub source_id: Option<u64>,
    pub username: String,
    pub visibility: String,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct EditUserOption {
    pub active: Option<bool>,
    pub admin: Option<bool>,
    pub allow_create_organization: Option<bool>,
    pub allow_git_hook: Option<bool>,
    pub allow_import_local: Option<bool>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub location: Option<String>,
    pub login_name: Option<String>,
    pub max_repo_creation: Option<u64>,
    pub must_change_password: Option<bool>,
    pub password: Option<String>,
    pub prohibit_login: Option<bool>,
    pub restricted: Option<bool>,
    pub source_id: Option<u64>,
    pub visibility: Option<String>,
    pub website: Option<String>,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateKeyOption {
    pub key: String,
    pub read_only: Option<bool>,
    pub title: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PublicKey {
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub fingerprint: String,
    pub id: u64,
    pub key: String,
    pub key_type: String,
    pub read_only: bool,
    pub title: String,
    pub url: Option<Url>,
    pub user: User,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct CreateOrgOption {
    pub description: Option<String>,
    pub full_name: Option<String>,
    pub location: Option<String>,
    pub repo_admin_change_team_access: Option<bool>,
    pub username: String,
    pub visibility: OrgVisibility,
    pub website: Option<Url>,
}

#[derive(serde::Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrgVisibility {
    Public,
    Limited,
    Private,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct RenameUserOption {
    pub new_username: String,
}
