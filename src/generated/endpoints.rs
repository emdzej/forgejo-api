use super::structs::*;
use crate::sealed::Sealed;
use crate::Endpoint;
use crate::{ApiError, ForgejoError, StructureError};
use crate::{ApiRequest, ApiResponse, RequestBody};
use bytes::Bytes;
use reqwest::Method;
use std::collections::BTreeMap;

fn json_error<E: Into<crate::ApiError> + serde::de::DeserializeOwned>(
    response: &ApiResponse,
) -> ForgejoError {
    let error =
        serde_json::from_slice::<E>(&response.body[..]).map_err(|e| StructureError::Serde {
            e,
            contents: response.body.clone(),
        });
    match error {
        Ok(error) => ForgejoError::ApiError(error.into()),
        Err(error) => ForgejoError::from(error),
    }
}
pub struct ActivitypubInstanceActor {}

impl Sealed for ActivitypubInstanceActor {}
impl Endpoint for ActivitypubInstanceActor {
    type Response = ActivityPub;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/activitypub/actor".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubInstanceActorInbox {}

impl Sealed for ActivitypubInstanceActorInbox {}
impl Endpoint for ActivitypubInstanceActorInbox {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/activitypub/actor/inbox".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubRepository {
    pub repository_id: i64,
}

impl Sealed for ActivitypubRepository {}
impl Endpoint for ActivitypubRepository {
    type Response = ActivityPub;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/activitypub/repository-id/{}", self.repository_id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubRepositoryInbox {
    pub repository_id: i64,
    pub body: ForgeLike,
}

impl Sealed for ActivitypubRepositoryInbox {}
impl Endpoint for ActivitypubRepositoryInbox {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/activitypub/repository-id/{}/inbox",
                self.repository_id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubPerson {
    pub user_id: i64,
}

impl Sealed for ActivitypubPerson {}
impl Endpoint for ActivitypubPerson {
    type Response = ActivityPub;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/activitypub/user-id/{}", self.user_id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubPersonActivityNote {
    pub user_id: u32,
    pub activity_id: u32,
}

impl Sealed for ActivitypubPersonActivityNote {}
impl Endpoint for ActivitypubPersonActivityNote {
    type Response = ActivityPub;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/activitypub/user-id/{}/activities/{}",
                self.user_id, self.activity_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubPersonActivity {
    pub user_id: u32,
    pub activity_id: u32,
}

impl Sealed for ActivitypubPersonActivity {}
impl Endpoint for ActivitypubPersonActivity {
    type Response = ActivityPub;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/activitypub/user-id/{}/activities/{}/activity",
                self.user_id, self.activity_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubPersonInbox {
    pub user_id: i64,
}

impl Sealed for ActivitypubPersonInbox {}
impl Endpoint for ActivitypubPersonInbox {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/activitypub/user-id/{}/inbox", self.user_id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            202 => Ok((response, false)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ActivitypubPersonFeed {
    pub user_id: u32,
}

impl Sealed for ActivitypubPersonFeed {}
impl Endpoint for ActivitypubPersonFeed {
    type Response = Vec<APPersonFollowItem>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/activitypub/user-id/{}/outbox", self.user_id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCronList {}

impl Sealed for AdminCronList {}
impl Endpoint for AdminCronList {
    type Response = (CronListHeaders, Vec<Cron>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/cron".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCronRun<'a> {
    pub task: &'a str,
}

impl Sealed for AdminCronRun<'_> {}
impl Endpoint for AdminCronRun<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/cron/{}", self.task).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetAllEmails {}

impl Sealed for AdminGetAllEmails {}
impl Endpoint for AdminGetAllEmails {
    type Response = Vec<Email>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/emails".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminSearchEmails {
    pub query: AdminSearchEmailsQuery,
}

impl Sealed for AdminSearchEmails {}
impl Endpoint for AdminSearchEmails {
    type Response = Vec<Email>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/emails/search".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminListHooks {}

impl Sealed for AdminListHooks {}
impl Endpoint for AdminListHooks {
    type Response = (HookListHeaders, Vec<Hook>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/hooks".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreateHook {
    pub body: CreateHookOption,
}

impl Sealed for AdminCreateHook {}
impl Endpoint for AdminCreateHook {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/admin/hooks".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetHook {
    pub id: i64,
}

impl Sealed for AdminGetHook {}
impl Endpoint for AdminGetHook {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/admin/hooks/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminDeleteHook {
    pub id: i64,
}

impl Sealed for AdminDeleteHook {}
impl Endpoint for AdminDeleteHook {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/admin/hooks/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminEditHook {
    pub id: i64,
    pub body: EditHookOption,
}

impl Sealed for AdminEditHook {}
impl Endpoint for AdminEditHook {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/admin/hooks/{}", self.id).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetAllOrgs {}

impl Sealed for AdminGetAllOrgs {}
impl Endpoint for AdminGetAllOrgs {
    type Response = (OrganizationListHeaders, Vec<Organization>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/orgs".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminListQuotaGroups {}

impl Sealed for AdminListQuotaGroups {}
impl Endpoint for AdminListQuotaGroups {
    type Response = (QuotaGroupListHeaders, Vec<QuotaGroup>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/quota/groups".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreateQuotaGroup {
    pub body: CreateQuotaGroupOptions,
}

impl Sealed for AdminCreateQuotaGroup {}
impl Endpoint for AdminCreateQuotaGroup {
    type Response = QuotaGroup;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/admin/quota/groups".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetQuotaGroup<'a> {
    pub quotagroup: &'a str,
}

impl Sealed for AdminGetQuotaGroup<'_> {}
impl Endpoint for AdminGetQuotaGroup<'_> {
    type Response = QuotaGroup;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/admin/quota/groups/{}", self.quotagroup).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminDeleteQuotaGroup<'a> {
    pub quotagroup: &'a str,
}

impl Sealed for AdminDeleteQuotaGroup<'_> {}
impl Endpoint for AdminDeleteQuotaGroup<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/admin/quota/groups/{}", self.quotagroup).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminAddRuleToQuotaGroup<'a> {
    pub quotagroup: &'a str,
    pub quotarule: &'a str,
}

impl Sealed for AdminAddRuleToQuotaGroup<'_> {}
impl Endpoint for AdminAddRuleToQuotaGroup<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/admin/quota/groups/{}/rules/{}",
                self.quotagroup, self.quotarule
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminRemoveRuleFromQuotaGroup<'a> {
    pub quotagroup: &'a str,
    pub quotarule: &'a str,
}

impl Sealed for AdminRemoveRuleFromQuotaGroup<'_> {}
impl Endpoint for AdminRemoveRuleFromQuotaGroup<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/admin/quota/groups/{}/rules/{}",
                self.quotagroup, self.quotarule
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminListUsersInQuotaGroup<'a> {
    pub quotagroup: &'a str,
}

impl Sealed for AdminListUsersInQuotaGroup<'_> {}
impl Endpoint for AdminListUsersInQuotaGroup<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/admin/quota/groups/{}/users", self.quotagroup).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminAddUserToQuotaGroup<'a> {
    pub quotagroup: &'a str,
    pub username: &'a str,
}

impl Sealed for AdminAddUserToQuotaGroup<'_> {}
impl Endpoint for AdminAddUserToQuotaGroup<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/admin/quota/groups/{}/users/{}",
                self.quotagroup, self.username
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminRemoveUserFromQuotaGroup<'a> {
    pub quotagroup: &'a str,
    pub username: &'a str,
}

impl Sealed for AdminRemoveUserFromQuotaGroup<'_> {}
impl Endpoint for AdminRemoveUserFromQuotaGroup<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/admin/quota/groups/{}/users/{}",
                self.quotagroup, self.username
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminListQuotaRules {}

impl Sealed for AdminListQuotaRules {}
impl Endpoint for AdminListQuotaRules {
    type Response = (QuotaRuleInfoListHeaders, Vec<QuotaRuleInfo>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/quota/rules".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreateQuotaRule {
    pub body: CreateQuotaRuleOptions,
}

impl Sealed for AdminCreateQuotaRule {}
impl Endpoint for AdminCreateQuotaRule {
    type Response = QuotaRuleInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/admin/quota/rules".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetQuotaRule<'a> {
    pub quotarule: &'a str,
}

impl Sealed for AdminGetQuotaRule<'_> {}
impl Endpoint for AdminGetQuotaRule<'_> {
    type Response = QuotaRuleInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/admin/quota/rules/{}", self.quotarule).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminDEleteQuotaRule<'a> {
    pub quotarule: &'a str,
}

impl Sealed for AdminDEleteQuotaRule<'_> {}
impl Endpoint for AdminDEleteQuotaRule<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/admin/quota/rules/{}", self.quotarule).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminEditQuotaRule<'a> {
    pub quotarule: &'a str,
    pub body: EditQuotaRuleOptions,
}

impl Sealed for AdminEditQuotaRule<'_> {}
impl Endpoint for AdminEditQuotaRule<'_> {
    type Response = QuotaRuleInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/admin/quota/rules/{}", self.quotarule).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminSearchRunJobs {
    pub query: AdminSearchRunJobsQuery,
}

impl Sealed for AdminSearchRunJobs {}
impl Endpoint for AdminSearchRunJobs {
    type Response = Vec<ActionRunJob>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/runners/jobs".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetRunnerRegistrationToken {}

impl Sealed for AdminGetRunnerRegistrationToken {}
impl Endpoint for AdminGetRunnerRegistrationToken {
    type Response = RegistrationToken;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/runners/registration-token".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminUnadoptedList {
    pub query: AdminUnadoptedListQuery,
}

impl Sealed for AdminUnadoptedList {}
impl Endpoint for AdminUnadoptedList {
    type Response = Vec<String>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/unadopted".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminAdoptRepository<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for AdminAdoptRepository<'_> {}
impl Endpoint for AdminAdoptRepository<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/unadopted/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminDeleteUnadoptedRepository<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for AdminDeleteUnadoptedRepository<'_> {}
impl Endpoint for AdminDeleteUnadoptedRepository<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/admin/unadopted/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminSearchUsers {
    pub query: AdminSearchUsersQuery,
}

impl Sealed for AdminSearchUsers {}
impl Endpoint for AdminSearchUsers {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/admin/users".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreateUser {
    pub body: CreateUserOption,
}

impl Sealed for AdminCreateUser {}
impl Endpoint for AdminCreateUser {
    type Response = User;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/admin/users".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminDeleteUser<'a> {
    pub username: &'a str,
    pub query: AdminDeleteUserQuery,
}

impl Sealed for AdminDeleteUser<'_> {}
impl Endpoint for AdminDeleteUser<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/admin/users/{}", self.username).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminEditUser<'a> {
    pub username: &'a str,
    pub body: EditUserOption,
}

impl Sealed for AdminEditUser<'_> {}
impl Endpoint for AdminEditUser<'_> {
    type Response = User;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/admin/users/{}", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreatePublicKey<'a> {
    pub username: &'a str,
    pub body: CreateKeyOption,
}

impl Sealed for AdminCreatePublicKey<'_> {}
impl Endpoint for AdminCreatePublicKey<'_> {
    type Response = PublicKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/users/{}/keys", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminDeleteUserPublicKey<'a> {
    pub username: &'a str,
    pub id: i64,
}

impl Sealed for AdminDeleteUserPublicKey<'_> {}
impl Endpoint for AdminDeleteUserPublicKey<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/admin/users/{}/keys/{}", self.username, self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreateOrg<'a> {
    pub username: &'a str,
    pub body: CreateOrgOption,
}

impl Sealed for AdminCreateOrg<'_> {}
impl Endpoint for AdminCreateOrg<'_> {
    type Response = Organization;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/users/{}/orgs", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminGetUserQuota<'a> {
    pub username: &'a str,
}

impl Sealed for AdminGetUserQuota<'_> {}
impl Endpoint for AdminGetUserQuota<'_> {
    type Response = QuotaInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/admin/users/{}/quota", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminSetUserQuotaGroups<'a> {
    pub username: &'a str,
    pub body: SetUserQuotaGroupsOptions,
}

impl Sealed for AdminSetUserQuotaGroups<'_> {}
impl Endpoint for AdminSetUserQuotaGroups<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/users/{}/quota/groups", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminRenameUser<'a> {
    pub username: &'a str,
    pub body: RenameUserOption,
}

impl Sealed for AdminRenameUser<'_> {}
impl Endpoint for AdminRenameUser<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/users/{}/rename", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AdminCreateRepo<'a> {
    pub username: &'a str,
    pub body: CreateRepoOption,
}

impl Sealed for AdminCreateRepo<'_> {}
impl Endpoint for AdminCreateRepo<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/admin/users/{}/repos", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListGitignoresTemplates {}

impl Sealed for ListGitignoresTemplates {}
impl Endpoint for ListGitignoresTemplates {
    type Response = Vec<String>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/gitignore/templates".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetGitignoreTemplateInfo<'a> {
    pub name: &'a str,
}

impl Sealed for GetGitignoreTemplateInfo<'_> {}
impl Endpoint for GetGitignoreTemplateInfo<'_> {
    type Response = GitignoreTemplateInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/gitignore/templates/{}", self.name).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListLabelTemplates {}

impl Sealed for ListLabelTemplates {}
impl Endpoint for ListLabelTemplates {
    type Response = Vec<String>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/label/templates".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetLabelTemplateInfo<'a> {
    pub name: &'a str,
}

impl Sealed for GetLabelTemplateInfo<'_> {}
impl Endpoint for GetLabelTemplateInfo<'_> {
    type Response = Vec<LabelTemplate>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/label/templates/{}", self.name).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListLicenseTemplates {}

impl Sealed for ListLicenseTemplates {}
impl Endpoint for ListLicenseTemplates {
    type Response = Vec<LicensesTemplateListEntry>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/licenses".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetLicenseTemplateInfo<'a> {
    pub name: &'a str,
}

impl Sealed for GetLicenseTemplateInfo<'_> {}
impl Endpoint for GetLicenseTemplateInfo<'_> {
    type Response = LicenseTemplateInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/licenses/{}", self.name).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RenderMarkdown {
    pub body: MarkdownOption,
}

impl Sealed for RenderMarkdown {}
impl Endpoint for RenderMarkdown {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/markdown".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RenderMarkdownRaw {
    pub body: String,
}

impl Sealed for RenderMarkdownRaw {}
impl Endpoint for RenderMarkdownRaw {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/markdown/raw".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RenderMarkup {
    pub body: MarkupOption,
}

impl Sealed for RenderMarkup {}
impl Endpoint for RenderMarkup {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/markup".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetNodeInfo {}

impl Sealed for GetNodeInfo {}
impl Endpoint for GetNodeInfo {
    type Response = NodeInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/nodeinfo".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyGetList {
    pub query: NotifyGetListQuery,
}

impl Sealed for NotifyGetList {}
impl Endpoint for NotifyGetList {
    type Response = (NotificationThreadListHeaders, Vec<NotificationThread>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/notifications".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyReadList {
    pub query: NotifyReadListQuery,
}

impl Sealed for NotifyReadList {}
impl Endpoint for NotifyReadList {
    type Response = (NotificationThreadListHeaders, Vec<NotificationThread>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: "/api/v1/notifications".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            205 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyNewAvailable {}

impl Sealed for NotifyNewAvailable {}
impl Endpoint for NotifyNewAvailable {
    type Response = NotificationCount;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/notifications/new".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyGetThread {
    pub id: i64,
}

impl Sealed for NotifyGetThread {}
impl Endpoint for NotifyGetThread {
    type Response = NotificationThread;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/notifications/threads/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyReadThread {
    pub id: i64,
    pub query: NotifyReadThreadQuery,
}

impl Sealed for NotifyReadThread {}
impl Endpoint for NotifyReadThread {
    type Response = NotificationThread;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/notifications/threads/{}", self.id).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            205 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateOrgRepoDeprecated<'a> {
    pub org: &'a str,
    pub body: CreateRepoOption,
}

impl Sealed for CreateOrgRepoDeprecated<'_> {}
impl Endpoint for CreateOrgRepoDeprecated<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/org/{}/repos", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetAll {}

impl Sealed for OrgGetAll {}
impl Endpoint for OrgGetAll {
    type Response = (OrganizationListHeaders, Vec<Organization>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/orgs".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgCreate {
    pub body: CreateOrgOption,
}

impl Sealed for OrgCreate {}
impl Endpoint for OrgCreate {
    type Response = Organization;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/orgs".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGet<'a> {
    pub org: &'a str,
}

impl Sealed for OrgGet<'_> {}
impl Endpoint for OrgGet<'_> {
    type Response = Organization;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgDelete<'a> {
    pub org: &'a str,
}

impl Sealed for OrgDelete<'_> {}
impl Endpoint for OrgDelete<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/orgs/{}", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgEdit<'a> {
    pub org: &'a str,
    pub body: EditOrgOption,
}

impl Sealed for OrgEdit<'_> {}
impl Endpoint for OrgEdit<'_> {
    type Response = Organization;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/orgs/{}", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgSearchRunJobs<'a> {
    pub org: &'a str,
    pub query: OrgSearchRunJobsQuery,
}

impl Sealed for OrgSearchRunJobs<'_> {}
impl Endpoint for OrgSearchRunJobs<'_> {
    type Response = Vec<ActionRunJob>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/actions/runners/jobs", self.org).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetRunnerRegistrationToken<'a> {
    pub org: &'a str,
}

impl Sealed for OrgGetRunnerRegistrationToken<'_> {}
impl Endpoint for OrgGetRunnerRegistrationToken<'_> {
    type Response = RegistrationToken;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/orgs/{}/actions/runners/registration-token",
                self.org
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListActionsSecrets<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListActionsSecrets<'_> {}
impl Endpoint for OrgListActionsSecrets<'_> {
    type Response = (SecretListHeaders, Vec<Secret>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/actions/secrets", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateOrgSecret<'a> {
    pub org: &'a str,
    pub secretname: &'a str,
    pub body: CreateOrUpdateSecretOption,
}

impl Sealed for UpdateOrgSecret<'_> {}
impl Endpoint for UpdateOrgSecret<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/orgs/{}/actions/secrets/{}",
                self.org, self.secretname
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeleteOrgSecret<'a> {
    pub org: &'a str,
    pub secretname: &'a str,
}

impl Sealed for DeleteOrgSecret<'_> {}
impl Endpoint for DeleteOrgSecret<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/orgs/{}/actions/secrets/{}",
                self.org, self.secretname
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetOrgVariablesList<'a> {
    pub org: &'a str,
}

impl Sealed for GetOrgVariablesList<'_> {}
impl Endpoint for GetOrgVariablesList<'_> {
    type Response = (VariableListHeaders, Vec<ActionVariable>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/actions/variables", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetOrgVariable<'a> {
    pub org: &'a str,
    pub variablename: &'a str,
}

impl Sealed for GetOrgVariable<'_> {}
impl Endpoint for GetOrgVariable<'_> {
    type Response = ActionVariable;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/orgs/{}/actions/variables/{}",
                self.org, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateOrgVariable<'a> {
    pub org: &'a str,
    pub variablename: &'a str,
    pub body: UpdateVariableOption,
}

impl Sealed for UpdateOrgVariable<'_> {}
impl Endpoint for UpdateOrgVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/orgs/{}/actions/variables/{}",
                self.org, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateOrgVariable<'a> {
    pub org: &'a str,
    pub variablename: &'a str,
    pub body: CreateVariableOption,
}

impl Sealed for CreateOrgVariable<'_> {}
impl Endpoint for CreateOrgVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/orgs/{}/actions/variables/{}",
                self.org, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeleteOrgVariable<'a> {
    pub org: &'a str,
    pub variablename: &'a str,
}

impl Sealed for DeleteOrgVariable<'_> {}
impl Endpoint for DeleteOrgVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/orgs/{}/actions/variables/{}",
                self.org, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListActivityFeeds<'a> {
    pub org: &'a str,
    pub query: OrgListActivityFeedsQuery,
}

impl Sealed for OrgListActivityFeeds<'_> {}
impl Endpoint for OrgListActivityFeeds<'_> {
    type Response = (ActivityFeedsListHeaders, Vec<Activity>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/activities/feeds", self.org).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgUpdateAvatar<'a> {
    pub org: &'a str,
    pub body: UpdateUserAvatarOption,
}

impl Sealed for OrgUpdateAvatar<'_> {}
impl Endpoint for OrgUpdateAvatar<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/orgs/{}/avatar", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgDeleteAvatar<'a> {
    pub org: &'a str,
}

impl Sealed for OrgDeleteAvatar<'_> {}
impl Endpoint for OrgDeleteAvatar<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/orgs/{}/avatar", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgBlockUser<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgBlockUser<'_> {}
impl Endpoint for OrgBlockUser<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/orgs/{}/block/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListHooks<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListHooks<'_> {}
impl Endpoint for OrgListHooks<'_> {
    type Response = (HookListHeaders, Vec<Hook>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/hooks", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgCreateHook<'a> {
    pub org: &'a str,
    pub body: CreateHookOption,
}

impl Sealed for OrgCreateHook<'_> {}
impl Endpoint for OrgCreateHook<'_> {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/orgs/{}/hooks", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetHook<'a> {
    pub org: &'a str,
    pub id: i64,
}

impl Sealed for OrgGetHook<'_> {}
impl Endpoint for OrgGetHook<'_> {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/hooks/{}", self.org, self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgDeleteHook<'a> {
    pub org: &'a str,
    pub id: i64,
}

impl Sealed for OrgDeleteHook<'_> {}
impl Endpoint for OrgDeleteHook<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/orgs/{}/hooks/{}", self.org, self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgEditHook<'a> {
    pub org: &'a str,
    pub id: i64,
    pub body: EditHookOption,
}

impl Sealed for OrgEditHook<'_> {}
impl Endpoint for OrgEditHook<'_> {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/orgs/{}/hooks/{}", self.org, self.id).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListLabels<'a> {
    pub org: &'a str,
    pub query: OrgListLabelsQuery,
}

impl Sealed for OrgListLabels<'_> {}
impl Endpoint for OrgListLabels<'_> {
    type Response = (LabelListHeaders, Vec<Label>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/labels", self.org).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgCreateLabel<'a> {
    pub org: &'a str,
    pub body: CreateLabelOption,
}

impl Sealed for OrgCreateLabel<'_> {}
impl Endpoint for OrgCreateLabel<'_> {
    type Response = Label;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/orgs/{}/labels", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetLabel<'a> {
    pub org: &'a str,
    pub id: i64,
}

impl Sealed for OrgGetLabel<'_> {}
impl Endpoint for OrgGetLabel<'_> {
    type Response = Label;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/labels/{}", self.org, self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgDeleteLabel<'a> {
    pub org: &'a str,
    pub id: i64,
}

impl Sealed for OrgDeleteLabel<'_> {}
impl Endpoint for OrgDeleteLabel<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/orgs/{}/labels/{}", self.org, self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgEditLabel<'a> {
    pub org: &'a str,
    pub id: i64,
    pub body: EditLabelOption,
}

impl Sealed for OrgEditLabel<'_> {}
impl Endpoint for OrgEditLabel<'_> {
    type Response = Label;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/orgs/{}/labels/{}", self.org, self.id).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListBlockedUsers<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListBlockedUsers<'_> {}
impl Endpoint for OrgListBlockedUsers<'_> {
    type Response = (BlockedUserListHeaders, Vec<BlockedUser>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/list_blocked", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListMembers<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListMembers<'_> {}
impl Endpoint for OrgListMembers<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/members", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgIsMember<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgIsMember<'_> {}
impl Endpoint for OrgIsMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/members/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgDeleteMember<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgDeleteMember<'_> {}
impl Endpoint for OrgDeleteMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/orgs/{}/members/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListPublicMembers<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListPublicMembers<'_> {}
impl Endpoint for OrgListPublicMembers<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/public_members", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgIsPublicMember<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgIsPublicMember<'_> {}
impl Endpoint for OrgIsPublicMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/public_members/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgPublicizeMember<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgPublicizeMember<'_> {}
impl Endpoint for OrgPublicizeMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/orgs/{}/public_members/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgConcealMember<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgConcealMember<'_> {}
impl Endpoint for OrgConcealMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/orgs/{}/public_members/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetQuota<'a> {
    pub org: &'a str,
}

impl Sealed for OrgGetQuota<'_> {}
impl Endpoint for OrgGetQuota<'_> {
    type Response = QuotaInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/quota", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListQuotaArtifacts<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListQuotaArtifacts<'_> {}
impl Endpoint for OrgListQuotaArtifacts<'_> {
    type Response = (QuotaUsedArtifactListHeaders, Vec<QuotaUsedArtifact>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/quota/artifacts", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListQuotaAttachments<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListQuotaAttachments<'_> {}
impl Endpoint for OrgListQuotaAttachments<'_> {
    type Response = (QuotaUsedAttachmentListHeaders, Vec<QuotaUsedAttachment>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/quota/attachments", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgCheckQuota<'a> {
    pub org: &'a str,
    pub query: OrgCheckQuotaQuery,
}

impl Sealed for OrgCheckQuota<'_> {}
impl Endpoint for OrgCheckQuota<'_> {
    type Response = bool;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/quota/check", self.org).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListQuotaPackages<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListQuotaPackages<'_> {}
impl Endpoint for OrgListQuotaPackages<'_> {
    type Response = (QuotaUsedPackageListHeaders, Vec<QuotaUsedPackage>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/quota/packages", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RenameOrg<'a> {
    pub org: &'a str,
    pub body: RenameOrgOption,
}

impl Sealed for RenameOrg<'_> {}
impl Endpoint for RenameOrg<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/orgs/{}/rename", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListRepos<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListRepos<'_> {}
impl Endpoint for OrgListRepos<'_> {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/repos", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateOrgRepo<'a> {
    pub org: &'a str,
    pub body: CreateRepoOption,
}

impl Sealed for CreateOrgRepo<'_> {}
impl Endpoint for CreateOrgRepo<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/orgs/{}/repos", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListTeams<'a> {
    pub org: &'a str,
}

impl Sealed for OrgListTeams<'_> {}
impl Endpoint for OrgListTeams<'_> {
    type Response = (TeamListHeaders, Vec<Team>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/teams", self.org).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgCreateTeam<'a> {
    pub org: &'a str,
    pub body: CreateTeamOption,
}

impl Sealed for OrgCreateTeam<'_> {}
impl Endpoint for OrgCreateTeam<'_> {
    type Response = Team;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/orgs/{}/teams", self.org).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct TeamSearch<'a> {
    pub org: &'a str,
    pub query: TeamSearchQuery,
}

impl Sealed for TeamSearch<'_> {}
impl Endpoint for TeamSearch<'_> {
    type Response = TeamSearchResults;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/orgs/{}/teams/search", self.org).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgUnblockUser<'a> {
    pub org: &'a str,
    pub username: &'a str,
}

impl Sealed for OrgUnblockUser<'_> {}
impl Endpoint for OrgUnblockUser<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/orgs/{}/unblock/{}", self.org, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListPackages<'a> {
    pub owner: &'a str,
    pub query: ListPackagesQuery,
}

impl Sealed for ListPackages<'_> {}
impl Endpoint for ListPackages<'_> {
    type Response = (PackageListHeaders, Vec<Package>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/packages/{}", self.owner).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct LinkPackage<'a> {
    pub owner: &'a str,
    pub r#type: &'a str,
    pub name: &'a str,
    pub repo_name: &'a str,
}

impl Sealed for LinkPackage<'_> {}
impl Endpoint for LinkPackage<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/packages/{}/{}/{}/-/link/{}",
                self.owner, self.r#type, self.name, self.repo_name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UnlinkPackage<'a> {
    pub owner: &'a str,
    pub r#type: &'a str,
    pub name: &'a str,
}

impl Sealed for UnlinkPackage<'_> {}
impl Endpoint for UnlinkPackage<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/packages/{}/{}/{}/-/unlink",
                self.owner, self.r#type, self.name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetPackage<'a> {
    pub owner: &'a str,
    pub r#type: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

impl Sealed for GetPackage<'_> {}
impl Endpoint for GetPackage<'_> {
    type Response = Package;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/packages/{}/{}/{}/{}",
                self.owner, self.r#type, self.name, self.version
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeletePackage<'a> {
    pub owner: &'a str,
    pub r#type: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

impl Sealed for DeletePackage<'_> {}
impl Endpoint for DeletePackage<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/packages/{}/{}/{}/{}",
                self.owner, self.r#type, self.name, self.version
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListPackageFiles<'a> {
    pub owner: &'a str,
    pub r#type: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

impl Sealed for ListPackageFiles<'_> {}
impl Endpoint for ListPackageFiles<'_> {
    type Response = (PackageFileListHeaders, Vec<PackageFile>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/packages/{}/{}/{}/{}/files",
                self.owner, self.r#type, self.name, self.version
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueSearchIssues {
    pub query: IssueSearchIssuesQuery,
}

impl Sealed for IssueSearchIssues {}
impl Endpoint for IssueSearchIssues {
    type Response = (IssueListHeaders, Vec<Issue>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/repos/issues/search".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoMigrate {
    pub body: MigrateRepoOptions,
}

impl Sealed for RepoMigrate {}
impl Endpoint for RepoMigrate {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/repos/migrate".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSearch {
    pub query: RepoSearchQuery,
}

impl Sealed for RepoSearch {}
impl Endpoint for RepoSearch {
    type Response = SearchResults;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/repos/search".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGet<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGet<'_> {}
impl Endpoint for RepoGet<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDelete<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoDelete<'_> {}
impl Endpoint for RepoDelete<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/repos/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEdit<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: EditRepoOption,
}

impl Sealed for RepoEdit<'_> {}
impl Endpoint for RepoEdit<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/repos/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSearchRunJobs<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoSearchRunJobsQuery,
}

impl Sealed for RepoSearchRunJobs<'_> {}
impl Endpoint for RepoSearchRunJobs<'_> {
    type Response = Vec<ActionRunJob>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/actions/runners/jobs",
                self.owner, self.repo
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetRunnerRegistrationToken<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetRunnerRegistrationToken<'_> {}
impl Endpoint for RepoGetRunnerRegistrationToken<'_> {
    type Response = RegistrationToken;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/actions/runners/registration-token",
                self.owner, self.repo
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListActionRuns<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: ListActionRunsQuery,
}

impl Sealed for ListActionRuns<'_> {}
impl Endpoint for ListActionRuns<'_> {
    type Response = ListActionRunResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/actions/runs", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetActionRun<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub run_id: i64,
}

impl Sealed for GetActionRun<'_> {}
impl Endpoint for GetActionRun<'_> {
    type Response = ActionRun;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/actions/runs/{}",
                self.owner, self.repo, self.run_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListActionsSecrets<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListActionsSecrets<'_> {}
impl Endpoint for RepoListActionsSecrets<'_> {
    type Response = (SecretListHeaders, Vec<Secret>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/actions/secrets", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateRepoSecret<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub secretname: &'a str,
    pub body: CreateOrUpdateSecretOption,
}

impl Sealed for UpdateRepoSecret<'_> {}
impl Endpoint for UpdateRepoSecret<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/actions/secrets/{}",
                self.owner, self.repo, self.secretname
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeleteRepoSecret<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub secretname: &'a str,
}

impl Sealed for DeleteRepoSecret<'_> {}
impl Endpoint for DeleteRepoSecret<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/actions/secrets/{}",
                self.owner, self.repo, self.secretname
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListActionTasks<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for ListActionTasks<'_> {}
impl Endpoint for ListActionTasks<'_> {
    type Response = ActionTaskResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/actions/tasks", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetRepoVariablesList<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for GetRepoVariablesList<'_> {}
impl Endpoint for GetRepoVariablesList<'_> {
    type Response = (VariableListHeaders, Vec<ActionVariable>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/actions/variables",
                self.owner, self.repo
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetRepoVariable<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub variablename: &'a str,
}

impl Sealed for GetRepoVariable<'_> {}
impl Endpoint for GetRepoVariable<'_> {
    type Response = ActionVariable;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/actions/variables/{}",
                self.owner, self.repo, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateRepoVariable<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub variablename: &'a str,
    pub body: UpdateVariableOption,
}

impl Sealed for UpdateRepoVariable<'_> {}
impl Endpoint for UpdateRepoVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/actions/variables/{}",
                self.owner, self.repo, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateRepoVariable<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub variablename: &'a str,
    pub body: CreateVariableOption,
}

impl Sealed for CreateRepoVariable<'_> {}
impl Endpoint for CreateRepoVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/actions/variables/{}",
                self.owner, self.repo, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeleteRepoVariable<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub variablename: &'a str,
}

impl Sealed for DeleteRepoVariable<'_> {}
impl Endpoint for DeleteRepoVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/actions/variables/{}",
                self.owner, self.repo, self.variablename
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DispatchWorkflow<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub workflowfilename: &'a str,
    pub body: DispatchWorkflowOption,
}

impl Sealed for DispatchWorkflow<'_> {}
impl Endpoint for DispatchWorkflow<'_> {
    type Response = Option<DispatchWorkflowRun>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/actions/workflows/{}/dispatches",
                self.owner, self.repo, self.workflowfilename
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListActivityFeeds<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoListActivityFeedsQuery,
}

impl Sealed for RepoListActivityFeeds<'_> {}
impl Endpoint for RepoListActivityFeeds<'_> {
    type Response = (ActivityFeedsListHeaders, Vec<Activity>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/activities/feeds",
                self.owner, self.repo
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetArchive<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub archive: &'a str,
}

impl Sealed for RepoGetArchive<'_> {}
impl Endpoint for RepoGetArchive<'_> {
    type Response = Bytes;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/archive/{}",
                self.owner, self.repo, self.archive
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetAssignees<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetAssignees<'_> {}
impl Endpoint for RepoGetAssignees<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/assignees", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoUpdateAvatar<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: UpdateRepoAvatarOption,
}

impl Sealed for RepoUpdateAvatar<'_> {}
impl Endpoint for RepoUpdateAvatar<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/avatar", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteAvatar<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoDeleteAvatar<'_> {}
impl Endpoint for RepoDeleteAvatar<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/repos/{}/{}/avatar", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListBranchProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListBranchProtection<'_> {}
impl Endpoint for RepoListBranchProtection<'_> {
    type Response = Vec<BranchProtection>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/branch_protections",
                self.owner, self.repo
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateBranchProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateBranchProtectionOption,
}

impl Sealed for RepoCreateBranchProtection<'_> {}
impl Endpoint for RepoCreateBranchProtection<'_> {
    type Response = BranchProtection;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/branch_protections",
                self.owner, self.repo
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetBranchProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub name: &'a str,
}

impl Sealed for RepoGetBranchProtection<'_> {}
impl Endpoint for RepoGetBranchProtection<'_> {
    type Response = BranchProtection;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/branch_protections/{}",
                self.owner, self.repo, self.name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteBranchProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub name: &'a str,
}

impl Sealed for RepoDeleteBranchProtection<'_> {}
impl Endpoint for RepoDeleteBranchProtection<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/branch_protections/{}",
                self.owner, self.repo, self.name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditBranchProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub name: &'a str,
    pub body: EditBranchProtectionOption,
}

impl Sealed for RepoEditBranchProtection<'_> {}
impl Endpoint for RepoEditBranchProtection<'_> {
    type Response = BranchProtection;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/branch_protections/{}",
                self.owner, self.repo, self.name
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListBranches<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListBranches<'_> {}
impl Endpoint for RepoListBranches<'_> {
    type Response = (BranchListHeaders, Vec<Branch>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/branches", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateBranch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateBranchRepoOption,
}

impl Sealed for RepoCreateBranch<'_> {}
impl Endpoint for RepoCreateBranch<'_> {
    type Response = Branch;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/branches", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetBranch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
}

impl Sealed for RepoGetBranch<'_> {}
impl Endpoint for RepoGetBranch<'_> {
    type Response = Branch;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/branches/{}",
                self.owner, self.repo, self.branch
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteBranch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
}

impl Sealed for RepoDeleteBranch<'_> {}
impl Endpoint for RepoDeleteBranch<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/branches/{}",
                self.owner, self.repo, self.branch
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoUpdateBranch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
    pub body: UpdateBranchRepoOption,
}

impl Sealed for RepoUpdateBranch<'_> {}
impl Endpoint for RepoUpdateBranch<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/branches/{}",
                self.owner, self.repo, self.branch
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListCollaborators<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListCollaborators<'_> {}
impl Endpoint for RepoListCollaborators<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/collaborators", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCheckCollaborator<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub collaborator: &'a str,
}

impl Sealed for RepoCheckCollaborator<'_> {}
impl Endpoint for RepoCheckCollaborator<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/collaborators/{}",
                self.owner, self.repo, self.collaborator
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoAddCollaborator<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub collaborator: &'a str,
    pub body: AddCollaboratorOption,
}

impl Sealed for RepoAddCollaborator<'_> {}
impl Endpoint for RepoAddCollaborator<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/collaborators/{}",
                self.owner, self.repo, self.collaborator
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteCollaborator<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub collaborator: &'a str,
}

impl Sealed for RepoDeleteCollaborator<'_> {}
impl Endpoint for RepoDeleteCollaborator<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/collaborators/{}",
                self.owner, self.repo, self.collaborator
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetRepoPermissions<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub collaborator: &'a str,
}

impl Sealed for RepoGetRepoPermissions<'_> {}
impl Endpoint for RepoGetRepoPermissions<'_> {
    type Response = RepoCollaboratorPermission;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/collaborators/{}/permission",
                self.owner, self.repo, self.collaborator
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetAllCommits<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoGetAllCommitsQuery,
}

impl Sealed for RepoGetAllCommits<'_> {}
impl Endpoint for RepoGetAllCommits<'_> {
    type Response = (CommitListHeaders, Vec<Commit>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/commits", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetCombinedStatusByRef<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub r#ref: &'a str,
}

impl Sealed for RepoGetCombinedStatusByRef<'_> {}
impl Endpoint for RepoGetCombinedStatusByRef<'_> {
    type Response = (CombinedStatusHeaders, CombinedStatus);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/commits/{}/status",
                self.owner, self.repo, self.r#ref
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListStatusesByRef<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub r#ref: &'a str,
    pub query: RepoListStatusesByRefQuery,
}

impl Sealed for RepoListStatusesByRef<'_> {}
impl Endpoint for RepoListStatusesByRef<'_> {
    type Response = (CommitStatusListHeaders, Vec<CommitStatus>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/commits/{}/statuses",
                self.owner, self.repo, self.r#ref
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetCommitPullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
}

impl Sealed for RepoGetCommitPullRequest<'_> {}
impl Endpoint for RepoGetCommitPullRequest<'_> {
    type Response = PullRequest;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/commits/{}/pull",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCompareDiff<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub basehead: &'a str,
}

impl Sealed for RepoCompareDiff<'_> {}
impl Endpoint for RepoCompareDiff<'_> {
    type Response = Compare;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/compare/{}",
                self.owner, self.repo, self.basehead
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetContentsList<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoGetContentsListQuery,
}

impl Sealed for RepoGetContentsList<'_> {}
impl Endpoint for RepoGetContentsList<'_> {
    type Response = Vec<ContentsResponse>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/contents", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoChangeFiles<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: ChangeFilesOptions,
}

impl Sealed for RepoChangeFiles<'_> {}
impl Endpoint for RepoChangeFiles<'_> {
    type Response = FilesResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/contents", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetContents<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub query: RepoGetContentsQuery,
}

impl Sealed for RepoGetContents<'_> {}
impl Endpoint for RepoGetContents<'_> {
    type Response = ContentsResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/contents/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoUpdateFile<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub body: UpdateFileOptions,
}

impl Sealed for RepoUpdateFile<'_> {}
impl Endpoint for RepoUpdateFile<'_> {
    type Response = FileResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/contents/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateFile<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub body: CreateFileOptions,
}

impl Sealed for RepoCreateFile<'_> {}
impl Endpoint for RepoCreateFile<'_> {
    type Response = FileResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/contents/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteFile<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub body: DeleteFileOptions,
}

impl Sealed for RepoDeleteFile<'_> {}
impl Endpoint for RepoDeleteFile<'_> {
    type Response = FileDeleteResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/contents/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoConvert<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoConvert<'_> {}
impl Endpoint for RepoConvert<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/convert", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoApplyDiffPatch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: UpdateFileOptions,
}

impl Sealed for RepoApplyDiffPatch<'_> {}
impl Endpoint for RepoApplyDiffPatch<'_> {
    type Response = FileResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/diffpatch", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetEditorConfig<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub query: RepoGetEditorConfigQuery,
}

impl Sealed for RepoGetEditorConfig<'_> {}
impl Endpoint for RepoGetEditorConfig<'_> {
    type Response = BTreeMap<String, String>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/editorconfig/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListFlags<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListFlags<'_> {}
impl Endpoint for RepoListFlags<'_> {
    type Response = Vec<String>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/flags", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoReplaceAllFlags<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: ReplaceFlagsOption,
}

impl Sealed for RepoReplaceAllFlags<'_> {}
impl Endpoint for RepoReplaceAllFlags<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/repos/{}/{}/flags", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteAllFlags<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoDeleteAllFlags<'_> {}
impl Endpoint for RepoDeleteAllFlags<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/repos/{}/{}/flags", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCheckFlag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub flag: &'a str,
}

impl Sealed for RepoCheckFlag<'_> {}
impl Endpoint for RepoCheckFlag<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/flags/{}",
                self.owner, self.repo, self.flag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoAddFlag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub flag: &'a str,
}

impl Sealed for RepoAddFlag<'_> {}
impl Endpoint for RepoAddFlag<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/flags/{}",
                self.owner, self.repo, self.flag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteFlag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub flag: &'a str,
}

impl Sealed for RepoDeleteFlag<'_> {}
impl Endpoint for RepoDeleteFlag<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/flags/{}",
                self.owner, self.repo, self.flag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct ListForks<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for ListForks<'_> {}
impl Endpoint for ListForks<'_> {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/forks", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateFork<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateForkOption,
}

impl Sealed for CreateFork<'_> {}
impl Endpoint for CreateFork<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/forks", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            202 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetBlobs<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: GetBlobsQuery,
}

impl Sealed for GetBlobs<'_> {}
impl Endpoint for GetBlobs<'_> {
    type Response = Vec<GitBlob>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/git/blobs", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetBlob<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
}

impl Sealed for GetBlob<'_> {}
impl Endpoint for GetBlob<'_> {
    type Response = GitBlob;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/blobs/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetSingleCommit<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub query: RepoGetSingleCommitQuery,
}

impl Sealed for RepoGetSingleCommit<'_> {}
impl Endpoint for RepoGetSingleCommit<'_> {
    type Response = Commit;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/commits/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDownloadCommitDiffOrPatch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub diff_type: &'a str,
}

impl Sealed for RepoDownloadCommitDiffOrPatch<'_> {}
impl Endpoint for RepoDownloadCommitDiffOrPatch<'_> {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/commits/{}.{}",
                self.owner, self.repo, self.sha, self.diff_type
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetNote<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub query: RepoGetNoteQuery,
}

impl Sealed for RepoGetNote<'_> {}
impl Endpoint for RepoGetNote<'_> {
    type Response = Note;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/notes/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSetNote<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub body: NoteOptions,
}

impl Sealed for RepoSetNote<'_> {}
impl Endpoint for RepoSetNote<'_> {
    type Response = Note;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/git/notes/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoRemoveNote<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
}

impl Sealed for RepoRemoveNote<'_> {}
impl Endpoint for RepoRemoveNote<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/git/notes/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListAllGitRefs<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListAllGitRefs<'_> {}
impl Endpoint for RepoListAllGitRefs<'_> {
    type Response = Vec<Reference>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/git/refs", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListGitRefs<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub r#ref: &'a str,
}

impl Sealed for RepoListGitRefs<'_> {}
impl Endpoint for RepoListGitRefs<'_> {
    type Response = Vec<Reference>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/refs/{}",
                self.owner, self.repo, self.r#ref
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetAnnotatedTag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
}

impl Sealed for GetAnnotatedTag<'_> {}
impl Endpoint for GetAnnotatedTag<'_> {
    type Response = AnnotatedTag;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/tags/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetTree<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub query: GetTreeQuery,
}

impl Sealed for GetTree<'_> {}
impl Endpoint for GetTree<'_> {
    type Response = GitTreeResponse;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/git/trees/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListHooks<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListHooks<'_> {}
impl Endpoint for RepoListHooks<'_> {
    type Response = (HookListHeaders, Vec<Hook>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/hooks", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateHookOption,
}

impl Sealed for RepoCreateHook<'_> {}
impl Endpoint for RepoCreateHook<'_> {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/hooks", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListGitHooks<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListGitHooks<'_> {}
impl Endpoint for RepoListGitHooks<'_> {
    type Response = (GitHookListHeaders, Vec<GitHook>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/hooks/git", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetGitHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: &'a str,
}

impl Sealed for RepoGetGitHook<'_> {}
impl Endpoint for RepoGetGitHook<'_> {
    type Response = GitHook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/git/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteGitHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: &'a str,
}

impl Sealed for RepoDeleteGitHook<'_> {}
impl Endpoint for RepoDeleteGitHook<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/git/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditGitHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: &'a str,
    pub body: EditGitHookOption,
}

impl Sealed for RepoEditGitHook<'_> {}
impl Endpoint for RepoEditGitHook<'_> {
    type Response = GitHook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/git/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoGetHook<'_> {}
impl Endpoint for RepoGetHook<'_> {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoDeleteHook<'_> {}
impl Endpoint for RepoDeleteHook<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditHookOption,
}

impl Sealed for RepoEditHook<'_> {}
impl Endpoint for RepoEditHook<'_> {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoTestHook<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub query: RepoTestHookQuery,
}

impl Sealed for RepoTestHook<'_> {}
impl Endpoint for RepoTestHook<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/hooks/{}/tests",
                self.owner, self.repo, self.id
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetIssueConfig<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetIssueConfig<'_> {}
impl Endpoint for RepoGetIssueConfig<'_> {
    type Response = IssueConfig;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/issue_config", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoValidateIssueConfig<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoValidateIssueConfig<'_> {}
impl Endpoint for RepoValidateIssueConfig<'_> {
    type Response = IssueConfigValidation;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issue_config/validate",
                self.owner, self.repo
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetIssueTemplates<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetIssueTemplates<'_> {}
impl Endpoint for RepoGetIssueTemplates<'_> {
    type Response = Vec<IssueTemplate>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/issue_templates", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueListIssues<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: IssueListIssuesQuery,
}

impl Sealed for IssueListIssues<'_> {}
impl Endpoint for IssueListIssues<'_> {
    type Response = (IssueListHeaders, Vec<Issue>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/issues", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateIssue<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateIssueOption,
}

impl Sealed for IssueCreateIssue<'_> {}
impl Endpoint for IssueCreateIssue<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/issues", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            412 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetRepoComments<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: IssueGetRepoCommentsQuery,
}

impl Sealed for IssueGetRepoComments<'_> {}
impl Endpoint for IssueGetRepoComments<'_> {
    type Response = (CommentListHeaders, Vec<Comment>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/issues/comments", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueGetComment<'_> {}
impl Endpoint for IssueGetComment<'_> {
    type Response = Option<Comment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueDeleteComment<'_> {}
impl Endpoint for IssueDeleteComment<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditIssueCommentOption,
}

impl Sealed for IssueEditComment<'_> {}
impl Endpoint for IssueEditComment<'_> {
    type Response = Option<Comment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueListIssueCommentAttachments<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueListIssueCommentAttachments<'_> {}
impl Endpoint for IssueListIssueCommentAttachments<'_> {
    type Response = Vec<Attachment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/assets",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateIssueCommentAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment: &'a [u8],
    pub query: IssueCreateIssueCommentAttachmentQuery,
}

impl Sealed for IssueCreateIssueCommentAttachment<'_> {}
impl Endpoint for IssueCreateIssueCommentAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/assets",
                self.owner, self.repo, self.id
            )
            .into(),
            query: Some(self.query.into_list()),
            body: {
                let mut list = Vec::new();
                let attachment = self.attachment;
                list.push(("attachment", attachment.to_vec()));
                RequestBody::Form(list)
            },
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetIssueCommentAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment_id: i64,
}

impl Sealed for IssueGetIssueCommentAttachment<'_> {}
impl Endpoint for IssueGetIssueCommentAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/assets/{}",
                self.owner, self.repo, self.id, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteIssueCommentAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment_id: i64,
}

impl Sealed for IssueDeleteIssueCommentAttachment<'_> {}
impl Endpoint for IssueDeleteIssueCommentAttachment<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/assets/{}",
                self.owner, self.repo, self.id, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APIError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditIssueCommentAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment_id: i64,
    pub body: EditAttachmentOptions,
}

impl Sealed for IssueEditIssueCommentAttachment<'_> {}
impl Endpoint for IssueEditIssueCommentAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/assets/{}",
                self.owner, self.repo, self.id, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetCommentReactions<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueGetCommentReactions<'_> {}
impl Endpoint for IssueGetCommentReactions<'_> {
    type Response = (ReactionListHeaders, Vec<Reaction>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/reactions",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssuePostCommentReaction<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditReactionOption,
}

impl Sealed for IssuePostCommentReaction<'_> {}
impl Endpoint for IssuePostCommentReaction<'_> {
    type Response = Reaction;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/reactions",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteCommentReaction<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditReactionOption,
}

impl Sealed for IssueDeleteCommentReaction<'_> {}
impl Endpoint for IssueDeleteCommentReaction<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/comments/{}/reactions",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListPinnedIssues<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListPinnedIssues<'_> {}
impl Endpoint for RepoListPinnedIssues<'_> {
    type Response = (IssueListHeaders, Vec<Issue>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/issues/pinned", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetIssue<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueGetIssue<'_> {}
impl Endpoint for IssueGetIssue<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDelete<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueDelete<'_> {}
impl Endpoint for IssueDelete<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditIssue<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: EditIssueOption,
}

impl Sealed for IssueEditIssue<'_> {}
impl Endpoint for IssueEditIssue<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            412 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueListIssueAttachments<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueListIssueAttachments<'_> {}
impl Endpoint for IssueListIssueAttachments<'_> {
    type Response = Vec<Attachment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/assets",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateIssueAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub attachment: &'a [u8],
    pub query: IssueCreateIssueAttachmentQuery,
}

impl Sealed for IssueCreateIssueAttachment<'_> {}
impl Endpoint for IssueCreateIssueAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/assets",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: {
                let mut list = Vec::new();
                let attachment = self.attachment;
                list.push(("attachment", attachment.to_vec()));
                RequestBody::Form(list)
            },
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetIssueAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub attachment_id: i64,
}

impl Sealed for IssueGetIssueAttachment<'_> {}
impl Endpoint for IssueGetIssueAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/assets/{}",
                self.owner, self.repo, self.index, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteIssueAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub attachment_id: i64,
}

impl Sealed for IssueDeleteIssueAttachment<'_> {}
impl Endpoint for IssueDeleteIssueAttachment<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/assets/{}",
                self.owner, self.repo, self.index, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APIError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditIssueAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub attachment_id: i64,
    pub body: EditAttachmentOptions,
}

impl Sealed for IssueEditIssueAttachment<'_> {}
impl Endpoint for IssueEditIssueAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/assets/{}",
                self.owner, self.repo, self.index, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueListBlocks<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueListBlocks<'_> {}
impl Endpoint for IssueListBlocks<'_> {
    type Response = (IssueListHeaders, Vec<Issue>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/blocks",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateIssueBlocking<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: IssueMeta,
}

impl Sealed for IssueCreateIssueBlocking<'_> {}
impl Endpoint for IssueCreateIssueBlocking<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/blocks",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueRemoveIssueBlocking<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: IssueMeta,
}

impl Sealed for IssueRemoveIssueBlocking<'_> {}
impl Endpoint for IssueRemoveIssueBlocking<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/blocks",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetComments<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub query: IssueGetCommentsQuery,
}

impl Sealed for IssueGetComments<'_> {}
impl Endpoint for IssueGetComments<'_> {
    type Response = (CommentListHeaders, Vec<Comment>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/comments",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: CreateIssueCommentOption,
}

impl Sealed for IssueCreateComment<'_> {}
impl Endpoint for IssueCreateComment<'_> {
    type Response = Comment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/comments",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteCommentDeprecated<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: u32,
    pub id: i64,
}

impl Sealed for IssueDeleteCommentDeprecated<'_> {}
impl Endpoint for IssueDeleteCommentDeprecated<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/comments/{}",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditCommentDeprecated<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: u32,
    pub id: i64,
    pub body: EditIssueCommentOption,
}

impl Sealed for IssueEditCommentDeprecated<'_> {}
impl Endpoint for IssueEditCommentDeprecated<'_> {
    type Response = Option<Comment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/comments/{}",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditIssueDeadline<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: EditDeadlineOption,
}

impl Sealed for IssueEditIssueDeadline<'_> {}
impl Endpoint for IssueEditIssueDeadline<'_> {
    type Response = IssueDeadline;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/deadline",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueListIssueDependencies<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueListIssueDependencies<'_> {}
impl Endpoint for IssueListIssueDependencies<'_> {
    type Response = (IssueListHeaders, Vec<Issue>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/dependencies",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateIssueDependencies<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: IssueMeta,
}

impl Sealed for IssueCreateIssueDependencies<'_> {}
impl Endpoint for IssueCreateIssueDependencies<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/dependencies",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueRemoveIssueDependencies<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: IssueMeta,
}

impl Sealed for IssueRemoveIssueDependencies<'_> {}
impl Endpoint for IssueRemoveIssueDependencies<'_> {
    type Response = Issue;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/dependencies",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetLabels<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueGetLabels<'_> {}
impl Endpoint for IssueGetLabels<'_> {
    type Response = (LabelListHeaders, Vec<Label>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/labels",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueReplaceLabels<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: IssueLabelsOption,
}

impl Sealed for IssueReplaceLabels<'_> {}
impl Endpoint for IssueReplaceLabels<'_> {
    type Response = (LabelListHeaders, Vec<Label>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/labels",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueAddLabel<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: IssueLabelsOption,
}

impl Sealed for IssueAddLabel<'_> {}
impl Endpoint for IssueAddLabel<'_> {
    type Response = (LabelListHeaders, Vec<Label>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/labels",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueClearLabels<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: DeleteLabelsOption,
}

impl Sealed for IssueClearLabels<'_> {}
impl Endpoint for IssueClearLabels<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/labels",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueRemoveLabel<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub identifier: &'a str,
    pub body: DeleteLabelsOption,
}

impl Sealed for IssueRemoveLabel<'_> {}
impl Endpoint for IssueRemoveLabel<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/labels/{}",
                self.owner, self.repo, self.index, self.identifier
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct PinIssue<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for PinIssue<'_> {}
impl Endpoint for PinIssue<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/pin",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UnpinIssue<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for UnpinIssue<'_> {}
impl Endpoint for UnpinIssue<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/pin",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct MoveIssuePin<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub position: i64,
}

impl Sealed for MoveIssuePin<'_> {}
impl Endpoint for MoveIssuePin<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/pin/{}",
                self.owner, self.repo, self.index, self.position
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetIssueReactions<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueGetIssueReactions<'_> {}
impl Endpoint for IssueGetIssueReactions<'_> {
    type Response = (ReactionListHeaders, Vec<Reaction>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/reactions",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssuePostIssueReaction<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: EditReactionOption,
}

impl Sealed for IssuePostIssueReaction<'_> {}
impl Endpoint for IssuePostIssueReaction<'_> {
    type Response = Reaction;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/reactions",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteIssueReaction<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: EditReactionOption,
}

impl Sealed for IssueDeleteIssueReaction<'_> {}
impl Endpoint for IssueDeleteIssueReaction<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/reactions",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteStopWatch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueDeleteStopWatch<'_> {}
impl Endpoint for IssueDeleteStopWatch<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/stopwatch/delete",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueStartStopWatch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueStartStopWatch<'_> {}
impl Endpoint for IssueStartStopWatch<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/stopwatch/start",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            403 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueStopStopWatch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueStopStopWatch<'_> {}
impl Endpoint for IssueStopStopWatch<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/stopwatch/stop",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            403 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueSubscriptions<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueSubscriptions<'_> {}
impl Endpoint for IssueSubscriptions<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/subscriptions",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCheckSubscription<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueCheckSubscription<'_> {}
impl Endpoint for IssueCheckSubscription<'_> {
    type Response = WatchInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/subscriptions/check",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueAddSubscription<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub user: &'a str,
}

impl Sealed for IssueAddSubscription<'_> {}
impl Endpoint for IssueAddSubscription<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/subscriptions/{}",
                self.owner, self.repo, self.index, self.user
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, false)),
            201 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteSubscription<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub user: &'a str,
}

impl Sealed for IssueDeleteSubscription<'_> {}
impl Endpoint for IssueDeleteSubscription<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/subscriptions/{}",
                self.owner, self.repo, self.index, self.user
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, false)),
            201 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetCommentsAndTimeline<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub query: IssueGetCommentsAndTimelineQuery,
}

impl Sealed for IssueGetCommentsAndTimeline<'_> {}
impl Endpoint for IssueGetCommentsAndTimeline<'_> {
    type Response = (TimelineListHeaders, Vec<TimelineComment>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/timeline",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueTrackedTimes<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub query: IssueTrackedTimesQuery,
}

impl Sealed for IssueTrackedTimes<'_> {}
impl Endpoint for IssueTrackedTimes<'_> {
    type Response = (TrackedTimeListHeaders, Vec<TrackedTime>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/times",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueAddTime<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: AddTimeOption,
}

impl Sealed for IssueAddTime<'_> {}
impl Endpoint for IssueAddTime<'_> {
    type Response = TrackedTime;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/times",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueResetTime<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for IssueResetTime<'_> {}
impl Endpoint for IssueResetTime<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/times",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteTime<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
}

impl Sealed for IssueDeleteTime<'_> {}
impl Endpoint for IssueDeleteTime<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/issues/{}/times/{}",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListKeys<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoListKeysQuery,
}

impl Sealed for RepoListKeys<'_> {}
impl Endpoint for RepoListKeys<'_> {
    type Response = (DeployKeyListHeaders, Vec<DeployKey>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/keys", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateKey<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateKeyOption,
}

impl Sealed for RepoCreateKey<'_> {}
impl Endpoint for RepoCreateKey<'_> {
    type Response = DeployKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/keys", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetKey<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoGetKey<'_> {}
impl Endpoint for RepoGetKey<'_> {
    type Response = DeployKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/keys/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteKey<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoDeleteKey<'_> {}
impl Endpoint for RepoDeleteKey<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/keys/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueListLabels<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: IssueListLabelsQuery,
}

impl Sealed for IssueListLabels<'_> {}
impl Endpoint for IssueListLabels<'_> {
    type Response = (LabelListHeaders, Vec<Label>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/labels", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateLabel<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateLabelOption,
}

impl Sealed for IssueCreateLabel<'_> {}
impl Endpoint for IssueCreateLabel<'_> {
    type Response = Label;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/labels", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetLabel<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueGetLabel<'_> {}
impl Endpoint for IssueGetLabel<'_> {
    type Response = Label;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/labels/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteLabel<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueDeleteLabel<'_> {}
impl Endpoint for IssueDeleteLabel<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/labels/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditLabel<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditLabelOption,
}

impl Sealed for IssueEditLabel<'_> {}
impl Endpoint for IssueEditLabel<'_> {
    type Response = Label;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/labels/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetLanguages<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetLanguages<'_> {}
impl Endpoint for RepoGetLanguages<'_> {
    type Response = BTreeMap<String, i64>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/languages", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetRawFileOrLfs<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub query: RepoGetRawFileOrLfsQuery,
}

impl Sealed for RepoGetRawFileOrLfs<'_> {}
impl Endpoint for RepoGetRawFileOrLfs<'_> {
    type Response = Bytes;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/media/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetMilestonesList<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: IssueGetMilestonesListQuery,
}

impl Sealed for IssueGetMilestonesList<'_> {}
impl Endpoint for IssueGetMilestonesList<'_> {
    type Response = (MilestoneListHeaders, Vec<Milestone>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/milestones", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueCreateMilestone<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateMilestoneOption,
}

impl Sealed for IssueCreateMilestone<'_> {}
impl Endpoint for IssueCreateMilestone<'_> {
    type Response = Milestone;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/milestones", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueGetMilestone<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueGetMilestone<'_> {}
impl Endpoint for IssueGetMilestone<'_> {
    type Response = Milestone;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/milestones/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueDeleteMilestone<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for IssueDeleteMilestone<'_> {}
impl Endpoint for IssueDeleteMilestone<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/milestones/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct IssueEditMilestone<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditMilestoneOption,
}

impl Sealed for IssueEditMilestone<'_> {}
impl Endpoint for IssueEditMilestone<'_> {
    type Response = Milestone;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/milestones/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoMirrorSync<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoMirrorSync<'_> {}
impl Endpoint for RepoMirrorSync<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/mirror-sync", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoNewPinAllowed<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoNewPinAllowed<'_> {}
impl Endpoint for RepoNewPinAllowed<'_> {
    type Response = NewIssuePinsAllowed;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/new_pin_allowed", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyGetRepoList<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: NotifyGetRepoListQuery,
}

impl Sealed for NotifyGetRepoList<'_> {}
impl Endpoint for NotifyGetRepoList<'_> {
    type Response = (NotificationThreadListHeaders, Vec<NotificationThread>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/notifications", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct NotifyReadRepoList<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: NotifyReadRepoListQuery,
}

impl Sealed for NotifyReadRepoList<'_> {}
impl Endpoint for NotifyReadRepoList<'_> {
    type Response = (NotificationThreadListHeaders, Vec<NotificationThread>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/repos/{}/{}/notifications", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            205 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListPullRequests<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoListPullRequestsQuery,
}

impl Sealed for RepoListPullRequests<'_> {}
impl Endpoint for RepoListPullRequests<'_> {
    type Response = (PullRequestListHeaders, Vec<PullRequest>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/pulls", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreatePullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreatePullRequestOption,
}

impl Sealed for RepoCreatePullRequest<'_> {}
impl Endpoint for RepoCreatePullRequest<'_> {
    type Response = PullRequest;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/pulls", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListPinnedPullRequests<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListPinnedPullRequests<'_> {}
impl Endpoint for RepoListPinnedPullRequests<'_> {
    type Response = (PullRequestListHeaders, Vec<PullRequest>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/pulls/pinned", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullRequestByBaseHead<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub base: &'a str,
    pub head: &'a str,
}

impl Sealed for RepoGetPullRequestByBaseHead<'_> {}
impl Endpoint for RepoGetPullRequestByBaseHead<'_> {
    type Response = PullRequest;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/{}",
                self.owner, self.repo, self.base, self.head
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for RepoGetPullRequest<'_> {}
impl Endpoint for RepoGetPullRequest<'_> {
    type Response = PullRequest;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditPullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: EditPullRequestOption,
}

impl Sealed for RepoEditPullRequest<'_> {}
impl Endpoint for RepoEditPullRequest<'_> {
    type Response = PullRequest;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            412 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDownloadPullDiffOrPatch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub diff_type: &'a str,
    pub query: RepoDownloadPullDiffOrPatchQuery,
}

impl Sealed for RepoDownloadPullDiffOrPatch<'_> {}
impl Endpoint for RepoDownloadPullDiffOrPatch<'_> {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}.{}",
                self.owner, self.repo, self.index, self.diff_type
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullRequestCommits<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub query: RepoGetPullRequestCommitsQuery,
}

impl Sealed for RepoGetPullRequestCommits<'_> {}
impl Endpoint for RepoGetPullRequestCommits<'_> {
    type Response = (CommitListHeaders, Vec<Commit>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/commits",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullRequestFiles<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub query: RepoGetPullRequestFilesQuery,
}

impl Sealed for RepoGetPullRequestFiles<'_> {}
impl Endpoint for RepoGetPullRequestFiles<'_> {
    type Response = (ChangedFileListHeaders, Vec<ChangedFile>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/files",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoPullRequestIsMerged<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for RepoPullRequestIsMerged<'_> {}
impl Endpoint for RepoPullRequestIsMerged<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/merge",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoMergePullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: MergePullRequestOption,
}

impl Sealed for RepoMergePullRequest<'_> {}
impl Endpoint for RepoMergePullRequest<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/merge",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, false)),
            403 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            405 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            409 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCancelScheduledAutoMerge<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for RepoCancelScheduledAutoMerge<'_> {}
impl Endpoint for RepoCancelScheduledAutoMerge<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/merge",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreatePullReviewRequests<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: PullReviewRequestOptions,
}

impl Sealed for RepoCreatePullReviewRequests<'_> {}
impl Endpoint for RepoCreatePullReviewRequests<'_> {
    type Response = (PullReviewListHeaders, Vec<PullReview>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/requested_reviewers",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeletePullReviewRequests<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: PullReviewRequestOptions,
}

impl Sealed for RepoDeletePullReviewRequests<'_> {}
impl Endpoint for RepoDeletePullReviewRequests<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/requested_reviewers",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListPullReviews<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
}

impl Sealed for RepoListPullReviews<'_> {}
impl Endpoint for RepoListPullReviews<'_> {
    type Response = (PullReviewListHeaders, Vec<PullReview>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreatePullReview<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub body: CreatePullReviewOptions,
}

impl Sealed for RepoCreatePullReview<'_> {}
impl Endpoint for RepoCreatePullReview<'_> {
    type Response = PullReview;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews",
                self.owner, self.repo, self.index
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullReview<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
}

impl Sealed for RepoGetPullReview<'_> {}
impl Endpoint for RepoGetPullReview<'_> {
    type Response = PullReview;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSubmitPullReview<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
    pub body: SubmitPullReviewOptions,
}

impl Sealed for RepoSubmitPullReview<'_> {}
impl Endpoint for RepoSubmitPullReview<'_> {
    type Response = PullReview;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeletePullReview<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
}

impl Sealed for RepoDeletePullReview<'_> {}
impl Endpoint for RepoDeletePullReview<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullReviewComments<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
}

impl Sealed for RepoGetPullReviewComments<'_> {}
impl Endpoint for RepoGetPullReviewComments<'_> {
    type Response = Vec<PullReviewComment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}/comments",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreatePullReviewComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
    pub body: serde_json::Value,
}

impl Sealed for RepoCreatePullReviewComment<'_> {}
impl Endpoint for RepoCreatePullReviewComment<'_> {
    type Response = PullReviewComment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}/comments",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPullReviewComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
    pub comment: i64,
}

impl Sealed for RepoGetPullReviewComment<'_> {}
impl Endpoint for RepoGetPullReviewComment<'_> {
    type Response = PullReviewComment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}/comments/{}",
                self.owner, self.repo, self.index, self.id, self.comment
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeletePullReviewComment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
    pub comment: i64,
}

impl Sealed for RepoDeletePullReviewComment<'_> {}
impl Endpoint for RepoDeletePullReviewComment<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}/comments/{}",
                self.owner, self.repo, self.index, self.id, self.comment
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDismissPullReview<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
    pub body: DismissPullReviewOptions,
}

impl Sealed for RepoDismissPullReview<'_> {}
impl Endpoint for RepoDismissPullReview<'_> {
    type Response = PullReview;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}/dismissals",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoUnDismissPullReview<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub id: i64,
}

impl Sealed for RepoUnDismissPullReview<'_> {}
impl Endpoint for RepoUnDismissPullReview<'_> {
    type Response = PullReview;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/reviews/{}/undismissals",
                self.owner, self.repo, self.index, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoUpdatePullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub index: i64,
    pub query: RepoUpdatePullRequestQuery,
}

impl Sealed for RepoUpdatePullRequest<'_> {}
impl Endpoint for RepoUpdatePullRequest<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/pulls/{}/update",
                self.owner, self.repo, self.index
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListPushMirrors<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListPushMirrors<'_> {}
impl Endpoint for RepoListPushMirrors<'_> {
    type Response = (PushMirrorListHeaders, Vec<PushMirror>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/push_mirrors", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoAddPushMirror<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreatePushMirrorOption,
}

impl Sealed for RepoAddPushMirror<'_> {}
impl Endpoint for RepoAddPushMirror<'_> {
    type Response = PushMirror;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/push_mirrors", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoPushMirrorSync<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoPushMirrorSync<'_> {}
impl Endpoint for RepoPushMirrorSync<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/push_mirrors-sync",
                self.owner, self.repo
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetPushMirrorByRemoteName<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub name: &'a str,
}

impl Sealed for RepoGetPushMirrorByRemoteName<'_> {}
impl Endpoint for RepoGetPushMirrorByRemoteName<'_> {
    type Response = PushMirror;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/push_mirrors/{}",
                self.owner, self.repo, self.name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeletePushMirror<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub name: &'a str,
}

impl Sealed for RepoDeletePushMirror<'_> {}
impl Endpoint for RepoDeletePushMirror<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/push_mirrors/{}",
                self.owner, self.repo, self.name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetRawFile<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub filepath: &'a str,
    pub query: RepoGetRawFileQuery,
}

impl Sealed for RepoGetRawFile<'_> {}
impl Endpoint for RepoGetRawFile<'_> {
    type Response = Bytes;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/raw/{}",
                self.owner, self.repo, self.filepath
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListReleases<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoListReleasesQuery,
}

impl Sealed for RepoListReleases<'_> {}
impl Endpoint for RepoListReleases<'_> {
    type Response = (ReleaseListHeaders, Vec<Release>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/releases", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateRelease<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateReleaseOption,
}

impl Sealed for RepoCreateRelease<'_> {}
impl Endpoint for RepoCreateRelease<'_> {
    type Response = Release;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/releases", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetLatestRelease<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetLatestRelease<'_> {}
impl Endpoint for RepoGetLatestRelease<'_> {
    type Response = Release;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/releases/latest", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetReleaseByTag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub tag: &'a str,
}

impl Sealed for RepoGetReleaseByTag<'_> {}
impl Endpoint for RepoGetReleaseByTag<'_> {
    type Response = Release;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/releases/tags/{}",
                self.owner, self.repo, self.tag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteReleaseByTag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub tag: &'a str,
}

impl Sealed for RepoDeleteReleaseByTag<'_> {}
impl Endpoint for RepoDeleteReleaseByTag<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/releases/tags/{}",
                self.owner, self.repo, self.tag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetRelease<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoGetRelease<'_> {}
impl Endpoint for RepoGetRelease<'_> {
    type Response = Release;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteRelease<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoDeleteRelease<'_> {}
impl Endpoint for RepoDeleteRelease<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditRelease<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditReleaseOption,
}

impl Sealed for RepoEditRelease<'_> {}
impl Endpoint for RepoEditRelease<'_> {
    type Response = Release;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListReleaseAttachments<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoListReleaseAttachments<'_> {}
impl Endpoint for RepoListReleaseAttachments<'_> {
    type Response = Vec<Attachment>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}/assets",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateReleaseAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment: Option<&'a [u8]>,
    pub external_url: Option<&'a str>,
    pub query: RepoCreateReleaseAttachmentQuery,
}

impl Sealed for RepoCreateReleaseAttachment<'_> {}
impl Endpoint for RepoCreateReleaseAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}/assets",
                self.owner, self.repo, self.id
            )
            .into(),
            query: Some(self.query.into_list()),
            body: {
                let mut list = Vec::new();
                if let Some(attachment) = self.attachment {
                    list.push(("attachment", attachment.to_vec()));
                }
                if let Some(external_url) = self.external_url {
                    list.push(("external_url", external_url.as_bytes().into()));
                }
                RequestBody::Form(list)
            },
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetReleaseAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment_id: i64,
}

impl Sealed for RepoGetReleaseAttachment<'_> {}
impl Endpoint for RepoGetReleaseAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}/assets/{}",
                self.owner, self.repo, self.id, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteReleaseAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment_id: i64,
}

impl Sealed for RepoDeleteReleaseAttachment<'_> {}
impl Endpoint for RepoDeleteReleaseAttachment<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}/assets/{}",
                self.owner, self.repo, self.id, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditReleaseAttachment<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub attachment_id: i64,
    pub body: EditAttachmentOptions,
}

impl Sealed for RepoEditReleaseAttachment<'_> {}
impl Endpoint for RepoEditReleaseAttachment<'_> {
    type Response = Attachment;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/releases/{}/assets/{}",
                self.owner, self.repo, self.id, self.attachment_id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetReviewers<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetReviewers<'_> {}
impl Endpoint for RepoGetReviewers<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/reviewers", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSigningKey<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoSigningKey<'_> {}
impl Endpoint for RepoSigningKey<'_> {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/signing-key.gpg", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListStargazers<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListStargazers<'_> {}
impl Endpoint for RepoListStargazers<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/stargazers", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListStatuses<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub query: RepoListStatusesQuery,
}

impl Sealed for RepoListStatuses<'_> {}
impl Endpoint for RepoListStatuses<'_> {
    type Response = (CommitStatusListHeaders, Vec<CommitStatus>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/statuses/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateStatus<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub sha: &'a str,
    pub body: CreateStatusOption,
}

impl Sealed for RepoCreateStatus<'_> {}
impl Endpoint for RepoCreateStatus<'_> {
    type Response = CommitStatus;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/statuses/{}",
                self.owner, self.repo, self.sha
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListSubscribers<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListSubscribers<'_> {}
impl Endpoint for RepoListSubscribers<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/subscribers", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentCheckSubscription<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for UserCurrentCheckSubscription<'_> {}
impl Endpoint for UserCurrentCheckSubscription<'_> {
    type Response = WatchInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/subscription", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentPutSubscription<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for UserCurrentPutSubscription<'_> {}
impl Endpoint for UserCurrentPutSubscription<'_> {
    type Response = WatchInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/repos/{}/{}/subscription", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentDeleteSubscription<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for UserCurrentDeleteSubscription<'_> {}
impl Endpoint for UserCurrentDeleteSubscription<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/repos/{}/{}/subscription", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSyncForkDefaultInfo<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoSyncForkDefaultInfo<'_> {}
impl Endpoint for RepoSyncForkDefaultInfo<'_> {
    type Response = SyncForkInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/sync_fork", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSyncForkDefault<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoSyncForkDefault<'_> {}
impl Endpoint for RepoSyncForkDefault<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/sync_fork", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSyncForkBranchInfo<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
}

impl Sealed for RepoSyncForkBranchInfo<'_> {}
impl Endpoint for RepoSyncForkBranchInfo<'_> {
    type Response = SyncForkInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/sync_fork/{}",
                self.owner, self.repo, self.branch
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoSyncForkBranch<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub branch: &'a str,
}

impl Sealed for RepoSyncForkBranch<'_> {}
impl Endpoint for RepoSyncForkBranch<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/sync_fork/{}",
                self.owner, self.repo, self.branch
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListTagProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListTagProtection<'_> {}
impl Endpoint for RepoListTagProtection<'_> {
    type Response = Vec<TagProtection>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/tag_protections", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateTagProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateTagProtectionOption,
}

impl Sealed for RepoCreateTagProtection<'_> {}
impl Endpoint for RepoCreateTagProtection<'_> {
    type Response = TagProtection;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/tag_protections", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetTagProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoGetTagProtection<'_> {}
impl Endpoint for RepoGetTagProtection<'_> {
    type Response = TagProtection;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/tag_protections/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteTagProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
}

impl Sealed for RepoDeleteTagProtection<'_> {}
impl Endpoint for RepoDeleteTagProtection<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/tag_protections/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditTagProtection<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub id: i64,
    pub body: EditTagProtectionOption,
}

impl Sealed for RepoEditTagProtection<'_> {}
impl Endpoint for RepoEditTagProtection<'_> {
    type Response = TagProtection;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/tag_protections/{}",
                self.owner, self.repo, self.id
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListTags<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListTags<'_> {}
impl Endpoint for RepoListTags<'_> {
    type Response = (TagListHeaders, Vec<Tag>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/tags", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateTag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateTagOption,
}

impl Sealed for RepoCreateTag<'_> {}
impl Endpoint for RepoCreateTag<'_> {
    type Response = Tag;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/tags", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            405 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetTag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub tag: &'a str,
}

impl Sealed for RepoGetTag<'_> {}
impl Endpoint for RepoGetTag<'_> {
    type Response = Tag;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/tags/{}",
                self.owner, self.repo, self.tag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteTag<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub tag: &'a str,
}

impl Sealed for RepoDeleteTag<'_> {}
impl Endpoint for RepoDeleteTag<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/tags/{}",
                self.owner, self.repo, self.tag
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            405 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListTeams<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListTeams<'_> {}
impl Endpoint for RepoListTeams<'_> {
    type Response = (TeamListHeaders, Vec<Team>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/teams", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCheckTeam<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub team: &'a str,
}

impl Sealed for RepoCheckTeam<'_> {}
impl Endpoint for RepoCheckTeam<'_> {
    type Response = Team;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/teams/{}",
                self.owner, self.repo, self.team
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            405 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoAddTeam<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub team: &'a str,
}

impl Sealed for RepoAddTeam<'_> {}
impl Endpoint for RepoAddTeam<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/teams/{}",
                self.owner, self.repo, self.team
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            405 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteTeam<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub team: &'a str,
}

impl Sealed for RepoDeleteTeam<'_> {}
impl Endpoint for RepoDeleteTeam<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/teams/{}",
                self.owner, self.repo, self.team
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            405 => Err(json_error::<APIError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoTrackedTimes<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub query: RepoTrackedTimesQuery,
}

impl Sealed for RepoTrackedTimes<'_> {}
impl Endpoint for RepoTrackedTimes<'_> {
    type Response = (TrackedTimeListHeaders, Vec<TrackedTime>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/times", self.owner, self.repo).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserTrackedTimes<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub user: &'a str,
}

impl Sealed for UserTrackedTimes<'_> {}
impl Endpoint for UserTrackedTimes<'_> {
    type Response = (TrackedTimeListHeaders, Vec<TrackedTime>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/times/{}",
                self.owner, self.repo, self.user
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoListTopics<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoListTopics<'_> {}
impl Endpoint for RepoListTopics<'_> {
    type Response = TopicName;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/topics", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoUpdateTopics<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: RepoTopicOptions,
}

impl Sealed for RepoUpdateTopics<'_> {}
impl Endpoint for RepoUpdateTopics<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/repos/{}/{}/topics", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIInvalidTopicsError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoAddTopic<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub topic: &'a str,
}

impl Sealed for RepoAddTopic<'_> {}
impl Endpoint for RepoAddTopic<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!(
                "/api/v1/repos/{}/{}/topics/{}",
                self.owner, self.repo, self.topic
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIInvalidTopicsError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteTopic<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub topic: &'a str,
}

impl Sealed for RepoDeleteTopic<'_> {}
impl Endpoint for RepoDeleteTopic<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/topics/{}",
                self.owner, self.repo, self.topic
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIInvalidTopicsError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoTransfer<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: TransferRepoOption,
}

impl Sealed for RepoTransfer<'_> {}
impl Endpoint for RepoTransfer<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/transfer", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            202 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct AcceptRepoTransfer<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for AcceptRepoTransfer<'_> {}
impl Endpoint for AcceptRepoTransfer<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/transfer/accept", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            202 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RejectRepoTransfer<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RejectRepoTransfer<'_> {}
impl Endpoint for RejectRepoTransfer<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/transfer/reject", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoCreateWikiPage<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub body: CreateWikiPageOptions,
}

impl Sealed for RepoCreateWikiPage<'_> {}
impl Endpoint for RepoCreateWikiPage<'_> {
    type Response = WikiPage;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/repos/{}/{}/wiki/new", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetWikiPage<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub page_name: &'a str,
}

impl Sealed for RepoGetWikiPage<'_> {}
impl Endpoint for RepoGetWikiPage<'_> {
    type Response = WikiPage;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/wiki/page/{}",
                self.owner, self.repo, self.page_name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoDeleteWikiPage<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub page_name: &'a str,
}

impl Sealed for RepoDeleteWikiPage<'_> {}
impl Endpoint for RepoDeleteWikiPage<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!(
                "/api/v1/repos/{}/{}/wiki/page/{}",
                self.owner, self.repo, self.page_name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoEditWikiPage<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub page_name: &'a str,
    pub body: CreateWikiPageOptions,
}

impl Sealed for RepoEditWikiPage<'_> {}
impl Endpoint for RepoEditWikiPage<'_> {
    type Response = WikiPage;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!(
                "/api/v1/repos/{}/{}/wiki/page/{}",
                self.owner, self.repo, self.page_name
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            423 => Err(json_error::<APIRepoArchivedError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetWikiPages<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for RepoGetWikiPages<'_> {}
impl Endpoint for RepoGetWikiPages<'_> {
    type Response = (WikiPageListHeaders, Vec<WikiPageMetaData>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repos/{}/{}/wiki/pages", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetWikiPageRevisions<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub page_name: &'a str,
}

impl Sealed for RepoGetWikiPageRevisions<'_> {}
impl Endpoint for RepoGetWikiPageRevisions<'_> {
    type Response = (WikiCommitListHeaders, WikiCommitList);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/repos/{}/{}/wiki/revisions/{}",
                self.owner, self.repo, self.page_name
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GenerateRepo<'a> {
    pub template_owner: &'a str,
    pub template_repo: &'a str,
    pub body: GenerateRepoOption,
}

impl Sealed for GenerateRepo<'_> {}
impl Endpoint for GenerateRepo<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!(
                "/api/v1/repos/{}/{}/generate",
                self.template_owner, self.template_repo
            )
            .into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct RepoGetById {
    pub id: i64,
}

impl Sealed for RepoGetById {}
impl Endpoint for RepoGetById {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/repositories/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetGeneralApiSettings {}

impl Sealed for GetGeneralApiSettings {}
impl Endpoint for GetGeneralApiSettings {
    type Response = GeneralAPISettings;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/settings/api".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetGeneralAttachmentSettings {}

impl Sealed for GetGeneralAttachmentSettings {}
impl Endpoint for GetGeneralAttachmentSettings {
    type Response = GeneralAttachmentSettings;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/settings/attachment".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetGeneralRepositorySettings {}

impl Sealed for GetGeneralRepositorySettings {}
impl Endpoint for GetGeneralRepositorySettings {
    type Response = GeneralRepoSettings;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/settings/repository".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetGeneralUiSettings {}

impl Sealed for GetGeneralUiSettings {}
impl Endpoint for GetGeneralUiSettings {
    type Response = GeneralUISettings;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/settings/ui".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetSigningKey {}

impl Sealed for GetSigningKey {}
impl Endpoint for GetSigningKey {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/signing-key.gpg".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetSshSigningKey {}

impl Sealed for GetSshSigningKey {}
impl Endpoint for GetSshSigningKey {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/signing-key.ssh".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetTeam {
    pub id: i64,
}

impl Sealed for OrgGetTeam {}
impl Endpoint for OrgGetTeam {
    type Response = Team;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/teams/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgDeleteTeam {
    pub id: i64,
}

impl Sealed for OrgDeleteTeam {}
impl Endpoint for OrgDeleteTeam {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/teams/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgEditTeam {
    pub id: i64,
    pub body: EditTeamOption,
}

impl Sealed for OrgEditTeam {}
impl Endpoint for OrgEditTeam {
    type Response = Team;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/teams/{}", self.id).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListTeamActivityFeeds {
    pub id: i64,
    pub query: OrgListTeamActivityFeedsQuery,
}

impl Sealed for OrgListTeamActivityFeeds {}
impl Endpoint for OrgListTeamActivityFeeds {
    type Response = (ActivityFeedsListHeaders, Vec<Activity>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/teams/{}/activities/feeds", self.id).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListTeamMembers {
    pub id: i64,
}

impl Sealed for OrgListTeamMembers {}
impl Endpoint for OrgListTeamMembers {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/teams/{}/members", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListTeamMember<'a> {
    pub id: i64,
    pub username: &'a str,
}

impl Sealed for OrgListTeamMember<'_> {}
impl Endpoint for OrgListTeamMember<'_> {
    type Response = User;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/teams/{}/members/{}", self.id, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgAddTeamMember<'a> {
    pub id: i64,
    pub username: &'a str,
}

impl Sealed for OrgAddTeamMember<'_> {}
impl Endpoint for OrgAddTeamMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/teams/{}/members/{}", self.id, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgRemoveTeamMember<'a> {
    pub id: i64,
    pub username: &'a str,
}

impl Sealed for OrgRemoveTeamMember<'_> {}
impl Endpoint for OrgRemoveTeamMember<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/teams/{}/members/{}", self.id, self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListTeamRepos {
    pub id: i64,
}

impl Sealed for OrgListTeamRepos {}
impl Endpoint for OrgListTeamRepos {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/teams/{}/repos", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListTeamRepo<'a> {
    pub id: i64,
    pub org: &'a str,
    pub repo: &'a str,
}

impl Sealed for OrgListTeamRepo<'_> {}
impl Endpoint for OrgListTeamRepo<'_> {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/teams/{}/repos/{}/{}", self.id, self.org, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgAddTeamRepository<'a> {
    pub id: i64,
    pub org: &'a str,
    pub repo: &'a str,
}

impl Sealed for OrgAddTeamRepository<'_> {}
impl Endpoint for OrgAddTeamRepository<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/teams/{}/repos/{}/{}", self.id, self.org, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgRemoveTeamRepository<'a> {
    pub id: i64,
    pub org: &'a str,
    pub repo: &'a str,
}

impl Sealed for OrgRemoveTeamRepository<'_> {}
impl Endpoint for OrgRemoveTeamRepository<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/teams/{}/repos/{}/{}", self.id, self.org, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct TopicSearch {
    pub query: TopicSearchQuery,
}

impl Sealed for TopicSearch {}
impl Endpoint for TopicSearch {
    type Response = TopicSearchResults;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/topics/search".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetCurrent {}

impl Sealed for UserGetCurrent {}
impl Endpoint for UserGetCurrent {
    type Response = User;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserSearchRunJobs {
    pub query: UserSearchRunJobsQuery,
}

impl Sealed for UserSearchRunJobs {}
impl Endpoint for UserSearchRunJobs {
    type Response = Vec<ActionRunJob>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/actions/runners/jobs".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetRunnerRegistrationToken {}

impl Sealed for UserGetRunnerRegistrationToken {}
impl Endpoint for UserGetRunnerRegistrationToken {
    type Response = RegistrationToken;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/actions/runners/registration-token".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateUserSecret<'a> {
    pub secretname: &'a str,
    pub body: CreateOrUpdateSecretOption,
}

impl Sealed for UpdateUserSecret<'_> {}
impl Endpoint for UpdateUserSecret<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/user/actions/secrets/{}", self.secretname).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeleteUserSecret<'a> {
    pub secretname: &'a str,
}

impl Sealed for DeleteUserSecret<'_> {}
impl Endpoint for DeleteUserSecret<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/actions/secrets/{}", self.secretname).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetUserVariablesList {}

impl Sealed for GetUserVariablesList {}
impl Endpoint for GetUserVariablesList {
    type Response = (VariableListHeaders, Vec<ActionVariable>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/actions/variables".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetUserVariable<'a> {
    pub variablename: &'a str,
}

impl Sealed for GetUserVariable<'_> {}
impl Endpoint for GetUserVariable<'_> {
    type Response = ActionVariable;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/actions/variables/{}", self.variablename).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateUserVariable<'a> {
    pub variablename: &'a str,
    pub body: UpdateVariableOption,
}

impl Sealed for UpdateUserVariable<'_> {}
impl Endpoint for UpdateUserVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/user/actions/variables/{}", self.variablename).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateUserVariable<'a> {
    pub variablename: &'a str,
    pub body: CreateVariableOption,
}

impl Sealed for CreateUserVariable<'_> {}
impl Endpoint for CreateUserVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/user/actions/variables/{}", self.variablename).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct DeleteUserVariable<'a> {
    pub variablename: &'a str,
}

impl Sealed for DeleteUserVariable<'_> {}
impl Endpoint for DeleteUserVariable<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/actions/variables/{}", self.variablename).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, false)),
            204 => Ok((response, false)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetOAuth2Applications {}

impl Sealed for UserGetOAuth2Applications {}
impl Endpoint for UserGetOAuth2Applications {
    type Response = (OAuth2ApplicationListHeaders, Vec<OAuth2Application>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/applications/oauth2".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCreateOAuth2Application {
    pub body: CreateOAuth2ApplicationOptions,
}

impl Sealed for UserCreateOAuth2Application {}
impl Endpoint for UserCreateOAuth2Application {
    type Response = OAuth2Application;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/applications/oauth2".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetOAuth2Application {
    pub id: i64,
}

impl Sealed for UserGetOAuth2Application {}
impl Endpoint for UserGetOAuth2Application {
    type Response = OAuth2Application;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/applications/oauth2/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserDeleteOAuth2Application {
    pub id: i64,
}

impl Sealed for UserDeleteOAuth2Application {}
impl Endpoint for UserDeleteOAuth2Application {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/applications/oauth2/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserUpdateOAuth2Application {
    pub id: i64,
    pub body: CreateOAuth2ApplicationOptions,
}

impl Sealed for UserUpdateOAuth2Application {}
impl Endpoint for UserUpdateOAuth2Application {
    type Response = OAuth2Application;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/user/applications/oauth2/{}", self.id).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserUpdateAvatar {
    pub body: UpdateUserAvatarOption,
}

impl Sealed for UserUpdateAvatar {}
impl Endpoint for UserUpdateAvatar {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/avatar".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserDeleteAvatar {}

impl Sealed for UserDeleteAvatar {}
impl Endpoint for UserDeleteAvatar {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: "/api/v1/user/avatar".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserBlockUser<'a> {
    pub username: &'a str,
}

impl Sealed for UserBlockUser<'_> {}
impl Endpoint for UserBlockUser<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/user/block/{}", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListEmails {}

impl Sealed for UserListEmails {}
impl Endpoint for UserListEmails {
    type Response = Vec<Email>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/emails".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserAddEmail {
    pub body: CreateEmailOption,
}

impl Sealed for UserAddEmail {}
impl Endpoint for UserAddEmail {
    type Response = Vec<Email>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/emails".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserDeleteEmail {
    pub body: DeleteEmailOption,
}

impl Sealed for UserDeleteEmail {}
impl Endpoint for UserDeleteEmail {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: "/api/v1/user/emails".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListFollowers {}

impl Sealed for UserCurrentListFollowers {}
impl Endpoint for UserCurrentListFollowers {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/followers".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListFollowing {}

impl Sealed for UserCurrentListFollowing {}
impl Endpoint for UserCurrentListFollowing {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/following".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentCheckFollowing<'a> {
    pub username: &'a str,
}

impl Sealed for UserCurrentCheckFollowing<'_> {}
impl Endpoint for UserCurrentCheckFollowing<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/following/{}", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentPutFollow<'a> {
    pub username: &'a str,
}

impl Sealed for UserCurrentPutFollow<'_> {}
impl Endpoint for UserCurrentPutFollow<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/user/following/{}", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentDeleteFollow<'a> {
    pub username: &'a str,
}

impl Sealed for UserCurrentDeleteFollow<'_> {}
impl Endpoint for UserCurrentDeleteFollow<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/following/{}", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetVerificationToken {}

impl Sealed for GetVerificationToken {}
impl Endpoint for GetVerificationToken {
    type Response = String;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/gpg_key_token".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserVerifyGpgKey {
    pub body: VerifyGPGKeyOption,
}

impl Sealed for UserVerifyGpgKey {}
impl Endpoint for UserVerifyGpgKey {
    type Response = GPGKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/gpg_key_verify".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListGpgKeys {}

impl Sealed for UserCurrentListGpgKeys {}
impl Endpoint for UserCurrentListGpgKeys {
    type Response = (GpgKeyListHeaders, Vec<GPGKey>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/gpg_keys".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentPostGpgKey {
    pub body: CreateGPGKeyOption,
}

impl Sealed for UserCurrentPostGpgKey {}
impl Endpoint for UserCurrentPostGpgKey {
    type Response = GPGKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/gpg_keys".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentGetGpgKey {
    pub id: i64,
}

impl Sealed for UserCurrentGetGpgKey {}
impl Endpoint for UserCurrentGetGpgKey {
    type Response = GPGKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/gpg_keys/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentDeleteGpgKey {
    pub id: i64,
}

impl Sealed for UserCurrentDeleteGpgKey {}
impl Endpoint for UserCurrentDeleteGpgKey {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/gpg_keys/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListHooks {}

impl Sealed for UserListHooks {}
impl Endpoint for UserListHooks {
    type Response = (HookListHeaders, Vec<Hook>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/hooks".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCreateHook {
    pub body: CreateHookOption,
}

impl Sealed for UserCreateHook {}
impl Endpoint for UserCreateHook {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/hooks".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetHook {
    pub id: i64,
}

impl Sealed for UserGetHook {}
impl Endpoint for UserGetHook {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/hooks/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserDeleteHook {
    pub id: i64,
}

impl Sealed for UserDeleteHook {}
impl Endpoint for UserDeleteHook {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/hooks/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserEditHook {
    pub id: i64,
    pub body: EditHookOption,
}

impl Sealed for UserEditHook {}
impl Endpoint for UserEditHook {
    type Response = Hook;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: format!("/api/v1/user/hooks/{}", self.id).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListKeys {
    pub query: UserCurrentListKeysQuery,
}

impl Sealed for UserCurrentListKeys {}
impl Endpoint for UserCurrentListKeys {
    type Response = (PublicKeyListHeaders, Vec<PublicKey>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/keys".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentPostKey {
    pub body: CreateKeyOption,
}

impl Sealed for UserCurrentPostKey {}
impl Endpoint for UserCurrentPostKey {
    type Response = PublicKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/keys".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentGetKey {
    pub id: i64,
}

impl Sealed for UserCurrentGetKey {}
impl Endpoint for UserCurrentGetKey {
    type Response = PublicKey;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/keys/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentDeleteKey {
    pub id: i64,
}

impl Sealed for UserCurrentDeleteKey {}
impl Endpoint for UserCurrentDeleteKey {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/keys/{}", self.id).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListBlockedUsers {}

impl Sealed for UserListBlockedUsers {}
impl Endpoint for UserListBlockedUsers {
    type Response = (BlockedUserListHeaders, Vec<BlockedUser>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/list_blocked".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListCurrentUserOrgs {}

impl Sealed for OrgListCurrentUserOrgs {}
impl Endpoint for OrgListCurrentUserOrgs {
    type Response = (OrganizationListHeaders, Vec<Organization>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/orgs".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetQuota {}

impl Sealed for UserGetQuota {}
impl Endpoint for UserGetQuota {
    type Response = QuotaInfo;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/quota".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListQuotaArtifacts {}

impl Sealed for UserListQuotaArtifacts {}
impl Endpoint for UserListQuotaArtifacts {
    type Response = (QuotaUsedArtifactListHeaders, Vec<QuotaUsedArtifact>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/quota/artifacts".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListQuotaAttachments {}

impl Sealed for UserListQuotaAttachments {}
impl Endpoint for UserListQuotaAttachments {
    type Response = (QuotaUsedAttachmentListHeaders, Vec<QuotaUsedAttachment>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/quota/attachments".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCheckQuota {
    pub query: UserCheckQuotaQuery,
}

impl Sealed for UserCheckQuota {}
impl Endpoint for UserCheckQuota {
    type Response = bool;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/quota/check".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListQuotaPackages {}

impl Sealed for UserListQuotaPackages {}
impl Endpoint for UserListQuotaPackages {
    type Response = (QuotaUsedPackageListHeaders, Vec<QuotaUsedPackage>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/quota/packages".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListRepos {
    pub query: UserCurrentListReposQuery,
}

impl Sealed for UserCurrentListRepos {}
impl Endpoint for UserCurrentListRepos {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/repos".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct CreateCurrentUserRepo {
    pub body: CreateRepoOption,
}

impl Sealed for CreateCurrentUserRepo {}
impl Endpoint for CreateCurrentUserRepo {
    type Response = Repository;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: "/api/v1/user/repos".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            409 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            413 => Err(ForgejoError::from(ApiError::from(response.status_code))),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetUserSettings {}

impl Sealed for GetUserSettings {}
impl Endpoint for GetUserSettings {
    type Response = UserSettings;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/settings".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UpdateUserSettings {
    pub body: UserSettingsOptions,
}

impl Sealed for UpdateUserSettings {}
impl Endpoint for UpdateUserSettings {
    type Response = UserSettings;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PATCH,
            path: "/api/v1/user/settings".into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListStarred {}

impl Sealed for UserCurrentListStarred {}
impl Endpoint for UserCurrentListStarred {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/starred".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentCheckStarring<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for UserCurrentCheckStarring<'_> {}
impl Endpoint for UserCurrentCheckStarring<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/user/starred/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentPutStar<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for UserCurrentPutStar<'_> {}
impl Endpoint for UserCurrentPutStar<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/user/starred/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentDeleteStar<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
}

impl Sealed for UserCurrentDeleteStar<'_> {}
impl Endpoint for UserCurrentDeleteStar<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/user/starred/{}/{}", self.owner, self.repo).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetStopWatches {}

impl Sealed for UserGetStopWatches {}
impl Endpoint for UserGetStopWatches {
    type Response = (StopWatchListHeaders, Vec<StopWatch>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/stopwatches".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentListSubscriptions {}

impl Sealed for UserCurrentListSubscriptions {}
impl Endpoint for UserCurrentListSubscriptions {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/subscriptions".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListTeams {}

impl Sealed for UserListTeams {}
impl Endpoint for UserListTeams {
    type Response = (TeamListHeaders, Vec<Team>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/teams".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCurrentTrackedTimes {
    pub query: UserCurrentTrackedTimesQuery,
}

impl Sealed for UserCurrentTrackedTimes {}
impl Endpoint for UserCurrentTrackedTimes {
    type Response = (TrackedTimeListHeaders, Vec<TrackedTime>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/user/times".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserUnblockUser<'a> {
    pub username: &'a str,
}

impl Sealed for UserUnblockUser<'_> {}
impl Endpoint for UserUnblockUser<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::PUT,
            path: format!("/api/v1/user/unblock/{}", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            401 => Err(json_error::<APIUnauthorizedError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIValidationError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserSearch {
    pub query: UserSearchQuery,
}

impl Sealed for UserSearch {}
impl Endpoint for UserSearch {
    type Response = UserSearchResults;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/users/search".into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGet<'a> {
    pub username: &'a str,
}

impl Sealed for UserGet<'_> {}
impl Endpoint for UserGet<'_> {
    type Response = User;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListActivityFeeds<'a> {
    pub username: &'a str,
    pub query: UserListActivityFeedsQuery,
}

impl Sealed for UserListActivityFeeds<'_> {}
impl Endpoint for UserListActivityFeeds<'_> {
    type Response = (ActivityFeedsListHeaders, Vec<Activity>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/activities/feeds", self.username).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListFollowers<'a> {
    pub username: &'a str,
}

impl Sealed for UserListFollowers<'_> {}
impl Endpoint for UserListFollowers<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/followers", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListFollowing<'a> {
    pub username: &'a str,
}

impl Sealed for UserListFollowing<'_> {}
impl Endpoint for UserListFollowing<'_> {
    type Response = (UserListHeaders, Vec<User>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/following", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCheckFollowing<'a> {
    pub username: &'a str,
    pub target: &'a str,
}

impl Sealed for UserCheckFollowing<'_> {}
impl Endpoint for UserCheckFollowing<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/following/{}", self.username, self.target).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListGpgKeys<'a> {
    pub username: &'a str,
}

impl Sealed for UserListGpgKeys<'_> {}
impl Endpoint for UserListGpgKeys<'_> {
    type Response = (GpgKeyListHeaders, Vec<GPGKey>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/gpg_keys", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetHeatmapData<'a> {
    pub username: &'a str,
}

impl Sealed for UserGetHeatmapData<'_> {}
impl Endpoint for UserGetHeatmapData<'_> {
    type Response = Vec<UserHeatmapData>;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/heatmap", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListKeys<'a> {
    pub username: &'a str,
    pub query: UserListKeysQuery,
}

impl Sealed for UserListKeys<'_> {}
impl Endpoint for UserListKeys<'_> {
    type Response = (PublicKeyListHeaders, Vec<PublicKey>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/keys", self.username).into(),
            query: Some(self.query.into_list()),
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgListUserOrgs<'a> {
    pub username: &'a str,
}

impl Sealed for OrgListUserOrgs<'_> {}
impl Endpoint for OrgListUserOrgs<'_> {
    type Response = (OrganizationListHeaders, Vec<Organization>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/orgs", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct OrgGetUserPermissions<'a> {
    pub username: &'a str,
    pub org: &'a str,
}

impl Sealed for OrgGetUserPermissions<'_> {}
impl Endpoint for OrgGetUserPermissions<'_> {
    type Response = OrganizationPermissions;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!(
                "/api/v1/users/{}/orgs/{}/permissions",
                self.username, self.org
            )
            .into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListRepos<'a> {
    pub username: &'a str,
}

impl Sealed for UserListRepos<'_> {}
impl Endpoint for UserListRepos<'_> {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/repos", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListStarred<'a> {
    pub username: &'a str,
}

impl Sealed for UserListStarred<'_> {}
impl Endpoint for UserListStarred<'_> {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/starred", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserListSubscriptions<'a> {
    pub username: &'a str,
}

impl Sealed for UserListSubscriptions<'_> {}
impl Endpoint for UserListSubscriptions<'_> {
    type Response = (RepositoryListHeaders, Vec<Repository>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/subscriptions", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserGetTokens<'a> {
    pub username: &'a str,
}

impl Sealed for UserGetTokens<'_> {}
impl Endpoint for UserGetTokens<'_> {
    type Response = (AccessTokenListHeaders, Vec<AccessToken>);
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: format!("/api/v1/users/{}/tokens", self.username).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserCreateToken<'a> {
    pub username: &'a str,
    pub body: CreateAccessTokenOption,
}

impl Sealed for UserCreateToken<'_> {}
impl Endpoint for UserCreateToken<'_> {
    type Response = AccessToken;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::POST,
            path: format!("/api/v1/users/{}/tokens", self.username).into(),
            query: None,
            body: RequestBody::Json(Bytes::from(
                serde_json::to_vec(&self.body)
                    .expect("failed to serialize value. This is probably a bug in forgejo-api"),
            )),
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            201 => Ok((response, true)),
            400 => Err(json_error::<APIError>(&response)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct UserDeleteAccessToken<'a> {
    pub username: &'a str,
    pub token: &'a str,
}

impl Sealed for UserDeleteAccessToken<'_> {}
impl Endpoint for UserDeleteAccessToken<'_> {
    type Response = ();
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::DELETE,
            path: format!("/api/v1/users/{}/tokens/{}", self.username, self.token).into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            204 => Ok((response, false)),
            403 => Err(json_error::<APIForbiddenError>(&response)),
            404 => Err(json_error::<APINotFound>(&response)),
            422 => Err(json_error::<APIError>(&response)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}

pub struct GetVersion {}

impl Sealed for GetVersion {}
impl Endpoint for GetVersion {
    type Response = ServerVersion;
    fn make_request(self) -> ApiRequest {
        ApiRequest {
            method: Method::GET,
            path: "/api/v1/version".into(),
            query: None,
            body: RequestBody::None,
            page: None,
            limit: None,
        }
    }

    fn handle_error(response: ApiResponse) -> Result<(ApiResponse, bool), ForgejoError> {
        match response.status_code.as_u16() {
            200 => Ok((response, true)),
            _ => Err(ForgejoError::UnexpectedStatusCode(
                response.status_code.clone(),
            )),
        }
    }
}
