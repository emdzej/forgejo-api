use crate::ForgejoError;
impl crate::Forgejo {
    /// Returns the Person actor for a user
    ///
    /// - `user-id`: user ID of the user
    pub async fn activitypub_person(&self, user_id: u32) -> Result<ActivityPub, ForgejoError> {
        let request = self
            .get(&format!("activitypub/user-id/{user_id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Send to the inbox
    ///
    /// - `user-id`: user ID of the user
    pub async fn activitypub_person_inbox(&self, user_id: u32) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("activitypub/user-id/{user_id}/inbox"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List cron tasks
    ///
    pub async fn admin_cron_list(
        &self,
        query: AdminCronListQuery,
    ) -> Result<Vec<Cron>, ForgejoError> {
        let request = self.get(&format!("admin/cron?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Run cron task
    ///
    /// - `task`: task to run
    pub async fn admin_cron_run(&self, task: &str) -> Result<(), ForgejoError> {
        let request = self.post(&format!("admin/cron/{task}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all emails
    ///
    pub async fn admin_get_all_emails(
        &self,
        query: AdminGetAllEmailsQuery,
    ) -> Result<Vec<Email>, ForgejoError> {
        let request = self.get(&format!("admin/emails?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Search all emails
    ///
    pub async fn admin_search_emails(
        &self,
        query: AdminSearchEmailsQuery,
    ) -> Result<Vec<Email>, ForgejoError> {
        let request = self.get(&format!("admin/emails/search?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List system's webhooks
    ///
    pub async fn admin_list_hooks(
        &self,
        query: AdminListHooksQuery,
    ) -> Result<Vec<Hook>, ForgejoError> {
        let request = self.get(&format!("admin/hooks?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a hook
    ///
    /// - `body`: See [`CreateHookOption`]
    pub async fn admin_create_hook(&self, body: CreateHookOption) -> Result<Hook, ForgejoError> {
        let request = self.post("admin/hooks").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a hook
    ///
    /// - `id`: id of the hook to get
    pub async fn admin_get_hook(&self, id: u64) -> Result<Hook, ForgejoError> {
        let request = self.get(&format!("admin/hooks/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a hook
    ///
    /// - `id`: id of the hook to delete
    pub async fn admin_delete_hook(&self, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("admin/hooks/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a hook
    ///
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub async fn admin_edit_hook(
        &self,
        id: u64,
        body: EditHookOption,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .patch(&format!("admin/hooks/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all organizations
    ///
    pub async fn admin_get_all_orgs(
        &self,
        query: AdminGetAllOrgsQuery,
    ) -> Result<Vec<Organization>, ForgejoError> {
        let request = self.get(&format!("admin/orgs?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List unadopted repositories
    ///
    pub async fn admin_unadopted_list(
        &self,
        query: AdminUnadoptedListQuery,
    ) -> Result<Vec<String>, ForgejoError> {
        let request = self.get(&format!("admin/unadopted?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Adopt unadopted files as a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn admin_adopt_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("admin/unadopted/{owner}/{repo}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete unadopted files
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn admin_delete_unadopted_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("admin/unadopted/{owner}/{repo}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Search users according filter conditions
    ///
    pub async fn admin_search_users(
        &self,
        query: AdminSearchUsersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self.get(&format!("admin/users?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a user
    ///
    /// - `body`: See [`CreateUserOption`]
    pub async fn admin_create_user(&self, body: CreateUserOption) -> Result<User, ForgejoError> {
        let request = self.post("admin/users").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a user
    ///
    /// - `username`: username of user to delete
    pub async fn admin_delete_user(
        &self,
        username: &str,
        query: AdminDeleteUserQuery,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("admin/users/{username}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit an existing user
    ///
    /// - `username`: username of user to edit
    /// - `body`: See [`EditUserOption`]
    pub async fn admin_edit_user(
        &self,
        username: &str,
        body: EditUserOption,
    ) -> Result<User, ForgejoError> {
        let request = self
            .patch(&format!("admin/users/{username}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a public key on behalf of a user
    ///
    /// - `username`: username of the user
    /// - `key`: See [`CreateKeyOption`]
    pub async fn admin_create_public_key(
        &self,
        username: &str,
        key: CreateKeyOption,
    ) -> Result<PublicKey, ForgejoError> {
        let request = self
            .post(&format!("admin/users/{username}/keys"))
            .json(&key)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a user's public key
    ///
    /// - `username`: username of user
    /// - `id`: id of the key to delete
    pub async fn admin_delete_user_public_key(
        &self,
        username: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("admin/users/{username}/keys/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create an organization
    ///
    /// - `username`: username of the user that will own the created organization
    /// - `organization`: See [`CreateOrgOption`]
    pub async fn admin_create_org(
        &self,
        username: &str,
        organization: CreateOrgOption,
    ) -> Result<Organization, ForgejoError> {
        let request = self
            .post(&format!("admin/users/{username}/orgs"))
            .json(&organization)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Rename a user
    ///
    /// - `username`: existing username of user
    /// - `body`: See [`RenameUserOption`]
    pub async fn admin_rename_user(
        &self,
        username: &str,
        body: RenameUserOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("admin/users/{username}/rename"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a repository on behalf of a user
    ///
    /// - `username`: username of the user. This user will own the created repository
    /// - `repository`: See [`CreateRepoOption`]
    pub async fn admin_create_repo(
        &self,
        username: &str,
        repository: CreateRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("admin/users/{username}/repos"))
            .json(&repository)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns a list of all gitignore templates
    pub async fn list_gitignores_templates(&self) -> Result<Vec<String>, ForgejoError> {
        let request = self.get("gitignore/templates").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns information about a gitignore template
    ///
    /// - `name`: name of the template
    pub async fn get_gitignore_template_info(
        &self,
        name: &str,
    ) -> Result<GitignoreTemplateInfo, ForgejoError> {
        let request = self.get(&format!("gitignore/templates/{name}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns a list of all label templates
    pub async fn list_label_templates(&self) -> Result<Vec<String>, ForgejoError> {
        let request = self.get("label/templates").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns all labels in a template
    ///
    /// - `name`: name of the template
    pub async fn get_label_template_info(
        &self,
        name: &str,
    ) -> Result<Vec<LabelTemplate>, ForgejoError> {
        let request = self.get(&format!("label/templates/{name}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns a list of all license templates
    pub async fn list_license_templates(
        &self,
    ) -> Result<Vec<LicensesTemplateListEntry>, ForgejoError> {
        let request = self.get("licenses").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns information about a license template
    ///
    /// - `name`: name of the license
    pub async fn get_license_template_info(
        &self,
        name: &str,
    ) -> Result<LicenseTemplateInfo, ForgejoError> {
        let request = self.get(&format!("licenses/{name}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Render a markdown document as HTML
    ///
    /// - `body`: See [`MarkdownOption`]
    pub async fn render_markdown(&self, body: MarkdownOption) -> Result<String, ForgejoError> {
        let request = self.post("markdown").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Render raw markdown as HTML
    ///
    /// - `body`: Request body to render

    ///   See [`String`]
    pub async fn render_markdown_raw(&self, body: String) -> Result<String, ForgejoError> {
        let request = self.post("markdown/raw").body(body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Render a markup document as HTML
    ///
    /// - `body`: See [`MarkupOption`]
    pub async fn render_markup(&self, body: MarkupOption) -> Result<String, ForgejoError> {
        let request = self.post("markup").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns the nodeinfo of the Gitea application
    pub async fn get_node_info(&self) -> Result<NodeInfo, ForgejoError> {
        let request = self.get("nodeinfo").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List users's notification threads
    ///
    pub async fn notify_get_list(
        &self,
        query: NotifyGetListQuery,
    ) -> Result<Vec<NotificationThread>, ForgejoError> {
        let request = self.get(&format!("notifications?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Mark notification threads as read, pinned or unread
    ///
    pub async fn notify_read_list(
        &self,
        query: NotifyReadListQuery,
    ) -> Result<Vec<NotificationThread>, ForgejoError> {
        let request = self.put(&format!("notifications?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            205 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if unread notifications exist
    pub async fn notify_new_available(&self) -> Result<NotificationCount, ForgejoError> {
        let request = self.get("notifications/new").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get notification thread by ID
    ///
    /// - `id`: id of notification thread
    pub async fn notify_get_thread(&self, id: &str) -> Result<NotificationThread, ForgejoError> {
        let request = self.get(&format!("notifications/threads/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Mark notification thread as read by ID
    ///
    /// - `id`: id of notification thread
    pub async fn notify_read_thread(
        &self,
        id: &str,
        query: NotifyReadThreadQuery,
    ) -> Result<NotificationThread, ForgejoError> {
        let request = self
            .patch(&format!("notifications/threads/{id}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            205 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a repository in an organization
    ///
    /// - `org`: name of organization
    /// - `body`: See [`CreateRepoOption`]
    pub async fn create_org_repo_deprecated(
        &self,
        org: &str,
        body: CreateRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self.post(&format!("org/{org}/repos")).json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get list of organizations
    ///
    pub async fn org_get_all(
        &self,
        query: OrgGetAllQuery,
    ) -> Result<Vec<Organization>, ForgejoError> {
        let request = self.get(&format!("orgs?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create an organization
    ///
    /// - `organization`: See [`CreateOrgOption`]
    pub async fn org_create(
        &self,
        organization: CreateOrgOption,
    ) -> Result<Organization, ForgejoError> {
        let request = self.post("orgs").json(&organization).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an organization
    ///
    /// - `org`: name of the organization to get
    pub async fn org_get(&self, org: &str) -> Result<Organization, ForgejoError> {
        let request = self.get(&format!("orgs/{org}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete an organization
    ///
    /// - `org`: organization that is to be deleted
    pub async fn org_delete(&self, org: &str) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("orgs/{org}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit an organization
    ///
    /// - `org`: name of the organization to edit
    /// - `body`: See [`EditOrgOption`]
    pub async fn org_edit(
        &self,
        org: &str,
        body: EditOrgOption,
    ) -> Result<Organization, ForgejoError> {
        let request = self.patch(&format!("orgs/{org}")).json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's actions secrets
    ///
    /// - `org`: name of the organization
    pub async fn org_list_actions_secrets(
        &self,
        org: &str,
        query: OrgListActionsSecretsQuery,
    ) -> Result<Vec<Secret>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/actions/secrets?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create or Update a secret value in an organization
    ///
    /// - `org`: name of organization
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub async fn update_org_secret(
        &self,
        org: &str,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("orgs/{org}/actions/secrets/{secretname}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a secret in an organization
    ///
    /// - `org`: name of organization
    /// - `secretname`: name of the secret
    pub async fn delete_org_secret(&self, org: &str, secretname: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("orgs/{org}/actions/secrets/{secretname}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's activity feeds
    ///
    /// - `org`: name of the org
    pub async fn org_list_activity_feeds(
        &self,
        org: &str,
        query: OrgListActivityFeedsQuery,
    ) -> Result<Vec<Activity>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/activities/feeds?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update Avatar
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`UpdateUserAvatarOption`]
    pub async fn org_update_avatar(
        &self,
        org: &str,
        body: UpdateUserAvatarOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("orgs/{org}/avatar"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete Avatar
    ///
    /// - `org`: name of the organization
    pub async fn org_delete_avatar(&self, org: &str) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("orgs/{org}/avatar")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Blocks a user from the organization
    ///
    /// - `org`: name of the org
    /// - `username`: username of the user
    pub async fn org_block_user(&self, org: &str, username: &str) -> Result<(), ForgejoError> {
        let request = self.put(&format!("orgs/{org}/block/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's webhooks
    ///
    /// - `org`: name of the organization
    pub async fn org_list_hooks(
        &self,
        org: &str,
        query: OrgListHooksQuery,
    ) -> Result<Vec<Hook>, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/hooks?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a hook
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateHookOption`]
    pub async fn org_create_hook(
        &self,
        org: &str,
        body: CreateHookOption,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .post(&format!("orgs/{org}/hooks"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to get
    pub async fn org_get_hook(&self, org: &str, id: u64) -> Result<Hook, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/hooks/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to delete
    pub async fn org_delete_hook(&self, org: &str, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("orgs/{org}/hooks/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub async fn org_edit_hook(
        &self,
        org: &str,
        id: u64,
        body: EditHookOption,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .patch(&format!("orgs/{org}/hooks/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's labels
    ///
    /// - `org`: name of the organization
    pub async fn org_list_labels(
        &self,
        org: &str,
        query: OrgListLabelsQuery,
    ) -> Result<Vec<Label>, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/labels?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a label for an organization
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateLabelOption`]
    pub async fn org_create_label(
        &self,
        org: &str,
        body: CreateLabelOption,
    ) -> Result<Label, ForgejoError> {
        let request = self
            .post(&format!("orgs/{org}/labels"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a single label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to get
    pub async fn org_get_label(&self, org: &str, id: u64) -> Result<Label, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/labels/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to delete
    pub async fn org_delete_label(&self, org: &str, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("orgs/{org}/labels/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to edit
    /// - `body`: See [`EditLabelOption`]
    pub async fn org_edit_label(
        &self,
        org: &str,
        id: u64,
        body: EditLabelOption,
    ) -> Result<Label, ForgejoError> {
        let request = self
            .patch(&format!("orgs/{org}/labels/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the organization's blocked users
    ///
    /// - `org`: name of the org
    pub async fn org_list_blocked_users(
        &self,
        org: &str,
        query: OrgListBlockedUsersQuery,
    ) -> Result<Vec<BlockedUser>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/list_blocked?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's members
    ///
    /// - `org`: name of the organization
    pub async fn org_list_members(
        &self,
        org: &str,
        query: OrgListMembersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/members?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if a user is a member of an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub async fn org_is_member(&self, org: &str, username: &str) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a member from an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub async fn org_delete_member(&self, org: &str, username: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("orgs/{org}/members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's public members
    ///
    /// - `org`: name of the organization
    pub async fn org_list_public_members(
        &self,
        org: &str,
        query: OrgListPublicMembersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/public_members?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if a user is a public member of an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub async fn org_is_public_member(
        &self,
        org: &str,
        username: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/public_members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Publicize a user's membership
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub async fn org_publicize_member(
        &self,
        org: &str,
        username: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("orgs/{org}/public_members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Conceal a user's membership
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub async fn org_conceal_member(&self, org: &str, username: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("orgs/{org}/public_members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's repos
    ///
    /// - `org`: name of the organization
    pub async fn org_list_repos(
        &self,
        org: &str,
        query: OrgListReposQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/repos?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a repository in an organization
    ///
    /// - `org`: name of organization
    /// - `body`: See [`CreateRepoOption`]
    pub async fn create_org_repo(
        &self,
        org: &str,
        body: CreateRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("orgs/{org}/repos"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an organization's teams
    ///
    /// - `org`: name of the organization
    pub async fn org_list_teams(
        &self,
        org: &str,
        query: OrgListTeamsQuery,
    ) -> Result<Vec<Team>, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/teams?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a team
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateTeamOption`]
    pub async fn org_create_team(
        &self,
        org: &str,
        body: CreateTeamOption,
    ) -> Result<Team, ForgejoError> {
        let request = self
            .post(&format!("orgs/{org}/teams"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Search for teams within an organization
    ///
    /// - `org`: name of the organization
    pub async fn team_search(
        &self,
        org: &str,
        query: TeamSearchQuery,
    ) -> Result<TeamSearchResponse, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/teams/search?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unblock a user from the organization
    ///
    /// - `org`: name of the org
    /// - `username`: username of the user
    pub async fn org_unblock_user(&self, org: &str, username: &str) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("orgs/{org}/unblock/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets all packages of an owner
    ///
    /// - `owner`: owner of the packages
    pub async fn list_packages(
        &self,
        owner: &str,
        query: ListPackagesQuery,
    ) -> Result<Vec<Package>, ForgejoError> {
        let request = self.get(&format!("packages/{owner}?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets a package
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `version`: version of the package
    pub async fn get_package(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        version: &str,
    ) -> Result<Package, ForgejoError> {
        let request = self
            .get(&format!("packages/{owner}/{type}/{name}/{version}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a package
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `version`: version of the package
    pub async fn delete_package(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        version: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("packages/{owner}/{type}/{name}/{version}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets all files of a package
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `version`: version of the package
    pub async fn list_package_files(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        version: &str,
    ) -> Result<Vec<PackageFile>, ForgejoError> {
        let request = self
            .get(&format!("packages/{owner}/{type}/{name}/{version}/files"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Search for issues across the repositories that the user has access to
    ///
    pub async fn issue_search_issues(
        &self,
        query: IssueSearchIssuesQuery,
    ) -> Result<Vec<Issue>, ForgejoError> {
        let request = self.get(&format!("repos/issues/search?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Migrate a remote git repository
    ///
    /// - `body`: See [`MigrateRepoOptions`]
    pub async fn repo_migrate(&self, body: MigrateRepoOptions) -> Result<Repository, ForgejoError> {
        let request = self.post("repos/migrate").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Search for repositories
    ///
    pub async fn repo_search(&self, query: RepoSearchQuery) -> Result<SearchResults, ForgejoError> {
        let request = self.get(&format!("repos/search?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get(&self, owner: &str, repo: &str) -> Result<Repository, ForgejoError> {
        let request = self.get(&format!("repos/{owner}/{repo}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a repository
    ///
    /// - `owner`: owner of the repo to delete
    /// - `repo`: name of the repo to delete
    pub async fn repo_delete(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("repos/{owner}/{repo}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a repository's properties. Only fields that are set will be changed.
    ///
    /// - `owner`: owner of the repo to edit
    /// - `repo`: name of the repo to edit
    /// - `body`: Properties of a repo that you can edit

    ///   See [`EditRepoOption`]
    pub async fn repo_edit(
        &self,
        owner: &str,
        repo: &str,
        body: EditRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create or Update a secret value in a repository
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub async fn update_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!(
                "repos/{owner}/{repo}/actions/secrets/{secretname}"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a secret in a repository
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    /// - `secretname`: name of the secret
    pub async fn delete_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secretname: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/actions/secrets/{secretname}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's activity feeds
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_activity_feeds(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListActivityFeedsQuery,
    ) -> Result<Vec<Activity>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/activities/feeds?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an archive of a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `archive`: the git reference for download with attached archive format (e.g. master.zip)
    pub async fn repo_get_archive(
        &self,
        owner: &str,
        repo: &str,
        archive: &str,
    ) -> Result<Vec<u8>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/archive/{archive}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.bytes().await?[..].to_vec()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Return all users that have write access and can be assigned to issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/assignees"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update avatar
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`UpdateRepoAvatarOption`]
    pub async fn repo_update_avatar(
        &self,
        owner: &str,
        repo: &str,
        body: UpdateRepoAvatarOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/avatar"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete avatar
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_delete_avatar(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/avatar"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List branch protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_branch_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<BranchProtection>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/branch_protections"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a branch protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateBranchProtectionOption`]
    pub async fn repo_create_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        body: CreateBranchProtectionOption,
    ) -> Result<BranchProtection, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/branch_protections"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a specific branch protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: name of protected branch
    pub async fn repo_get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<BranchProtection, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/branch_protections/{name}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a specific branch protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: name of protected branch
    pub async fn repo_delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/branch_protections/{name}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a branch protections for a repository. Only fields that are set will be changed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: name of protected branch
    /// - `body`: See [`EditBranchProtectionOption`]
    pub async fn repo_edit_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: EditBranchProtectionOption,
    ) -> Result<BranchProtection, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/branch_protections/{name}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's branches
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_branches(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListBranchesQuery,
    ) -> Result<Vec<Branch>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/branches?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateBranchRepoOption`]
    pub async fn repo_create_branch(
        &self,
        owner: &str,
        repo: &str,
        body: CreateBranchRepoOption,
    ) -> Result<Branch, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/branches"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Retrieve a specific branch from a repository, including its effective branch protection
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: branch to get
    pub async fn repo_get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<Branch, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/branches/{branch}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a specific branch from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: branch to delete
    pub async fn repo_delete_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/branches/{branch}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's collaborators
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListCollaboratorsQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/collaborators?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if a user is a collaborator of a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator
    pub async fn repo_check_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/collaborators/{collaborator}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a collaborator to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator to add
    /// - `body`: See [`AddCollaboratorOption`]
    pub async fn repo_add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
        body: AddCollaboratorOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!(
                "repos/{owner}/{repo}/collaborators/{collaborator}"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a collaborator from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator to delete
    pub async fn repo_delete_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/collaborators/{collaborator}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get repository permissions for a user
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator
    pub async fn repo_get_repo_permissions(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> Result<RepoCollaboratorPermission, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/collaborators/{collaborator}/permission"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a list of all commits from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_all_commits(
        &self,
        owner: &str,
        repo: &str,
        query: RepoGetAllCommitsQuery,
    ) -> Result<(CommitListHeaders, Vec<Commit>), ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/commits?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok((response.headers().try_into()?, response.json().await?)),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a commit's combined status, by branch/tag/commit reference
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `ref`: name of branch/tag/commit
    pub async fn repo_get_combined_status_by_ref(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
        query: RepoGetCombinedStatusByRefQuery,
    ) -> Result<CombinedStatus, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/commits/{ref}/status?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a commit's statuses, by branch/tag/commit reference
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `ref`: name of branch/tag/commit
    pub async fn repo_list_statuses_by_ref(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
        query: RepoListStatusesByRefQuery,
    ) -> Result<Vec<CommitStatus>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/commits/{ref}/statuses?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets the metadata of all the entries of the root dir
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_contents_list(
        &self,
        owner: &str,
        repo: &str,
        query: RepoGetContentsListQuery,
    ) -> Result<Vec<ContentsResponse>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/contents?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Modify multiple files in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`ChangeFilesOptions`]
    pub async fn repo_change_files(
        &self,
        owner: &str,
        repo: &str,
        body: ChangeFilesOptions,
    ) -> Result<FilesResponse, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/contents"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets the metadata and contents (if a file) of an entry in a repository, or a list of entries if a dir
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the dir, file, symlink or submodule in the repo
    pub async fn repo_get_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetContentsQuery,
    ) -> Result<ContentsResponse, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/contents/{filepath}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the file to update
    /// - `body`: See [`UpdateFileOptions`]
    pub async fn repo_update_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: UpdateFileOptions,
    ) -> Result<FileResponse, ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/contents/{filepath}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the file to create
    /// - `body`: See [`CreateFileOptions`]
    pub async fn repo_create_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: CreateFileOptions,
    ) -> Result<FileResponse, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/contents/{filepath}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the file to delete
    /// - `body`: See [`DeleteFileOptions`]
    pub async fn repo_delete_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: DeleteFileOptions,
    ) -> Result<FileDeleteResponse, ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/contents/{filepath}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Apply diff patch to repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`UpdateFileOptions`]
    pub async fn repo_apply_diff_patch(
        &self,
        owner: &str,
        repo: &str,
        body: UpdateFileOptions,
    ) -> Result<FileResponse, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/diffpatch"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get the EditorConfig definitions of a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: filepath of file to get
    pub async fn repo_get_editor_config(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetEditorConfigQuery,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/editorconfig/{filepath}?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's forks
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn list_forks(
        &self,
        owner: &str,
        repo: &str,
        query: ListForksQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/forks?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Fork a repository
    ///
    /// - `owner`: owner of the repo to fork
    /// - `repo`: name of the repo to fork
    /// - `body`: See [`CreateForkOption`]
    pub async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        body: CreateForkOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/forks"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            202 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets the blob of a repository.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    pub async fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<GitBlobResponse, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/blobs/{sha}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a single commit from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: a git ref or commit sha
    pub async fn repo_get_single_commit(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: RepoGetSingleCommitQuery,
    ) -> Result<Commit, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/commits/{sha}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a commit's diff or patch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: SHA of the commit to get
    /// - `diffType`: whether the output is diff or patch
    pub async fn repo_download_commit_diff_or_patch(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        diff_type: &str,
    ) -> Result<String, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/git/commits/{sha}.{diff_type}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a note corresponding to a single commit from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: a git ref or commit sha
    pub async fn repo_get_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: RepoGetNoteQuery,
    ) -> Result<Note, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/notes/{sha}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get specified ref or filtered repository's refs
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_all_git_refs(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Reference>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/refs"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get specified ref or filtered repository's refs
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `ref`: part or full name of the ref
    pub async fn repo_list_git_refs(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
    ) -> Result<Vec<Reference>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/refs/{ref}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets the tag object of an annotated tag (not lightweight tags)
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the tag. The Git tags API only supports annotated tag objects, not lightweight tags.
    pub async fn get_annotated_tag(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<AnnotatedTag, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/tags/{sha}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets the tree of a repository.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    pub async fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: GetTreeQuery,
    ) -> Result<GitTreeResponse, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/git/trees/{sha}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the hooks in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_hooks(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListHooksQuery,
    ) -> Result<Vec<Hook>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/hooks?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a hook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateHookOption`]
    pub async fn repo_create_hook(
        &self,
        owner: &str,
        repo: &str,
        body: CreateHookOption,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/hooks"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the Git hooks in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_git_hooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<GitHook>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/hooks/git"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a Git hook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    pub async fn repo_get_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> Result<GitHook, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/hooks/git/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a Git hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    pub async fn repo_delete_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/hooks/git/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a Git hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    /// - `body`: See [`EditGitHookOption`]
    pub async fn repo_edit_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        body: EditGitHookOption,
    ) -> Result<GitHook, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/hooks/git/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a hook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    pub async fn repo_get_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/hooks/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to delete
    pub async fn repo_delete_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/hooks/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: index of the hook
    /// - `body`: See [`EditHookOption`]
    pub async fn repo_edit_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditHookOption,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/hooks/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Test a push webhook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to test
    pub async fn repo_test_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        query: RepoTestHookQuery,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/hooks/{id}/tests?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns the issue config for a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_issue_config(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<IssueConfig, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issue_config"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns the validation information for a issue config
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_validate_issue_config(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<IssueConfigValidation, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issue_config/validate"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get available issue templates for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_issue_templates(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<IssueTemplate>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issue_templates"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn issue_list_issues(
        &self,
        owner: &str,
        repo: &str,
        query: IssueListIssuesQuery,
    ) -> Result<Vec<Issue>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create an issue. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateIssueOption`]
    pub async fn issue_create_issue(
        &self,
        owner: &str,
        repo: &str,
        body: CreateIssueOption,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all comments in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn issue_get_repo_comments(
        &self,
        owner: &str,
        repo: &str,
        query: IssueGetRepoCommentsQuery,
    ) -> Result<Vec<Comment>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/comments?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    pub async fn issue_get_comment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Option<Comment>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/comments/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(Some(response.json().await?)),
            204 => Ok(Some(response.json().await?)),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of comment to delete
    pub async fn issue_delete_comment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/comments/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    /// - `body`: See [`EditIssueCommentOption`]
    pub async fn issue_edit_comment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditIssueCommentOption,
    ) -> Result<Option<Comment>, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/issues/comments/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(Some(response.json().await?)),
            204 => Ok(Some(response.json().await?)),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List comment's attachments
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    pub async fn issue_list_issue_comment_attachments(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Vec<Attachment>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/comments/{id}/assets"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment`: attachment to upload
    pub async fn issue_create_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment: Vec<u8>,
        query: IssueCreateIssueCommentAttachmentQuery,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/assets?{query}"
            ))
            .multipart(
                reqwest::multipart::Form::new().part(
                    "attachment",
                    reqwest::multipart::Part::bytes(attachment)
                        .file_name("file")
                        .mime_str("*/*")
                        .unwrap(),
                ),
            )
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment_id`: id of the attachment to get
    pub async fn issue_get_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/assets/{attachment_id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment_id`: id of the attachment to delete
    pub async fn issue_delete_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/assets/{attachment_id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment_id`: id of the attachment to edit
    /// - `body`: See [`EditAttachmentOptions`]
    pub async fn issue_edit_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
        body: EditAttachmentOptions,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .patch(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/assets/{attachment_id}"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a list of reactions from a comment of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    pub async fn issue_get_comment_reactions(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Vec<Reaction>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/reactions"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a reaction to a comment of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    /// - `content`: See [`EditReactionOption`]
    pub async fn issue_post_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        content: EditReactionOption,
    ) -> Result<Reaction, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/reactions"
            ))
            .json(&content)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a reaction from a comment of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    /// - `content`: See [`EditReactionOption`]
    pub async fn issue_delete_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        content: EditReactionOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/issues/comments/{id}/reactions"
            ))
            .json(&content)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's pinned issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_pinned_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Issue>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/pinned"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to get
    pub async fn issue_get_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/{index}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue to delete
    pub async fn issue_delete(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit an issue. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to edit
    /// - `body`: See [`EditIssueOption`]
    pub async fn issue_edit_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: EditIssueOption,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/issues/{index}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List issue's attachments
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_list_issue_attachments(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<Vec<Attachment>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/{index}/assets"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment`: attachment to upload
    pub async fn issue_create_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment: Vec<u8>,
        query: IssueCreateIssueAttachmentQuery,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/issues/{index}/assets?{query}"
            ))
            .multipart(
                reqwest::multipart::Form::new().part(
                    "attachment",
                    reqwest::multipart::Part::bytes(attachment)
                        .file_name("file")
                        .mime_str("*/*")
                        .unwrap(),
                ),
            )
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment_id`: id of the attachment to get
    pub async fn issue_get_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment_id: u64,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/assets/{attachment_id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment_id`: id of the attachment to delete
    pub async fn issue_delete_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment_id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/issues/{index}/assets/{attachment_id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment_id`: id of the attachment to edit
    /// - `body`: See [`EditAttachmentOptions`]
    pub async fn issue_edit_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment_id: u64,
        body: EditAttachmentOptions,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .patch(&format!(
                "repos/{owner}/{repo}/issues/{index}/assets/{attachment_id}"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List issues that are blocked by this issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_list_blocks(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        query: IssueListBlocksQuery,
    ) -> Result<Vec<Issue>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/blocks?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Block the issue given in the body by the issue in path
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub async fn issue_create_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/blocks"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unblock the issue given in the body by the issue in path
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub async fn issue_remove_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/blocks"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all comments on an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_get_comments(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueGetCommentsQuery,
    ) -> Result<Vec<Comment>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/comments?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a comment to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`CreateIssueCommentOption`]
    pub async fn issue_create_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: CreateIssueCommentOption,
    ) -> Result<Comment, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/comments"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: this parameter is ignored
    /// - `id`: id of comment to delete
    pub async fn issue_delete_comment_deprecated(
        &self,
        owner: &str,
        repo: &str,
        index: u32,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/issues/{index}/comments/{id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: this parameter is ignored
    /// - `id`: id of the comment to edit
    /// - `body`: See [`EditIssueCommentOption`]
    pub async fn issue_edit_comment_deprecated(
        &self,
        owner: &str,
        repo: &str,
        index: u32,
        id: u64,
        body: EditIssueCommentOption,
    ) -> Result<Option<Comment>, ForgejoError> {
        let request = self
            .patch(&format!(
                "repos/{owner}/{repo}/issues/{index}/comments/{id}"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(Some(response.json().await?)),
            204 => Ok(Some(response.json().await?)),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Set an issue deadline. If set to null, the deadline is deleted. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to create or update a deadline on
    /// - `body`: See [`EditDeadlineOption`]
    pub async fn issue_edit_issue_deadline(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: EditDeadlineOption,
    ) -> Result<IssueDeadline, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/deadline"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an issue's dependencies, i.e all issues that block this issue.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_list_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        query: IssueListIssueDependenciesQuery,
    ) -> Result<Vec<Issue>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/dependencies?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Make the issue in the url depend on the issue in the form.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub async fn issue_create_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/dependencies"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove an issue dependency
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub async fn issue_remove_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> Result<Issue, ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/dependencies"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an issue's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_get_labels(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<Vec<Label>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/issues/{index}/labels"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Replace an issue's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueLabelsOption`]
    pub async fn issue_replace_labels(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: IssueLabelsOption,
    ) -> Result<Vec<Label>, ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/issues/{index}/labels"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a label to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueLabelsOption`]
    pub async fn issue_add_label(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: IssueLabelsOption,
    ) -> Result<Vec<Label>, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/labels"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove all labels from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`DeleteLabelsOption`]
    pub async fn issue_clear_labels(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: DeleteLabelsOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/labels"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a label from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `id`: id of the label to remove
    /// - `body`: See [`DeleteLabelsOption`]
    pub async fn issue_remove_label(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: DeleteLabelsOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/labels/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Pin an Issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue to pin
    pub async fn pin_issue(&self, owner: &str, repo: &str, index: u64) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/pin"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unpin an Issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue to unpin
    pub async fn unpin_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/pin"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Moves the Pin to the given Position
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue
    /// - `position`: the new position
    pub async fn move_issue_pin(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        position: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .patch(&format!(
                "repos/{owner}/{repo}/issues/{index}/pin/{position}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a list reactions of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_get_issue_reactions(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueGetIssueReactionsQuery,
    ) -> Result<Vec<Reaction>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/reactions?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a reaction to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `content`: See [`EditReactionOption`]
    pub async fn issue_post_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        content: EditReactionOption,
    ) -> Result<Reaction, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/reactions"))
            .json(&content)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a reaction from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `content`: See [`EditReactionOption`]
    pub async fn issue_delete_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        content: EditReactionOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/reactions"))
            .json(&content)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete an issue's existing stopwatch.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to stop the stopwatch on
    pub async fn issue_delete_stop_watch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/issues/{index}/stopwatch/delete"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Start stopwatch on an issue.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to create the stopwatch on
    pub async fn issue_start_stop_watch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/issues/{index}/stopwatch/start"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Stop an issue's existing stopwatch.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to stop the stopwatch on
    pub async fn issue_stop_stop_watch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/issues/{index}/stopwatch/stop"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get users who subscribed on an issue.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_subscriptions(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueSubscriptionsQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/subscriptions?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if user is subscribed to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_check_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<WatchInfo, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/subscriptions/check"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Subscribe user to issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `user`: user to subscribe
    pub async fn issue_add_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        user: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!(
                "repos/{owner}/{repo}/issues/{index}/subscriptions/{user}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            201 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unsubscribe user from issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `user`: user witch unsubscribe
    pub async fn issue_delete_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        user: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/issues/{index}/subscriptions/{user}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            201 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all comments and events on an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_get_comments_and_timeline(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueGetCommentsAndTimelineQuery,
    ) -> Result<Vec<TimelineComment>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/timeline?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an issue's tracked times
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub async fn issue_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueTrackedTimesQuery,
    ) -> Result<Vec<TrackedTime>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/issues/{index}/times?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add tracked time to a issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`AddTimeOption`]
    pub async fn issue_add_time(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: AddTimeOption,
    ) -> Result<TrackedTime, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/issues/{index}/times"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Reset a tracked time of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to add tracked time to
    pub async fn issue_reset_time(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/times"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete specific tracked time
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `id`: id of time to delete
    pub async fn issue_delete_time(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/issues/{index}/times/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's keys
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_keys(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListKeysQuery,
    ) -> Result<Vec<DeployKey>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/keys?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a key to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateKeyOption`]
    pub async fn repo_create_key(
        &self,
        owner: &str,
        repo: &str,
        body: CreateKeyOption,
    ) -> Result<DeployKey, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/keys"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a repository's key by id
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the key to get
    pub async fn repo_get_key(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<DeployKey, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/keys/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a key from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the key to delete
    pub async fn repo_delete_key(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/keys/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get all of a repository's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn issue_list_labels(
        &self,
        owner: &str,
        repo: &str,
        query: IssueListLabelsQuery,
    ) -> Result<Vec<Label>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/labels?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateLabelOption`]
    pub async fn issue_create_label(
        &self,
        owner: &str,
        repo: &str,
        body: CreateLabelOption,
    ) -> Result<Label, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/labels"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a single label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the label to get
    pub async fn issue_get_label(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Label, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/labels/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the label to delete
    pub async fn issue_delete_label(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/labels/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the label to edit
    /// - `body`: See [`EditLabelOption`]
    pub async fn issue_edit_label(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditLabelOption,
    ) -> Result<Label, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/labels/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get languages and number of bytes of code written
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<RepoGetLanguagesResponse, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/languages"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a file or it's LFS object from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: filepath of the file to get
    pub async fn repo_get_raw_file_or_lfs(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetRawFileOrLfsQuery,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/media/{filepath}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get all of a repository's opened milestones
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn issue_get_milestones_list(
        &self,
        owner: &str,
        repo: &str,
        query: IssueGetMilestonesListQuery,
    ) -> Result<Vec<Milestone>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/milestones?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateMilestoneOption`]
    pub async fn issue_create_milestone(
        &self,
        owner: &str,
        repo: &str,
        body: CreateMilestoneOption,
    ) -> Result<Milestone, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/milestones"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: the milestone to get, identified by ID and if not available by name
    pub async fn issue_get_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> Result<Milestone, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/milestones/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: the milestone to delete, identified by ID and if not available by name
    pub async fn issue_delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/milestones/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: the milestone to edit, identified by ID and if not available by name
    /// - `body`: See [`EditMilestoneOption`]
    pub async fn issue_edit_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        body: EditMilestoneOption,
    ) -> Result<Milestone, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/milestones/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Sync a mirrored repository
    ///
    /// - `owner`: owner of the repo to sync
    /// - `repo`: name of the repo to sync
    pub async fn repo_mirror_sync(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/mirror-sync"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns if new Issue Pins are allowed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_new_pin_allowed(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<NewIssuePinsAllowed, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/new_pin_allowed"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List users's notification threads on a specific repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn notify_get_repo_list(
        &self,
        owner: &str,
        repo: &str,
        query: NotifyGetRepoListQuery,
    ) -> Result<Vec<NotificationThread>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/notifications?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Mark notification threads as read, pinned or unread on a specific repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn notify_read_repo_list(
        &self,
        owner: &str,
        repo: &str,
        query: NotifyReadRepoListQuery,
    ) -> Result<Vec<NotificationThread>, ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/notifications?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            205 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's pull requests
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListPullRequestsQuery,
    ) -> Result<Vec<PullRequest>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreatePullRequestOption`]
    pub async fn repo_create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        body: CreatePullRequestOption,
    ) -> Result<PullRequest, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/pulls"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's pinned pull requests
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_pinned_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<PullRequest>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls/pinned"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub async fn repo_get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<PullRequest, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls/{index}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a pull request. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to edit
    /// - `body`: See [`EditPullRequestOption`]
    pub async fn repo_edit_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: EditPullRequestOption,
    ) -> Result<PullRequest, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/pulls/{index}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a pull request diff or patch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    /// - `diffType`: whether the output is diff or patch
    pub async fn repo_download_pull_diff_or_patch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        diff_type: &str,
        query: RepoDownloadPullDiffOrPatchQuery,
    ) -> Result<String, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/pulls/{index}.{diff_type}?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get commits for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub async fn repo_get_pull_request_commits(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoGetPullRequestCommitsQuery,
    ) -> Result<(CommitListHeaders, Vec<Commit>), ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/pulls/{index}/commits?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok((response.headers().try_into()?, response.json().await?)),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get changed files for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub async fn repo_get_pull_request_files(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoGetPullRequestFilesQuery,
    ) -> Result<(ChangedFileListHeaders, Vec<ChangedFile>), ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls/{index}/files?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok((response.headers().try_into()?, response.json().await?)),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if a pull request has been merged
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    pub async fn repo_pull_request_is_merged(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls/{index}/merge"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Merge a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to merge
    /// - `body`: See [`MergePullRequestOption`]
    pub async fn repo_merge_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: MergePullRequestOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/pulls/{index}/merge"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Cancel the scheduled auto merge for the given pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to merge
    pub async fn repo_cancel_scheduled_auto_merge(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/pulls/{index}/merge"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// create review requests for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`PullReviewRequestOptions`]
    pub async fn repo_create_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: PullReviewRequestOptions,
    ) -> Result<Vec<PullReview>, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/pulls/{index}/requested_reviewers"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// cancel review requests for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`PullReviewRequestOptions`]
    pub async fn repo_delete_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: PullReviewRequestOptions,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/pulls/{index}/requested_reviewers"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all reviews for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    pub async fn repo_list_pull_reviews(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoListPullReviewsQuery,
    ) -> Result<Vec<PullReview>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a review to an pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`CreatePullReviewOptions`]
    pub async fn repo_create_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: CreatePullReviewOptions,
    ) -> Result<PullReview, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/pulls/{index}/reviews"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a specific review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub async fn repo_get_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> Result<PullReview, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls/{index}/reviews/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Submit a pending review to an pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `body`: See [`SubmitPullReviewOptions`]
    pub async fn repo_submit_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: SubmitPullReviewOptions,
    ) -> Result<PullReview, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/pulls/{index}/reviews/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a specific review from a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub async fn repo_delete_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/pulls/{index}/reviews/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a specific review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub async fn repo_get_pull_review_comments(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> Result<Vec<PullReviewComment>, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Dismiss a review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `body`: See [`DismissPullReviewOptions`]
    pub async fn repo_dismiss_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: DismissPullReviewOptions,
    ) -> Result<PullReview, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews/{id}/dismissals"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Cancel to dismiss a review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub async fn repo_un_dismiss_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> Result<PullReview, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews/{id}/undismissals"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Merge PR's baseBranch into headBranch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub async fn repo_update_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoUpdatePullRequestQuery,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/pulls/{index}/update?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get all push mirrors of the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_push_mirrors(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListPushMirrorsQuery,
    ) -> Result<Vec<PushMirror>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/push_mirrors?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// add a push mirror to the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreatePushMirrorOption`]
    pub async fn repo_add_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        body: CreatePushMirrorOption,
    ) -> Result<PushMirror, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/push_mirrors"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Sync all push mirrored repository
    ///
    /// - `owner`: owner of the repo to sync
    /// - `repo`: name of the repo to sync
    pub async fn repo_push_mirror_sync(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/push_mirrors-sync"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get push mirror of the repository by remoteName
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: remote name of push mirror
    pub async fn repo_get_push_mirror_by_remote_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<PushMirror, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/push_mirrors/{name}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// deletes a push mirror from a repository by remoteName
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: remote name of the pushMirror
    pub async fn repo_delete_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/push_mirrors/{name}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a file from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: filepath of the file to get
    pub async fn repo_get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetRawFileQuery,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/raw/{filepath}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's releases
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_releases(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListReleasesQuery,
    ) -> Result<Vec<Release>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/releases?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateReleaseOption`]
    pub async fn repo_create_release(
        &self,
        owner: &str,
        repo: &str,
        body: CreateReleaseOption,
    ) -> Result<Release, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/releases"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Gets the most recent non-prerelease, non-draft release of a repository, sorted by created_at
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Release, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/releases/latest"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a release by tag name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: tag name of the release to get
    pub async fn repo_get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<Release, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/releases/tags/{tag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a release by tag name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: tag name of the release to delete
    pub async fn repo_delete_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/releases/tags/{tag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release to get
    pub async fn repo_get_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Release, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/releases/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release to delete
    pub async fn repo_delete_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/releases/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release to edit
    /// - `body`: See [`EditReleaseOption`]
    pub async fn repo_edit_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditReleaseOption,
    ) -> Result<Release, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/releases/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List release's attachments
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    pub async fn repo_list_release_attachments(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> Result<Vec<Attachment>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/releases/{id}/assets"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment`: attachment to upload
    pub async fn repo_create_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment: Vec<u8>,
        query: RepoCreateReleaseAttachmentQuery,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/releases/{id}/assets?{query}"
            ))
            .multipart(
                reqwest::multipart::Form::new().part(
                    "attachment",
                    reqwest::multipart::Part::bytes(attachment)
                        .file_name("file")
                        .mime_str("*/*")
                        .unwrap(),
                ),
            )
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment_id`: id of the attachment to get
    pub async fn repo_get_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/releases/{id}/assets/{attachment_id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment_id`: id of the attachment to delete
    pub async fn repo_delete_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/releases/{id}/assets/{attachment_id}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment_id`: id of the attachment to edit
    /// - `body`: See [`EditAttachmentOptions`]
    pub async fn repo_edit_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
        body: EditAttachmentOptions,
    ) -> Result<Attachment, ForgejoError> {
        let request = self
            .patch(&format!(
                "repos/{owner}/{repo}/releases/{id}/assets/{attachment_id}"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Return all users that can be requested to review in this repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_reviewers(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/reviewers"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get signing-key.gpg for given repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_signing_key(&self, owner: &str, repo: &str) -> Result<String, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/signing-key.gpg"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's stargazers
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_stargazers(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListStargazersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/stargazers?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a commit's statuses
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    pub async fn repo_list_statuses(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: RepoListStatusesQuery,
    ) -> Result<Vec<CommitStatus>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/statuses/{sha}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a commit status
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    /// - `body`: See [`CreateStatusOption`]
    pub async fn repo_create_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: CreateStatusOption,
    ) -> Result<CommitStatus, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/statuses/{sha}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's watchers
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_subscribers(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListSubscribersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/subscribers?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if the current user is watching a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn user_current_check_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<WatchInfo, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/subscription"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Watch a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn user_current_put_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<WatchInfo, ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/subscription"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unwatch a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn user_current_delete_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/subscription"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's tags
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_tags(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListTagsQuery,
    ) -> Result<Vec<Tag>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/tags?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a new git tag in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateTagOption`]
    pub async fn repo_create_tag(
        &self,
        owner: &str,
        repo: &str,
        body: CreateTagOption,
    ) -> Result<Tag, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/tags"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get the tag of a repository by tag name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: name of tag
    pub async fn repo_get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<Tag, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/tags/{tag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a repository's tag by name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: name of tag to delete
    pub async fn repo_delete_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/tags/{tag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repository's teams
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Team>, ForgejoError> {
        let request = self.get(&format!("repos/{owner}/{repo}/teams")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if a team is assigned to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `team`: team name
    pub async fn repo_check_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> Result<Team, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/teams/{team}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a team to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `team`: team name
    pub async fn repo_add_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/teams/{team}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a team from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `team`: team name
    pub async fn repo_delete_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/teams/{team}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a repo's tracked times
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        query: RepoTrackedTimesQuery,
    ) -> Result<Vec<TrackedTime>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/times?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a user's tracked times in a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `user`: username of user
    pub async fn user_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        user: &str,
    ) -> Result<Vec<TrackedTime>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/times/{user}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get list of topics that a repository has
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_topics(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListTopicsQuery,
    ) -> Result<TopicName, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/topics?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Replace list of topics for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`RepoTopicOptions`]
    pub async fn repo_update_topics(
        &self,
        owner: &str,
        repo: &str,
        body: RepoTopicOptions,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/topics"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a topic to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `topic`: name of the topic to add
    pub async fn repo_add_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/topics/{topic}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a topic from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `topic`: name of the topic to delete
    pub async fn repo_delete_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/topics/{topic}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Transfer a repo ownership
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    /// - `body`: Transfer Options

    ///   See [`TransferRepoOption`]
    pub async fn repo_transfer(
        &self,
        owner: &str,
        repo: &str,
        body: TransferRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/transfer"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            202 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Accept a repo transfer
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    pub async fn accept_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/transfer/accept"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            202 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Reject a repo transfer
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    pub async fn reject_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/transfer/reject"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateWikiPageOptions`]
    pub async fn repo_create_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        body: CreateWikiPageOptions,
    ) -> Result<WikiPage, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/wiki/new"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    pub async fn repo_get_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> Result<WikiPage, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/wiki/page/{page_name}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    pub async fn repo_delete_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/wiki/page/{page_name}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    /// - `body`: See [`CreateWikiPageOptions`]
    pub async fn repo_edit_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
        body: CreateWikiPageOptions,
    ) -> Result<WikiPage, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/wiki/page/{page_name}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get all wiki pages
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_wiki_pages(
        &self,
        owner: &str,
        repo: &str,
        query: RepoGetWikiPagesQuery,
    ) -> Result<Vec<WikiPageMetaData>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/wiki/pages?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get revisions of a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    pub async fn repo_get_wiki_page_revisions(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
        query: RepoGetWikiPageRevisionsQuery,
    ) -> Result<WikiCommitList, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/wiki/revisions/{page_name}?{query}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a repository using a template
    ///
    /// - `template_owner`: name of the template repository owner
    /// - `template_repo`: name of the template repository
    /// - `body`: See [`GenerateRepoOption`]
    pub async fn generate_repo(
        &self,
        template_owner: &str,
        template_repo: &str,
        body: GenerateRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .post(&format!("repos/{template_owner}/{template_repo}/generate"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a repository by id
    ///
    /// - `id`: id of the repo to get
    pub async fn repo_get_by_id(&self, id: u64) -> Result<Repository, ForgejoError> {
        let request = self.get(&format!("repositories/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get instance's global settings for api
    pub async fn get_general_api_settings(&self) -> Result<GeneralAPISettings, ForgejoError> {
        let request = self.get("settings/api").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get instance's global settings for Attachment
    pub async fn get_general_attachment_settings(
        &self,
    ) -> Result<GeneralAttachmentSettings, ForgejoError> {
        let request = self.get("settings/attachment").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get instance's global settings for repositories
    pub async fn get_general_repository_settings(
        &self,
    ) -> Result<GeneralRepoSettings, ForgejoError> {
        let request = self.get("settings/repository").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get instance's global settings for ui
    pub async fn get_general_ui_settings(&self) -> Result<GeneralUISettings, ForgejoError> {
        let request = self.get("settings/ui").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get default signing-key.gpg
    pub async fn get_signing_key(&self) -> Result<String, ForgejoError> {
        let request = self.get("signing-key.gpg").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a team
    ///
    /// - `id`: id of the team to get
    pub async fn org_get_team(&self, id: u64) -> Result<Team, ForgejoError> {
        let request = self.get(&format!("teams/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a team
    ///
    /// - `id`: id of the team to delete
    pub async fn org_delete_team(&self, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("teams/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a team
    ///
    /// - `id`: id of the team to edit
    /// - `body`: See [`EditTeamOption`]
    pub async fn org_edit_team(&self, id: u32, body: EditTeamOption) -> Result<Team, ForgejoError> {
        let request = self.patch(&format!("teams/{id}")).json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a team's activity feeds
    ///
    /// - `id`: id of the team
    pub async fn org_list_team_activity_feeds(
        &self,
        id: u64,
        query: OrgListTeamActivityFeedsQuery,
    ) -> Result<Vec<Activity>, ForgejoError> {
        let request = self
            .get(&format!("teams/{id}/activities/feeds?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a team's members
    ///
    /// - `id`: id of the team
    pub async fn org_list_team_members(
        &self,
        id: u64,
        query: OrgListTeamMembersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self.get(&format!("teams/{id}/members?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a particular member of team
    ///
    /// - `id`: id of the team
    /// - `username`: username of the member to list
    pub async fn org_list_team_member(
        &self,
        id: u64,
        username: &str,
    ) -> Result<User, ForgejoError> {
        let request = self
            .get(&format!("teams/{id}/members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a team member
    ///
    /// - `id`: id of the team
    /// - `username`: username of the user to add
    pub async fn org_add_team_member(&self, id: u64, username: &str) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("teams/{id}/members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a team member
    ///
    /// - `id`: id of the team
    /// - `username`: username of the user to remove
    pub async fn org_remove_team_member(
        &self,
        id: u64,
        username: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("teams/{id}/members/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a team's repos
    ///
    /// - `id`: id of the team
    pub async fn org_list_team_repos(
        &self,
        id: u64,
        query: OrgListTeamReposQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self.get(&format!("teams/{id}/repos?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a particular repo of team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to list
    /// - `repo`: name of the repo to list
    pub async fn org_list_team_repo(
        &self,
        id: u64,
        org: &str,
        repo: &str,
    ) -> Result<Repository, ForgejoError> {
        let request = self
            .get(&format!("teams/{id}/repos/{org}/{repo}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a repository to a team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to add
    /// - `repo`: name of the repo to add
    pub async fn org_add_team_repository(
        &self,
        id: u64,
        org: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("teams/{id}/repos/{org}/{repo}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a repository from a team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to remove
    /// - `repo`: name of the repo to remove
    pub async fn org_remove_team_repository(
        &self,
        id: u64,
        org: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("teams/{id}/repos/{org}/{repo}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// search topics via keyword
    ///
    pub async fn topic_search(
        &self,
        query: TopicSearchQuery,
    ) -> Result<Vec<TopicResponse>, ForgejoError> {
        let request = self.get(&format!("topics/search?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get the authenticated user
    pub async fn user_get_current(&self) -> Result<User, ForgejoError> {
        let request = self.get("user").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create or Update a secret value in a user scope
    ///
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub async fn update_user_secret(
        &self,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("user/actions/secrets/{secretname}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a secret in a user scope
    ///
    /// - `secretname`: name of the secret
    pub async fn delete_user_secret(&self, secretname: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("user/actions/secrets/{secretname}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's oauth2 applications
    ///
    pub async fn user_get_oauth2_applications(
        &self,
        query: UserGetOAuth2ApplicationsQuery,
    ) -> Result<Vec<OAuth2Application>, ForgejoError> {
        let request = self
            .get(&format!("user/applications/oauth2?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// creates a new OAuth2 application
    ///
    /// - `body`: See [`CreateOAuth2ApplicationOptions`]
    pub async fn user_create_oauth2_application(
        &self,
        body: CreateOAuth2ApplicationOptions,
    ) -> Result<OAuth2Application, ForgejoError> {
        let request = self.post("user/applications/oauth2").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// get an OAuth2 Application
    ///
    /// - `id`: Application ID to be found
    pub async fn user_get_oauth2_application(
        &self,
        id: u64,
    ) -> Result<OAuth2Application, ForgejoError> {
        let request = self
            .get(&format!("user/applications/oauth2/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// delete an OAuth2 Application
    ///
    /// - `id`: token to be deleted
    pub async fn user_delete_oauth2_application(&self, id: u64) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("user/applications/oauth2/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// update an OAuth2 Application, this includes regenerating the client secret
    ///
    /// - `id`: application to be updated
    /// - `body`: See [`CreateOAuth2ApplicationOptions`]
    pub async fn user_update_oauth2_application(
        &self,
        id: u64,
        body: CreateOAuth2ApplicationOptions,
    ) -> Result<OAuth2Application, ForgejoError> {
        let request = self
            .patch(&format!("user/applications/oauth2/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update Avatar
    ///
    /// - `body`: See [`UpdateUserAvatarOption`]
    pub async fn user_update_avatar(
        &self,
        body: UpdateUserAvatarOption,
    ) -> Result<(), ForgejoError> {
        let request = self.post("user/avatar").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete Avatar
    pub async fn user_delete_avatar(&self) -> Result<(), ForgejoError> {
        let request = self.delete("user/avatar").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Blocks a user from the doer.
    ///
    /// - `username`: username of the user
    pub async fn user_block_user(&self, username: &str) -> Result<(), ForgejoError> {
        let request = self.put(&format!("user/block/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's email addresses
    pub async fn user_list_emails(&self) -> Result<Vec<Email>, ForgejoError> {
        let request = self.get("user/emails").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add email addresses
    ///
    /// - `body`: See [`CreateEmailOption`]
    pub async fn user_add_email(
        &self,
        body: CreateEmailOption,
    ) -> Result<Vec<Email>, ForgejoError> {
        let request = self.post("user/emails").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete email addresses
    ///
    /// - `body`: See [`DeleteEmailOption`]
    pub async fn user_delete_email(&self, body: DeleteEmailOption) -> Result<(), ForgejoError> {
        let request = self.delete("user/emails").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's followers
    ///
    pub async fn user_current_list_followers(
        &self,
        query: UserCurrentListFollowersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self.get(&format!("user/followers?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the users that the authenticated user is following
    ///
    pub async fn user_current_list_following(
        &self,
        query: UserCurrentListFollowingQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self.get(&format!("user/following?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check whether a user is followed by the authenticated user
    ///
    /// - `username`: username of followed user
    pub async fn user_current_check_following(&self, username: &str) -> Result<(), ForgejoError> {
        let request = self.get(&format!("user/following/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Follow a user
    ///
    /// - `username`: username of user to follow
    pub async fn user_current_put_follow(&self, username: &str) -> Result<(), ForgejoError> {
        let request = self.put(&format!("user/following/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unfollow a user
    ///
    /// - `username`: username of user to unfollow
    pub async fn user_current_delete_follow(&self, username: &str) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("user/following/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a Token to verify
    pub async fn get_verification_token(&self) -> Result<String, ForgejoError> {
        let request = self.get("user/gpg_key_token").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.text().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Verify a GPG key
    pub async fn user_verify_gpg_key(&self) -> Result<GPGKey, ForgejoError> {
        let request = self.post("user/gpg_key_verify").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's GPG keys
    ///
    pub async fn user_current_list_gpg_keys(
        &self,
        query: UserCurrentListGpgKeysQuery,
    ) -> Result<Vec<GPGKey>, ForgejoError> {
        let request = self.get(&format!("user/gpg_keys?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a GPG key
    ///
    /// - `Form`: See [`CreateGPGKeyOption`]
    pub async fn user_current_post_gpg_key(
        &self,
        form: CreateGPGKeyOption,
    ) -> Result<GPGKey, ForgejoError> {
        let request = self.post("user/gpg_keys").json(&form).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a GPG key
    ///
    /// - `id`: id of key to get
    pub async fn user_current_get_gpg_key(&self, id: u64) -> Result<GPGKey, ForgejoError> {
        let request = self.get(&format!("user/gpg_keys/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a GPG key
    ///
    /// - `id`: id of key to delete
    pub async fn user_current_delete_gpg_key(&self, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("user/gpg_keys/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's webhooks
    ///
    pub async fn user_list_hooks(
        &self,
        query: UserListHooksQuery,
    ) -> Result<Vec<Hook>, ForgejoError> {
        let request = self.get(&format!("user/hooks?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a hook
    ///
    /// - `body`: See [`CreateHookOption`]
    pub async fn user_create_hook(&self, body: CreateHookOption) -> Result<Hook, ForgejoError> {
        let request = self.post("user/hooks").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a hook
    ///
    /// - `id`: id of the hook to get
    pub async fn user_get_hook(&self, id: u64) -> Result<Hook, ForgejoError> {
        let request = self.get(&format!("user/hooks/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a hook
    ///
    /// - `id`: id of the hook to delete
    pub async fn user_delete_hook(&self, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("user/hooks/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a hook
    ///
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub async fn user_edit_hook(
        &self,
        id: u64,
        body: EditHookOption,
    ) -> Result<Hook, ForgejoError> {
        let request = self
            .patch(&format!("user/hooks/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's public keys
    ///
    pub async fn user_current_list_keys(
        &self,
        query: UserCurrentListKeysQuery,
    ) -> Result<Vec<PublicKey>, ForgejoError> {
        let request = self.get(&format!("user/keys?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a public key
    ///
    /// - `body`: See [`CreateKeyOption`]
    pub async fn user_current_post_key(
        &self,
        body: CreateKeyOption,
    ) -> Result<PublicKey, ForgejoError> {
        let request = self.post("user/keys").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a public key
    ///
    /// - `id`: id of key to get
    pub async fn user_current_get_key(&self, id: u64) -> Result<PublicKey, ForgejoError> {
        let request = self.get(&format!("user/keys/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a public key
    ///
    /// - `id`: id of key to delete
    pub async fn user_current_delete_key(&self, id: u64) -> Result<(), ForgejoError> {
        let request = self.delete(&format!("user/keys/{id}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's blocked users
    ///
    pub async fn user_list_blocked_users(
        &self,
        query: UserListBlockedUsersQuery,
    ) -> Result<Vec<BlockedUser>, ForgejoError> {
        let request = self.get(&format!("user/list_blocked?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the current user's organizations
    ///
    pub async fn org_list_current_user_orgs(
        &self,
        query: OrgListCurrentUserOrgsQuery,
    ) -> Result<Vec<Organization>, ForgejoError> {
        let request = self.get(&format!("user/orgs?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the repos that the authenticated user owns
    ///
    pub async fn user_current_list_repos(
        &self,
        query: UserCurrentListReposQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self.get(&format!("user/repos?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a repository
    ///
    /// - `body`: See [`CreateRepoOption`]
    pub async fn create_current_user_repo(
        &self,
        body: CreateRepoOption,
    ) -> Result<Repository, ForgejoError> {
        let request = self.post("user/repos").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get user settings
    pub async fn get_user_settings(&self) -> Result<Vec<UserSettings>, ForgejoError> {
        let request = self.get("user/settings").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update user settings
    ///
    /// - `body`: See [`UserSettingsOptions`]
    pub async fn update_user_settings(
        &self,
        body: UserSettingsOptions,
    ) -> Result<Vec<UserSettings>, ForgejoError> {
        let request = self.patch("user/settings").json(&body).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// The repos that the authenticated user has starred
    ///
    pub async fn user_current_list_starred(
        &self,
        query: UserCurrentListStarredQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self.get(&format!("user/starred?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Whether the authenticated is starring the repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn user_current_check_starring(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self.get(&format!("user/starred/{owner}/{repo}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Star the given repo
    ///
    /// - `owner`: owner of the repo to star
    /// - `repo`: name of the repo to star
    pub async fn user_current_put_star(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        let request = self.put(&format!("user/starred/{owner}/{repo}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unstar the given repo
    ///
    /// - `owner`: owner of the repo to unstar
    /// - `repo`: name of the repo to unstar
    pub async fn user_current_delete_star(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("user/starred/{owner}/{repo}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get list of all existing stopwatches
    ///
    pub async fn user_get_stop_watches(
        &self,
        query: UserGetStopWatchesQuery,
    ) -> Result<Vec<StopWatch>, ForgejoError> {
        let request = self.get(&format!("user/stopwatches?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List repositories watched by the authenticated user
    ///
    pub async fn user_current_list_subscriptions(
        &self,
        query: UserCurrentListSubscriptionsQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self.get(&format!("user/subscriptions?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List all the teams a user belongs to
    ///
    pub async fn user_list_teams(
        &self,
        query: UserListTeamsQuery,
    ) -> Result<Vec<Team>, ForgejoError> {
        let request = self.get(&format!("user/teams?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the current user's tracked times
    ///
    pub async fn user_current_tracked_times(
        &self,
        query: UserCurrentTrackedTimesQuery,
    ) -> Result<Vec<TrackedTime>, ForgejoError> {
        let request = self.get(&format!("user/times?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Unblocks a user from the doer.
    ///
    /// - `username`: username of the user
    pub async fn user_unblock_user(&self, username: &str) -> Result<(), ForgejoError> {
        let request = self.put(&format!("user/unblock/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Search for users
    ///
    pub async fn user_search(
        &self,
        query: UserSearchQuery,
    ) -> Result<UserSearchResponse, ForgejoError> {
        let request = self.get(&format!("users/search?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a user
    ///
    /// - `username`: username of user to get
    pub async fn user_get(&self, username: &str) -> Result<User, ForgejoError> {
        let request = self.get(&format!("users/{username}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a user's activity feeds
    ///
    /// - `username`: username of user
    pub async fn user_list_activity_feeds(
        &self,
        username: &str,
        query: UserListActivityFeedsQuery,
    ) -> Result<Vec<Activity>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/activities/feeds?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the given user's followers
    ///
    /// - `username`: username of user
    pub async fn user_list_followers(
        &self,
        username: &str,
        query: UserListFollowersQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/followers?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the users that the given user is following
    ///
    /// - `username`: username of user
    pub async fn user_list_following(
        &self,
        username: &str,
        query: UserListFollowingQuery,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/following?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if one user is following another user
    ///
    /// - `username`: username of following user
    /// - `target`: username of followed user
    pub async fn user_check_following(
        &self,
        username: &str,
        target: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("users/{username}/following/{target}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the given user's GPG keys
    ///
    /// - `username`: username of user
    pub async fn user_list_gpg_keys(
        &self,
        username: &str,
        query: UserListGpgKeysQuery,
    ) -> Result<Vec<GPGKey>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/gpg_keys?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a user's heatmap
    ///
    /// - `username`: username of user to get
    pub async fn user_get_heatmap_data(
        &self,
        username: &str,
    ) -> Result<Vec<UserHeatmapData>, ForgejoError> {
        let request = self.get(&format!("users/{username}/heatmap")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the given user's public keys
    ///
    /// - `username`: username of user
    pub async fn user_list_keys(
        &self,
        username: &str,
        query: UserListKeysQuery,
    ) -> Result<Vec<PublicKey>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/keys?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List a user's organizations
    ///
    /// - `username`: username of user
    pub async fn org_list_user_orgs(
        &self,
        username: &str,
        query: OrgListUserOrgsQuery,
    ) -> Result<Vec<Organization>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/orgs?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get user permissions in organization
    ///
    /// - `username`: username of user
    /// - `org`: name of the organization
    pub async fn org_get_user_permissions(
        &self,
        username: &str,
        org: &str,
    ) -> Result<OrganizationPermissions, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/orgs/{org}/permissions"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the repos owned by the given user
    ///
    /// - `username`: username of user
    pub async fn user_list_repos(
        &self,
        username: &str,
        query: UserListReposQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/repos?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// The repos that the given user has starred
    ///
    /// - `username`: username of user
    pub async fn user_list_starred(
        &self,
        username: &str,
        query: UserListStarredQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/starred?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the repositories watched by a user
    ///
    /// - `username`: username of the user
    pub async fn user_list_subscriptions(
        &self,
        username: &str,
        query: UserListSubscriptionsQuery,
    ) -> Result<Vec<Repository>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/subscriptions?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the authenticated user's access tokens
    ///
    /// - `username`: username of user
    pub async fn user_get_tokens(
        &self,
        username: &str,
        query: UserGetTokensQuery,
    ) -> Result<Vec<AccessToken>, ForgejoError> {
        let request = self
            .get(&format!("users/{username}/tokens?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create an access token
    ///
    /// - `username`: username of user
    /// - `body`: See [`CreateAccessTokenOption`]
    pub async fn user_create_token(
        &self,
        username: &str,
        body: CreateAccessTokenOption,
    ) -> Result<AccessToken, ForgejoError> {
        let request = self
            .post(&format!("users/{username}/tokens"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// delete an access token
    ///
    /// - `username`: username of user
    /// - `token`: token to be deleted, identified by ID and if not available by name
    pub async fn user_delete_access_token(
        &self,
        username: &str,
        token: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("users/{username}/tokens/{token}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns the version of the Gitea application
    pub async fn get_version(&self) -> Result<ServerVersion, ForgejoError> {
        let request = self.get("version").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }
}
use structs::*;
pub mod structs {
    use crate::StructureError;
    /// APIError is an api error with a message
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct APIError {
        pub message: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct AccessToken {
        pub id: Option<u64>,
        pub name: Option<String>,
        pub scopes: Option<Vec<String>>,
        pub sha1: Option<String>,
        pub token_last_eight: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Activity {
        pub act_user: Option<User>,
        pub act_user_id: Option<u64>,
        pub comment: Option<Comment>,
        pub comment_id: Option<u64>,
        pub content: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub id: Option<u64>,
        pub is_private: Option<bool>,
        pub op_type: Option<String>,
        pub ref_name: Option<String>,
        pub repo: Option<Repository>,
        pub repo_id: Option<u64>,
        pub user_id: Option<u64>,
    }

    /// ActivityPub type
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ActivityPub {
        #[serde(rename = "@context")]
        pub context: Option<String>,
    }

    /// AddCollaboratorOption options when adding a user as a collaborator of a repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct AddCollaboratorOption {
        pub permission: Option<String>,
    }

    /// AddTimeOption options for adding time to an issue
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct AddTimeOption {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        /// time in seconds
        pub time: u64,
        /// User who spent the time (optional)
        pub user_name: Option<String>,
    }

    /// AnnotatedTag represents an annotated tag
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct AnnotatedTag {
        pub message: Option<String>,
        pub object: Option<AnnotatedTagObject>,
        pub sha: Option<String>,
        pub tag: Option<String>,
        pub tagger: Option<CommitUser>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub verification: Option<PayloadCommitVerification>,
    }

    /// AnnotatedTagObject contains meta information of the tag object
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct AnnotatedTagObject {
        pub sha: Option<String>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// Attachment a generic attachment
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Attachment {
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub browser_download_url: Option<url::Url>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub download_count: Option<u64>,
        pub id: Option<u64>,
        pub name: Option<String>,
        pub size: Option<u64>,
        pub uuid: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct BlockedUser {
        pub block_id: Option<u64>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
    }

    /// Branch represents a repository branch
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Branch {
        pub commit: Option<PayloadCommit>,
        pub effective_branch_protection_name: Option<String>,
        pub enable_status_check: Option<bool>,
        pub name: Option<String>,
        pub protected: Option<bool>,
        pub required_approvals: Option<u64>,
        pub status_check_contexts: Option<Vec<String>>,
        pub user_can_merge: Option<bool>,
        pub user_can_push: Option<bool>,
    }

    /// BranchProtection represents a branch protection for a repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct BranchProtection {
        pub approvals_whitelist_teams: Option<Vec<String>>,
        pub approvals_whitelist_username: Option<Vec<String>>,
        pub block_on_official_review_requests: Option<bool>,
        pub block_on_outdated_branch: Option<bool>,
        pub block_on_rejected_reviews: Option<bool>,
        /// Deprecated: true
        pub branch_name: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub dismiss_stale_approvals: Option<bool>,
        pub enable_approvals_whitelist: Option<bool>,
        pub enable_merge_whitelist: Option<bool>,
        pub enable_push: Option<bool>,
        pub enable_push_whitelist: Option<bool>,
        pub enable_status_check: Option<bool>,
        pub merge_whitelist_teams: Option<Vec<String>>,
        pub merge_whitelist_usernames: Option<Vec<String>>,
        pub protected_file_patterns: Option<String>,
        pub push_whitelist_deploy_keys: Option<bool>,
        pub push_whitelist_teams: Option<Vec<String>>,
        pub push_whitelist_usernames: Option<Vec<String>>,
        pub require_signed_commits: Option<bool>,
        pub required_approvals: Option<u64>,
        pub rule_name: Option<String>,
        pub status_check_contexts: Option<Vec<String>>,
        pub unprotected_file_patterns: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// ChangeFileOperation for creating, updating or deleting a file
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ChangeFileOperation {
        /// new or updated file content, must be base64 encoded
        pub content: Option<String>,
        /// old path of the file to move
        pub from_path: Option<String>,
        /// indicates what to do with the file
        pub operation: ChangeFileOperationOperation,
        /// path to the existing or new file
        pub path: String,
        /// sha is the SHA for the file that already exists, required for update or delete
        pub sha: Option<String>,
    }

    /// indicates what to do with the file

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum ChangeFileOperationOperation {
        Create,
        Update,
        Delete,
    }

    impl ChangeFileOperationOperation {
        fn as_str(&self) -> &'static str {
            match self {
                ChangeFileOperationOperation::Create => "create",
                ChangeFileOperationOperation::Update => "update",
                ChangeFileOperationOperation::Delete => "delete",
            }
        }
    }
    /// ChangeFilesOptions options for creating, updating or deleting multiple files
    ///
    /// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ChangeFilesOptions {
        pub author: Option<Identity>,
        /// branch (optional) to base this file from. if not given, the default branch is used
        pub branch: Option<String>,
        pub committer: Option<Identity>,
        pub dates: Option<CommitDateOptions>,
        /// list of file operations
        pub files: Vec<ChangeFileOperation>,
        /// message (optional) for the commit of this file. if not supplied, a default message will be used
        pub message: Option<String>,
        /// new_branch (optional) will make a new branch from `branch` before creating the file
        pub new_branch: Option<String>,
        /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
        pub signoff: Option<bool>,
    }

    /// ChangedFile store information about files affected by the pull request
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ChangedFile {
        pub additions: Option<u64>,
        pub changes: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub contents_url: Option<url::Url>,
        pub deletions: Option<u64>,
        pub filename: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub previous_filename: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub raw_url: Option<url::Url>,
        pub status: Option<String>,
    }

    /// CombinedStatus holds the combined state of several statuses for a single commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CombinedStatus {
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub commit_url: Option<url::Url>,
        pub repository: Option<Repository>,
        pub sha: Option<String>,
        pub state: Option<String>,
        pub statuses: Option<Vec<CommitStatus>>,
        pub total_count: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// Comment represents a comment on a commit or issue
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Comment {
        pub assets: Option<Vec<Attachment>>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub issue_url: Option<url::Url>,
        pub original_author: Option<String>,
        pub original_author_id: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub pull_request_url: Option<url::Url>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        pub user: Option<User>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Commit {
        pub author: Option<User>,
        pub commit: Option<RepoCommit>,
        pub committer: Option<User>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub files: Option<Vec<CommitAffectedFiles>>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub parents: Option<Vec<CommitMeta>>,
        pub sha: Option<String>,
        pub stats: Option<CommitStats>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// CommitAffectedFiles store information about files affected by the commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitAffectedFiles {
        pub filename: Option<String>,
        pub status: Option<String>,
    }

    /// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitDateOptions {
        #[serde(with = "time::serde::rfc3339::option")]
        pub author: Option<time::OffsetDateTime>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub committer: Option<time::OffsetDateTime>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitMeta {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub sha: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// CommitStats is statistics for a RepoCommit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitStats {
        pub additions: Option<u64>,
        pub deletions: Option<u64>,
        pub total: Option<u64>,
    }

    /// CommitStatus holds a single status of a single Commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitStatus {
        pub context: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub creator: Option<User>,
        pub description: Option<String>,
        pub id: Option<u64>,
        pub status: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub target_url: Option<url::Url>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// CommitStatusState holds the state of a CommitStatus
    ///
    /// It can be "pending", "success", "error" and "failure"
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitStatusState {}

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CommitUser {
        pub date: Option<String>,
        pub email: Option<String>,
        pub name: Option<String>,
    }

    /// ContentsResponse contains information about a repo's entry's (dir, file, symlink, submodule) metadata and content
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ContentsResponse {
        #[serde(rename = "_links")]
        pub links: Option<FileLinksResponse>,
        /// `content` is populated when `type` is `file`, otherwise null
        pub content: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub download_url: Option<url::Url>,
        /// `encoding` is populated when `type` is `file`, otherwise null
        pub encoding: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub git_url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub last_commit_sha: Option<String>,
        pub name: Option<String>,
        pub path: Option<String>,
        pub sha: Option<String>,
        pub size: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        /// `submodule_git_url` is populated when `type` is `submodule`, otherwise null
        pub submodule_git_url: Option<url::Url>,
        /// `target` is populated when `type` is `symlink`, otherwise null
        pub target: Option<String>,
        /// `type` will be `file`, `dir`, `symlink`, or `submodule`
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// CreateAccessTokenOption options when create access token
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateAccessTokenOption {
        pub name: String,
        pub scopes: Option<Vec<String>>,
    }

    /// CreateBranchProtectionOption options for creating a branch protection
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateBranchProtectionOption {
        pub approvals_whitelist_teams: Option<Vec<String>>,
        pub approvals_whitelist_username: Option<Vec<String>>,
        pub block_on_official_review_requests: Option<bool>,
        pub block_on_outdated_branch: Option<bool>,
        pub block_on_rejected_reviews: Option<bool>,
        /// Deprecated: true
        pub branch_name: Option<String>,
        pub dismiss_stale_approvals: Option<bool>,
        pub enable_approvals_whitelist: Option<bool>,
        pub enable_merge_whitelist: Option<bool>,
        pub enable_push: Option<bool>,
        pub enable_push_whitelist: Option<bool>,
        pub enable_status_check: Option<bool>,
        pub merge_whitelist_teams: Option<Vec<String>>,
        pub merge_whitelist_usernames: Option<Vec<String>>,
        pub protected_file_patterns: Option<String>,
        pub push_whitelist_deploy_keys: Option<bool>,
        pub push_whitelist_teams: Option<Vec<String>>,
        pub push_whitelist_usernames: Option<Vec<String>>,
        pub require_signed_commits: Option<bool>,
        pub required_approvals: Option<u64>,
        pub rule_name: Option<String>,
        pub status_check_contexts: Option<Vec<String>>,
        pub unprotected_file_patterns: Option<String>,
    }

    /// CreateBranchRepoOption options when creating a branch in a repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateBranchRepoOption {
        /// Name of the branch to create
        pub new_branch_name: String,
        /// Deprecated: true
        ///
        /// Name of the old branch to create from
        pub old_branch_name: Option<String>,
        /// Name of the old branch/tag/commit to create from
        pub old_ref_name: Option<String>,
    }

    /// CreateEmailOption options when creating email addresses
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateEmailOption {
        /// email addresses to add
        pub emails: Option<Vec<String>>,
    }

    /// CreateFileOptions options for creating files
    ///
    /// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateFileOptions {
        pub author: Option<Identity>,
        /// branch (optional) to base this file from. if not given, the default branch is used
        pub branch: Option<String>,
        pub committer: Option<Identity>,
        /// content must be base64 encoded
        pub content: String,
        pub dates: Option<CommitDateOptions>,
        /// message (optional) for the commit of this file. if not supplied, a default message will be used
        pub message: Option<String>,
        /// new_branch (optional) will make a new branch from `branch` before creating the file
        pub new_branch: Option<String>,
        /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
        pub signoff: Option<bool>,
    }

    /// CreateForkOption options for creating a fork
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateForkOption {
        /// name of the forked repository
        pub name: Option<String>,
        /// organization name, if forking into an organization
        pub organization: Option<String>,
    }

    /// CreateGPGKeyOption options create user GPG key
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateGPGKeyOption {
        /// An armored GPG key to add
        pub armored_public_key: String,
        pub armored_signature: Option<String>,
    }

    /// CreateHookOption options when create a hook
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateHookOption {
        pub active: Option<bool>,
        pub authorization_header: Option<String>,
        pub branch_filter: Option<String>,
        pub config: CreateHookOptionConfig,
        pub events: Option<Vec<String>>,
        #[serde(rename = "type")]
        pub r#type: CreateHookOptionType,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum CreateHookOptionType {
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
        Packagist,
    }

    impl CreateHookOptionType {
        fn as_str(&self) -> &'static str {
            match self {
                CreateHookOptionType::Forgejo => "forgejo",
                CreateHookOptionType::Dingtalk => "dingtalk",
                CreateHookOptionType::Discord => "discord",
                CreateHookOptionType::Gitea => "gitea",
                CreateHookOptionType::Gogs => "gogs",
                CreateHookOptionType::Msteams => "msteams",
                CreateHookOptionType::Slack => "slack",
                CreateHookOptionType::Telegram => "telegram",
                CreateHookOptionType::Feishu => "feishu",
                CreateHookOptionType::Wechatwork => "wechatwork",
                CreateHookOptionType::Packagist => "packagist",
            }
        }
    }
    /// CreateHookOptionConfig has all config options in it
    ///
    /// required are "content_type" and "url" Required
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateHookOptionConfig {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, String>,
    }

    /// CreateIssueCommentOption options for creating a comment on an issue
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateIssueCommentOption {
        pub body: String,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// CreateIssueOption options to create one issue
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateIssueOption {
        /// deprecated
        pub assignee: Option<String>,
        pub assignees: Option<Vec<String>>,
        pub body: Option<String>,
        pub closed: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
        /// list of label ids
        pub labels: Option<Vec<u64>>,
        /// milestone id
        pub milestone: Option<u64>,
        #[serde(rename = "ref")]
        pub r#ref: Option<String>,
        pub title: String,
    }

    /// CreateKeyOption options when creating a key
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateKeyOption {
        /// An armored SSH key to add
        pub key: String,
        /// Describe if the key has only read access or read/write
        pub read_only: Option<bool>,
        /// Title of the key to add
        pub title: String,
    }

    /// CreateLabelOption options for creating a label
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateLabelOption {
        pub color: String,
        pub description: Option<String>,
        pub exclusive: Option<bool>,
        pub is_archived: Option<bool>,
        pub name: String,
    }

    /// CreateMilestoneOption options for creating a milestone
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateMilestoneOption {
        pub description: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_on: Option<time::OffsetDateTime>,
        pub state: Option<CreateMilestoneOptionState>,
        pub title: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum CreateMilestoneOptionState {
        Open,
        Closed,
    }

    impl CreateMilestoneOptionState {
        fn as_str(&self) -> &'static str {
            match self {
                CreateMilestoneOptionState::Open => "open",
                CreateMilestoneOptionState::Closed => "closed",
            }
        }
    }
    /// CreateOAuth2ApplicationOptions holds options to create an oauth2 application
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateOAuth2ApplicationOptions {
        pub confidential_client: Option<bool>,
        pub name: Option<String>,
        pub redirect_uris: Option<Vec<String>>,
    }

    /// CreateOrUpdateSecretOption options when creating or updating secret
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateOrUpdateSecretOption {
        /// Data of the secret to update
        pub data: String,
    }

    /// CreateOrgOption options for creating an organization
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateOrgOption {
        pub description: Option<String>,
        pub email: Option<String>,
        pub full_name: Option<String>,
        pub location: Option<String>,
        pub repo_admin_change_team_access: Option<bool>,
        pub username: String,
        /// possible values are `public` (default), `limited` or `private`
        pub visibility: Option<CreateOrgOptionVisibility>,
        pub website: Option<String>,
    }

    /// possible values are `public` (default), `limited` or `private`

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum CreateOrgOptionVisibility {
        Public,
        Limited,
        Private,
    }

    impl CreateOrgOptionVisibility {
        fn as_str(&self) -> &'static str {
            match self {
                CreateOrgOptionVisibility::Public => "public",
                CreateOrgOptionVisibility::Limited => "limited",
                CreateOrgOptionVisibility::Private => "private",
            }
        }
    }
    /// CreatePullRequestOption options when creating a pull request
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreatePullRequestOption {
        pub assignee: Option<String>,
        pub assignees: Option<Vec<String>>,
        pub base: Option<String>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
        pub head: Option<String>,
        pub labels: Option<Vec<u64>>,
        pub milestone: Option<u64>,
        pub title: Option<String>,
    }

    /// CreatePullReviewComment represent a review comment for creation api
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreatePullReviewComment {
        pub body: Option<String>,
        /// if comment to new file line or 0
        pub new_position: Option<u64>,
        /// if comment to old file line or 0
        pub old_position: Option<u64>,
        /// the tree path
        pub path: Option<String>,
    }

    /// CreatePullReviewOptions are options to create a pull review
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreatePullReviewOptions {
        pub body: Option<String>,
        pub comments: Option<Vec<CreatePullReviewComment>>,
        pub commit_id: Option<String>,
        pub event: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreatePushMirrorOption {
        pub interval: Option<String>,
        pub remote_address: Option<String>,
        pub remote_password: Option<String>,
        pub remote_username: Option<String>,
        pub sync_on_commit: Option<bool>,
    }

    /// CreateReleaseOption options when creating a release
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateReleaseOption {
        pub body: Option<String>,
        pub draft: Option<bool>,
        pub name: Option<String>,
        pub prerelease: Option<bool>,
        pub tag_name: String,
        pub target_commitish: Option<String>,
    }

    /// CreateRepoOption options when creating repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateRepoOption {
        /// Whether the repository should be auto-initialized?
        pub auto_init: Option<bool>,
        /// DefaultBranch of the repository (used when initializes and in template)
        pub default_branch: Option<String>,
        /// Description of the repository to create
        pub description: Option<String>,
        /// Gitignores to use
        pub gitignores: Option<String>,
        /// Label-Set to use
        pub issue_labels: Option<String>,
        /// License to use
        pub license: Option<String>,
        /// Name of the repository to create
        pub name: String,
        /// Whether the repository is private
        pub private: Option<bool>,
        /// Readme of the repository to create
        pub readme: Option<String>,
        /// Whether the repository is template
        pub template: Option<bool>,
        /// TrustModel of the repository
        pub trust_model: Option<CreateRepoOptionTrustModel>,
    }

    /// TrustModel of the repository

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum CreateRepoOptionTrustModel {
        Default,
        Collaborator,
        Committer,
        Collaboratorcommitter,
    }

    impl CreateRepoOptionTrustModel {
        fn as_str(&self) -> &'static str {
            match self {
                CreateRepoOptionTrustModel::Default => "default",
                CreateRepoOptionTrustModel::Collaborator => "collaborator",
                CreateRepoOptionTrustModel::Committer => "committer",
                CreateRepoOptionTrustModel::Collaboratorcommitter => "collaboratorcommitter",
            }
        }
    }
    /// CreateStatusOption holds the information needed to create a new CommitStatus for a Commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateStatusOption {
        pub context: Option<String>,
        pub description: Option<String>,
        pub state: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub target_url: Option<url::Url>,
    }

    /// CreateTagOption options when creating a tag
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateTagOption {
        pub message: Option<String>,
        pub tag_name: String,
        pub target: Option<String>,
    }

    /// CreateTeamOption options for creating a team
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateTeamOption {
        pub can_create_org_repo: Option<bool>,
        pub description: Option<String>,
        pub includes_all_repositories: Option<bool>,
        pub name: String,
        pub permission: Option<CreateTeamOptionPermission>,
        pub units: Option<Vec<String>>,
        pub units_map: Option<CreateTeamOptionUnitsMap>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum CreateTeamOptionPermission {
        Read,
        Write,
        Admin,
    }

    impl CreateTeamOptionPermission {
        fn as_str(&self) -> &'static str {
            match self {
                CreateTeamOptionPermission::Read => "read",
                CreateTeamOptionPermission::Write => "write",
                CreateTeamOptionPermission::Admin => "admin",
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateTeamOptionUnitsMap {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, String>,
    }

    /// CreateUserOption create user options
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateUserOption {
        #[serde(with = "time::serde::rfc3339::option")]
        /// For explicitly setting the user creation timestamp. Useful when users are
        ///
        /// migrated from other systems. When omitted, the user's creation timestamp
        ///
        /// will be set to "now".
        pub created_at: Option<time::OffsetDateTime>,
        pub email: String,
        pub full_name: Option<String>,
        pub login_name: Option<String>,
        pub must_change_password: Option<bool>,
        pub password: Option<String>,
        pub restricted: Option<bool>,
        pub send_notify: Option<bool>,
        pub source_id: Option<u64>,
        pub username: String,
        pub visibility: Option<String>,
    }

    /// CreateWikiPageOptions form for creating wiki
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct CreateWikiPageOptions {
        /// content must be base64 encoded
        pub content_base64: Option<String>,
        /// optional commit message summarizing the change
        pub message: Option<String>,
        /// page title. leave empty to keep unchanged
        pub title: Option<String>,
    }

    /// Cron represents a Cron task
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Cron {
        pub exec_times: Option<u64>,
        pub name: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub next: Option<time::OffsetDateTime>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub prev: Option<time::OffsetDateTime>,
        pub schedule: Option<String>,
    }

    /// DeleteEmailOption options when deleting email addresses
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct DeleteEmailOption {
        /// email addresses to delete
        pub emails: Option<Vec<String>>,
    }

    /// DeleteFileOptions options for deleting files (used for other File structs below)
    ///
    /// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct DeleteFileOptions {
        pub author: Option<Identity>,
        /// branch (optional) to base this file from. if not given, the default branch is used
        pub branch: Option<String>,
        pub committer: Option<Identity>,
        pub dates: Option<CommitDateOptions>,
        /// message (optional) for the commit of this file. if not supplied, a default message will be used
        pub message: Option<String>,
        /// new_branch (optional) will make a new branch from `branch` before creating the file
        pub new_branch: Option<String>,
        /// sha is the SHA for the file that already exists
        pub sha: String,
        /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
        pub signoff: Option<bool>,
    }

    /// DeleteLabelOption options for deleting a label
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct DeleteLabelsOption {
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// DeployKey a deploy key
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct DeployKey {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub fingerprint: Option<String>,
        pub id: Option<u64>,
        pub key: Option<String>,
        pub key_id: Option<u64>,
        pub read_only: Option<bool>,
        pub repository: Option<Repository>,
        pub title: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// DismissPullReviewOptions are options to dismiss a pull review
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct DismissPullReviewOptions {
        pub message: Option<String>,
        pub priors: Option<bool>,
    }

    /// EditAttachmentOptions options for editing attachments
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditAttachmentOptions {
        pub name: Option<String>,
    }

    /// EditBranchProtectionOption options for editing a branch protection
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditBranchProtectionOption {
        pub approvals_whitelist_teams: Option<Vec<String>>,
        pub approvals_whitelist_username: Option<Vec<String>>,
        pub block_on_official_review_requests: Option<bool>,
        pub block_on_outdated_branch: Option<bool>,
        pub block_on_rejected_reviews: Option<bool>,
        pub dismiss_stale_approvals: Option<bool>,
        pub enable_approvals_whitelist: Option<bool>,
        pub enable_merge_whitelist: Option<bool>,
        pub enable_push: Option<bool>,
        pub enable_push_whitelist: Option<bool>,
        pub enable_status_check: Option<bool>,
        pub merge_whitelist_teams: Option<Vec<String>>,
        pub merge_whitelist_usernames: Option<Vec<String>>,
        pub protected_file_patterns: Option<String>,
        pub push_whitelist_deploy_keys: Option<bool>,
        pub push_whitelist_teams: Option<Vec<String>>,
        pub push_whitelist_usernames: Option<Vec<String>>,
        pub require_signed_commits: Option<bool>,
        pub required_approvals: Option<u64>,
        pub status_check_contexts: Option<Vec<String>>,
        pub unprotected_file_patterns: Option<String>,
    }

    /// EditDeadlineOption options for creating a deadline
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditDeadlineOption {
        #[serde(with = "time::serde::rfc3339")]
        pub due_date: time::OffsetDateTime,
    }

    /// EditGitHookOption options when modifying one Git hook
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditGitHookOption {
        pub content: Option<String>,
    }

    /// EditHookOption options when modify one hook
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditHookOption {
        pub active: Option<bool>,
        pub authorization_header: Option<String>,
        pub branch_filter: Option<String>,
        pub config: Option<EditHookOptionConfig>,
        pub events: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditHookOptionConfig {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, String>,
    }

    /// EditIssueCommentOption options for editing a comment
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditIssueCommentOption {
        pub body: String,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// EditIssueOption options for editing an issue
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditIssueOption {
        /// deprecated
        pub assignee: Option<String>,
        pub assignees: Option<Vec<String>>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
        pub milestone: Option<u64>,
        #[serde(rename = "ref")]
        pub r#ref: Option<String>,
        pub state: Option<String>,
        pub title: Option<String>,
        pub unset_due_date: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// EditLabelOption options for editing a label
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditLabelOption {
        pub color: Option<String>,
        pub description: Option<String>,
        pub exclusive: Option<bool>,
        pub is_archived: Option<bool>,
        pub name: Option<String>,
    }

    /// EditMilestoneOption options for editing a milestone
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditMilestoneOption {
        pub description: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_on: Option<time::OffsetDateTime>,
        pub state: Option<String>,
        pub title: Option<String>,
    }

    /// EditOrgOption options for editing an organization
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditOrgOption {
        pub description: Option<String>,
        pub email: Option<String>,
        pub full_name: Option<String>,
        pub location: Option<String>,
        pub repo_admin_change_team_access: Option<bool>,
        /// possible values are `public`, `limited` or `private`
        pub visibility: Option<EditOrgOptionVisibility>,
        pub website: Option<String>,
    }

    /// possible values are `public`, `limited` or `private`

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum EditOrgOptionVisibility {
        Public,
        Limited,
        Private,
    }

    impl EditOrgOptionVisibility {
        fn as_str(&self) -> &'static str {
            match self {
                EditOrgOptionVisibility::Public => "public",
                EditOrgOptionVisibility::Limited => "limited",
                EditOrgOptionVisibility::Private => "private",
            }
        }
    }
    /// EditPullRequestOption options when modify pull request
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditPullRequestOption {
        pub allow_maintainer_edit: Option<bool>,
        pub assignee: Option<String>,
        pub assignees: Option<Vec<String>>,
        pub base: Option<String>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
        pub labels: Option<Vec<u64>>,
        pub milestone: Option<u64>,
        pub state: Option<String>,
        pub title: Option<String>,
        pub unset_due_date: Option<bool>,
    }

    /// EditReactionOption contain the reaction type
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditReactionOption {
        pub content: Option<String>,
    }

    /// EditReleaseOption options when editing a release
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditReleaseOption {
        pub body: Option<String>,
        pub draft: Option<bool>,
        pub name: Option<String>,
        pub prerelease: Option<bool>,
        pub tag_name: Option<String>,
        pub target_commitish: Option<String>,
    }

    /// EditRepoOption options when editing a repository's properties
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditRepoOption {
        /// either `true` to allow mark pr as merged manually, or `false` to prevent it.
        pub allow_manual_merge: Option<bool>,
        /// either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
        pub allow_merge_commits: Option<bool>,
        /// either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
        pub allow_rebase: Option<bool>,
        /// either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits.
        pub allow_rebase_explicit: Option<bool>,
        /// either `true` to allow updating pull request branch by rebase, or `false` to prevent it.
        pub allow_rebase_update: Option<bool>,
        /// either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
        pub allow_squash_merge: Option<bool>,
        /// set to `true` to archive this repository.
        pub archived: Option<bool>,
        /// either `true` to enable AutodetectManualMerge, or `false` to prevent it. Note: In some special cases, misjudgments can occur.
        pub autodetect_manual_merge: Option<bool>,
        /// set to `true` to allow edits from maintainers by default
        pub default_allow_maintainer_edit: Option<bool>,
        /// sets the default branch for this repository.
        pub default_branch: Option<String>,
        /// set to `true` to delete pr branch after merge by default
        pub default_delete_branch_after_merge: Option<bool>,
        /// set to a merge style to be used by this repository: "merge", "rebase", "rebase-merge", or "squash".
        pub default_merge_style: Option<String>,
        /// a short description of the repository.
        pub description: Option<String>,
        /// enable prune - remove obsolete remote-tracking references
        pub enable_prune: Option<bool>,
        pub external_tracker: Option<ExternalTracker>,
        pub external_wiki: Option<ExternalWiki>,
        /// either `true` to enable actions unit, or `false` to disable them.
        pub has_actions: Option<bool>,
        /// either `true` to enable issues for this repository or `false` to disable them.
        pub has_issues: Option<bool>,
        /// either `true` to enable packages unit, or `false` to disable them.
        pub has_packages: Option<bool>,
        /// either `true` to enable project unit, or `false` to disable them.
        pub has_projects: Option<bool>,
        /// either `true` to allow pull requests, or `false` to prevent pull request.
        pub has_pull_requests: Option<bool>,
        /// either `true` to enable releases unit, or `false` to disable them.
        pub has_releases: Option<bool>,
        /// either `true` to enable the wiki for this repository or `false` to disable it.
        pub has_wiki: Option<bool>,
        /// either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace.
        pub ignore_whitespace_conflicts: Option<bool>,
        pub internal_tracker: Option<InternalTracker>,
        /// set to a string like `8h30m0s` to set the mirror interval time
        pub mirror_interval: Option<String>,
        /// name of the repository
        pub name: Option<String>,
        /// either `true` to make the repository private or `false` to make it public.
        ///
        /// Note: you will get a 422 error if the organization restricts changing repository visibility to organization
        ///
        /// owners and a non-owner tries to change the value of private.
        pub private: Option<bool>,
        /// either `true` to make this repository a template or `false` to make it a normal repository
        pub template: Option<bool>,
        /// a URL with more information about the repository.
        pub website: Option<String>,
    }

    /// EditTeamOption options for editing a team
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditTeamOption {
        pub can_create_org_repo: Option<bool>,
        pub description: Option<String>,
        pub includes_all_repositories: Option<bool>,
        pub name: String,
        pub permission: Option<EditTeamOptionPermission>,
        pub units: Option<Vec<String>>,
        pub units_map: Option<EditTeamOptionUnitsMap>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum EditTeamOptionPermission {
        Read,
        Write,
        Admin,
    }

    impl EditTeamOptionPermission {
        fn as_str(&self) -> &'static str {
            match self {
                EditTeamOptionPermission::Read => "read",
                EditTeamOptionPermission::Write => "write",
                EditTeamOptionPermission::Admin => "admin",
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct EditTeamOptionUnitsMap {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, String>,
    }

    /// EditUserOption edit user options
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
        pub login_name: String,
        pub max_repo_creation: Option<u64>,
        pub must_change_password: Option<bool>,
        pub password: Option<String>,
        pub prohibit_login: Option<bool>,
        pub restricted: Option<bool>,
        pub source_id: u64,
        pub visibility: Option<String>,
        pub website: Option<String>,
    }

    /// Email an email address belonging to a user
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Email {
        pub email: Option<String>,
        pub primary: Option<bool>,
        pub user_id: Option<u64>,
        pub username: Option<String>,
        pub verified: Option<bool>,
    }

    /// ExternalTracker represents settings for external tracker
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ExternalTracker {
        /// External Issue Tracker URL Format. Use the placeholders {user}, {repo} and {index} for the username, repository name and issue index.
        pub external_tracker_format: Option<String>,
        /// External Issue Tracker issue regular expression
        pub external_tracker_regexp_pattern: Option<String>,
        /// External Issue Tracker Number Format, either `numeric`, `alphanumeric`, or `regexp`
        pub external_tracker_style: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        /// URL of external issue tracker.
        pub external_tracker_url: Option<url::Url>,
    }

    /// ExternalWiki represents setting for external wiki
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ExternalWiki {
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        /// URL of external wiki.
        pub external_wiki_url: Option<url::Url>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct FileCommitResponse {
        pub author: Option<CommitUser>,
        pub committer: Option<CommitUser>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub message: Option<String>,
        pub parents: Option<Vec<CommitMeta>>,
        pub sha: Option<String>,
        pub tree: Option<CommitMeta>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// FileDeleteResponse contains information about a repo's file that was deleted
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct FileDeleteResponse {
        pub commit: Option<FileCommitResponse>,
        pub content: Option<()>,
        pub verification: Option<PayloadCommitVerification>,
    }

    /// FileLinksResponse contains the links for a repo's file
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct FileLinksResponse {
        pub git: Option<String>,
        pub html: Option<String>,
        #[serde(rename = "self")]
        pub this: Option<String>,
    }

    /// FileResponse contains information about a repo's file
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct FileResponse {
        pub commit: Option<FileCommitResponse>,
        pub content: Option<ContentsResponse>,
        pub verification: Option<PayloadCommitVerification>,
    }

    /// FilesResponse contains information about multiple files from a repo
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct FilesResponse {
        pub commit: Option<FileCommitResponse>,
        pub files: Option<Vec<ContentsResponse>>,
        pub verification: Option<PayloadCommitVerification>,
    }

    /// GPGKey a user GPG key to sign commit and tag in repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GPGKey {
        pub can_certify: Option<bool>,
        pub can_encrypt_comms: Option<bool>,
        pub can_encrypt_storage: Option<bool>,
        pub can_sign: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub emails: Option<Vec<GPGKeyEmail>>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub expires_at: Option<time::OffsetDateTime>,
        pub id: Option<u64>,
        pub key_id: Option<String>,
        pub primary_key_id: Option<String>,
        pub public_key: Option<String>,
        pub subkeys: Option<Vec<GPGKey>>,
        pub verified: Option<bool>,
    }

    /// GPGKeyEmail an email attached to a GPGKey
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GPGKeyEmail {
        pub email: Option<String>,
        pub verified: Option<bool>,
    }

    /// GeneralAPISettings contains global api settings exposed by it
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GeneralAPISettings {
        pub default_git_trees_per_page: Option<u64>,
        pub default_max_blob_size: Option<u64>,
        pub default_paging_num: Option<u64>,
        pub max_response_items: Option<u64>,
    }

    /// GeneralAttachmentSettings contains global Attachment settings exposed by API
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GeneralAttachmentSettings {
        pub allowed_types: Option<String>,
        pub enabled: Option<bool>,
        pub max_files: Option<u64>,
        pub max_size: Option<u64>,
    }

    /// GeneralRepoSettings contains global repository settings exposed by API
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GeneralRepoSettings {
        pub http_git_disabled: Option<bool>,
        pub lfs_disabled: Option<bool>,
        pub migrations_disabled: Option<bool>,
        pub mirrors_disabled: Option<bool>,
        pub stars_disabled: Option<bool>,
        pub time_tracking_disabled: Option<bool>,
    }

    /// GeneralUISettings contains global ui settings exposed by API
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GeneralUISettings {
        pub allowed_reactions: Option<Vec<String>>,
        pub custom_emojis: Option<Vec<String>>,
        pub default_theme: Option<String>,
    }

    /// GenerateRepoOption options when creating repository using a template
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GenerateRepoOption {
        /// include avatar of the template repo
        pub avatar: Option<bool>,
        /// Default branch of the new repository
        pub default_branch: Option<String>,
        /// Description of the repository to create
        pub description: Option<String>,
        /// include git content of default branch in template repo
        pub git_content: Option<bool>,
        /// include git hooks in template repo
        pub git_hooks: Option<bool>,
        /// include labels in template repo
        pub labels: Option<bool>,
        /// Name of the repository to create
        pub name: String,
        /// The organization or person who will own the new repository
        pub owner: String,
        /// Whether the repository is private
        pub private: Option<bool>,
        /// include protected branches in template repo
        pub protected_branch: Option<bool>,
        /// include topics in template repo
        pub topics: Option<bool>,
        /// include webhooks in template repo
        pub webhooks: Option<bool>,
    }

    /// GitBlobResponse represents a git blob
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GitBlobResponse {
        pub content: Option<String>,
        pub encoding: Option<String>,
        pub sha: Option<String>,
        pub size: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// GitEntry represents a git tree
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GitEntry {
        pub mode: Option<String>,
        pub path: Option<String>,
        pub sha: Option<String>,
        pub size: Option<u64>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// GitHook represents a Git repository hook
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GitHook {
        pub content: Option<String>,
        pub is_active: Option<bool>,
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GitObject {
        pub sha: Option<String>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// GitTreeResponse returns a git tree
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GitTreeResponse {
        pub page: Option<u64>,
        pub sha: Option<String>,
        pub total_count: Option<u64>,
        pub tree: Option<Vec<GitEntry>>,
        pub truncated: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// GitignoreTemplateInfo name and text of a gitignore template
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct GitignoreTemplateInfo {
        pub name: Option<String>,
        pub source: Option<String>,
    }

    /// Hook a hook is a web hook when one repository changed
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Hook {
        pub active: Option<bool>,
        pub authorization_header: Option<String>,
        pub branch_filter: Option<String>,
        pub config: Option<HookConfig>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub events: Option<Vec<String>>,
        pub id: Option<u64>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct HookConfig {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, String>,
    }

    /// Identity for a person's identity like an author or committer
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Identity {
        pub email: Option<String>,
        pub name: Option<String>,
    }

    /// InternalTracker represents settings for internal tracker
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct InternalTracker {
        /// Let only contributors track time (Built-in issue tracker)
        pub allow_only_contributors_to_track_time: Option<bool>,
        /// Enable dependencies for issues and pull requests (Built-in issue tracker)
        pub enable_issue_dependencies: Option<bool>,
        /// Enable time tracking (Built-in issue tracker)
        pub enable_time_tracker: Option<bool>,
    }

    /// Issue represents an issue in a repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Issue {
        pub assets: Option<Vec<Attachment>>,
        pub assignee: Option<User>,
        pub assignees: Option<Vec<User>>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub closed_at: Option<time::OffsetDateTime>,
        pub comments: Option<u64>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub is_locked: Option<bool>,
        pub labels: Option<Vec<Label>>,
        pub milestone: Option<Milestone>,
        pub number: Option<u64>,
        pub original_author: Option<String>,
        pub original_author_id: Option<u64>,
        pub pin_order: Option<u64>,
        pub pull_request: Option<PullRequestMeta>,
        #[serde(rename = "ref")]
        pub r#ref: Option<String>,
        pub repository: Option<RepositoryMeta>,
        pub state: Option<String>,
        pub title: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub user: Option<User>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueConfig {
        pub blank_issues_enabled: Option<bool>,
        pub contact_links: Option<Vec<IssueConfigContactLink>>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueConfigContactLink {
        pub about: Option<String>,
        pub name: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueConfigValidation {
        pub message: Option<String>,
        pub valid: Option<bool>,
    }

    /// IssueDeadline represents an issue deadline
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueDeadline {
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
    }

    /// IssueFormField represents a form field
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueFormField {
        pub attributes: Option<IssueFormFieldAttributes>,
        pub id: Option<String>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        pub validations: Option<IssueFormFieldValidations>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueFormFieldAttributes {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, ()>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueFormFieldValidations {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, ()>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueFormFieldType {}

    /// IssueLabelsOption a collection of labels
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueLabelsOption {
        /// list of label IDs
        pub labels: Option<Vec<u64>>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// IssueMeta basic issue information
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueMeta {
        pub index: Option<u64>,
        pub owner: Option<String>,
        pub repo: Option<String>,
    }

    /// IssueTemplate represents an issue template for a repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct IssueTemplate {
        pub about: Option<String>,
        pub body: Option<Vec<IssueFormField>>,
        pub content: Option<String>,
        pub file_name: Option<String>,
        pub labels: Option<Vec<String>>,
        pub name: Option<String>,
        #[serde(rename = "ref")]
        pub r#ref: Option<String>,
        pub title: Option<String>,
    }

    /// Label a label to an issue or a pr
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Label {
        pub color: Option<String>,
        pub description: Option<String>,
        pub exclusive: Option<bool>,
        pub id: Option<u64>,
        pub is_archived: Option<bool>,
        pub name: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// LabelTemplate info of a Label template
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct LabelTemplate {
        pub color: Option<String>,
        pub description: Option<String>,
        pub exclusive: Option<bool>,
        pub name: Option<String>,
    }

    /// LicensesInfo contains information about a License
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct LicenseTemplateInfo {
        pub body: Option<String>,
        pub implementation: Option<String>,
        pub key: Option<String>,
        pub name: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// LicensesListEntry is used for the API
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct LicensesTemplateListEntry {
        pub key: Option<String>,
        pub name: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// MarkdownOption markdown options
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct MarkdownOption {
        /// Context to render
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Context")]
        pub context: Option<String>,
        /// Mode to render (comment, gfm, markdown)
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Mode")]
        pub mode: Option<String>,
        /// Text markdown to render
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Text")]
        pub text: Option<String>,
        /// Is it a wiki page ?
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Wiki")]
        pub wiki: Option<bool>,
    }

    /// MarkupOption markup options
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct MarkupOption {
        /// Context to render
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Context")]
        pub context: Option<String>,
        /// File path for detecting extension in file mode
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "FilePath")]
        pub file_path: Option<String>,
        /// Mode to render (comment, gfm, markdown, file)
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Mode")]
        pub mode: Option<String>,
        /// Text markup to render
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Text")]
        pub text: Option<String>,
        /// Is it a wiki page ?
        ///
        ///
        ///
        /// in: body
        #[serde(rename = "Wiki")]
        pub wiki: Option<bool>,
    }

    /// MergePullRequestForm form for merging Pull Request
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct MergePullRequestOption {
        #[serde(rename = "Do")]
        pub r#do: MergePullRequestOptionDo,
        #[serde(rename = "MergeCommitID")]
        pub merge_commit_id: Option<String>,
        #[serde(rename = "MergeMessageField")]
        pub merge_message_field: Option<String>,
        #[serde(rename = "MergeTitleField")]
        pub merge_title_field: Option<String>,
        pub delete_branch_after_merge: Option<bool>,
        pub force_merge: Option<bool>,
        pub head_commit_id: Option<String>,
        pub merge_when_checks_succeed: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum MergePullRequestOptionDo {
        Merge,
        Rebase,
        RebaseMerge,
        Squash,
        ManuallyMerged,
    }

    impl MergePullRequestOptionDo {
        fn as_str(&self) -> &'static str {
            match self {
                MergePullRequestOptionDo::Merge => "merge",
                MergePullRequestOptionDo::Rebase => "rebase",
                MergePullRequestOptionDo::RebaseMerge => "rebase-merge",
                MergePullRequestOptionDo::Squash => "squash",
                MergePullRequestOptionDo::ManuallyMerged => "manually-merged",
            }
        }
    }
    /// MigrateRepoOptions options for migrating repository's
    ///
    /// this is used to interact with api v1
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct MigrateRepoOptions {
        pub auth_password: Option<String>,
        pub auth_token: Option<String>,
        pub auth_username: Option<String>,
        pub clone_addr: String,
        pub description: Option<String>,
        pub issues: Option<bool>,
        pub labels: Option<bool>,
        pub lfs: Option<bool>,
        pub lfs_endpoint: Option<String>,
        pub milestones: Option<bool>,
        pub mirror: Option<bool>,
        pub mirror_interval: Option<String>,
        pub private: Option<bool>,
        pub pull_requests: Option<bool>,
        pub releases: Option<bool>,
        pub repo_name: String,
        /// Name of User or Organisation who will own Repo after migration
        pub repo_owner: Option<String>,
        pub service: Option<MigrateRepoOptionsService>,
        /// deprecated (only for backwards compatibility)
        pub uid: Option<u64>,
        pub wiki: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum MigrateRepoOptionsService {
        Git,
        Github,
        Gitea,
        Gitlab,
        Gogs,
        Onedev,
        Gitbucket,
        Codebase,
    }

    impl MigrateRepoOptionsService {
        fn as_str(&self) -> &'static str {
            match self {
                MigrateRepoOptionsService::Git => "git",
                MigrateRepoOptionsService::Github => "github",
                MigrateRepoOptionsService::Gitea => "gitea",
                MigrateRepoOptionsService::Gitlab => "gitlab",
                MigrateRepoOptionsService::Gogs => "gogs",
                MigrateRepoOptionsService::Onedev => "onedev",
                MigrateRepoOptionsService::Gitbucket => "gitbucket",
                MigrateRepoOptionsService::Codebase => "codebase",
            }
        }
    }
    /// Milestone milestone is a collection of issues on one repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Milestone {
        #[serde(with = "time::serde::rfc3339::option")]
        pub closed_at: Option<time::OffsetDateTime>,
        pub closed_issues: Option<u64>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub description: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_on: Option<time::OffsetDateTime>,
        pub id: Option<u64>,
        pub open_issues: Option<u64>,
        pub state: Option<String>,
        pub title: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
    }

    /// NewIssuePinsAllowed represents an API response that says if new Issue Pins are allowed
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NewIssuePinsAllowed {
        pub issues: Option<bool>,
        pub pull_requests: Option<bool>,
    }

    /// NodeInfo contains standardized way of exposing metadata about a server running one of the distributed social networks
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NodeInfo {
        pub metadata: Option<NodeInfoMetadata>,
        #[serde(rename = "openRegistrations")]
        pub open_registrations: Option<bool>,
        pub protocols: Option<Vec<String>>,
        pub services: Option<NodeInfoServices>,
        pub software: Option<NodeInfoSoftware>,
        pub usage: Option<NodeInfoUsage>,
        pub version: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NodeInfoMetadata {}

    /// NodeInfoServices contains the third party sites this server can connect to via their application API
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NodeInfoServices {
        pub inbound: Option<Vec<String>>,
        pub outbound: Option<Vec<String>>,
    }

    /// NodeInfoSoftware contains Metadata about server software in use
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NodeInfoSoftware {
        pub homepage: Option<String>,
        pub name: Option<String>,
        pub repository: Option<String>,
        pub version: Option<String>,
    }

    /// NodeInfoUsage contains usage statistics for this server
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NodeInfoUsage {
        #[serde(rename = "localComments")]
        pub local_comments: Option<u64>,
        #[serde(rename = "localPosts")]
        pub local_posts: Option<u64>,
        pub users: Option<NodeInfoUsageUsers>,
    }

    /// NodeInfoUsageUsers contains statistics about the users of this server
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NodeInfoUsageUsers {
        #[serde(rename = "activeHalfyear")]
        pub active_halfyear: Option<u64>,
        #[serde(rename = "activeMonth")]
        pub active_month: Option<u64>,
        pub total: Option<u64>,
    }

    /// Note contains information related to a git note
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Note {
        pub commit: Option<Commit>,
        pub message: Option<String>,
    }

    /// NotificationCount number of unread notifications
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NotificationCount {
        pub new: Option<u64>,
    }

    /// NotificationSubject contains the notification subject (Issue/Pull/Commit)
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NotificationSubject {
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub latest_comment_html_url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub latest_comment_url: Option<url::Url>,
        pub state: Option<String>,
        pub title: Option<String>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// NotificationThread expose Notification on API
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NotificationThread {
        pub id: Option<u64>,
        pub pinned: Option<bool>,
        pub repository: Option<Repository>,
        pub subject: Option<NotificationSubject>,
        pub unread: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// NotifySubjectType represent type of notification subject
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct NotifySubjectType {}

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct OAuth2Application {
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
        pub confidential_client: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub id: Option<u64>,
        pub name: Option<String>,
        pub redirect_uris: Option<Vec<String>>,
    }

    /// Organization represents an organization
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Organization {
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub avatar_url: Option<url::Url>,
        pub description: Option<String>,
        pub email: Option<String>,
        pub full_name: Option<String>,
        pub id: Option<u64>,
        pub location: Option<String>,
        pub name: Option<String>,
        pub repo_admin_change_team_access: Option<bool>,
        /// deprecated
        pub username: Option<String>,
        pub visibility: Option<String>,
        pub website: Option<String>,
    }

    /// OrganizationPermissions list different users permissions on an organization
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct OrganizationPermissions {
        pub can_create_repository: Option<bool>,
        pub can_read: Option<bool>,
        pub can_write: Option<bool>,
        pub is_admin: Option<bool>,
        pub is_owner: Option<bool>,
    }

    /// PRBranchInfo information about a branch
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PRBranchInfo {
        pub label: Option<String>,
        #[serde(rename = "ref")]
        pub r#ref: Option<String>,
        pub repo: Option<Repository>,
        pub repo_id: Option<u64>,
        pub sha: Option<String>,
    }

    /// Package represents a package
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Package {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub creator: Option<User>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub name: Option<String>,
        pub owner: Option<User>,
        pub repository: Option<Repository>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        pub version: Option<String>,
    }

    /// PackageFile represents a package file
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PackageFile {
        #[serde(rename = "Size")]
        pub size: Option<u64>,
        pub id: Option<u64>,
        pub md5: Option<String>,
        pub name: Option<String>,
        pub sha1: Option<String>,
        pub sha256: Option<String>,
        pub sha512: Option<String>,
    }

    /// PayloadCommit represents a commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PayloadCommit {
        pub added: Option<Vec<String>>,
        pub author: Option<PayloadUser>,
        pub committer: Option<PayloadUser>,
        /// sha1 hash of the commit
        pub id: Option<String>,
        pub message: Option<String>,
        pub modified: Option<Vec<String>>,
        pub removed: Option<Vec<String>>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub timestamp: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub verification: Option<PayloadCommitVerification>,
    }

    /// PayloadCommitVerification represents the GPG verification of a commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PayloadCommitVerification {
        pub payload: Option<String>,
        pub reason: Option<String>,
        pub signature: Option<String>,
        pub signer: Option<PayloadUser>,
        pub verified: Option<bool>,
    }

    /// PayloadUser represents the author or committer of a commit
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PayloadUser {
        pub email: Option<String>,
        /// Full name of the commit author
        pub name: Option<String>,
        pub username: Option<String>,
    }

    /// Permission represents a set of permissions
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Permission {
        pub admin: Option<bool>,
        pub pull: Option<bool>,
        pub push: Option<bool>,
    }

    /// PublicKey publickey is a user key to push code to repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PublicKey {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub fingerprint: Option<String>,
        pub id: Option<u64>,
        pub key: Option<String>,
        pub key_type: Option<String>,
        pub read_only: Option<bool>,
        pub title: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub user: Option<User>,
    }

    /// PullRequest represents a pull request
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PullRequest {
        pub allow_maintainer_edit: Option<bool>,
        pub assignee: Option<User>,
        pub assignees: Option<Vec<User>>,
        pub base: Option<PRBranchInfo>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub closed_at: Option<time::OffsetDateTime>,
        pub comments: Option<u64>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub diff_url: Option<url::Url>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub due_date: Option<time::OffsetDateTime>,
        pub head: Option<PRBranchInfo>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub is_locked: Option<bool>,
        pub labels: Option<Vec<Label>>,
        pub merge_base: Option<String>,
        pub merge_commit_sha: Option<String>,
        pub mergeable: Option<bool>,
        pub merged: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub merged_at: Option<time::OffsetDateTime>,
        pub merged_by: Option<User>,
        pub milestone: Option<Milestone>,
        pub number: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub patch_url: Option<url::Url>,
        pub pin_order: Option<u64>,
        pub requested_reviewers: Option<Vec<User>>,
        pub state: Option<String>,
        pub title: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub user: Option<User>,
    }

    /// PullRequestMeta PR info if an issue is a PR
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PullRequestMeta {
        pub merged: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub merged_at: Option<time::OffsetDateTime>,
    }

    /// PullReview represents a pull request review
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PullReview {
        pub body: Option<String>,
        pub comments_count: Option<u64>,
        pub commit_id: Option<String>,
        pub dismissed: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub official: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub pull_request_url: Option<url::Url>,
        pub stale: Option<bool>,
        pub state: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub submitted_at: Option<time::OffsetDateTime>,
        pub team: Option<Team>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        pub user: Option<User>,
    }

    /// PullReviewComment represents a comment on a pull request review
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PullReviewComment {
        pub body: Option<String>,
        pub commit_id: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub diff_hunk: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub original_commit_id: Option<String>,
        pub original_position: Option<u32>,
        pub path: Option<String>,
        pub position: Option<u32>,
        pub pull_request_review_id: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub pull_request_url: Option<url::Url>,
        pub resolver: Option<User>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        pub user: Option<User>,
    }

    /// PullReviewRequestOptions are options to add or remove pull review requests
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PullReviewRequestOptions {
        pub reviewers: Option<Vec<String>>,
        pub team_reviewers: Option<Vec<String>>,
    }

    /// PushMirror represents information of a push mirror
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PushMirror {
        pub created: Option<String>,
        pub interval: Option<String>,
        pub last_error: Option<String>,
        pub last_update: Option<String>,
        pub remote_address: Option<String>,
        pub remote_name: Option<String>,
        pub repo_name: Option<String>,
        pub sync_on_commit: Option<bool>,
    }

    /// Reaction contain one reaction
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Reaction {
        pub content: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub user: Option<User>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Reference {
        pub object: Option<GitObject>,
        #[serde(rename = "ref")]
        pub r#ref: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// Release represents a repository release
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Release {
        pub assets: Option<Vec<Attachment>>,
        pub author: Option<User>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub draft: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub name: Option<String>,
        pub prerelease: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub published_at: Option<time::OffsetDateTime>,
        pub tag_name: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub tarball_url: Option<url::Url>,
        pub target_commitish: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub upload_url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub zipball_url: Option<url::Url>,
    }

    /// RenameUserOption options when renaming a user
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RenameUserOption {
        /// New username for this user. This name cannot be in use yet by any other user.
        pub new_username: String,
    }

    /// RepoCollaboratorPermission to get repository permission for a collaborator
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepoCollaboratorPermission {
        pub permission: Option<String>,
        pub role_name: Option<String>,
        pub user: Option<User>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepoCommit {
        pub author: Option<CommitUser>,
        pub committer: Option<CommitUser>,
        pub message: Option<String>,
        pub tree: Option<CommitMeta>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub verification: Option<PayloadCommitVerification>,
    }

    /// RepoTopicOptions a collection of repo topic names
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepoTopicOptions {
        /// list of topic names
        pub topics: Option<Vec<String>>,
    }

    /// RepoTransfer represents a pending repo transfer
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepoTransfer {
        pub doer: Option<User>,
        pub recipient: Option<User>,
        pub teams: Option<Vec<Team>>,
    }

    /// Repository represents a repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Repository {
        pub allow_merge_commits: Option<bool>,
        pub allow_rebase: Option<bool>,
        pub allow_rebase_explicit: Option<bool>,
        pub allow_rebase_update: Option<bool>,
        pub allow_squash_merge: Option<bool>,
        pub archived: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub archived_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub avatar_url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub clone_url: Option<url::Url>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub default_allow_maintainer_edit: Option<bool>,
        pub default_branch: Option<String>,
        pub default_delete_branch_after_merge: Option<bool>,
        pub default_merge_style: Option<String>,
        pub description: Option<String>,
        pub empty: Option<bool>,
        pub external_tracker: Option<ExternalTracker>,
        pub external_wiki: Option<ExternalWiki>,
        pub fork: Option<bool>,
        pub forks_count: Option<u64>,
        pub full_name: Option<String>,
        pub has_actions: Option<bool>,
        pub has_issues: Option<bool>,
        pub has_packages: Option<bool>,
        pub has_projects: Option<bool>,
        pub has_pull_requests: Option<bool>,
        pub has_releases: Option<bool>,
        pub has_wiki: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        pub ignore_whitespace_conflicts: Option<bool>,
        pub internal: Option<bool>,
        pub internal_tracker: Option<InternalTracker>,
        pub language: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub languages_url: Option<url::Url>,
        pub link: Option<String>,
        pub mirror: Option<bool>,
        pub mirror_interval: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub mirror_updated: Option<time::OffsetDateTime>,
        pub name: Option<String>,
        pub open_issues_count: Option<u64>,
        pub open_pr_counter: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub original_url: Option<url::Url>,
        pub owner: Option<User>,
        pub parent: Option<Box<Repository>>,
        pub permissions: Option<Permission>,
        pub private: Option<bool>,
        pub release_counter: Option<u64>,
        pub repo_transfer: Option<RepoTransfer>,
        pub size: Option<u64>,
        pub ssh_url: Option<String>,
        pub stars_count: Option<u64>,
        pub template: Option<bool>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
        pub watchers_count: Option<u64>,
        pub website: Option<String>,
    }

    /// RepositoryMeta basic repository information
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepositoryMeta {
        pub full_name: Option<String>,
        pub id: Option<u64>,
        pub name: Option<String>,
        pub owner: Option<String>,
    }

    /// ReviewStateType review state type
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ReviewStateType {}

    /// SearchResults results of a successful search
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct SearchResults {
        pub data: Option<Vec<Repository>>,
        pub ok: Option<bool>,
    }

    /// Secret represents a secret
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Secret {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        /// the secret's name
        pub name: Option<String>,
    }

    /// ServerVersion wraps the version of the server
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ServerVersion {
        pub version: Option<String>,
    }

    /// StateType issue state type
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct StateType {}

    /// StopWatch represent a running stopwatch
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct StopWatch {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub duration: Option<String>,
        pub issue_index: Option<u64>,
        pub issue_title: Option<String>,
        pub repo_name: Option<String>,
        pub repo_owner_name: Option<String>,
        pub seconds: Option<u64>,
    }

    /// SubmitPullReviewOptions are options to submit a pending pull review
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct SubmitPullReviewOptions {
        pub body: Option<String>,
        pub event: Option<String>,
    }

    /// Tag represents a repository tag
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Tag {
        pub commit: Option<CommitMeta>,
        pub id: Option<String>,
        pub message: Option<String>,
        pub name: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub tarball_url: Option<url::Url>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub zipball_url: Option<url::Url>,
    }

    /// Team represents a team in an organization
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Team {
        pub can_create_org_repo: Option<bool>,
        pub description: Option<String>,
        pub id: Option<u64>,
        pub includes_all_repositories: Option<bool>,
        pub name: Option<String>,
        pub organization: Option<Organization>,
        pub permission: Option<TeamPermission>,
        pub units: Option<Vec<String>>,
        pub units_map: Option<TeamUnitsMap>,
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum TeamPermission {
        None,
        Read,
        Write,
        Admin,
        Owner,
    }

    impl TeamPermission {
        fn as_str(&self) -> &'static str {
            match self {
                TeamPermission::None => "none",
                TeamPermission::Read => "read",
                TeamPermission::Write => "write",
                TeamPermission::Admin => "admin",
                TeamPermission::Owner => "owner",
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TeamUnitsMap {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, String>,
    }

    /// TimeStamp defines a timestamp
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TimeStamp {}

    /// TimelineComment represents a timeline comment (comment of any type) on a commit or issue
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TimelineComment {
        pub assignee: Option<User>,
        pub assignee_team: Option<Team>,
        pub body: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub dependent_issue: Option<Issue>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub id: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub issue_url: Option<url::Url>,
        pub label: Option<Label>,
        pub milestone: Option<Milestone>,
        pub new_ref: Option<String>,
        pub new_title: Option<String>,
        pub old_milestone: Option<Milestone>,
        pub old_project_id: Option<u64>,
        pub old_ref: Option<String>,
        pub old_title: Option<String>,
        pub project_id: Option<u64>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub pull_request_url: Option<url::Url>,
        pub ref_action: Option<String>,
        pub ref_comment: Option<Comment>,
        /// commit SHA where issue/PR was referenced
        pub ref_commit_sha: Option<String>,
        pub ref_issue: Option<Issue>,
        /// whether the assignees were removed or added
        pub removed_assignee: Option<bool>,
        pub resolve_doer: Option<User>,
        pub review_id: Option<u64>,
        pub tracked_time: Option<TrackedTime>,
        #[serde(rename = "type")]
        pub r#type: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        pub user: Option<User>,
    }

    /// TopicName a list of repo topic names
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TopicName {
        pub topics: Option<Vec<String>>,
    }

    /// TopicResponse for returning topics
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TopicResponse {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub id: Option<u64>,
        pub repo_count: Option<u64>,
        pub topic_name: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub updated: Option<time::OffsetDateTime>,
    }

    /// TrackedTime worked time for an issue / pr
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TrackedTime {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        pub id: Option<u64>,
        pub issue: Option<Issue>,
        /// deprecated (only for backwards compatibility)
        pub issue_id: Option<u64>,
        /// Time in seconds
        pub time: Option<u64>,
        /// deprecated (only for backwards compatibility)
        pub user_id: Option<u64>,
        pub user_name: Option<String>,
    }

    /// TransferRepoOption options when transfer a repository's ownership
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TransferRepoOption {
        pub new_owner: String,
        /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
        pub team_ids: Option<Vec<u64>>,
    }

    /// UpdateFileOptions options for updating files
    ///
    /// Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UpdateFileOptions {
        pub author: Option<Identity>,
        /// branch (optional) to base this file from. if not given, the default branch is used
        pub branch: Option<String>,
        pub committer: Option<Identity>,
        /// content must be base64 encoded
        pub content: String,
        pub dates: Option<CommitDateOptions>,
        /// from_path (optional) is the path of the original file which will be moved/renamed to the path in the URL
        pub from_path: Option<String>,
        /// message (optional) for the commit of this file. if not supplied, a default message will be used
        pub message: Option<String>,
        /// new_branch (optional) will make a new branch from `branch` before creating the file
        pub new_branch: Option<String>,
        /// sha is the SHA for the file that already exists
        pub sha: String,
        /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
        pub signoff: Option<bool>,
    }

    /// UpdateRepoAvatarUserOption options when updating the repo avatar
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UpdateRepoAvatarOption {
        /// image must be base64 encoded
        pub image: Option<String>,
    }

    /// UpdateUserAvatarUserOption options when updating the user avatar
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UpdateUserAvatarOption {
        /// image must be base64 encoded
        pub image: Option<String>,
    }

    /// User represents a user
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct User {
        /// Is user active
        pub active: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        /// URL to the user's avatar
        pub avatar_url: Option<url::Url>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub created: Option<time::OffsetDateTime>,
        /// the user's description
        pub description: Option<String>,
        pub email: Option<String>,
        /// user counts
        pub followers_count: Option<u64>,
        pub following_count: Option<u64>,
        /// the user's full name
        pub full_name: Option<String>,
        /// the user's id
        pub id: Option<u64>,
        /// Is the user an administrator
        pub is_admin: Option<bool>,
        /// User locale
        pub language: Option<String>,
        #[serde(with = "time::serde::rfc3339::option")]
        pub last_login: Option<time::OffsetDateTime>,
        /// the user's location
        pub location: Option<String>,
        /// the user's username
        pub login: Option<String>,
        /// the user's authentication sign-in name.
        pub login_name: Option<String>,
        /// Is user login prohibited
        pub prohibit_login: Option<bool>,
        /// Is user restricted
        pub restricted: Option<bool>,
        pub starred_repos_count: Option<u64>,
        /// User visibility level option: public, limited, private
        pub visibility: Option<String>,
        /// the user's website
        pub website: Option<String>,
    }

    /// UserHeatmapData represents the data needed to create a heatmap
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UserHeatmapData {
        pub contributions: Option<u64>,
        pub timestamp: Option<u64>,
    }

    /// UserSettings represents user settings
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UserSettings {
        pub description: Option<String>,
        pub diff_view_style: Option<String>,
        pub full_name: Option<String>,
        pub hide_activity: Option<bool>,
        /// Privacy
        pub hide_email: Option<bool>,
        pub language: Option<String>,
        pub location: Option<String>,
        pub theme: Option<String>,
        pub website: Option<String>,
    }

    /// UserSettingsOptions represents options to change user settings
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UserSettingsOptions {
        pub description: Option<String>,
        pub diff_view_style: Option<String>,
        pub full_name: Option<String>,
        pub hide_activity: Option<bool>,
        /// Privacy
        pub hide_email: Option<bool>,
        pub language: Option<String>,
        pub location: Option<String>,
        pub theme: Option<String>,
        pub website: Option<String>,
    }

    /// WatchInfo represents an API watch status of one repository
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct WatchInfo {
        #[serde(with = "time::serde::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        pub ignored: Option<bool>,
        pub reason: Option<()>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub repository_url: Option<url::Url>,
        pub subscribed: Option<bool>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub url: Option<url::Url>,
    }

    /// WikiCommit page commit/revision
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct WikiCommit {
        pub author: Option<CommitUser>,
        pub commiter: Option<CommitUser>,
        pub message: Option<String>,
        pub sha: Option<String>,
    }

    /// WikiCommitList commit/revision list
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct WikiCommitList {
        pub commits: Option<Vec<WikiCommit>>,
        pub count: Option<u64>,
    }

    /// WikiPage a wiki page
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct WikiPage {
        pub commit_count: Option<u64>,
        /// Page content, base64 encoded
        pub content_base64: Option<String>,
        pub footer: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub last_commit: Option<WikiCommit>,
        pub sidebar: Option<String>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub sub_url: Option<url::Url>,
        pub title: Option<String>,
    }

    /// WikiPageMetaData wiki page meta information
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct WikiPageMetaData {
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub html_url: Option<url::Url>,
        pub last_commit: Option<WikiCommit>,
        #[serde(deserialize_with = "crate::none_if_blank_url")]
        pub sub_url: Option<url::Url>,
        pub title: Option<String>,
    }

    pub struct ChangedFileListHeaders {
        pub x_has_more: Option<bool>,
        pub x_page: Option<u64>,
        pub x_page_count: Option<u64>,
        pub x_per_page: Option<u64>,
        pub x_total: Option<u64>,
    }

    impl TryFrom<&reqwest::header::HeaderMap> for ChangedFileListHeaders {
        type Error = StructureError;

        fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
            let x_has_more = map
                .get("X-HasMore")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<bool>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_page = map
                .get("X-Page")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_page_count = map
                .get("X-PageCount")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_per_page = map
                .get("X-PerPage")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_total = map
                .get("X-Total")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            Ok(Self {
                x_has_more,
                x_page,
                x_page_count,
                x_per_page,
                x_total,
            })
        }
    }

    pub struct CommitListHeaders {
        pub x_has_more: Option<bool>,
        pub x_page: Option<u64>,
        pub x_page_count: Option<u64>,
        pub x_per_page: Option<u64>,
        pub x_total: Option<u64>,
    }

    impl TryFrom<&reqwest::header::HeaderMap> for CommitListHeaders {
        type Error = StructureError;

        fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
            let x_has_more = map
                .get("X-HasMore")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<bool>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_page = map
                .get("X-Page")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_page_count = map
                .get("X-PageCount")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_per_page = map
                .get("X-PerPage")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            let x_total = map
                .get("X-Total")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    s.parse::<u64>()
                        .map_err(|_| StructureError::HeaderParseFailed)
                })
                .transpose()?;
            Ok(Self {
                x_has_more,
                x_page,
                x_page_count,
                x_per_page,
                x_total,
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct LanguageStatisticsResponse {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, u64>,
    }

    pub struct ErrorHeaders {
        pub message: Option<String>,
        pub url: Option<String>,
    }

    impl TryFrom<&reqwest::header::HeaderMap> for ErrorHeaders {
        type Error = StructureError;

        fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
            let message = map
                .get("message")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            let url = map
                .get("url")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            Ok(Self { message, url })
        }
    }

    pub struct ForbiddenHeaders {
        pub message: Option<String>,
        pub url: Option<String>,
    }

    impl TryFrom<&reqwest::header::HeaderMap> for ForbiddenHeaders {
        type Error = StructureError;

        fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
            let message = map
                .get("message")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            let url = map
                .get("url")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            Ok(Self { message, url })
        }
    }

    pub struct InvalidTopicsErrorHeaders {
        pub invalid_topics: Option<Vec<String>>,
        pub message: Option<String>,
    }

    impl TryFrom<&reqwest::header::HeaderMap> for InvalidTopicsErrorHeaders {
        type Error = StructureError;

        fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
            let invalid_topics = map
                .get("invalidTopics")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.split(",").map(|s| s.to_string()).collect::<Vec<_>>())
                })
                .transpose()?;
            let message = map
                .get("message")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            Ok(Self {
                invalid_topics,
                message,
            })
        }
    }

    pub struct ValidationErrorHeaders {
        pub message: Option<String>,
        pub url: Option<String>,
    }

    impl TryFrom<&reqwest::header::HeaderMap> for ValidationErrorHeaders {
        type Error = StructureError;

        fn try_from(map: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
            let message = map
                .get("message")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            let url = map
                .get("url")
                .map(|s| -> Result<_, _> {
                    let s = s.to_str().map_err(|_| StructureError::HeaderNotAscii)?;
                    Ok(s.to_string())
                })
                .transpose()?;
            Ok(Self { message, url })
        }
    }

    pub struct AdminCronListQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for AdminCronListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminGetAllEmailsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for AdminGetAllEmailsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminSearchEmailsQuery {
        /// keyword
        pub q: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for AdminSearchEmailsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminListHooksQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for AdminListHooksQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminGetAllOrgsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for AdminGetAllOrgsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminUnadoptedListQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
        /// pattern of repositories to search for
        pub pattern: Option<String>,
    }

    impl std::fmt::Display for AdminUnadoptedListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }
            if let Some(pattern) = &self.pattern {
                write!(f, "pattern={pattern}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminSearchUsersQuery {
        /// ID of the user's login source to search for
        pub source_id: Option<u64>,
        /// user's login name to search for
        pub login_name: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for AdminSearchUsersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(source_id) = &self.source_id {
                write!(f, "source_id={source_id}&")?;
            }
            if let Some(login_name) = &self.login_name {
                write!(f, "login_name={login_name}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct AdminDeleteUserQuery {
        /// purge the user from the system completely
        pub purge: Option<bool>,
    }

    impl std::fmt::Display for AdminDeleteUserQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(purge) = &self.purge {
                write!(f, "purge={purge}&")?;
            }

            Ok(())
        }
    }

    pub struct NotifyGetListQuery {
        /// If true, show notifications marked as read. Default value is false
        pub all: Option<bool>,
        /// Show notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread & pinned.
        pub status_types: Option<Vec<String>>,
        /// filter notifications by subject type
        pub subject_type: Option<Vec<NotifyGetListQuerySubjectType>>,
        /// Only show notifications updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show notifications updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for NotifyGetListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(all) = &self.all {
                write!(f, "all={all}&")?;
            }
            if let Some(status_types) = &self.status_types {
                if !status_types.is_empty() {
                    for item in status_types {
                        write!(f, "status-types=")?;
                        write!(f, "{item}")?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(subject_type) = &self.subject_type {
                if !subject_type.is_empty() {
                    for item in subject_type {
                        write!(f, "subject-type=")?;
                        write!(f, "{}", item.as_str())?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum NotifyGetListQuerySubjectType {
        Issue,
        Pull,
        Commit,
        Repository,
    }

    impl NotifyGetListQuerySubjectType {
        fn as_str(&self) -> &'static str {
            match self {
                NotifyGetListQuerySubjectType::Issue => "issue",
                NotifyGetListQuerySubjectType::Pull => "pull",
                NotifyGetListQuerySubjectType::Commit => "commit",
                NotifyGetListQuerySubjectType::Repository => "repository",
            }
        }
    }
    pub struct NotifyReadListQuery {
        /// Describes the last point that notifications were checked. Anything updated since this time will not be updated.
        pub last_read_at: Option<time::OffsetDateTime>,
        /// If true, mark all notifications on this repo. Default value is false
        pub all: Option<String>,
        /// Mark notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread.
        pub status_types: Option<Vec<String>>,
        /// Status to mark notifications as, Defaults to read.
        pub to_status: Option<String>,
    }

    impl std::fmt::Display for NotifyReadListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(last_read_at) = &self.last_read_at {
                write!(
                    f,
                    "last_read_at={field_name}&",
                    field_name = last_read_at
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(all) = &self.all {
                write!(f, "all={all}&")?;
            }
            if let Some(status_types) = &self.status_types {
                if !status_types.is_empty() {
                    for item in status_types {
                        write!(f, "status-types=")?;
                        write!(f, "{item}")?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(to_status) = &self.to_status {
                write!(f, "to-status={to_status}&")?;
            }

            Ok(())
        }
    }

    pub struct NotifyReadThreadQuery {
        /// Status to mark notifications as
        pub to_status: Option<String>,
    }

    impl std::fmt::Display for NotifyReadThreadQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(to_status) = &self.to_status {
                write!(f, "to-status={to_status}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgGetAllQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgGetAllQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListActionsSecretsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListActionsSecretsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListActivityFeedsQuery {
        /// the date of the activities to be found
        pub date: Option<time::Date>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListActivityFeedsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(date) = &self.date {
                write!(
                    f,
                    "date={field_name}&",
                    field_name = date
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListHooksQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListHooksQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListLabelsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListLabelsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListBlockedUsersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListBlockedUsersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListMembersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListMembersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListPublicMembersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListPublicMembersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListReposQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListReposQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListTeamsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListTeamsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct TeamSearchQuery {
        /// keywords to search
        pub q: Option<String>,
        /// include search within team description (defaults to true)
        pub include_desc: Option<bool>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for TeamSearchQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }
            if let Some(include_desc) = &self.include_desc {
                write!(f, "include_desc={include_desc}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TeamSearchResponse {
        pub data: Option<Vec<Team>>,
        pub ok: Option<bool>,
    }

    pub struct ListPackagesQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
        /// package type filter
        pub r#type: Option<ListPackagesQueryType>,
        /// name filter
        pub q: Option<String>,
    }

    impl std::fmt::Display for ListPackagesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }
            if let Some(r#type) = &self.r#type {
                write!(f, "type={}&", r#type.as_str())?;
            }
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum ListPackagesQueryType {
        Alpine,
        Cargo,
        Chef,
        Composer,
        Conan,
        Conda,
        Container,
        Cran,
        Debian,
        Generic,
        Go,
        Helm,
        Maven,
        Npm,
        Nuget,
        Pub,
        Pypi,
        Rpm,
        Rubygems,
        Swift,
        Vagrant,
    }

    impl ListPackagesQueryType {
        fn as_str(&self) -> &'static str {
            match self {
                ListPackagesQueryType::Alpine => "alpine",
                ListPackagesQueryType::Cargo => "cargo",
                ListPackagesQueryType::Chef => "chef",
                ListPackagesQueryType::Composer => "composer",
                ListPackagesQueryType::Conan => "conan",
                ListPackagesQueryType::Conda => "conda",
                ListPackagesQueryType::Container => "container",
                ListPackagesQueryType::Cran => "cran",
                ListPackagesQueryType::Debian => "debian",
                ListPackagesQueryType::Generic => "generic",
                ListPackagesQueryType::Go => "go",
                ListPackagesQueryType::Helm => "helm",
                ListPackagesQueryType::Maven => "maven",
                ListPackagesQueryType::Npm => "npm",
                ListPackagesQueryType::Nuget => "nuget",
                ListPackagesQueryType::Pub => "pub",
                ListPackagesQueryType::Pypi => "pypi",
                ListPackagesQueryType::Rpm => "rpm",
                ListPackagesQueryType::Rubygems => "rubygems",
                ListPackagesQueryType::Swift => "swift",
                ListPackagesQueryType::Vagrant => "vagrant",
            }
        }
    }
    pub struct IssueSearchIssuesQuery {
        /// whether issue is open or closed
        pub state: Option<String>,
        /// comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded
        pub labels: Option<String>,
        /// comma separated list of milestone names. Fetch only issues that have any of this milestones. Non existent are discarded
        pub milestones: Option<String>,
        /// search string
        pub q: Option<String>,
        /// repository to prioritize in the results
        pub priority_repo_id: Option<u64>,
        /// filter by type (issues / pulls) if set
        pub r#type: Option<String>,
        /// Only show notifications updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show notifications updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
        /// filter (issues / pulls) assigned to you, default is false
        pub assigned: Option<bool>,
        /// filter (issues / pulls) created by you, default is false
        pub created: Option<bool>,
        /// filter (issues / pulls) mentioning you, default is false
        pub mentioned: Option<bool>,
        /// filter pulls requesting your review, default is false
        pub review_requested: Option<bool>,
        /// filter pulls reviewed by you, default is false
        pub reviewed: Option<bool>,
        /// filter by owner
        pub owner: Option<String>,
        /// filter by team (requires organization owner parameter to be provided)
        pub team: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueSearchIssuesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(state) = &self.state {
                write!(f, "state={state}&")?;
            }
            if let Some(labels) = &self.labels {
                write!(f, "labels={labels}&")?;
            }
            if let Some(milestones) = &self.milestones {
                write!(f, "milestones={milestones}&")?;
            }
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }
            if let Some(priority_repo_id) = &self.priority_repo_id {
                write!(f, "priority_repo_id={priority_repo_id}&")?;
            }
            if let Some(r#type) = &self.r#type {
                write!(f, "type={type}&")?;
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(assigned) = &self.assigned {
                write!(f, "assigned={assigned}&")?;
            }
            if let Some(created) = &self.created {
                write!(f, "created={created}&")?;
            }
            if let Some(mentioned) = &self.mentioned {
                write!(f, "mentioned={mentioned}&")?;
            }
            if let Some(review_requested) = &self.review_requested {
                write!(f, "review_requested={review_requested}&")?;
            }
            if let Some(reviewed) = &self.reviewed {
                write!(f, "reviewed={reviewed}&")?;
            }
            if let Some(owner) = &self.owner {
                write!(f, "owner={owner}&")?;
            }
            if let Some(team) = &self.team {
                write!(f, "team={team}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoSearchQuery {
        /// keyword
        pub q: Option<String>,
        /// Limit search to repositories with keyword as topic
        pub topic: Option<bool>,
        /// include search of keyword within repository description
        pub include_desc: Option<bool>,
        /// search only for repos that the user with the given id owns or contributes to
        pub uid: Option<u64>,
        /// repo owner to prioritize in the results
        pub priority_owner_id: Option<u64>,
        /// search only for repos that belong to the given team id
        pub team_id: Option<u64>,
        /// search only for repos that the user with the given id has starred
        pub starred_by: Option<u64>,
        /// include private repositories this user has access to (defaults to true)
        pub private: Option<bool>,
        /// show only pubic, private or all repositories (defaults to all)
        pub is_private: Option<bool>,
        /// include template repositories this user has access to (defaults to true)
        pub template: Option<bool>,
        /// show only archived, non-archived or all repositories (defaults to all)
        pub archived: Option<bool>,
        /// type of repository to search for. Supported values are "fork", "source", "mirror" and "collaborative"
        pub mode: Option<String>,
        /// if `uid` is given, search only for repos that the user owns
        pub exclusive: Option<bool>,
        /// sort repos by attribute. Supported values are "alpha", "created", "updated", "size", and "id". Default is "alpha"
        pub sort: Option<String>,
        /// sort order, either "asc" (ascending) or "desc" (descending). Default is "asc", ignored if "sort" is not specified.
        pub order: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoSearchQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }
            if let Some(topic) = &self.topic {
                write!(f, "topic={topic}&")?;
            }
            if let Some(include_desc) = &self.include_desc {
                write!(f, "includeDesc={include_desc}&")?;
            }
            if let Some(uid) = &self.uid {
                write!(f, "uid={uid}&")?;
            }
            if let Some(priority_owner_id) = &self.priority_owner_id {
                write!(f, "priority_owner_id={priority_owner_id}&")?;
            }
            if let Some(team_id) = &self.team_id {
                write!(f, "team_id={team_id}&")?;
            }
            if let Some(starred_by) = &self.starred_by {
                write!(f, "starredBy={starred_by}&")?;
            }
            if let Some(private) = &self.private {
                write!(f, "private={private}&")?;
            }
            if let Some(is_private) = &self.is_private {
                write!(f, "is_private={is_private}&")?;
            }
            if let Some(template) = &self.template {
                write!(f, "template={template}&")?;
            }
            if let Some(archived) = &self.archived {
                write!(f, "archived={archived}&")?;
            }
            if let Some(mode) = &self.mode {
                write!(f, "mode={mode}&")?;
            }
            if let Some(exclusive) = &self.exclusive {
                write!(f, "exclusive={exclusive}&")?;
            }
            if let Some(sort) = &self.sort {
                write!(f, "sort={sort}&")?;
            }
            if let Some(order) = &self.order {
                write!(f, "order={order}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListActivityFeedsQuery {
        /// the date of the activities to be found
        pub date: Option<time::Date>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListActivityFeedsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(date) = &self.date {
                write!(
                    f,
                    "date={field_name}&",
                    field_name = date
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListBranchesQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListBranchesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListCollaboratorsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListCollaboratorsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetAllCommitsQuery {
        /// SHA or branch to start listing commits from (usually 'master')
        pub sha: Option<String>,
        /// filepath of a file/dir
        pub path: Option<String>,
        /// include diff stats for every commit (disable for speedup, default 'true')
        pub stat: Option<bool>,
        /// include verification for every commit (disable for speedup, default 'true')
        pub verification: Option<bool>,
        /// include a list of affected files for every commit (disable for speedup, default 'true')
        pub files: Option<bool>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results (ignored if used with 'path')
        pub limit: Option<u32>,
        /// commits that match the given specifier will not be listed.
        pub not: Option<String>,
    }

    impl std::fmt::Display for RepoGetAllCommitsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(sha) = &self.sha {
                write!(f, "sha={sha}&")?;
            }
            if let Some(path) = &self.path {
                write!(f, "path={path}&")?;
            }
            if let Some(stat) = &self.stat {
                write!(f, "stat={stat}&")?;
            }
            if let Some(verification) = &self.verification {
                write!(f, "verification={verification}&")?;
            }
            if let Some(files) = &self.files {
                write!(f, "files={files}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }
            if let Some(not) = &self.not {
                write!(f, "not={not}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetCombinedStatusByRefQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoGetCombinedStatusByRefQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListStatusesByRefQuery {
        /// type of sort
        pub sort: Option<RepoListStatusesByRefQuerySort>,
        /// type of state
        pub state: Option<RepoListStatusesByRefQueryState>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListStatusesByRefQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(sort) = &self.sort {
                write!(f, "sort={}&", sort.as_str())?;
            }
            if let Some(state) = &self.state {
                write!(f, "state={}&", state.as_str())?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoListStatusesByRefQuerySort {
        Oldest,
        Recentupdate,
        Leastupdate,
        Leastindex,
        Highestindex,
    }

    impl RepoListStatusesByRefQuerySort {
        fn as_str(&self) -> &'static str {
            match self {
                RepoListStatusesByRefQuerySort::Oldest => "oldest",
                RepoListStatusesByRefQuerySort::Recentupdate => "recentupdate",
                RepoListStatusesByRefQuerySort::Leastupdate => "leastupdate",
                RepoListStatusesByRefQuerySort::Leastindex => "leastindex",
                RepoListStatusesByRefQuerySort::Highestindex => "highestindex",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoListStatusesByRefQueryState {
        Pending,
        Success,
        Error,
        Failure,
        Warning,
    }

    impl RepoListStatusesByRefQueryState {
        fn as_str(&self) -> &'static str {
            match self {
                RepoListStatusesByRefQueryState::Pending => "pending",
                RepoListStatusesByRefQueryState::Success => "success",
                RepoListStatusesByRefQueryState::Error => "error",
                RepoListStatusesByRefQueryState::Failure => "failure",
                RepoListStatusesByRefQueryState::Warning => "warning",
            }
        }
    }
    pub struct RepoGetContentsListQuery {
        /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
        pub r#ref: Option<String>,
    }

    impl std::fmt::Display for RepoGetContentsListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(r#ref) = &self.r#ref {
                write!(f, "ref={ref}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetContentsQuery {
        /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
        pub r#ref: Option<String>,
    }

    impl std::fmt::Display for RepoGetContentsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(r#ref) = &self.r#ref {
                write!(f, "ref={ref}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetEditorConfigQuery {
        /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
        pub r#ref: Option<String>,
    }

    impl std::fmt::Display for RepoGetEditorConfigQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(r#ref) = &self.r#ref {
                write!(f, "ref={ref}&")?;
            }

            Ok(())
        }
    }

    pub struct ListForksQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for ListForksQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetSingleCommitQuery {
        /// include diff stats for every commit (disable for speedup, default 'true')
        pub stat: Option<bool>,
        /// include verification for every commit (disable for speedup, default 'true')
        pub verification: Option<bool>,
        /// include a list of affected files for every commit (disable for speedup, default 'true')
        pub files: Option<bool>,
    }

    impl std::fmt::Display for RepoGetSingleCommitQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(stat) = &self.stat {
                write!(f, "stat={stat}&")?;
            }
            if let Some(verification) = &self.verification {
                write!(f, "verification={verification}&")?;
            }
            if let Some(files) = &self.files {
                write!(f, "files={files}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetNoteQuery {
        /// include verification for every commit (disable for speedup, default 'true')
        pub verification: Option<bool>,
        /// include a list of affected files for every commit (disable for speedup, default 'true')
        pub files: Option<bool>,
    }

    impl std::fmt::Display for RepoGetNoteQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(verification) = &self.verification {
                write!(f, "verification={verification}&")?;
            }
            if let Some(files) = &self.files {
                write!(f, "files={files}&")?;
            }

            Ok(())
        }
    }

    pub struct GetTreeQuery {
        /// show all directories and files
        pub recursive: Option<bool>,
        /// page number; the 'truncated' field in the response will be true if there are still more items after this page, false if the last page
        pub page: Option<u32>,
        /// number of items per page
        pub per_page: Option<u32>,
    }

    impl std::fmt::Display for GetTreeQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(recursive) = &self.recursive {
                write!(f, "recursive={recursive}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(per_page) = &self.per_page {
                write!(f, "per_page={per_page}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListHooksQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListHooksQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoTestHookQuery {
        /// The name of the commit/branch/tag, indicates which commit will be loaded to the webhook payload.
        pub r#ref: Option<String>,
    }

    impl std::fmt::Display for RepoTestHookQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(r#ref) = &self.r#ref {
                write!(f, "ref={ref}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueListIssuesQuery {
        /// whether issue is open or closed
        pub state: Option<IssueListIssuesQueryState>,
        /// comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded
        pub labels: Option<String>,
        /// search string
        pub q: Option<String>,
        /// filter by type (issues / pulls) if set
        pub r#type: Option<IssueListIssuesQueryType>,
        /// comma separated list of milestone names or ids. It uses names and fall back to ids. Fetch only issues that have any of this milestones. Non existent milestones are discarded
        pub milestones: Option<String>,
        /// Only show items updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show items updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
        /// Only show items which were created by the the given user
        pub created_by: Option<String>,
        /// Only show items for which the given user is assigned
        pub assigned_by: Option<String>,
        /// Only show items in which the given user was mentioned
        pub mentioned_by: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueListIssuesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(state) = &self.state {
                write!(f, "state={}&", state.as_str())?;
            }
            if let Some(labels) = &self.labels {
                write!(f, "labels={labels}&")?;
            }
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }
            if let Some(r#type) = &self.r#type {
                write!(f, "type={}&", r#type.as_str())?;
            }
            if let Some(milestones) = &self.milestones {
                write!(f, "milestones={milestones}&")?;
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(created_by) = &self.created_by {
                write!(f, "created_by={created_by}&")?;
            }
            if let Some(assigned_by) = &self.assigned_by {
                write!(f, "assigned_by={assigned_by}&")?;
            }
            if let Some(mentioned_by) = &self.mentioned_by {
                write!(f, "mentioned_by={mentioned_by}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum IssueListIssuesQueryState {
        Closed,
        Open,
        All,
    }

    impl IssueListIssuesQueryState {
        fn as_str(&self) -> &'static str {
            match self {
                IssueListIssuesQueryState::Closed => "closed",
                IssueListIssuesQueryState::Open => "open",
                IssueListIssuesQueryState::All => "all",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum IssueListIssuesQueryType {
        Issues,
        Pulls,
    }

    impl IssueListIssuesQueryType {
        fn as_str(&self) -> &'static str {
            match self {
                IssueListIssuesQueryType::Issues => "issues",
                IssueListIssuesQueryType::Pulls => "pulls",
            }
        }
    }
    pub struct IssueGetRepoCommentsQuery {
        /// if provided, only comments updated since the provided time are returned.
        pub since: Option<time::OffsetDateTime>,
        /// if provided, only comments updated before the provided time are returned.
        pub before: Option<time::OffsetDateTime>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueGetRepoCommentsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueCreateIssueCommentAttachmentQuery {
        /// name of the attachment
        pub name: Option<String>,
        /// time of the attachment's creation. This is a timestamp in RFC 3339 format
        pub updated_at: Option<time::OffsetDateTime>,
    }

    impl std::fmt::Display for IssueCreateIssueCommentAttachmentQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(name) = &self.name {
                write!(f, "name={name}&")?;
            }
            if let Some(updated_at) = &self.updated_at {
                write!(
                    f,
                    "updated_at={field_name}&",
                    field_name = updated_at
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }

            Ok(())
        }
    }

    pub struct IssueCreateIssueAttachmentQuery {
        /// name of the attachment
        pub name: Option<String>,
        /// time of the attachment's creation. This is a timestamp in RFC 3339 format
        pub updated_at: Option<time::OffsetDateTime>,
    }

    impl std::fmt::Display for IssueCreateIssueAttachmentQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(name) = &self.name {
                write!(f, "name={name}&")?;
            }
            if let Some(updated_at) = &self.updated_at {
                write!(
                    f,
                    "updated_at={field_name}&",
                    field_name = updated_at
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }

            Ok(())
        }
    }

    pub struct IssueListBlocksQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueListBlocksQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueGetCommentsQuery {
        /// if provided, only comments updated since the specified time are returned.
        pub since: Option<time::OffsetDateTime>,
        /// if provided, only comments updated before the provided time are returned.
        pub before: Option<time::OffsetDateTime>,
    }

    impl std::fmt::Display for IssueGetCommentsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }

            Ok(())
        }
    }

    pub struct IssueListIssueDependenciesQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueListIssueDependenciesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueGetIssueReactionsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueGetIssueReactionsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueSubscriptionsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueSubscriptionsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueGetCommentsAndTimelineQuery {
        /// if provided, only comments updated since the specified time are returned.
        pub since: Option<time::OffsetDateTime>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
        /// if provided, only comments updated before the provided time are returned.
        pub before: Option<time::OffsetDateTime>,
    }

    impl std::fmt::Display for IssueGetCommentsAndTimelineQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }

            Ok(())
        }
    }

    pub struct IssueTrackedTimesQuery {
        /// optional filter by user (available for issue managers)
        pub user: Option<String>,
        /// Only show times updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show times updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueTrackedTimesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(user) = &self.user {
                write!(f, "user={user}&")?;
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListKeysQuery {
        /// the key_id to search for
        pub key_id: Option<u32>,
        /// fingerprint of the key
        pub fingerprint: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListKeysQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(key_id) = &self.key_id {
                write!(f, "key_id={key_id}&")?;
            }
            if let Some(fingerprint) = &self.fingerprint {
                write!(f, "fingerprint={fingerprint}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueListLabelsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueListLabelsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RepoGetLanguagesResponse {
        #[serde(flatten)]
        pub additional: std::collections::BTreeMap<String, u64>,
    }

    pub struct RepoGetRawFileOrLfsQuery {
        /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
        pub r#ref: Option<String>,
    }

    impl std::fmt::Display for RepoGetRawFileOrLfsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(r#ref) = &self.r#ref {
                write!(f, "ref={ref}&")?;
            }

            Ok(())
        }
    }

    pub struct IssueGetMilestonesListQuery {
        /// Milestone state, Recognized values are open, closed and all. Defaults to "open"
        pub state: Option<String>,
        /// filter by milestone name
        pub name: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for IssueGetMilestonesListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(state) = &self.state {
                write!(f, "state={state}&")?;
            }
            if let Some(name) = &self.name {
                write!(f, "name={name}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct NotifyGetRepoListQuery {
        /// If true, show notifications marked as read. Default value is false
        pub all: Option<bool>,
        /// Show notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread & pinned
        pub status_types: Option<Vec<String>>,
        /// filter notifications by subject type
        pub subject_type: Option<Vec<NotifyGetRepoListQuerySubjectType>>,
        /// Only show notifications updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show notifications updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for NotifyGetRepoListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(all) = &self.all {
                write!(f, "all={all}&")?;
            }
            if let Some(status_types) = &self.status_types {
                if !status_types.is_empty() {
                    for item in status_types {
                        write!(f, "status-types=")?;
                        write!(f, "{item}")?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(subject_type) = &self.subject_type {
                if !subject_type.is_empty() {
                    for item in subject_type {
                        write!(f, "subject-type=")?;
                        write!(f, "{}", item.as_str())?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum NotifyGetRepoListQuerySubjectType {
        Issue,
        Pull,
        Commit,
        Repository,
    }

    impl NotifyGetRepoListQuerySubjectType {
        fn as_str(&self) -> &'static str {
            match self {
                NotifyGetRepoListQuerySubjectType::Issue => "issue",
                NotifyGetRepoListQuerySubjectType::Pull => "pull",
                NotifyGetRepoListQuerySubjectType::Commit => "commit",
                NotifyGetRepoListQuerySubjectType::Repository => "repository",
            }
        }
    }
    pub struct NotifyReadRepoListQuery {
        /// If true, mark all notifications on this repo. Default value is false
        pub all: Option<String>,
        /// Mark notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread.
        pub status_types: Option<Vec<String>>,
        /// Status to mark notifications as. Defaults to read.
        pub to_status: Option<String>,
        /// Describes the last point that notifications were checked. Anything updated since this time will not be updated.
        pub last_read_at: Option<time::OffsetDateTime>,
    }

    impl std::fmt::Display for NotifyReadRepoListQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(all) = &self.all {
                write!(f, "all={all}&")?;
            }
            if let Some(status_types) = &self.status_types {
                if !status_types.is_empty() {
                    for item in status_types {
                        write!(f, "status-types=")?;
                        write!(f, "{item}")?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(to_status) = &self.to_status {
                write!(f, "to-status={to_status}&")?;
            }
            if let Some(last_read_at) = &self.last_read_at {
                write!(
                    f,
                    "last_read_at={field_name}&",
                    field_name = last_read_at
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }

            Ok(())
        }
    }

    pub struct RepoListPullRequestsQuery {
        /// State of pull request: open or closed (optional)
        pub state: Option<RepoListPullRequestsQueryState>,
        /// Type of sort
        pub sort: Option<RepoListPullRequestsQuerySort>,
        /// ID of the milestone
        pub milestone: Option<u64>,
        /// Label IDs
        pub labels: Option<Vec<u64>>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListPullRequestsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(state) = &self.state {
                write!(f, "state={}&", state.as_str())?;
            }
            if let Some(sort) = &self.sort {
                write!(f, "sort={}&", sort.as_str())?;
            }
            if let Some(milestone) = &self.milestone {
                write!(f, "milestone={milestone}&")?;
            }
            if let Some(labels) = &self.labels {
                if !labels.is_empty() {
                    for item in labels {
                        write!(f, "labels=")?;
                        write!(f, "{item}")?;
                        write!(f, "&")?;
                    }
                }
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoListPullRequestsQueryState {
        Closed,
        Open,
        All,
    }

    impl RepoListPullRequestsQueryState {
        fn as_str(&self) -> &'static str {
            match self {
                RepoListPullRequestsQueryState::Closed => "closed",
                RepoListPullRequestsQueryState::Open => "open",
                RepoListPullRequestsQueryState::All => "all",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoListPullRequestsQuerySort {
        Oldest,
        Recentupdate,
        Leastupdate,
        Mostcomment,
        Leastcomment,
        Priority,
    }

    impl RepoListPullRequestsQuerySort {
        fn as_str(&self) -> &'static str {
            match self {
                RepoListPullRequestsQuerySort::Oldest => "oldest",
                RepoListPullRequestsQuerySort::Recentupdate => "recentupdate",
                RepoListPullRequestsQuerySort::Leastupdate => "leastupdate",
                RepoListPullRequestsQuerySort::Mostcomment => "mostcomment",
                RepoListPullRequestsQuerySort::Leastcomment => "leastcomment",
                RepoListPullRequestsQuerySort::Priority => "priority",
            }
        }
    }
    pub struct RepoDownloadPullDiffOrPatchQuery {
        /// whether to include binary file changes. if true, the diff is applicable with `git apply`
        pub binary: Option<bool>,
    }

    impl std::fmt::Display for RepoDownloadPullDiffOrPatchQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(binary) = &self.binary {
                write!(f, "binary={binary}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetPullRequestCommitsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
        /// include verification for every commit (disable for speedup, default 'true')
        pub verification: Option<bool>,
        /// include a list of affected files for every commit (disable for speedup, default 'true')
        pub files: Option<bool>,
    }

    impl std::fmt::Display for RepoGetPullRequestCommitsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }
            if let Some(verification) = &self.verification {
                write!(f, "verification={verification}&")?;
            }
            if let Some(files) = &self.files {
                write!(f, "files={files}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetPullRequestFilesQuery {
        /// skip to given file
        pub skip_to: Option<String>,
        /// whitespace behavior
        pub whitespace: Option<RepoGetPullRequestFilesQueryWhitespace>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoGetPullRequestFilesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(skip_to) = &self.skip_to {
                write!(f, "skip-to={skip_to}&")?;
            }
            if let Some(whitespace) = &self.whitespace {
                write!(f, "whitespace={}&", whitespace.as_str())?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoGetPullRequestFilesQueryWhitespace {
        IgnoreAll,
        IgnoreChange,
        IgnoreEol,
        ShowAll,
    }

    impl RepoGetPullRequestFilesQueryWhitespace {
        fn as_str(&self) -> &'static str {
            match self {
                RepoGetPullRequestFilesQueryWhitespace::IgnoreAll => "ignore-all",
                RepoGetPullRequestFilesQueryWhitespace::IgnoreChange => "ignore-change",
                RepoGetPullRequestFilesQueryWhitespace::IgnoreEol => "ignore-eol",
                RepoGetPullRequestFilesQueryWhitespace::ShowAll => "show-all",
            }
        }
    }
    pub struct RepoListPullReviewsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListPullReviewsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoUpdatePullRequestQuery {
        /// how to update pull request
        pub style: Option<RepoUpdatePullRequestQueryStyle>,
    }

    impl std::fmt::Display for RepoUpdatePullRequestQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(style) = &self.style {
                write!(f, "style={}&", style.as_str())?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoUpdatePullRequestQueryStyle {
        Merge,
        Rebase,
    }

    impl RepoUpdatePullRequestQueryStyle {
        fn as_str(&self) -> &'static str {
            match self {
                RepoUpdatePullRequestQueryStyle::Merge => "merge",
                RepoUpdatePullRequestQueryStyle::Rebase => "rebase",
            }
        }
    }
    pub struct RepoListPushMirrorsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListPushMirrorsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetRawFileQuery {
        /// The name of the commit/branch/tag. Default the repository’s default branch (usually master)
        pub r#ref: Option<String>,
    }

    impl std::fmt::Display for RepoGetRawFileQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(r#ref) = &self.r#ref {
                write!(f, "ref={ref}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListReleasesQuery {
        /// filter (exclude / include) drafts, if you dont have repo write access none will show
        pub draft: Option<bool>,
        /// filter (exclude / include) pre-releases
        pub pre_release: Option<bool>,
        /// page size of results, deprecated - use limit
        pub per_page: Option<u32>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListReleasesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(draft) = &self.draft {
                write!(f, "draft={draft}&")?;
            }
            if let Some(pre_release) = &self.pre_release {
                write!(f, "pre-release={pre_release}&")?;
            }
            if let Some(per_page) = &self.per_page {
                write!(f, "per_page={per_page}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoCreateReleaseAttachmentQuery {
        /// name of the attachment
        pub name: Option<String>,
    }

    impl std::fmt::Display for RepoCreateReleaseAttachmentQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(name) = &self.name {
                write!(f, "name={name}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListStargazersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListStargazersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListStatusesQuery {
        /// type of sort
        pub sort: Option<RepoListStatusesQuerySort>,
        /// type of state
        pub state: Option<RepoListStatusesQueryState>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListStatusesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(sort) = &self.sort {
                write!(f, "sort={}&", sort.as_str())?;
            }
            if let Some(state) = &self.state {
                write!(f, "state={}&", state.as_str())?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoListStatusesQuerySort {
        Oldest,
        Recentupdate,
        Leastupdate,
        Leastindex,
        Highestindex,
    }

    impl RepoListStatusesQuerySort {
        fn as_str(&self) -> &'static str {
            match self {
                RepoListStatusesQuerySort::Oldest => "oldest",
                RepoListStatusesQuerySort::Recentupdate => "recentupdate",
                RepoListStatusesQuerySort::Leastupdate => "leastupdate",
                RepoListStatusesQuerySort::Leastindex => "leastindex",
                RepoListStatusesQuerySort::Highestindex => "highestindex",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum RepoListStatusesQueryState {
        Pending,
        Success,
        Error,
        Failure,
        Warning,
    }

    impl RepoListStatusesQueryState {
        fn as_str(&self) -> &'static str {
            match self {
                RepoListStatusesQueryState::Pending => "pending",
                RepoListStatusesQueryState::Success => "success",
                RepoListStatusesQueryState::Error => "error",
                RepoListStatusesQueryState::Failure => "failure",
                RepoListStatusesQueryState::Warning => "warning",
            }
        }
    }
    pub struct RepoListSubscribersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListSubscribersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListTagsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results, default maximum page size is 50
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListTagsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoTrackedTimesQuery {
        /// optional filter by user (available for issue managers)
        pub user: Option<String>,
        /// Only show times updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show times updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoTrackedTimesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(user) = &self.user {
                write!(f, "user={user}&")?;
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoListTopicsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoListTopicsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetWikiPagesQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for RepoGetWikiPagesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct RepoGetWikiPageRevisionsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
    }

    impl std::fmt::Display for RepoGetWikiPageRevisionsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListTeamActivityFeedsQuery {
        /// the date of the activities to be found
        pub date: Option<time::Date>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListTeamActivityFeedsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(date) = &self.date {
                write!(
                    f,
                    "date={field_name}&",
                    field_name = date
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListTeamMembersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListTeamMembersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListTeamReposQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListTeamReposQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct TopicSearchQuery {
        /// keywords to search
        pub q: String,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for TopicSearchQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let q = &self.q;
            write!(f, "q={q}&")?;
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserGetOAuth2ApplicationsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserGetOAuth2ApplicationsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListFollowersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListFollowersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListFollowingQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListFollowingQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListGpgKeysQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListGpgKeysQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListHooksQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListHooksQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListKeysQuery {
        /// fingerprint of the key
        pub fingerprint: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListKeysQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(fingerprint) = &self.fingerprint {
                write!(f, "fingerprint={fingerprint}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListBlockedUsersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListBlockedUsersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListCurrentUserOrgsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListCurrentUserOrgsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListReposQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListReposQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListStarredQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListStarredQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserGetStopWatchesQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserGetStopWatchesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentListSubscriptionsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserCurrentListSubscriptionsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListTeamsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListTeamsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserCurrentTrackedTimesQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
        /// Only show times updated after the given time. This is a timestamp in RFC 3339 format
        pub since: Option<time::OffsetDateTime>,
        /// Only show times updated before the given time. This is a timestamp in RFC 3339 format
        pub before: Option<time::OffsetDateTime>,
    }

    impl std::fmt::Display for UserCurrentTrackedTimesQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }
            if let Some(since) = &self.since {
                write!(
                    f,
                    "since={field_name}&",
                    field_name = since
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(before) = &self.before {
                write!(
                    f,
                    "before={field_name}&",
                    field_name = before
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }

            Ok(())
        }
    }

    pub struct UserSearchQuery {
        /// keyword
        pub q: Option<String>,
        /// ID of the user to search for
        pub uid: Option<u64>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserSearchQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(q) = &self.q {
                write!(f, "q={q}&")?;
            }
            if let Some(uid) = &self.uid {
                write!(f, "uid={uid}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct UserSearchResponse {
        pub data: Option<Vec<User>>,
        pub ok: Option<bool>,
    }

    pub struct UserListActivityFeedsQuery {
        /// if true, only show actions performed by the requested user
        pub only_performed_by: Option<bool>,
        /// the date of the activities to be found
        pub date: Option<time::Date>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListActivityFeedsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(only_performed_by) = &self.only_performed_by {
                write!(f, "only-performed-by={only_performed_by}&")?;
            }
            if let Some(date) = &self.date {
                write!(
                    f,
                    "date={field_name}&",
                    field_name = date
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap()
                )?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListFollowersQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListFollowersQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListFollowingQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListFollowingQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListGpgKeysQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListGpgKeysQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListKeysQuery {
        /// fingerprint of the key
        pub fingerprint: Option<String>,
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListKeysQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(fingerprint) = &self.fingerprint {
                write!(f, "fingerprint={fingerprint}&")?;
            }
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct OrgListUserOrgsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for OrgListUserOrgsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListReposQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListReposQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListStarredQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListStarredQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserListSubscriptionsQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserListSubscriptionsQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }

    pub struct UserGetTokensQuery {
        /// page number of results to return (1-based)
        pub page: Option<u32>,
        /// page size of results
        pub limit: Option<u32>,
    }

    impl std::fmt::Display for UserGetTokensQuery {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(page) = &self.page {
                write!(f, "page={page}&")?;
            }
            if let Some(limit) = &self.limit {
                write!(f, "limit={limit}&")?;
            }

            Ok(())
        }
    }
}
