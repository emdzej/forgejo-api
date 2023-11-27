use super::*;

impl Forgejo {
    pub async fn notifications(&self, query: NotificationQuery) -> Result<Vec<NotificationThread>, ForgejoError> {
        self.get(&format!("notifications?{}", query.query_string())).await
    }

    pub async fn set_notifications_state(&self, query: NotificationPutQuery) -> Result<Vec<NotificationThread>, ForgejoError> {
        self.put(&format!("notifications?{}", query.query_string())).await
    }

    pub async fn notification_count(&self) -> Result<Vec<NotificationCount>, ForgejoError> {
        self.get("notifications/new").await
    }

    pub async fn get_notification(&self, id: u64) -> Result<Option<NotificationThread>, ForgejoError> {
        self.get_opt(&format!("notifications/threads/{id}")).await
    }

    pub async fn set_notification_state(&self, id: u64, to_status: ToStatus) -> Result<Option<NotificationThread>, ForgejoError> {
        self.patch(&format!("notifications/threads/{id}?to-status={}", to_status.as_str()), &()).await
    }

    pub async fn get_repo_notifications(&self, owner: &str, name: &str, query: NotificationQuery) -> Result<Vec<NotificationThread>, ForgejoError> {
        self.get(&format!("repos/{owner}/{name}/notifications?{}", query.query_string())).await
    }

    pub async fn set_repo_notifications_state(&self, owner: &str, name: &str, query: NotificationPutQuery) -> Result<Vec<NotificationThread>, ForgejoError> {
        self.put(&format!("repos/{owner}/{name}/notifications?{}", query.query_string())).await
    }
}

#[derive(Debug)]
pub struct NotificationQuery {
    pub all: bool,
    pub include_unread: bool,
    pub include_read: bool,
    pub include_pinned: bool,
    pub subject_type: Option<NotificationSubjectType>,
    pub since: Option<time::OffsetDateTime>,
    pub before: Option<time::OffsetDateTime>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl Default for NotificationQuery {
    fn default() -> Self {
        NotificationQuery {
            all: false,
            include_unread: true,
            include_read: false,
            include_pinned: true,
            subject_type: None,
            since: None,
            before: None,
            page: None,
            limit: None,
        }
    }
}

impl NotificationQuery {
    fn query_string(&self) -> String {
        use std::fmt::Write;
        let mut s = String::new();
        if self.all {
            s.push_str("all=true&");
        }
        if self.include_unread {
            s.push_str("status-types=unread&");
        }
        if self.include_read {
            s.push_str("status-types=read&");
        }
        if self.include_pinned {
            s.push_str("status-types=pinned&");
        }
        if let Some(subject_type) = self.subject_type {
            s.push_str("subject-type=");
            s.push_str(subject_type.as_str());
            s.push('&');
        }
        if let Some(since) = &self.since {
            s.push_str("since=");
            s.push_str(&since.format(&time::format_description::well_known::Rfc3339).unwrap());
            s.push('&');
        }
        if let Some(before) = &self.before {
            s.push_str("before=");
            s.push_str(&before.format(&time::format_description::well_known::Rfc3339).unwrap());
            s.push('&');
        }
        if let Some(page) = self.page {
            s.push_str("page=");
            s.write_fmt(format_args!("{page}"))
                .expect("writing to a string never fails");
            s.push('&');
        }
        if let Some(limit) = self.limit {
            s.push_str("limit=");
            s.write_fmt(format_args!("{limit}"))
                .expect("writing to a string never fails");
        }
        s
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NotificationSubjectType {
    Issue,
    Pull,
    Commit,
    Repository,
}

impl NotificationSubjectType {
    fn as_str(&self) -> &'static str {
        match self {
            NotificationSubjectType::Issue => "issue",
            NotificationSubjectType::Pull => "pull",
            NotificationSubjectType::Commit => "commit",
            NotificationSubjectType::Repository => "repository",
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NotificationThread {
    pub id: u64,
    pub pinned: bool,
    pub repository: Repository,
    pub subject: NotificationSubject,
    pub unread: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
    pub url: Url,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NotificationSubject {
    pub html_url: Url,
    pub latest_comment_html_url: Url,
    pub latest_comment_url: Url,
    pub state: String,
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: Url,
}

#[derive(Debug)]
pub struct NotificationPutQuery {
    pub last_read_at: Option<time::OffsetDateTime>,
    pub all: bool,
    pub include_unread: bool,
    pub include_read: bool,
    pub include_pinned: bool,
    pub to_status: ToStatus,
}

impl Default for NotificationPutQuery {
    fn default() -> Self {
        NotificationPutQuery {
            last_read_at: None,
            all: false,
            include_unread: true,
            include_read: false,
            include_pinned: false,
            to_status: ToStatus::default(),
        }
    }
}

impl NotificationPutQuery {
    fn query_string(&self) -> String {
        let mut s = String::new();
        if let Some(last_read_at) = &self.last_read_at {
            s.push_str("since=");
            s.push_str(&last_read_at.format(&time::format_description::well_known::Rfc3339).unwrap());
            s.push('&');
        }
        if self.all {
            s.push_str("all=true&");
        }
        if self.include_unread {
            s.push_str("status-types=unread&");
        }
        if self.include_read {
            s.push_str("status-types=read&");
        }
        if self.include_pinned {
            s.push_str("status-types=pinned&");
        }
        s.push_str("subject-type=");
        s.push_str(self.to_status.as_str());
        s
    }
}

#[derive(Default, Debug)]
pub enum ToStatus {
    #[default]
    Read,
    Unread,
    Pinned,
}

impl ToStatus {
    fn as_str(&self) -> &'static str {
        match self {
            ToStatus::Read => "read",
            ToStatus::Unread => "unread",
            ToStatus::Pinned => "pinned",
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct NotificationCount {
    pub new: u64,
}
