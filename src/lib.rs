use std::{
    borrow::Cow, collections::VecDeque, future::Future, marker::PhantomData, pin::Pin, task::Poll,
};

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Deserializer};
use soft_assert::*;
use url::Url;
use zeroize::Zeroize;

pub struct Forgejo {
    url: Url,
    client: Client,
}

mod generated;
#[cfg(feature = "sync")]
pub mod sync;

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
    #[error("{e}")]
    Serde {
        e: serde_json::Error,
        contents: bytes::Bytes,
    },
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error("failed to find header `{0}`")]
    HeaderMissing(&'static str),
    #[error("header was not ascii")]
    HeaderNotAscii,
    #[error("failed to parse header")]
    HeaderParseFailed,
}

impl From<std::str::Utf8Error> for ForgejoError {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::BadStructure(StructureError::Utf8(error))
    }
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
    /// OAuth2 Token. Grants full access to the user's account, except for
    /// creating application access tokens.
    ///
    /// To learn how to create an OAuth2 token, see
    /// [the official Forgejo docs on the subject](https://forgejo.org/docs/latest/user/oauth2-provider).
    OAuth2(&'a str),
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

impl Auth<'_> {
    fn to_headers(&self) -> Result<reqwest::header::HeaderMap, ForgejoError> {
        let mut headers = reqwest::header::HeaderMap::new();
        match self {
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
                let unencoded_len = username.len() + password.len() + 1;
                let unpadded_len = unencoded_len
                    .checked_mul(4)
                    .ok_or(ForgejoError::AuthTooLong)?
                    .div_ceil(3);
                // round up to next multiple of 4, to account for padding
                let len = unpadded_len.div_ceil(4) * 4;
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
                        (*mfa).try_into().map_err(|_| ForgejoError::KeyNotAscii)?;
                    key_header.set_sensitive(true);
                    headers.insert("X-FORGEJO-OTP", key_header);
                }
            }
            Auth::OAuth2(token) => {
                let mut header: reqwest::header::HeaderValue = format!("Bearer {token}")
                    .try_into()
                    .map_err(|_| ForgejoError::KeyNotAscii)?;
                header.set_sensitive(true);
                headers.insert("Authorization", header);
            }
            Auth::None => (),
        }
        Ok(headers)
    }
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

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(auth.to_headers()?)
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
        Ok(self.client.execute(request).await?.bytes().await?)
    }

    /// Requests a new OAuth2 access token
    ///
    /// More info at [Forgejo's docs](https://forgejo.org/docs/latest/user/oauth2-provider).
    pub async fn oauth_get_access_token(
        &self,
        body: structs::OAuthTokenRequest<'_>,
    ) -> Result<structs::OAuthToken, ForgejoError> {
        let url = self.url.join("login/oauth/access_token").unwrap();
        let request = self.client.post(url).json(&body).build()?;
        let response = self.client.execute(request).await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json().await?),
            status if status.is_client_error() => {
                let err_msg = response
                    .json::<ErrorMessage>()
                    .await
                    .ok()
                    .map(|e| e.message);
                Err(ForgejoError::ApiError(status, err_msg))
            }
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    pub async fn send_request(&self, request: &ApiRequest) -> Result<ApiResponse, ForgejoError> {
        let mut url = self
            .url
            .join(&request.path)
            .expect("url fail. bug in forgejo-api");
        let mut query_pairs = url.query_pairs_mut();
        if let Some(query) = &request.query {
            query_pairs.extend_pairs(query.iter());
        }
        if let Some(page) = request.page {
            query_pairs.append_pair("page", &format!("{page}"));
        }
        if let Some(limit) = request.limit {
            query_pairs.append_pair("limit", &format!("{limit}"));
        }
        drop(query_pairs);
        let mut reqwest_request = self.client.request(request.method.clone(), url);
        reqwest_request = match &request.body {
            RequestBody::Json(bytes) => reqwest_request
                .body(bytes.clone())
                .header(reqwest::header::CONTENT_TYPE, "application/json"),
            RequestBody::Form(list) => {
                let mut form = reqwest::multipart::Form::new();
                for (k, v) in list {
                    form = form.part(
                        *k,
                        reqwest::multipart::Part::bytes(v.clone()).file_name("file"),
                    );
                }
                reqwest_request.multipart(form)
            }
            RequestBody::None => reqwest_request,
        };
        let mut reqwest_response = reqwest_request.send().await?;
        let response = ApiResponse {
            status_code: reqwest_response.status(),
            headers: std::mem::take(reqwest_response.headers_mut()),
            body: reqwest_response.bytes().await?,
        };
        Ok(response)
    }

    pub async fn hit_endpoint<E: Endpoint<Response = R>, R>(
        &self,
        endpoint: E,
    ) -> Result<R, ForgejoError> {
        E::handle_response(self.send_request(&endpoint.make_request()).await?)
    }
}

#[derive(serde::Deserialize)]
struct ErrorMessage {
    message: String,
    // intentionally ignored, no need for now
    // url: Url
}

pub mod structs {
    pub use crate::generated::structs::*;

    /// A Request for a new OAuth2 access token
    ///
    /// More info at [Forgejo's docs](https://forgejo.org/docs/latest/user/oauth2-provider).
    #[derive(serde::Serialize)]
    #[serde(tag = "grant_type")]
    pub enum OAuthTokenRequest<'a> {
        /// Request for getting an access code for a confidential app
        ///
        /// The `code` field must have come from sending the user to
        /// `/login/oauth/authorize` in their browser
        #[serde(rename = "authorization_code")]
        Confidential {
            client_id: &'a str,
            client_secret: &'a str,
            code: &'a str,
            redirect_uri: url::Url,
        },
        /// Request for getting an access code for a public app
        ///
        /// The `code` field must have come from sending the user to
        /// `/login/oauth/authorize` in their browser
        #[serde(rename = "authorization_code")]
        Public {
            client_id: &'a str,
            code_verifier: &'a str,
            code: &'a str,
            redirect_uri: url::Url,
        },
        /// Request for refreshing an access code
        #[serde(rename = "refresh_token")]
        Refresh {
            refresh_token: &'a str,
            client_id: &'a str,
            client_secret: &'a str,
        },
    }

    #[derive(serde::Deserialize)]
    pub struct OAuthToken {
        pub access_token: String,
        pub refresh_token: String,
        pub token_type: String,
        /// Number of seconds until the access token expires.
        pub expires_in: u32,
    }
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
        .map(|res| res.map_err(DE::custom))
        .transpose()
        .or(Ok(None))
}

fn requested_reviewers_ignore_null<'de, D, DE>(
    deserializer: D,
) -> Result<Option<Vec<structs::User>>, DE>
where
    D: Deserializer<'de>,
    DE: serde::de::Error,
{
    let list: Option<Vec<Option<structs::User>>> =
        Option::deserialize(deserializer).map_err(DE::custom)?;
    Ok(list.map(|list| list.into_iter().flatten().collect::<Vec<_>>()))
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

#[test]
fn ssh_url_deserialization() {
    #[derive(serde::Deserialize)]
    struct SshUrl {
        #[serde(deserialize_with = "deserialize_ssh_url")]
        url: url::Url,
    }
    let full_url = r#"{ "url": "ssh://git@codeberg.org/Cyborus/forgejo-api" }"#;
    let ssh_url = r#"{ "url": "git@codeberg.org:Cyborus/forgejo-api" }"#;

    let full_url_de =
        serde_json::from_str::<SshUrl>(full_url).expect("failed to deserialize full url");
    let ssh_url_de =
        serde_json::from_str::<SshUrl>(ssh_url).expect("failed to deserialize ssh url");

    let expected = "ssh://git@codeberg.org/Cyborus/forgejo-api";
    assert_eq!(full_url_de.url.as_str(), expected);
    assert_eq!(ssh_url_de.url.as_str(), expected);

    #[derive(serde::Deserialize)]
    struct OptSshUrl {
        #[serde(deserialize_with = "deserialize_optional_ssh_url")]
        url: Option<url::Url>,
    }
    let null_url = r#"{ "url": null }"#;

    let full_url_de = serde_json::from_str::<OptSshUrl>(full_url)
        .expect("failed to deserialize optional full url");
    let ssh_url_de =
        serde_json::from_str::<OptSshUrl>(ssh_url).expect("failed to deserialize optional ssh url");
    let null_url_de =
        serde_json::from_str::<OptSshUrl>(null_url).expect("failed to deserialize null url");

    let expected = Some("ssh://git@codeberg.org/Cyborus/forgejo-api");
    assert_eq!(full_url_de.url.as_ref().map(|u| u.as_ref()), expected);
    assert_eq!(ssh_url_de.url.as_ref().map(|u| u.as_ref()), expected);
    assert!(null_url_de.url.is_none());
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

mod sealed {
    pub trait Sealed {}
}

pub trait Endpoint: sealed::Sealed {
    type Response;
    fn make_request(self) -> ApiRequest;
    fn handle_response(response: ApiResponse) -> Result<Self::Response, ForgejoError>;
}

#[derive(Clone)]
pub struct ApiRequest {
    method: reqwest::Method,
    path: Cow<'static, str>,
    query: Option<Vec<(&'static str, String)>>,
    body: RequestBody,
    page: Option<u32>,
    limit: Option<u32>,
}

impl ApiRequest {
    pub(crate) fn wrap<E: Endpoint>(self, client: &Forgejo) -> WrappedRequest<'_, E> {
        WrappedRequest {
            inner: TypedRequest {
                inner: self,
                __endpoint: PhantomData,
            },
            client,
        }
    }

    #[cfg(feature = "sync")]
    pub(crate) fn wrap_sync<E: Endpoint>(
        self,
        client: &sync::Forgejo,
    ) -> sync::WrappedRequest<'_, E> {
        sync::WrappedRequest {
            inner: TypedRequest {
                inner: self,
                __endpoint: PhantomData,
            },
            client,
        }
    }
}

#[derive(Clone)]
pub enum RequestBody {
    Json(bytes::Bytes),
    Form(Vec<(&'static str, Vec<u8>)>),
    None,
}

pub struct TypedRequest<E> {
    inner: ApiRequest,
    __endpoint: PhantomData<*const E>,
}

impl<E: Endpoint> TypedRequest<E> {
    async fn send(&self, client: &Forgejo) -> Result<E::Response, ForgejoError> {
        E::handle_response(client.send_request(&self.inner).await?)
    }

    #[cfg(feature = "sync")]
    fn send_sync(&self, client: &sync::Forgejo) -> Result<E::Response, ForgejoError> {
        E::handle_response(client.send_request(&self.inner)?)
    }
}

pub struct ApiResponse {
    status_code: StatusCode,
    headers: reqwest::header::HeaderMap,
    body: bytes::Bytes,
}

pub struct WrappedRequest<'a, E> {
    inner: TypedRequest<E>,
    client: &'a Forgejo,
}

impl<'a, E: Endpoint<Response = R>, R> WrappedRequest<'a, E> {
    pub async fn send(self) -> Result<R, ForgejoError> {
        self.inner.send(&self.client).await
    }

    pub fn page(mut self, page: u32) -> Self {
        self.inner.inner.page = Some(page);
        self
    }

    pub fn page_size(mut self, limit: u32) -> Self {
        self.inner.inner.limit = Some(limit);
        self
    }
}

pub trait CountHeader: sealed::Sealed {
    fn count(&self) -> Option<usize>;
}

impl<'a, E: Endpoint<Response = (H, Vec<R>)>, R, H: CountHeader> WrappedRequest<'a, E> {
    pub fn stream(self) -> ResponseStream<'a, E, R, H> {
        ResponseStream {
            request: self,
            total_seen: 0,
            last_page: false,
            state: StreamState::Results(Default::default()),
        }
    }
}

impl<'a, E: Endpoint<Response = R>, R> std::future::IntoFuture for WrappedRequest<'a, E> {
    type Output = Result<R, ForgejoError>;

    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(
            async move { E::handle_response(self.client.send_request(&self.inner.inner).await?) },
        )
    }
}

pub struct ResponseStream<'a, E: Endpoint<Response = (H, Vec<T>)>, T, H> {
    request: WrappedRequest<'a, E>,
    total_seen: usize,
    last_page: bool,
    state: StreamState<'a, T, H>,
}

enum StreamState<'a, T, H> {
    Results(VecDeque<T>),
    Future(Pin<Box<dyn Future<Output = Result<(H, Vec<T>), ForgejoError>> + 'a>>),
    Finished,
}

impl<'a, E: Endpoint<Response = (H, Vec<T>)>, T, H: CountHeader> futures::stream::Stream
    for ResponseStream<'a, E, T, H>
where
    Self: Unpin + 'a,
{
    type Item = Result<T, ForgejoError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match &mut self.state {
            StreamState::Results(list) => match list.pop_front() {
                Some(ret) => Poll::Ready(Some(Ok(ret))),
                None => {
                    if self.last_page {
                        self.state = StreamState::Finished;
                        Poll::Ready(None)
                    } else {
                        let page = match self.request.inner.inner.page {
                            Some(n) => n + 1,
                            None => 1,
                        };
                        self.request.inner.inner.page = Some(page);

                        let request = self.request.inner.inner.clone();
                        let client = self.request.client;
                        let fut = Box::pin(async move {
                            E::handle_response(client.send_request(&request).await?)
                        });
                        self.state = StreamState::Future(fut);

                        cx.waker().wake_by_ref();

                        Poll::Pending
                    }
                }
            },
            StreamState::Future(fut) => match fut.as_mut().poll(cx) {
                Poll::Ready(res) => match res {
                    Ok((headers, list)) => {
                        let total_count = match headers.count() {
                            Some(n) => n,
                            None => {
                                self.state = StreamState::Finished;
                                return Poll::Ready(Some(Err(StructureError::HeaderMissing(
                                    "x-total-count",
                                )
                                .into())));
                            }
                        };
                        self.total_seen += list.len();
                        if self.total_seen >= total_count {
                            self.last_page = true;
                        }
                        self.state = StreamState::Results(list.into());
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Err(e) => {
                        self.state = StreamState::Finished;
                        Poll::Ready(Some(Err(e)))
                    }
                },
                Poll::Pending => Poll::Pending,
            },
            StreamState::Finished => Poll::Ready(None),
        }
    }
}
