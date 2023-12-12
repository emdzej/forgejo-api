use super::*;

impl Forgejo {
    pub async fn get_gitignore_templates(&self) -> Result<Vec<String>, ForgejoError> {
        self.get("gitignore/templates").await
    }

    pub async fn get_gitignore_template(
        &self,
        name: &str,
    ) -> Result<Option<GitignoreTemplateInfo>, ForgejoError> {
        self.get_opt(&format!("gitignore/templates/{name}")).await
    }

    pub async fn get_label_templates(&self) -> Result<Vec<String>, ForgejoError> {
        self.get("label/templates").await
    }

    pub async fn get_label_template(&self, name: &str) -> Result<Vec<LabelTemplate>, ForgejoError> {
        self.get(&format!("label/templates/{name}")).await
    }

    pub async fn get_licenses(&self) -> Result<Vec<LicenseTemplateListEntry>, ForgejoError> {
        self.get("licenses").await
    }

    pub async fn get_license(
        &self,
        name: &str,
    ) -> Result<Option<GitignoreTemplateInfo>, ForgejoError> {
        self.get_opt(&format!("license/{name}")).await
    }

    pub async fn render_markdown(&self, opt: MarkdownOption) -> Result<String, ForgejoError> {
        self.post_str_out("markdown", &opt).await
    }

    pub async fn render_markdown_raw(&self, body: String) -> Result<String, ForgejoError> {
        self.post_raw("markdown/raw", body).await
    }

    pub async fn render_markup(&self, opt: MarkupOption) -> Result<String, ForgejoError> {
        self.post_str_out("markup", &opt).await
    }

    pub async fn nodeinfo(&self) -> Result<NodeInfo, ForgejoError> {
        self.get("nodeinfo").await
    }

    pub async fn signing_key(&self) -> Result<String, ForgejoError> {
        self.get_str("signing-key.gpg").await
    }

    pub async fn version(&self) -> Result<ServerVersion, ForgejoError> {
        self.get("version").await
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct GitignoreTemplateInfo {
    pub name: String,
    pub source: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct LabelTemplate {
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    pub name: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct LicenseTemplateListEntry {
    pub key: String,
    pub name: String,
    pub url: Url,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct LicenseTemplateInfo {
    pub body: String,
    pub implementation: String,
    pub key: String,
    pub name: String,
    pub url: Url,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct MarkdownOption {
    #[serde(rename = "Context")]
    pub context: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Wiki")]
    pub wiki: String,
}

#[derive(serde::Serialize, Debug, PartialEq, Default)]
pub struct MarkupOption {
    #[serde(rename = "Context")]
    pub context: String,
    #[serde(rename = "FilePath")]
    pub file_path: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Wiki")]
    pub wiki: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NodeInfo {
    pub metadata: std::collections::BTreeMap<String, String>,
    #[serde(rename = "openRegistrations")]
    pub open_registrations: bool,
    pub protocols: Vec<String>,
    pub services: NodeInfoServices,
    pub software: NodeInfoSoftware,
    pub usage: NodeInfoUsage,
    pub version: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NodeInfoServices {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NodeInfoSoftware {
    pub homepage: Url,
    pub name: String,
    pub repository: Url,
    pub version: String,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NodeInfoUsage {
    #[serde(rename = "localComments")]
    pub local_comments: u64,
    #[serde(rename = "localPosts")]
    pub local_posts: u64,
    pub users: NodeInfoUsageUsers,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NodeInfoUsageUsers {
    #[serde(rename = "activeHalfYear")]
    pub active_half_year: u64,
    #[serde(rename = "activeMonth")]
    pub active_month: u64,
    pub total: u64,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ServerVersion {
    pub version: String,
}
