use reqwest::{Client, Request, StatusCode};
use serde::{Deserialize, Deserializer};
use soft_assert::*;
use url::Url;
use zeroize::Zeroize;

pub struct Forgejo {
    url: Url,
    client: Client,
}

mod generated;

pub use generated::structs;

#[derive(thiserror::Error, Debug)]
pub enum ForgejoError {
    #[error("url must have a host")]
    HostRequired,
    #[error("scheme must be http or https")]
    HttpRequired,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("API key should be ascii")]
    KeyNotAscii,
    #[error("the response from forgejo was not properly structured")]
    BadStructure(#[from] StructureError),
    #[error("unexpected status code {} {}", .0.as_u16(), .0.canonical_reason().unwrap_or(""))]
    UnexpectedStatusCode(StatusCode),
    #[error("{} {}{}", .0.as_u16(), .0.canonical_reason().unwrap_or(""), .1.as_ref().map(|s| format!(": {s}")).unwrap_or_default())]
    ApiError(StatusCode, Option<String>),
    #[error("the provided authorization was too long to accept")]
    AuthTooLong,
}

#[derive(thiserror::Error, Debug)]
pub enum StructureError {
    #[error("{contents}")]
    Serde {
        e: serde_json::Error,
        contents: String,
    },
    #[error("failed to find header `{0}`")]
    HeaderMissing(&'static str),
    #[error("header was not ascii")]
    HeaderNotAscii,
    #[error("failed to parse header")]
    HeaderParseFailed,
}

/// Method of authentication to connect to the Forgejo host with.
pub enum Auth<'a> {
    /// Application Access Token. Grants access to scope enabled for the
    /// provided token, which may include full access.
    ///
    /// To learn how to create a token, see
    /// [the Codeberg docs on the subject](https://docs.codeberg.org/advanced/access-token/).
    ///
    /// To learn about token scope, see
    /// [the official Forgejo docs](https://forgejo.org/docs/latest/user/token-scope/).
    Token(&'a str),
    /// Username, password, and 2-factor auth code (if enabled). Grants full
    /// access to the user's account.
    Password {
        username: &'a str,
        password: &'a str,
        mfa: Option<&'a str>,
    },
    /// No authentication. Only grants access to access public endpoints.
    None,
}

impl Forgejo {
    pub fn new(auth: Auth, url: Url) -> Result<Self, ForgejoError> {
        Self::with_user_agent(auth, url, "forgejo-api-rs")
    }

    pub fn with_user_agent(auth: Auth, url: Url, user_agent: &str) -> Result<Self, ForgejoError> {
        soft_assert!(
            matches!(url.scheme(), "http" | "https"),
            Err(ForgejoError::HttpRequired)
        );

        let mut headers = reqwest::header::HeaderMap::new();
        match auth {
            Auth::Token(token) => {
                let mut header: reqwest::header::HeaderValue = format!("token {token}")
                    .try_into()
                    .map_err(|_| ForgejoError::KeyNotAscii)?;
                header.set_sensitive(true);
                headers.insert("Authorization", header);
            }
            Auth::Password {
                username,
                password,
                mfa,
            } => {
                let len = (((username.len() + password.len() + 1)
                    .checked_mul(4)
                    .ok_or(ForgejoError::AuthTooLong)?)
                    / 3)
                    + 1;
                let mut bytes = vec![0; len];

                // panic safety: len cannot be zero
                let mut encoder = base64ct::Encoder::<base64ct::Base64>::new(&mut bytes).unwrap();

                // panic safety: len will always be enough
                encoder.encode(username.as_bytes()).unwrap();
                encoder.encode(b":").unwrap();
                encoder.encode(password.as_bytes()).unwrap();

                let b64 = encoder.finish().unwrap();

                let mut header: reqwest::header::HeaderValue =
                    format!("Basic {b64}").try_into().unwrap(); // panic safety: base64 is always ascii
                header.set_sensitive(true);
                headers.insert("Authorization", header);

                bytes.zeroize();

                if let Some(mfa) = mfa {
                    let mut key_header: reqwest::header::HeaderValue =
                        mfa.try_into().map_err(|_| ForgejoError::KeyNotAscii)?;
                    key_header.set_sensitive(true);
                    headers.insert("X-FORGEJO-OTP", key_header);
                }
            }
            Auth::None => (),
        }
        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()?;
        Ok(Self { url, client })
    }

    pub async fn download_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: u64,
        attach: u64,
    ) -> Result<bytes::Bytes, ForgejoError> {
        let release = self
            .repo_get_release_attachment(owner, repo, release, attach)
            .await?;
        let mut url = self.url.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .extend(["attachments", &release.uuid.unwrap().to_string()]);
        let request = self.client.get(url).build()?;
        Ok(self.execute(request).await?.bytes().await?)
    }

    fn get(&self, path: &str) -> reqwest::RequestBuilder {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        self.client.get(url)
    }

    fn put(&self, path: &str) -> reqwest::RequestBuilder {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        self.client.put(url)
    }

    fn post(&self, path: &str) -> reqwest::RequestBuilder {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        self.client.post(url)
    }

    fn delete(&self, path: &str) -> reqwest::RequestBuilder {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        self.client.delete(url)
    }

    fn patch(&self, path: &str) -> reqwest::RequestBuilder {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        self.client.patch(url)
    }

    async fn execute(&self, request: Request) -> Result<reqwest::Response, ForgejoError> {
        let response = self.client.execute(request).await?;
        match response.status() {
            status if status.is_success() => Ok(response),
            status if status.is_client_error() => {
                Err(ForgejoError::ApiError(status, maybe_err(response).await))
            }
            status => Err(ForgejoError::UnexpectedStatusCode(status)),
        }
    }
}

async fn maybe_err(res: reqwest::Response) -> Option<String> {
    res.json::<ErrorMessage>().await.ok().map(|e| e.message)
}

#[derive(serde::Deserialize)]
struct ErrorMessage {
    message: String,
    // intentionally ignored, no need for now
    // url: Url
}

// Forgejo can return blank strings for URLs. This handles that by deserializing
// that as `None`
fn none_if_blank_url<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Url>, D::Error> {
    use serde::de::{Error, Unexpected, Visitor};
    use std::fmt;

    struct EmptyUrlVisitor;

    impl<'de> Visitor<'de> for EmptyUrlVisitor {
        type Value = Option<Url>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("option")
        }

        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        #[inline]
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        #[inline]
        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if s.is_empty() {
                return Ok(None);
            }
            Url::parse(s)
                .map_err(|err| {
                    let err_s = format!("{}", err);
                    Error::invalid_value(Unexpected::Str(s), &err_s.as_str())
                })
                .map(Some)
        }
    }

    deserializer.deserialize_str(EmptyUrlVisitor)
}

#[allow(dead_code)] // not used yet, but it might appear in the future
fn deserialize_ssh_url<'de, D, DE>(deserializer: D) -> Result<Url, DE>
where
    D: Deserializer<'de>,
    DE: serde::de::Error,
{
    let raw_url: String = String::deserialize(deserializer).map_err(DE::custom)?;
    parse_ssh_url(&raw_url).map_err(DE::custom)
}

fn deserialize_optional_ssh_url<'de, D, DE>(deserializer: D) -> Result<Option<Url>, DE>
where
    D: Deserializer<'de>,
    DE: serde::de::Error,
{
    let raw_url: Option<String> = Option::deserialize(deserializer).map_err(DE::custom)?;
    raw_url
        .as_ref()
        .map(parse_ssh_url)
        .map(|res| res.map_err(DE::custom).map(Some))
        .unwrap_or(Ok(None))
}

fn parse_ssh_url(raw_url: &String) -> Result<Url, url::ParseError> {
    // in case of a non-standard ssh-port (not 22), the ssh url coming from the forgejo API
    // is actually parseable by the url crate, so try to do that first
    Url::parse(raw_url).or_else(|_| {
        // otherwise the ssh url is not parseable by the url crate and we try again after some
        // pre-processing
        let url = format!("ssh://{url}", url = raw_url.replace(":", "/"));
        Url::parse(url.as_str())
    })
}

impl From<structs::DefaultMergeStyle> for structs::MergePullRequestOptionDo {
    fn from(value: structs::DefaultMergeStyle) -> Self {
        match value {
            structs::DefaultMergeStyle::Merge => structs::MergePullRequestOptionDo::Merge,
            structs::DefaultMergeStyle::Rebase => structs::MergePullRequestOptionDo::Rebase,
            structs::DefaultMergeStyle::RebaseMerge => {
                structs::MergePullRequestOptionDo::RebaseMerge
            }
            structs::DefaultMergeStyle::Squash => structs::MergePullRequestOptionDo::Squash,
            structs::DefaultMergeStyle::FastForwardOnly => {
                structs::MergePullRequestOptionDo::FastForwardOnly
            }
        }
    }
}
