use reqwest::{Client, Request, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use soft_assert::*;
use url::Url;

pub struct Forgejo {
    url: Url,
    client: Client,
}

mod misc;
mod notification;
mod organization;
mod package;
mod issue;
mod repository;
mod user;

pub use misc::*;
pub use notification::*;
pub use organization::*;
pub use package::*;
pub use issue::*;
pub use repository::*;
pub use user::*;

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
    BadStructure(#[source] serde_json::Error, String),
    #[error("unexpected status code {} {}", .0.as_u16(), .0.canonical_reason().unwrap_or(""))]
    UnexpectedStatusCode(StatusCode),
    #[error("{} {}: {}", .0.as_u16(), .0.canonical_reason().unwrap_or(""), .1)]
    ApiError(StatusCode, String),
}

impl Forgejo {
    pub fn new(api_key: &str, url: Url) -> Result<Self, ForgejoError> {
        Self::with_user_agent(api_key, url, "forgejo-api-rs")
    }

    pub fn with_user_agent(
        api_key: &str,
        url: Url,
        user_agent: &str,
    ) -> Result<Self, ForgejoError> {
        soft_assert!(
            matches!(url.scheme(), "http" | "https"),
            Err(ForgejoError::HttpRequired)
        );

        let mut headers = reqwest::header::HeaderMap::new();
        let mut key_header: reqwest::header::HeaderValue = format!("token {api_key}")
            .try_into()
            .map_err(|_| ForgejoError::KeyNotAscii)?;
        key_header.set_sensitive(true);
        headers.insert("Authorization", key_header);
        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()?;
        Ok(Self { url, client })
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.get(url).build()?;
        self.execute(request).await
    }

    async fn get_opt<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.get(url).build()?;
        self.execute_opt(request).await
    }

    async fn get_str(&self, path: &str) -> Result<String, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.get(url).build()?;
        self.execute_str(request).await
    }

    async fn post<T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<U, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.post(url).json(body).build()?;
        self.execute(request).await
    }

    async fn post_form<T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<U, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.post(url).form(body).build()?;
        self.execute(request).await
    }

    async fn post_str_out<T: Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<String, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.post(url).json(body).build()?;
        self.execute_str(request).await
    }

    async fn post_raw(
        &self,
        path: &str,
        body: String,
    ) -> Result<String, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.post(url).body(body).build()?;
        self.execute_str(request).await
    }

    async fn delete(&self, path: &str) -> Result<(), ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.delete(url).build()?;
        self.execute(request).await
    }

    async fn patch<T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<U, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.patch(url).json(body).build()?;
        self.execute(request).await
    }

    async fn put<T: DeserializeOwned>(&self, path: &str) -> Result<T, ForgejoError> {
        let url = self.url.join("api/v1/").unwrap().join(path).unwrap();
        let request = self.client.put(url).build()?;
        self.execute(request).await
    }

    async fn execute<T: DeserializeOwned>(&self, request: Request) -> Result<T, ForgejoError> {
        let response = self.client.execute(request).await?;
        match response.status() {
            status if status.is_success() => {
                let body = response.text().await?;
                let out = serde_json::from_str(&body).map_err(|e| ForgejoError::BadStructure(e, body))?;
                Ok(out)
            },
            status if status.is_client_error() => Err(ForgejoError::ApiError(
                status,
                response.json::<ErrorMessage>().await?.message.unwrap_or_else(|| String::from("[no message]")),
            )),
            status => Err(ForgejoError::UnexpectedStatusCode(status)),
        }
    }

    /// Like `execute`, but returns a `String`.
    async fn execute_str(&self, request: Request) -> Result<String, ForgejoError> {
        let response = self.client.execute(request).await?;
        match response.status() {
            status if status.is_success() => Ok(response.text().await?),
            status if status.is_client_error() => Err(ForgejoError::ApiError(
                status,
                response.json::<ErrorMessage>().await?.message.unwrap_or_else(|| String::from("[no message]")),
            )),
            status => Err(ForgejoError::UnexpectedStatusCode(status)),
        }
    }

    /// Like `execute`, but returns `Ok(None)` on 404.
    async fn execute_opt<T: DeserializeOwned>(
        &self,
        request: Request,
    ) -> Result<Option<T>, ForgejoError> {
        let response = self.client.execute(request).await?;
        match response.status() {
            status if status.is_success() => {
                let body = response.text().await?;
                let out = serde_json::from_str(&body).map_err(|e| ForgejoError::BadStructure(e, body))?;
                Ok(out)
            },
            StatusCode::NOT_FOUND => Ok(None),
            status if status.is_client_error() => Err(ForgejoError::ApiError(
                status,
                response.json::<ErrorMessage>().await?.message.unwrap_or_else(|| String::from("[no message]")),
            )),
            status => Err(ForgejoError::UnexpectedStatusCode(status)),
        }
    }
}

#[derive(serde::Deserialize)]
struct ErrorMessage {
    message: Option<String>,
    // intentionally ignored, no need for now
    // url: Url
}

// Forgejo can return blank strings for URLs. This handles that by deserializing
// that as `None`
fn none_if_blank_url<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<Url>, D::Error> {
    use serde::de::{Error, Unexpected, Visitor};
    use std::fmt;

    struct EmptyUrlVisitor;

    impl<'de> Visitor<'de> for EmptyUrlVisitor 
    {
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
            Url::parse(s).map_err(|err| {
                let err_s = format!("{}", err);
                Error::invalid_value(Unexpected::Str(s), &err_s.as_str())
            }).map(Some)
        }
    }

    deserializer.deserialize_str(EmptyUrlVisitor)
}

