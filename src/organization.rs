use crate::*;
use std::collections::BTreeMap;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Organization {
    #[serde(deserialize_with = "crate::none_if_blank_url")]
    pub avatar_url: Option<Url>,
    pub description: String,
    pub full_name: String,
    pub id: u64,
    pub location: Option<String>,
    pub name: String,
    pub repo_admin_change_team_access: bool,
    pub visibility: String,
    #[serde(deserialize_with = "crate::none_if_blank_url")]
    pub website: Option<Url>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Team {
    pub can_create_org_repo: bool,
    pub description: String,
    pub id: u64,
    pub includes_all_repositories: bool,
    pub name: String,
    pub organization: Organization,
    pub permission: String,
    pub units: Vec<String>,
    pub units_map: BTreeMap<String, String>,
}
