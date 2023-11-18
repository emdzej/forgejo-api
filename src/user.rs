use super::*;

/// User operations.
impl Forgejo {
    /// Returns info about the authorized user.
    pub async fn myself(&self) -> Result<User, ForgejoError> {
        self.get("user").await
    }

    /// Returns info about the specified user.
    pub async fn get_user(&self, user: &str) -> Result<Option<User>, ForgejoError> {
        self.get_opt(&format!("users/{user}/")).await
    }

    /// Gets the list of users that follow the specified user.
    pub async fn get_followers(&self, user: &str) -> Result<Option<Vec<User>>, ForgejoError> {
        self.get_opt(&format!("users/{user}/followers/")).await
    }

    /// Gets the list of users the specified user is following.
    pub async fn get_following(&self, user: &str) -> Result<Option<Vec<User>>, ForgejoError> {
        self.get_opt(&format!("users/{user}/following/")).await
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct User {
    pub active: bool,
    pub avatar_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub created: time::OffsetDateTime,
    pub description: String,
    pub email: String,
    pub followers_count: u64,
    pub following_count: u64,
    pub full_name: String,
    pub id: u64,
    pub is_admin: bool,
    pub language: String,
    #[serde(with = "time::serde::rfc3339")]
    pub last_login: time::OffsetDateTime,
    pub location: String,
    pub login: String,
    pub login_name: String,
    pub prohibit_login: bool,
    pub restricted: bool,
    pub starred_repos_count: u64,
    pub website: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub enum UserVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "private")]
    Private,
}
