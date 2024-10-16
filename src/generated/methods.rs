use super::structs::*;
use crate::ForgejoError;
use std::collections::BTreeMap;

impl crate::Forgejo {
    /// Returns the instance's Actor
    pub async fn activitypub_instance_actor(&self) -> Result<ActivityPub, ForgejoError> {
        let request = self.get("activitypub/actor").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Send to the inbox
    pub async fn activitypub_instance_actor_inbox(&self) -> Result<(), ForgejoError> {
        let request = self.post("activitypub/actor/inbox").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Returns the Repository actor for a repo
    ///
    /// - `repository-id`: repository ID of the repo
    pub async fn activitypub_repository(
        &self,
        repository_id: u32,
    ) -> Result<ActivityPub, ForgejoError> {
        let request = self
            .get(&format!("activitypub/repository-id/{repository_id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Send to the inbox
    ///
    /// - `repository-id`: repository ID of the repo
    /// - `body`: See [`ForgeLike`]
    pub async fn activitypub_repository_inbox(
        &self,
        repository_id: u32,
        body: ForgeLike,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("activitypub/repository-id/{repository_id}/inbox"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

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

    /// List the available quota groups
    pub async fn admin_list_quota_groups(&self) -> Result<Vec<QuotaGroup>, ForgejoError> {
        let request = self.get("admin/quota/groups").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a new quota group
    ///
    /// - `group`: Definition of the quota group

    ///   See [`CreateQuotaGroupOptions`]
    pub async fn admin_create_quota_group(
        &self,
        group: CreateQuotaGroupOptions,
    ) -> Result<QuotaGroup, ForgejoError> {
        let request = self.post("admin/quota/groups").json(&group).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get information about the quota group
    ///
    /// - `quotagroup`: quota group to query
    pub async fn admin_get_quota_group(
        &self,
        quotagroup: &str,
    ) -> Result<QuotaGroup, ForgejoError> {
        let request = self
            .get(&format!("admin/quota/groups/{quotagroup}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a quota group
    ///
    /// - `quotagroup`: quota group to delete
    pub async fn admin_delete_quota_group(&self, quotagroup: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("admin/quota/groups/{quotagroup}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Adds a rule to a quota group
    ///
    /// - `quotagroup`: quota group to add a rule to
    /// - `quotarule`: the name of the quota rule to add to the group
    pub async fn admin_add_rule_to_quota_group(
        &self,
        quotagroup: &str,
        quotarule: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!(
                "admin/quota/groups/{quotagroup}/rules/{quotarule}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Removes a rule from a quota group
    ///
    /// - `quotagroup`: quota group to remove a rule from
    /// - `quotarule`: the name of the quota rule to remove from the group
    pub async fn admin_remove_rule_from_quota_group(
        &self,
        quotagroup: &str,
        quotarule: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "admin/quota/groups/{quotagroup}/rules/{quotarule}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List users in a quota group
    ///
    /// - `quotagroup`: quota group to list members of
    pub async fn admin_list_users_in_quota_group(
        &self,
        quotagroup: &str,
    ) -> Result<Vec<User>, ForgejoError> {
        let request = self
            .get(&format!("admin/quota/groups/{quotagroup}/users"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a user to a quota group
    ///
    /// - `quotagroup`: quota group to add the user to
    /// - `username`: username of the user to add to the quota group
    pub async fn admin_add_user_to_quota_group(
        &self,
        quotagroup: &str,
        username: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("admin/quota/groups/{quotagroup}/users/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a user from a quota group
    ///
    /// - `quotagroup`: quota group to remove a user from
    /// - `username`: username of the user to remove from the quota group
    pub async fn admin_remove_user_from_quota_group(
        &self,
        quotagroup: &str,
        username: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("admin/quota/groups/{quotagroup}/users/{username}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the available quota rules
    pub async fn admin_list_quota_rules(&self) -> Result<Vec<QuotaRuleInfo>, ForgejoError> {
        let request = self.get("admin/quota/rules").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a new quota rule
    ///
    /// - `rule`: Definition of the quota rule

    ///   See [`CreateQuotaRuleOptions`]
    pub async fn admin_create_quota_rule(
        &self,
        rule: CreateQuotaRuleOptions,
    ) -> Result<QuotaRuleInfo, ForgejoError> {
        let request = self.post("admin/quota/rules").json(&rule).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get information about a quota rule
    ///
    /// - `quotarule`: quota rule to query
    pub async fn admin_get_quota_rule(
        &self,
        quotarule: &str,
    ) -> Result<QuotaRuleInfo, ForgejoError> {
        let request = self
            .get(&format!("admin/quota/rules/{quotarule}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Deletes a quota rule
    ///
    /// - `quotarule`: quota rule to delete
    pub async fn admin_d_elete_quota_rule(&self, quotarule: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("admin/quota/rules/{quotarule}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Change an existing quota rule
    ///
    /// - `quotarule`: Quota rule to change
    /// - `rule`: See [`EditQuotaRuleOptions`]
    pub async fn admin_edit_quota_rule(
        &self,
        quotarule: &str,
        rule: EditQuotaRuleOptions,
    ) -> Result<QuotaRuleInfo, ForgejoError> {
        let request = self
            .patch(&format!("admin/quota/rules/{quotarule}"))
            .json(&rule)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an global actions runner registration token
    pub async fn admin_get_runner_registration_token(
        &self,
    ) -> Result<RegistrationTokenHeaders, ForgejoError> {
        let request = self.get("admin/runners/registration-token").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.headers().try_into()?),
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

    /// Get the user's quota info
    ///
    /// - `username`: username of user to query
    pub async fn admin_get_user_quota(&self, username: &str) -> Result<QuotaInfo, ForgejoError> {
        let request = self.get(&format!("admin/users/{username}/quota")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Set the user's quota groups to a given list.
    ///
    /// - `username`: username of the user to modify the quota groups from
    /// - `groups`: list of groups that the user should be a member of

    ///   See [`SetUserQuotaGroupsOptions`]
    pub async fn admin_set_user_quota_groups(
        &self,
        username: &str,
        groups: SetUserQuotaGroupsOptions,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("admin/users/{username}/quota/groups"))
            .json(&groups)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
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

    /// Returns the nodeinfo of the Forgejo application
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

    /// Get an organization's actions runner registration token
    ///
    /// - `org`: name of the organization
    pub async fn org_get_runner_registration_token(
        &self,
        org: &str,
    ) -> Result<RegistrationTokenHeaders, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/actions/runners/registration-token"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.headers().try_into()?),
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

    /// Get an org-level variables list
    ///
    /// - `org`: name of the organization
    pub async fn get_org_variables_list(
        &self,
        org: &str,
        query: GetOrgVariablesListQuery,
    ) -> Result<Vec<ActionVariable>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/actions/variables?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    pub async fn get_org_variable(
        &self,
        org: &str,
        variablename: &str,
    ) -> Result<ActionVariable, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/actions/variables/{variablename}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub async fn update_org_variable(
        &self,
        org: &str,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("orgs/{org}/actions/variables/{variablename}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub async fn create_org_variable(
        &self,
        org: &str,
        variablename: &str,
        body: CreateVariableOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("orgs/{org}/actions/variables/{variablename}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    pub async fn delete_org_variable(
        &self,
        org: &str,
        variablename: &str,
    ) -> Result<Option<ActionVariable>, ForgejoError> {
        let request = self
            .delete(&format!("orgs/{org}/actions/variables/{variablename}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(Some(response.json().await?)),
            201 => Ok(None),
            204 => Ok(None),
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

    /// Get quota information for an organization
    ///
    /// - `org`: name of the organization
    pub async fn org_get_quota(&self, org: &str) -> Result<QuotaInfo, ForgejoError> {
        let request = self.get(&format!("orgs/{org}/quota")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the artifacts affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub async fn org_list_quota_artifacts(
        &self,
        org: &str,
        query: OrgListQuotaArtifactsQuery,
    ) -> Result<Vec<QuotaUsedArtifact>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/quota/artifacts?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the attachments affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub async fn org_list_quota_attachments(
        &self,
        org: &str,
        query: OrgListQuotaAttachmentsQuery,
    ) -> Result<Vec<QuotaUsedAttachment>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/quota/attachments?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if the organization is over quota for a given subject
    ///
    /// - `org`: name of the organization
    pub async fn org_check_quota(&self, org: &str) -> Result<(), ForgejoError> {
        let request = self.get(&format!("orgs/{org}/quota/check")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the packages affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub async fn org_list_quota_packages(
        &self,
        org: &str,
        query: OrgListQuotaPackagesQuery,
    ) -> Result<Vec<QuotaUsedPackage>, ForgejoError> {
        let request = self
            .get(&format!("orgs/{org}/quota/packages?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
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

    /// Get a repository's actions runner registration token
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_get_runner_registration_token(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<RegistrationTokenHeaders, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/actions/runners/registration-token"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.headers().try_into()?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List an repo's actions secrets
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    pub async fn repo_list_actions_secrets(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListActionsSecretsQuery,
    ) -> Result<Vec<Secret>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/actions/secrets?{query}"))
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

    /// List a repository's action tasks
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn list_action_tasks(
        &self,
        owner: &str,
        repo: &str,
        query: ListActionTasksQuery,
    ) -> Result<ActionTaskResponse, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/actions/tasks?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get repo-level variables list
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    pub async fn get_repo_variables_list(
        &self,
        owner: &str,
        repo: &str,
        query: GetRepoVariablesListQuery,
    ) -> Result<Vec<ActionVariable>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/actions/variables?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    pub async fn get_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
    ) -> Result<ActionVariable, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/actions/variables/{variablename}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub async fn update_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!(
                "repos/{owner}/{repo}/actions/variables/{variablename}"
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

    /// Create a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub async fn create_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
        body: CreateVariableOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/actions/variables/{variablename}"
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

    /// Delete a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    pub async fn delete_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
    ) -> Result<Option<ActionVariable>, ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/actions/variables/{variablename}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(Some(response.json().await?)),
            201 => Ok(None),
            204 => Ok(None),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Dispatches a workflow
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `workflowname`: name of the workflow
    /// - `body`: See [`DispatchWorkflowOption`]
    pub async fn dispatch_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflowname: &str,
        body: DispatchWorkflowOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/actions/workflows/{workflowname}/dispatches"
            ))
            .json(&body)
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

    /// Get the pull request of the commit
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: SHA of the commit to get
    pub async fn repo_get_commit_pull_request(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<PullRequest, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/commits/{sha}/pull"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get commit comparison information
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `basehead`: compare two branches or commits
    pub async fn repo_compare_diff(
        &self,
        owner: &str,
        repo: &str,
        basehead: &str,
    ) -> Result<Compare, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/compare/{basehead}"))
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

    /// List a repository's flags
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_flags(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<String>, ForgejoError> {
        let request = self.get(&format!("repos/{owner}/{repo}/flags")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Replace all flags of a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`ReplaceFlagsOption`]
    pub async fn repo_replace_all_flags(
        &self,
        owner: &str,
        repo: &str,
        body: ReplaceFlagsOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/flags"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove all flags from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_delete_all_flags(&self, owner: &str, repo: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/flags"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if a repository has a given flag
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `flag`: name of the flag
    pub async fn repo_check_flag(
        &self,
        owner: &str,
        repo: &str,
        flag: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/flags/{flag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Add a flag to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `flag`: name of the flag
    pub async fn repo_add_flag(
        &self,
        owner: &str,
        repo: &str,
        flag: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("repos/{owner}/{repo}/flags/{flag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Remove a flag from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `flag`: name of the flag
    pub async fn repo_delete_flag(
        &self,
        owner: &str,
        repo: &str,
        flag: &str,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/flags/{flag}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
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
            204 => Ok(None),
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
            204 => Ok(None),
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
            204 => Ok(None),
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
    ) -> Result<BTreeMap<String, i64>, ForgejoError> {
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
    ) -> Result<Vec<u8>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/media/{filepath}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.bytes().await?[..].to_vec()),
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

    /// Get a pull request by base and head
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `base`: base of the pull request to get
    /// - `head`: head of the pull request to get
    pub async fn repo_get_pull_request_by_base_head(
        &self,
        owner: &str,
        repo: &str,
        base: &str,
        head: &str,
    ) -> Result<PullRequest, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/pulls/{base}/{head}"))
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

    /// Add a new comment to a pull request review
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `body`: See [`serde_json::Value`]
    pub async fn repo_create_pull_review_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: serde_json::Value,
    ) -> Result<PullReviewComment, ForgejoError> {
        let request = self
            .post(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments"
            ))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a pull review comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `comment`: id of the comment
    pub async fn repo_get_pull_review_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        comment: u64,
    ) -> Result<PullReviewComment, ForgejoError> {
        let request = self
            .get(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments/{comment}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a pull review comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `comment`: id of the comment
    pub async fn repo_delete_pull_review_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        comment: u64,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!(
                "repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments/{comment}"
            ))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
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
    ) -> Result<Vec<u8>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/raw/{filepath}?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.bytes().await?[..].to_vec()),
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
    /// - `attachment`: attachment to upload (this parameter is incompatible with `external_url`)
    /// - `external_url`: url to external asset (this parameter is incompatible with `attachment`)
    pub async fn repo_create_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment: Vec<u8>,
        external_url: Vec<u8>,
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
            .multipart(
                reqwest::multipart::Form::new().part(
                    "attachment",
                    reqwest::multipart::Part::bytes(external_url)
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

    /// List tag protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub async fn repo_list_tag_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<TagProtection>, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/tag_protections"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a tag protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateTagProtectionOption`]
    pub async fn repo_create_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        body: CreateTagProtectionOption,
    ) -> Result<TagProtection, ForgejoError> {
        let request = self
            .post(&format!("repos/{owner}/{repo}/tag_protections"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a specific tag protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the tag protect to get
    pub async fn repo_get_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: u32,
    ) -> Result<TagProtection, ForgejoError> {
        let request = self
            .get(&format!("repos/{owner}/{repo}/tag_protections/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a specific tag protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of protected tag
    pub async fn repo_delete_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: u32,
    ) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("repos/{owner}/{repo}/tag_protections/{id}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Edit a tag protections for a repository. Only fields that are set will be changed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of protected tag
    /// - `body`: See [`EditTagProtectionOption`]
    pub async fn repo_edit_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: u32,
        body: EditTagProtectionOption,
    ) -> Result<TagProtection, ForgejoError> {
        let request = self
            .patch(&format!("repos/{owner}/{repo}/tag_protections/{id}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
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
            201 => Ok(response.json().await?),
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

    /// Get an user's actions runner registration token
    pub async fn user_get_runner_registration_token(
        &self,
    ) -> Result<RegistrationTokenHeaders, ForgejoError> {
        let request = self
            .get("user/actions/runners/registration-token")
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.headers().try_into()?),
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

    /// Get the user-level list of variables which is created by current doer
    ///
    pub async fn get_user_variables_list(
        &self,
        query: GetUserVariablesListQuery,
    ) -> Result<Vec<ActionVariable>, ForgejoError> {
        let request = self
            .get(&format!("user/actions/variables?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Get a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    pub async fn get_user_variable(
        &self,
        variablename: &str,
    ) -> Result<ActionVariable, ForgejoError> {
        let request = self
            .get(&format!("user/actions/variables/{variablename}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Update a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub async fn update_user_variable(
        &self,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .put(&format!("user/actions/variables/{variablename}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Create a user-level variable
    ///
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub async fn create_user_variable(
        &self,
        variablename: &str,
        body: CreateVariableOption,
    ) -> Result<(), ForgejoError> {
        let request = self
            .post(&format!("user/actions/variables/{variablename}"))
            .json(&body)
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
            204 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Delete a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    pub async fn delete_user_variable(&self, variablename: &str) -> Result<(), ForgejoError> {
        let request = self
            .delete(&format!("user/actions/variables/{variablename}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            201 => Ok(()),
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

    /// Get quota information for the authenticated user
    pub async fn user_get_quota(&self) -> Result<QuotaInfo, ForgejoError> {
        let request = self.get("user/quota").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the artifacts affecting the authenticated user's quota
    ///
    pub async fn user_list_quota_artifacts(
        &self,
        query: UserListQuotaArtifactsQuery,
    ) -> Result<Vec<QuotaUsedArtifact>, ForgejoError> {
        let request = self.get(&format!("user/quota/artifacts?{query}")).build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the attachments affecting the authenticated user's quota
    ///
    pub async fn user_list_quota_attachments(
        &self,
        query: UserListQuotaAttachmentsQuery,
    ) -> Result<Vec<QuotaUsedAttachment>, ForgejoError> {
        let request = self
            .get(&format!("user/quota/attachments?{query}"))
            .build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// Check if the authenticated user is over quota for a given subject
    pub async fn user_check_quota(&self) -> Result<(), ForgejoError> {
        let request = self.get("user/quota/check").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    /// List the packages affecting the authenticated user's quota
    ///
    pub async fn user_list_quota_packages(
        &self,
        query: UserListQuotaPackagesQuery,
    ) -> Result<Vec<QuotaUsedPackage>, ForgejoError> {
        let request = self.get(&format!("user/quota/packages?{query}")).build()?;
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
    pub async fn get_user_settings(&self) -> Result<UserSettings, ForgejoError> {
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
    ) -> Result<UserSettings, ForgejoError> {
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

    /// Returns the version of the Forgejo application
    pub async fn get_version(&self) -> Result<ServerVersion, ForgejoError> {
        let request = self.get("version").build()?;
        let response = self.execute(request).await?;
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }
}
