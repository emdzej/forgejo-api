//! Blocking (non-async) API

use super::{
    response_type::*, structs, ApiRequest, ApiResponse, Auth, Endpoint, ErrorMessage, ForgejoError,
    FromResponse, RequestBody, TypedRequest,
};
use reqwest::blocking::{Client, Request};
use soft_assert::soft_assert;
use url::Url;

pub struct Forgejo {
    url: Url,
    client: Client,
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

    pub fn download_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: u64,
        attach: u64,
    ) -> Result<bytes::Bytes, ForgejoError> {
        let release = self
            .repo_get_release_attachment(owner, repo, release, attach)
            .send()?;
        let mut url = self.url.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .extend(["attachments", &release.uuid.unwrap().to_string()]);
        let request = self.client.get(url).build()?;
        Ok(self.client.execute(request)?.bytes()?)
    }

    /// Requests a new OAuth2 access token
    ///
    /// More info at [Forgejo's docs](https://forgejo.org/docs/latest/user/oauth2-provider).
    pub fn oauth_get_access_token(
        &self,
        body: structs::OAuthTokenRequest<'_>,
    ) -> Result<structs::OAuthToken, ForgejoError> {
        let url = self.url.join("login/oauth/access_token").unwrap();
        let request = self.client.post(url).json(&body).build()?;
        let response = self.client.execute(request)?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json()?),
            status if status.is_client_error() => {
                let err_msg = response.json::<ErrorMessage>().ok().map(|e| e.message);
                Err(ForgejoError::ApiError(status, err_msg))
            }
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    pub fn send_request(&self, request: &ApiRequest) -> Result<ApiResponse, ForgejoError> {
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
                let mut form = reqwest::blocking::multipart::Form::new();
                for (k, v) in list {
                    form = form.part(
                        *k,
                        reqwest::blocking::multipart::Part::bytes(v.clone()).file_name("file"),
                    );
                }
                reqwest_request.multipart(form)
            }
            RequestBody::None => reqwest_request,
        };
        let mut reqwest_response = reqwest_request.send()?;
        let response = ApiResponse {
            status_code: reqwest_response.status(),
            headers: std::mem::take(reqwest_response.headers_mut()),
            body: reqwest_response.bytes()?,
        };
        Ok(response)
    }

    pub fn hit_endpoint<E: Endpoint, R: FromResponse>(
        &self,
        endpoint: E,
    ) -> Result<R::Output, ForgejoError> {
        let (response, has_body) = E::handle_error(self.send_request(&endpoint.make_request())?)?;
        Ok(R::from_response(response, has_body)?)
    }
}

pub struct WrappedRequest<'a, E, R> {
    pub(crate) inner: TypedRequest<E, R>,
    pub(crate) client: &'a Forgejo,
}

macro_rules! response_type_method_inner {
    ($self:ident) => {
        WrappedRequest {
            inner: TypedRequest {
                inner: $self.inner.inner,
                __endpoint: std::marker::PhantomData,
                __response: std::marker::PhantomData,
            },
            client: $self.client,
        }
    };
}

macro_rules! response_type_method {
    ($name:ident -> $response:ty) => {
        pub fn $name(self) -> WrappedRequest<'a, E, $response> {
            response_type_method_inner!(self)
        }
    };
    ($name:ident<T> -> $response:ty) => {
        pub fn $name<T: serde::de::DeserializeOwned>(self) -> WrappedRequest<'a, E, $response> {
            response_type_method_inner!(self)
        }
    };
    ($name:ident<H> -> $response:ty) => {
        pub fn $name<H: for<'h> TryFrom<&'h reqwest::header::HeaderMap>>(
            self,
        ) -> WrappedRequest<'a, E, $response> {
            response_type_method_inner!(self)
        }
    };
    ($name:ident<T, H> -> $response:ty) => {
        pub fn $name<
            T: serde::de::DeserializeOwned,
            H: for<'h> TryFrom<&'h reqwest::header::HeaderMap>,
        >(
            self,
        ) -> WrappedRequest<'a, E, $response> {
            response_type_method_inner!(self)
        }
    };
}

impl<'a, E: Endpoint, R: FromResponse> WrappedRequest<'a, E, R> {
    pub fn send(self) -> Result<R::Output, ForgejoError> {
        self.inner.send_sync(&self.client)
    }

    response_type_method!(json<T> -> Json<T>);
    response_type_method!(json_opt<T> -> Option<Json<T>>);
    response_type_method!(text -> Text);
    response_type_method!(bytes -> Bytes);
    response_type_method!(text_opt -> Option<Text>);
    response_type_method!(bytes_opt -> Option<Text>);

    response_type_method!(headers_json<T, H> -> WithHeaders<Json<T>, H>);
    response_type_method!(headers_json_opt<T, H> -> WithHeaders<Option<Json<T>>, H>);
    response_type_method!(headers_text<H> -> WithHeaders<Text, H>);
    response_type_method!(headers_bytes<H> -> WithHeaders<Bytes, H>);
    response_type_method!(headers_text_opt<H> -> WithHeaders<Option<Text>, H>);
    response_type_method!(headers_bytes_opt<H> -> WithHeaders<Option<Text>, H>);

    pub fn page(mut self, page: u32) -> Self {
        self.inner.inner.page = Some(page);
        self
    }

    pub fn page_size(mut self, limit: u32) -> Self {
        self.inner.inner.limit = Some(limit);
        self
    }
}
