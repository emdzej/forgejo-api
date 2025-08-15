use super::structs::*;
use crate::generated::endpoints;
use crate::Endpoint;
use crate::ForgejoError;
use crate::WrappedRequest;
use bytes::Bytes;
use std::collections::BTreeMap;

impl crate::sync::Forgejo {
    /// Returns the instance's Actor
    pub fn activitypub_instance_actor(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ActivitypubInstanceActor> {
        endpoints::ActivitypubInstanceActor {}
            .make_request()
            .wrap_sync::<endpoints::ActivitypubInstanceActor>(self)
    }

    /// Send to the inbox
    pub fn activitypub_instance_actor_inbox(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ActivitypubInstanceActorInbox> {
        endpoints::ActivitypubInstanceActorInbox {}
            .make_request()
            .wrap_sync::<endpoints::ActivitypubInstanceActorInbox>(self)
    }

    /// Returns the Repository actor for a repo
    ///
    /// - `repository-id`: repository ID of the repo
    pub fn activitypub_repository(
        &self,
        repository_id: u32,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ActivitypubRepository> {
        endpoints::ActivitypubRepository { repository_id }
            .make_request()
            .wrap_sync::<endpoints::ActivitypubRepository>(self)
    }

    /// Send to the inbox
    ///
    /// - `repository-id`: repository ID of the repo
    /// - `body`: See [`ForgeLike`]
    pub fn activitypub_repository_inbox(
        &self,
        repository_id: u32,
        body: ForgeLike,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ActivitypubRepositoryInbox> {
        endpoints::ActivitypubRepositoryInbox {
            repository_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::ActivitypubRepositoryInbox>(self)
    }

    /// Returns the Person actor for a user
    ///
    /// - `user-id`: user ID of the user
    pub fn activitypub_person(
        &self,
        user_id: u32,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ActivitypubPerson> {
        endpoints::ActivitypubPerson { user_id }
            .make_request()
            .wrap_sync::<endpoints::ActivitypubPerson>(self)
    }

    /// Send to the inbox
    ///
    /// - `user-id`: user ID of the user
    pub fn activitypub_person_inbox(
        &self,
        user_id: u32,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ActivitypubPersonInbox> {
        endpoints::ActivitypubPersonInbox { user_id }
            .make_request()
            .wrap_sync::<endpoints::ActivitypubPersonInbox>(self)
    }

    /// List cron tasks
    ///
    pub fn admin_cron_list(&self) -> crate::sync::WrappedRequest<'_, endpoints::AdminCronList> {
        endpoints::AdminCronList {}
            .make_request()
            .wrap_sync::<endpoints::AdminCronList>(self)
    }

    /// Run cron task
    ///
    /// - `task`: task to run
    pub fn admin_cron_run(
        &self,
        task: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCronRun> {
        endpoints::AdminCronRun { task }
            .make_request()
            .wrap_sync::<endpoints::AdminCronRun>(self)
    }

    /// List all emails
    ///
    pub fn admin_get_all_emails(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetAllEmails> {
        endpoints::AdminGetAllEmails {}
            .make_request()
            .wrap_sync::<endpoints::AdminGetAllEmails>(self)
    }

    /// Search all emails
    ///
    pub fn admin_search_emails(
        &self,
        query: AdminSearchEmailsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminSearchEmails> {
        endpoints::AdminSearchEmails { query }
            .make_request()
            .wrap_sync::<endpoints::AdminSearchEmails>(self)
    }

    /// List system's webhooks
    ///
    pub fn admin_list_hooks(&self) -> crate::sync::WrappedRequest<'_, endpoints::AdminListHooks> {
        endpoints::AdminListHooks {}
            .make_request()
            .wrap_sync::<endpoints::AdminListHooks>(self)
    }

    /// Create a hook
    ///
    /// - `body`: See [`CreateHookOption`]
    pub fn admin_create_hook(
        &self,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreateHook> {
        endpoints::AdminCreateHook { body: body }
            .make_request()
            .wrap_sync::<endpoints::AdminCreateHook>(self)
    }

    /// Get a hook
    ///
    /// - `id`: id of the hook to get
    pub fn admin_get_hook(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetHook> {
        endpoints::AdminGetHook { id }
            .make_request()
            .wrap_sync::<endpoints::AdminGetHook>(self)
    }

    /// Delete a hook
    ///
    /// - `id`: id of the hook to delete
    pub fn admin_delete_hook(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminDeleteHook> {
        endpoints::AdminDeleteHook { id }
            .make_request()
            .wrap_sync::<endpoints::AdminDeleteHook>(self)
    }

    /// Update a hook
    ///
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub fn admin_edit_hook(
        &self,
        id: u64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminEditHook> {
        endpoints::AdminEditHook { id, body: body }
            .make_request()
            .wrap_sync::<endpoints::AdminEditHook>(self)
    }

    /// List all organizations
    ///
    pub fn admin_get_all_orgs(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetAllOrgs> {
        endpoints::AdminGetAllOrgs {}
            .make_request()
            .wrap_sync::<endpoints::AdminGetAllOrgs>(self)
    }

    /// List the available quota groups
    pub fn admin_list_quota_groups(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminListQuotaGroups> {
        endpoints::AdminListQuotaGroups {}
            .make_request()
            .wrap_sync::<endpoints::AdminListQuotaGroups>(self)
    }

    /// Create a new quota group
    ///
    /// - `group`: Definition of the quota group

    ///   See [`CreateQuotaGroupOptions`]
    pub fn admin_create_quota_group(
        &self,
        group: CreateQuotaGroupOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreateQuotaGroup> {
        endpoints::AdminCreateQuotaGroup { body: group }
            .make_request()
            .wrap_sync::<endpoints::AdminCreateQuotaGroup>(self)
    }

    /// Get information about the quota group
    ///
    /// - `quotagroup`: quota group to query
    pub fn admin_get_quota_group(
        &self,
        quotagroup: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetQuotaGroup> {
        endpoints::AdminGetQuotaGroup { quotagroup }
            .make_request()
            .wrap_sync::<endpoints::AdminGetQuotaGroup>(self)
    }

    /// Delete a quota group
    ///
    /// - `quotagroup`: quota group to delete
    pub fn admin_delete_quota_group(
        &self,
        quotagroup: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminDeleteQuotaGroup> {
        endpoints::AdminDeleteQuotaGroup { quotagroup }
            .make_request()
            .wrap_sync::<endpoints::AdminDeleteQuotaGroup>(self)
    }

    /// Adds a rule to a quota group
    ///
    /// - `quotagroup`: quota group to add a rule to
    /// - `quotarule`: the name of the quota rule to add to the group
    pub fn admin_add_rule_to_quota_group(
        &self,
        quotagroup: &str,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminAddRuleToQuotaGroup> {
        endpoints::AdminAddRuleToQuotaGroup {
            quotagroup,
            quotarule,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminAddRuleToQuotaGroup>(self)
    }

    /// Removes a rule from a quota group
    ///
    /// - `quotagroup`: quota group to remove a rule from
    /// - `quotarule`: the name of the quota rule to remove from the group
    pub fn admin_remove_rule_from_quota_group(
        &self,
        quotagroup: &str,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminRemoveRuleFromQuotaGroup> {
        endpoints::AdminRemoveRuleFromQuotaGroup {
            quotagroup,
            quotarule,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminRemoveRuleFromQuotaGroup>(self)
    }

    /// List users in a quota group
    ///
    /// - `quotagroup`: quota group to list members of
    pub fn admin_list_users_in_quota_group(
        &self,
        quotagroup: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminListUsersInQuotaGroup> {
        endpoints::AdminListUsersInQuotaGroup { quotagroup }
            .make_request()
            .wrap_sync::<endpoints::AdminListUsersInQuotaGroup>(self)
    }

    /// Add a user to a quota group
    ///
    /// - `quotagroup`: quota group to add the user to
    /// - `username`: username of the user to add to the quota group
    pub fn admin_add_user_to_quota_group(
        &self,
        quotagroup: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminAddUserToQuotaGroup> {
        endpoints::AdminAddUserToQuotaGroup {
            quotagroup,
            username,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminAddUserToQuotaGroup>(self)
    }

    /// Remove a user from a quota group
    ///
    /// - `quotagroup`: quota group to remove a user from
    /// - `username`: username of the user to remove from the quota group
    pub fn admin_remove_user_from_quota_group(
        &self,
        quotagroup: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminRemoveUserFromQuotaGroup> {
        endpoints::AdminRemoveUserFromQuotaGroup {
            quotagroup,
            username,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminRemoveUserFromQuotaGroup>(self)
    }

    /// List the available quota rules
    pub fn admin_list_quota_rules(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminListQuotaRules> {
        endpoints::AdminListQuotaRules {}
            .make_request()
            .wrap_sync::<endpoints::AdminListQuotaRules>(self)
    }

    /// Create a new quota rule
    ///
    /// - `rule`: Definition of the quota rule

    ///   See [`CreateQuotaRuleOptions`]
    pub fn admin_create_quota_rule(
        &self,
        rule: CreateQuotaRuleOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreateQuotaRule> {
        endpoints::AdminCreateQuotaRule { body: rule }
            .make_request()
            .wrap_sync::<endpoints::AdminCreateQuotaRule>(self)
    }

    /// Get information about a quota rule
    ///
    /// - `quotarule`: quota rule to query
    pub fn admin_get_quota_rule(
        &self,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetQuotaRule> {
        endpoints::AdminGetQuotaRule { quotarule }
            .make_request()
            .wrap_sync::<endpoints::AdminGetQuotaRule>(self)
    }

    /// Deletes a quota rule
    ///
    /// - `quotarule`: quota rule to delete
    pub fn admin_delete_quota_rule(
        &self,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminDeleteQuotaRule> {
        endpoints::AdminDeleteQuotaRule { quotarule }
            .make_request()
            .wrap_sync::<endpoints::AdminDeleteQuotaRule>(self)
    }

    /// Change an existing quota rule
    ///
    /// - `quotarule`: Quota rule to change
    /// - `rule`: See [`EditQuotaRuleOptions`]
    pub fn admin_edit_quota_rule(
        &self,
        quotarule: &str,
        rule: EditQuotaRuleOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminEditQuotaRule> {
        endpoints::AdminEditQuotaRule {
            quotarule,
            body: rule,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminEditQuotaRule>(self)
    }

    /// Get an global actions runner registration token
    pub fn admin_get_runner_registration_token(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetRunnerRegistrationToken> {
        endpoints::AdminGetRunnerRegistrationToken {}
            .make_request()
            .wrap_sync::<endpoints::AdminGetRunnerRegistrationToken>(self)
    }

    /// List unadopted repositories
    ///
    pub fn admin_unadopted_list(
        &self,
        query: AdminUnadoptedListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminUnadoptedList> {
        endpoints::AdminUnadoptedList { query }
            .make_request()
            .wrap_sync::<endpoints::AdminUnadoptedList>(self)
    }

    /// Adopt unadopted files as a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn admin_adopt_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminAdoptRepository> {
        endpoints::AdminAdoptRepository { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::AdminAdoptRepository>(self)
    }

    /// Delete unadopted files
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn admin_delete_unadopted_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminDeleteUnadoptedRepository> {
        endpoints::AdminDeleteUnadoptedRepository { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::AdminDeleteUnadoptedRepository>(self)
    }

    /// Search users according filter conditions
    ///
    pub fn admin_search_users(
        &self,
        query: AdminSearchUsersQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminSearchUsers> {
        endpoints::AdminSearchUsers { query }
            .make_request()
            .wrap_sync::<endpoints::AdminSearchUsers>(self)
    }

    /// Create a user
    ///
    /// - `body`: See [`CreateUserOption`]
    pub fn admin_create_user(
        &self,
        body: CreateUserOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreateUser> {
        endpoints::AdminCreateUser { body: body }
            .make_request()
            .wrap_sync::<endpoints::AdminCreateUser>(self)
    }

    /// Delete a user
    ///
    /// - `username`: username of user to delete
    pub fn admin_delete_user(
        &self,
        username: &str,
        query: AdminDeleteUserQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminDeleteUser> {
        endpoints::AdminDeleteUser { username, query }
            .make_request()
            .wrap_sync::<endpoints::AdminDeleteUser>(self)
    }

    /// Edit an existing user
    ///
    /// - `username`: username of user to edit
    /// - `body`: See [`EditUserOption`]
    pub fn admin_edit_user(
        &self,
        username: &str,
        body: EditUserOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminEditUser> {
        endpoints::AdminEditUser {
            username,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminEditUser>(self)
    }

    /// Add a public key on behalf of a user
    ///
    /// - `username`: username of the user
    /// - `key`: See [`CreateKeyOption`]
    pub fn admin_create_public_key(
        &self,
        username: &str,
        key: CreateKeyOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreatePublicKey> {
        endpoints::AdminCreatePublicKey {
            username,
            body: key,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminCreatePublicKey>(self)
    }

    /// Delete a user's public key
    ///
    /// - `username`: username of user
    /// - `id`: id of the key to delete
    pub fn admin_delete_user_public_key(
        &self,
        username: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminDeleteUserPublicKey> {
        endpoints::AdminDeleteUserPublicKey { username, id }
            .make_request()
            .wrap_sync::<endpoints::AdminDeleteUserPublicKey>(self)
    }

    /// Create an organization
    ///
    /// - `username`: username of the user that will own the created organization
    /// - `organization`: See [`CreateOrgOption`]
    pub fn admin_create_org(
        &self,
        username: &str,
        organization: CreateOrgOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreateOrg> {
        endpoints::AdminCreateOrg {
            username,
            body: organization,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminCreateOrg>(self)
    }

    /// Get the user's quota info
    ///
    /// - `username`: username of user to query
    pub fn admin_get_user_quota(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminGetUserQuota> {
        endpoints::AdminGetUserQuota { username }
            .make_request()
            .wrap_sync::<endpoints::AdminGetUserQuota>(self)
    }

    /// Set the user's quota groups to a given list.
    ///
    /// - `username`: username of the user to modify the quota groups from
    /// - `groups`: list of groups that the user should be a member of

    ///   See [`SetUserQuotaGroupsOptions`]
    pub fn admin_set_user_quota_groups(
        &self,
        username: &str,
        groups: SetUserQuotaGroupsOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminSetUserQuotaGroups> {
        endpoints::AdminSetUserQuotaGroups {
            username,
            body: groups,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminSetUserQuotaGroups>(self)
    }

    /// Rename a user
    ///
    /// - `username`: existing username of user
    /// - `body`: See [`RenameUserOption`]
    pub fn admin_rename_user(
        &self,
        username: &str,
        body: RenameUserOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminRenameUser> {
        endpoints::AdminRenameUser {
            username,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminRenameUser>(self)
    }

    /// Create a repository on behalf of a user
    ///
    /// - `username`: username of the user. This user will own the created repository
    /// - `repository`: See [`CreateRepoOption`]
    pub fn admin_create_repo(
        &self,
        username: &str,
        repository: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AdminCreateRepo> {
        endpoints::AdminCreateRepo {
            username,
            body: repository,
        }
        .make_request()
        .wrap_sync::<endpoints::AdminCreateRepo>(self)
    }

    /// Returns a list of all gitignore templates
    pub fn list_gitignores_templates(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListGitignoresTemplates> {
        endpoints::ListGitignoresTemplates {}
            .make_request()
            .wrap_sync::<endpoints::ListGitignoresTemplates>(self)
    }

    /// Returns information about a gitignore template
    ///
    /// - `name`: name of the template
    pub fn get_gitignore_template_info(
        &self,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetGitignoreTemplateInfo> {
        endpoints::GetGitignoreTemplateInfo { name }
            .make_request()
            .wrap_sync::<endpoints::GetGitignoreTemplateInfo>(self)
    }

    /// Returns a list of all label templates
    pub fn list_label_templates(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListLabelTemplates> {
        endpoints::ListLabelTemplates {}
            .make_request()
            .wrap_sync::<endpoints::ListLabelTemplates>(self)
    }

    /// Returns all labels in a template
    ///
    /// - `name`: name of the template
    pub fn get_label_template_info(
        &self,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetLabelTemplateInfo> {
        endpoints::GetLabelTemplateInfo { name }
            .make_request()
            .wrap_sync::<endpoints::GetLabelTemplateInfo>(self)
    }

    /// Returns a list of all license templates
    pub fn list_license_templates(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListLicenseTemplates> {
        endpoints::ListLicenseTemplates {}
            .make_request()
            .wrap_sync::<endpoints::ListLicenseTemplates>(self)
    }

    /// Returns information about a license template
    ///
    /// - `name`: name of the license
    pub fn get_license_template_info(
        &self,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetLicenseTemplateInfo> {
        endpoints::GetLicenseTemplateInfo { name }
            .make_request()
            .wrap_sync::<endpoints::GetLicenseTemplateInfo>(self)
    }

    /// Render a markdown document as HTML
    ///
    /// - `body`: See [`MarkdownOption`]
    pub fn render_markdown(
        &self,
        body: MarkdownOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RenderMarkdown> {
        endpoints::RenderMarkdown { body: body }
            .make_request()
            .wrap_sync::<endpoints::RenderMarkdown>(self)
    }

    /// Render raw markdown as HTML
    ///
    /// - `body`: Request body to render

    ///   See [`String`]
    pub fn render_markdown_raw(
        &self,
        body: String,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RenderMarkdownRaw> {
        endpoints::RenderMarkdownRaw { body: body }
            .make_request()
            .wrap_sync::<endpoints::RenderMarkdownRaw>(self)
    }

    /// Render a markup document as HTML
    ///
    /// - `body`: See [`MarkupOption`]
    pub fn render_markup(
        &self,
        body: MarkupOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RenderMarkup> {
        endpoints::RenderMarkup { body: body }
            .make_request()
            .wrap_sync::<endpoints::RenderMarkup>(self)
    }

    /// Returns the nodeinfo of the Forgejo application
    pub fn get_node_info(&self) -> crate::sync::WrappedRequest<'_, endpoints::GetNodeInfo> {
        endpoints::GetNodeInfo {}
            .make_request()
            .wrap_sync::<endpoints::GetNodeInfo>(self)
    }

    /// List users's notification threads
    ///
    pub fn notify_get_list(
        &self,
        query: NotifyGetListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyGetList> {
        endpoints::NotifyGetList { query }
            .make_request()
            .wrap_sync::<endpoints::NotifyGetList>(self)
    }

    /// Mark notification threads as read, pinned or unread
    ///
    pub fn notify_read_list(
        &self,
        query: NotifyReadListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyReadList> {
        endpoints::NotifyReadList { query }
            .make_request()
            .wrap_sync::<endpoints::NotifyReadList>(self)
    }

    /// Check if unread notifications exist
    pub fn notify_new_available(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyNewAvailable> {
        endpoints::NotifyNewAvailable {}
            .make_request()
            .wrap_sync::<endpoints::NotifyNewAvailable>(self)
    }

    /// Get notification thread by ID
    ///
    /// - `id`: id of notification thread
    pub fn notify_get_thread(
        &self,
        id: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyGetThread> {
        endpoints::NotifyGetThread { id }
            .make_request()
            .wrap_sync::<endpoints::NotifyGetThread>(self)
    }

    /// Mark notification thread as read by ID
    ///
    /// - `id`: id of notification thread
    pub fn notify_read_thread(
        &self,
        id: &str,
        query: NotifyReadThreadQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyReadThread> {
        endpoints::NotifyReadThread { id, query }
            .make_request()
            .wrap_sync::<endpoints::NotifyReadThread>(self)
    }

    /// Create a repository in an organization
    ///
    /// - `org`: name of organization
    /// - `body`: See [`CreateRepoOption`]
    pub fn create_org_repo_deprecated(
        &self,
        org: &str,
        body: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateOrgRepoDeprecated> {
        endpoints::CreateOrgRepoDeprecated { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::CreateOrgRepoDeprecated>(self)
    }

    /// Get list of organizations
    ///
    pub fn org_get_all(&self) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetAll> {
        endpoints::OrgGetAll {}
            .make_request()
            .wrap_sync::<endpoints::OrgGetAll>(self)
    }

    /// Create an organization
    ///
    /// - `organization`: See [`CreateOrgOption`]
    pub fn org_create(
        &self,
        organization: CreateOrgOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgCreate> {
        endpoints::OrgCreate { body: organization }
            .make_request()
            .wrap_sync::<endpoints::OrgCreate>(self)
    }

    /// Get an organization
    ///
    /// - `org`: name of the organization to get
    pub fn org_get(&self, org: &str) -> crate::sync::WrappedRequest<'_, endpoints::OrgGet> {
        endpoints::OrgGet { org }
            .make_request()
            .wrap_sync::<endpoints::OrgGet>(self)
    }

    /// Delete an organization
    ///
    /// - `org`: organization that is to be deleted
    pub fn org_delete(&self, org: &str) -> crate::sync::WrappedRequest<'_, endpoints::OrgDelete> {
        endpoints::OrgDelete { org }
            .make_request()
            .wrap_sync::<endpoints::OrgDelete>(self)
    }

    /// Edit an organization
    ///
    /// - `org`: name of the organization to edit
    /// - `body`: See [`EditOrgOption`]
    pub fn org_edit(
        &self,
        org: &str,
        body: EditOrgOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgEdit> {
        endpoints::OrgEdit { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::OrgEdit>(self)
    }

    /// Get an organization's actions runner registration token
    ///
    /// - `org`: name of the organization
    pub fn org_get_runner_registration_token(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetRunnerRegistrationToken> {
        endpoints::OrgGetRunnerRegistrationToken { org }
            .make_request()
            .wrap_sync::<endpoints::OrgGetRunnerRegistrationToken>(self)
    }

    /// List an organization's actions secrets
    ///
    /// - `org`: name of the organization
    pub fn org_list_actions_secrets(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListActionsSecrets> {
        endpoints::OrgListActionsSecrets { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListActionsSecrets>(self)
    }

    /// Create or Update a secret value in an organization
    ///
    /// - `org`: name of organization
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub fn update_org_secret(
        &self,
        org: &str,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateOrgSecret> {
        endpoints::UpdateOrgSecret {
            org,
            secretname,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UpdateOrgSecret>(self)
    }

    /// Delete a secret in an organization
    ///
    /// - `org`: name of organization
    /// - `secretname`: name of the secret
    pub fn delete_org_secret(
        &self,
        org: &str,
        secretname: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeleteOrgSecret> {
        endpoints::DeleteOrgSecret { org, secretname }
            .make_request()
            .wrap_sync::<endpoints::DeleteOrgSecret>(self)
    }

    /// Get an org-level variables list
    ///
    /// - `org`: name of the organization
    pub fn get_org_variables_list(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetOrgVariablesList> {
        endpoints::GetOrgVariablesList { org }
            .make_request()
            .wrap_sync::<endpoints::GetOrgVariablesList>(self)
    }

    /// Get an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    pub fn get_org_variable(
        &self,
        org: &str,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetOrgVariable> {
        endpoints::GetOrgVariable { org, variablename }
            .make_request()
            .wrap_sync::<endpoints::GetOrgVariable>(self)
    }

    /// Update an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub fn update_org_variable(
        &self,
        org: &str,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateOrgVariable> {
        endpoints::UpdateOrgVariable {
            org,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UpdateOrgVariable>(self)
    }

    /// Create an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub fn create_org_variable(
        &self,
        org: &str,
        variablename: &str,
        body: CreateVariableOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateOrgVariable> {
        endpoints::CreateOrgVariable {
            org,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::CreateOrgVariable>(self)
    }

    /// Delete an org-level variable
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    pub fn delete_org_variable(
        &self,
        org: &str,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeleteOrgVariable> {
        endpoints::DeleteOrgVariable { org, variablename }
            .make_request()
            .wrap_sync::<endpoints::DeleteOrgVariable>(self)
    }

    /// List an organization's activity feeds
    ///
    /// - `org`: name of the org
    pub fn org_list_activity_feeds(
        &self,
        org: &str,
        query: OrgListActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListActivityFeeds> {
        endpoints::OrgListActivityFeeds { org, query }
            .make_request()
            .wrap_sync::<endpoints::OrgListActivityFeeds>(self)
    }

    /// Update Avatar
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`UpdateUserAvatarOption`]
    pub fn org_update_avatar(
        &self,
        org: &str,
        body: UpdateUserAvatarOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgUpdateAvatar> {
        endpoints::OrgUpdateAvatar { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::OrgUpdateAvatar>(self)
    }

    /// Delete Avatar
    ///
    /// - `org`: name of the organization
    pub fn org_delete_avatar(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgDeleteAvatar> {
        endpoints::OrgDeleteAvatar { org }
            .make_request()
            .wrap_sync::<endpoints::OrgDeleteAvatar>(self)
    }

    /// Blocks a user from the organization
    ///
    /// - `org`: name of the org
    /// - `username`: username of the user
    pub fn org_block_user(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgBlockUser> {
        endpoints::OrgBlockUser { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgBlockUser>(self)
    }

    /// List an organization's webhooks
    ///
    /// - `org`: name of the organization
    pub fn org_list_hooks(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListHooks> {
        endpoints::OrgListHooks { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListHooks>(self)
    }

    /// Create a hook
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateHookOption`]
    pub fn org_create_hook(
        &self,
        org: &str,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgCreateHook> {
        endpoints::OrgCreateHook { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::OrgCreateHook>(self)
    }

    /// Get a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to get
    pub fn org_get_hook(
        &self,
        org: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetHook> {
        endpoints::OrgGetHook { org, id }
            .make_request()
            .wrap_sync::<endpoints::OrgGetHook>(self)
    }

    /// Delete a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to delete
    pub fn org_delete_hook(
        &self,
        org: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgDeleteHook> {
        endpoints::OrgDeleteHook { org, id }
            .make_request()
            .wrap_sync::<endpoints::OrgDeleteHook>(self)
    }

    /// Update a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub fn org_edit_hook(
        &self,
        org: &str,
        id: u64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgEditHook> {
        endpoints::OrgEditHook {
            org,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::OrgEditHook>(self)
    }

    /// List an organization's labels
    ///
    /// - `org`: name of the organization
    pub fn org_list_labels(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListLabels> {
        endpoints::OrgListLabels { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListLabels>(self)
    }

    /// Create a label for an organization
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateLabelOption`]
    pub fn org_create_label(
        &self,
        org: &str,
        body: CreateLabelOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgCreateLabel> {
        endpoints::OrgCreateLabel { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::OrgCreateLabel>(self)
    }

    /// Get a single label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to get
    pub fn org_get_label(
        &self,
        org: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetLabel> {
        endpoints::OrgGetLabel { org, id }
            .make_request()
            .wrap_sync::<endpoints::OrgGetLabel>(self)
    }

    /// Delete a label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to delete
    pub fn org_delete_label(
        &self,
        org: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgDeleteLabel> {
        endpoints::OrgDeleteLabel { org, id }
            .make_request()
            .wrap_sync::<endpoints::OrgDeleteLabel>(self)
    }

    /// Update a label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to edit
    /// - `body`: See [`EditLabelOption`]
    pub fn org_edit_label(
        &self,
        org: &str,
        id: u64,
        body: EditLabelOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgEditLabel> {
        endpoints::OrgEditLabel {
            org,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::OrgEditLabel>(self)
    }

    /// List the organization's blocked users
    ///
    /// - `org`: name of the org
    pub fn org_list_blocked_users(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListBlockedUsers> {
        endpoints::OrgListBlockedUsers { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListBlockedUsers>(self)
    }

    /// List an organization's members
    ///
    /// - `org`: name of the organization
    pub fn org_list_members(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListMembers> {
        endpoints::OrgListMembers { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListMembers>(self)
    }

    /// Check if a user is a member of an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_is_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgIsMember> {
        endpoints::OrgIsMember { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgIsMember>(self)
    }

    /// Remove a member from an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_delete_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgDeleteMember> {
        endpoints::OrgDeleteMember { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgDeleteMember>(self)
    }

    /// List an organization's public members
    ///
    /// - `org`: name of the organization
    pub fn org_list_public_members(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListPublicMembers> {
        endpoints::OrgListPublicMembers { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListPublicMembers>(self)
    }

    /// Check if a user is a public member of an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_is_public_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgIsPublicMember> {
        endpoints::OrgIsPublicMember { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgIsPublicMember>(self)
    }

    /// Publicize a user's membership
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_publicize_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgPublicizeMember> {
        endpoints::OrgPublicizeMember { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgPublicizeMember>(self)
    }

    /// Conceal a user's membership
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_conceal_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgConcealMember> {
        endpoints::OrgConcealMember { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgConcealMember>(self)
    }

    /// Get quota information for an organization
    ///
    /// - `org`: name of the organization
    pub fn org_get_quota(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetQuota> {
        endpoints::OrgGetQuota { org }
            .make_request()
            .wrap_sync::<endpoints::OrgGetQuota>(self)
    }

    /// List the artifacts affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub fn org_list_quota_artifacts(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListQuotaArtifacts> {
        endpoints::OrgListQuotaArtifacts { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListQuotaArtifacts>(self)
    }

    /// List the attachments affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub fn org_list_quota_attachments(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListQuotaAttachments> {
        endpoints::OrgListQuotaAttachments { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListQuotaAttachments>(self)
    }

    /// Check if the organization is over quota for a given subject
    ///
    /// - `org`: name of the organization
    pub fn org_check_quota(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgCheckQuota> {
        endpoints::OrgCheckQuota { org }
            .make_request()
            .wrap_sync::<endpoints::OrgCheckQuota>(self)
    }

    /// List the packages affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub fn org_list_quota_packages(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListQuotaPackages> {
        endpoints::OrgListQuotaPackages { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListQuotaPackages>(self)
    }

    /// List an organization's repos
    ///
    /// - `org`: name of the organization
    pub fn org_list_repos(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListRepos> {
        endpoints::OrgListRepos { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListRepos>(self)
    }

    /// Create a repository in an organization
    ///
    /// - `org`: name of organization
    /// - `body`: See [`CreateRepoOption`]
    pub fn create_org_repo(
        &self,
        org: &str,
        body: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateOrgRepo> {
        endpoints::CreateOrgRepo { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::CreateOrgRepo>(self)
    }

    /// List an organization's teams
    ///
    /// - `org`: name of the organization
    pub fn org_list_teams(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListTeams> {
        endpoints::OrgListTeams { org }
            .make_request()
            .wrap_sync::<endpoints::OrgListTeams>(self)
    }

    /// Create a team
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateTeamOption`]
    pub fn org_create_team(
        &self,
        org: &str,
        body: CreateTeamOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgCreateTeam> {
        endpoints::OrgCreateTeam { org, body: body }
            .make_request()
            .wrap_sync::<endpoints::OrgCreateTeam>(self)
    }

    /// Search for teams within an organization
    ///
    /// - `org`: name of the organization
    pub fn team_search(
        &self,
        org: &str,
        query: TeamSearchQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::TeamSearch> {
        endpoints::TeamSearch { org, query }
            .make_request()
            .wrap_sync::<endpoints::TeamSearch>(self)
    }

    /// Unblock a user from the organization
    ///
    /// - `org`: name of the org
    /// - `username`: username of the user
    pub fn org_unblock_user(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgUnblockUser> {
        endpoints::OrgUnblockUser { org, username }
            .make_request()
            .wrap_sync::<endpoints::OrgUnblockUser>(self)
    }

    /// Gets all packages of an owner
    ///
    /// - `owner`: owner of the packages
    pub fn list_packages(
        &self,
        owner: &str,
        query: ListPackagesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListPackages> {
        endpoints::ListPackages { owner, query }
            .make_request()
            .wrap_sync::<endpoints::ListPackages>(self)
    }

    /// Gets a package
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `version`: version of the package
    pub fn get_package(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        version: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetPackage> {
        endpoints::GetPackage {
            owner,
            r#type,
            name,
            version,
        }
        .make_request()
        .wrap_sync::<endpoints::GetPackage>(self)
    }

    /// Delete a package
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `version`: version of the package
    pub fn delete_package(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        version: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeletePackage> {
        endpoints::DeletePackage {
            owner,
            r#type,
            name,
            version,
        }
        .make_request()
        .wrap_sync::<endpoints::DeletePackage>(self)
    }

    /// Gets all files of a package
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `version`: version of the package
    pub fn list_package_files(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        version: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListPackageFiles> {
        endpoints::ListPackageFiles {
            owner,
            r#type,
            name,
            version,
        }
        .make_request()
        .wrap_sync::<endpoints::ListPackageFiles>(self)
    }

    /// Search for issues across the repositories that the user has access to
    ///
    pub fn issue_search_issues(
        &self,
        query: IssueSearchIssuesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueSearchIssues> {
        endpoints::IssueSearchIssues { query }
            .make_request()
            .wrap_sync::<endpoints::IssueSearchIssues>(self)
    }

    /// Migrate a remote git repository
    ///
    /// - `body`: See [`MigrateRepoOptions`]
    pub fn repo_migrate(
        &self,
        body: MigrateRepoOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoMigrate> {
        endpoints::RepoMigrate { body: body }
            .make_request()
            .wrap_sync::<endpoints::RepoMigrate>(self)
    }

    /// Search for repositories
    ///
    pub fn repo_search(
        &self,
        query: RepoSearchQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoSearch> {
        endpoints::RepoSearch { query }
            .make_request()
            .wrap_sync::<endpoints::RepoSearch>(self)
    }

    /// Get a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGet> {
        endpoints::RepoGet { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGet>(self)
    }

    /// Delete a repository
    ///
    /// - `owner`: owner of the repo to delete
    /// - `repo`: name of the repo to delete
    pub fn repo_delete(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDelete> {
        endpoints::RepoDelete { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoDelete>(self)
    }

    /// Edit a repository's properties. Only fields that are set will be changed.
    ///
    /// - `owner`: owner of the repo to edit
    /// - `repo`: name of the repo to edit
    /// - `body`: Properties of a repo that you can edit

    ///   See [`EditRepoOption`]
    pub fn repo_edit(
        &self,
        owner: &str,
        repo: &str,
        body: EditRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEdit> {
        endpoints::RepoEdit {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEdit>(self)
    }

    /// Get a repository's actions runner registration token
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_runner_registration_token(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetRunnerRegistrationToken> {
        endpoints::RepoGetRunnerRegistrationToken { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetRunnerRegistrationToken>(self)
    }

    /// List an repo's actions secrets
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    pub fn repo_list_actions_secrets(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListActionsSecrets> {
        endpoints::RepoListActionsSecrets { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListActionsSecrets>(self)
    }

    /// Create or Update a secret value in a repository
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub fn update_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateRepoSecret> {
        endpoints::UpdateRepoSecret {
            owner,
            repo,
            secretname,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UpdateRepoSecret>(self)
    }

    /// Delete a secret in a repository
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    /// - `secretname`: name of the secret
    pub fn delete_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secretname: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeleteRepoSecret> {
        endpoints::DeleteRepoSecret {
            owner,
            repo,
            secretname,
        }
        .make_request()
        .wrap_sync::<endpoints::DeleteRepoSecret>(self)
    }

    /// List a repository's action tasks
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn list_action_tasks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListActionTasks> {
        endpoints::ListActionTasks { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::ListActionTasks>(self)
    }

    /// Get repo-level variables list
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    pub fn get_repo_variables_list(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetRepoVariablesList> {
        endpoints::GetRepoVariablesList { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::GetRepoVariablesList>(self)
    }

    /// Get a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    pub fn get_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetRepoVariable> {
        endpoints::GetRepoVariable {
            owner,
            repo,
            variablename,
        }
        .make_request()
        .wrap_sync::<endpoints::GetRepoVariable>(self)
    }

    /// Update a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub fn update_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateRepoVariable> {
        endpoints::UpdateRepoVariable {
            owner,
            repo,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UpdateRepoVariable>(self)
    }

    /// Create a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub fn create_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
        body: CreateVariableOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateRepoVariable> {
        endpoints::CreateRepoVariable {
            owner,
            repo,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::CreateRepoVariable>(self)
    }

    /// Delete a repo-level variable
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    /// - `variablename`: name of the variable
    pub fn delete_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeleteRepoVariable> {
        endpoints::DeleteRepoVariable {
            owner,
            repo,
            variablename,
        }
        .make_request()
        .wrap_sync::<endpoints::DeleteRepoVariable>(self)
    }

    /// Dispatches a workflow
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `workflowname`: name of the workflow
    /// - `body`: See [`DispatchWorkflowOption`]
    pub fn dispatch_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflowname: &str,
        body: DispatchWorkflowOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DispatchWorkflow> {
        endpoints::DispatchWorkflow {
            owner,
            repo,
            workflowname,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::DispatchWorkflow>(self)
    }

    /// List a repository's activity feeds
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_activity_feeds(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListActivityFeeds> {
        endpoints::RepoListActivityFeeds { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoListActivityFeeds>(self)
    }

    /// Get an archive of a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `archive`: the git reference for download with attached archive format (e.g. master.zip)
    pub fn repo_get_archive(
        &self,
        owner: &str,
        repo: &str,
        archive: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetArchive> {
        endpoints::RepoGetArchive {
            owner,
            repo,
            archive,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetArchive>(self)
    }

    /// Return all users that have write access and can be assigned to issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetAssignees> {
        endpoints::RepoGetAssignees { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetAssignees>(self)
    }

    /// Update avatar
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`UpdateRepoAvatarOption`]
    pub fn repo_update_avatar(
        &self,
        owner: &str,
        repo: &str,
        body: UpdateRepoAvatarOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoUpdateAvatar> {
        endpoints::RepoUpdateAvatar {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoUpdateAvatar>(self)
    }

    /// Delete avatar
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_delete_avatar(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteAvatar> {
        endpoints::RepoDeleteAvatar { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteAvatar>(self)
    }

    /// List branch protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_branch_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListBranchProtection> {
        endpoints::RepoListBranchProtection { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListBranchProtection>(self)
    }

    /// Create a branch protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateBranchProtectionOption`]
    pub fn repo_create_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        body: CreateBranchProtectionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateBranchProtection> {
        endpoints::RepoCreateBranchProtection {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateBranchProtection>(self)
    }

    /// Get a specific branch protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: name of protected branch
    pub fn repo_get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetBranchProtection> {
        endpoints::RepoGetBranchProtection { owner, repo, name }
            .make_request()
            .wrap_sync::<endpoints::RepoGetBranchProtection>(self)
    }

    /// Delete a specific branch protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: name of protected branch
    pub fn repo_delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteBranchProtection> {
        endpoints::RepoDeleteBranchProtection { owner, repo, name }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteBranchProtection>(self)
    }

    /// Edit a branch protections for a repository. Only fields that are set will be changed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: name of protected branch
    /// - `body`: See [`EditBranchProtectionOption`]
    pub fn repo_edit_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: EditBranchProtectionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditBranchProtection> {
        endpoints::RepoEditBranchProtection {
            owner,
            repo,
            name,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditBranchProtection>(self)
    }

    /// List a repository's branches
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_branches(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListBranches> {
        endpoints::RepoListBranches { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListBranches>(self)
    }

    /// Create a branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateBranchRepoOption`]
    pub fn repo_create_branch(
        &self,
        owner: &str,
        repo: &str,
        body: CreateBranchRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateBranch> {
        endpoints::RepoCreateBranch {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateBranch>(self)
    }

    /// Retrieve a specific branch from a repository, including its effective branch protection
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: branch to get
    pub fn repo_get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetBranch> {
        endpoints::RepoGetBranch {
            owner,
            repo,
            branch,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetBranch>(self)
    }

    /// Delete a specific branch from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: branch to delete
    pub fn repo_delete_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteBranch> {
        endpoints::RepoDeleteBranch {
            owner,
            repo,
            branch,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeleteBranch>(self)
    }

    /// Update a branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: name of the branch
    /// - `body`: See [`UpdateBranchRepoOption`]
    pub fn repo_update_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: UpdateBranchRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoUpdateBranch> {
        endpoints::RepoUpdateBranch {
            owner,
            repo,
            branch,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoUpdateBranch>(self)
    }

    /// List a repository's collaborators
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_collaborators(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListCollaborators> {
        endpoints::RepoListCollaborators { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListCollaborators>(self)
    }

    /// Check if a user is a collaborator of a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator
    pub fn repo_check_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCheckCollaborator> {
        endpoints::RepoCheckCollaborator {
            owner,
            repo,
            collaborator,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCheckCollaborator>(self)
    }

    /// Add a collaborator to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator to add
    /// - `body`: See [`AddCollaboratorOption`]
    pub fn repo_add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
        body: AddCollaboratorOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoAddCollaborator> {
        endpoints::RepoAddCollaborator {
            owner,
            repo,
            collaborator,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoAddCollaborator>(self)
    }

    /// Delete a collaborator from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator to delete
    pub fn repo_delete_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteCollaborator> {
        endpoints::RepoDeleteCollaborator {
            owner,
            repo,
            collaborator,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeleteCollaborator>(self)
    }

    /// Get repository permissions for a user
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `collaborator`: username of the collaborator
    pub fn repo_get_repo_permissions(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetRepoPermissions> {
        endpoints::RepoGetRepoPermissions {
            owner,
            repo,
            collaborator,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetRepoPermissions>(self)
    }

    /// Get a list of all commits from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_all_commits(
        &self,
        owner: &str,
        repo: &str,
        query: RepoGetAllCommitsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetAllCommits> {
        endpoints::RepoGetAllCommits { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoGetAllCommits>(self)
    }

    /// Get a commit's combined status, by branch/tag/commit reference
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `ref`: name of branch/tag/commit
    pub fn repo_get_combined_status_by_ref(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetCombinedStatusByRef> {
        endpoints::RepoGetCombinedStatusByRef { owner, repo, r#ref }
            .make_request()
            .wrap_sync::<endpoints::RepoGetCombinedStatusByRef>(self)
    }

    /// Get a commit's statuses, by branch/tag/commit reference
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `ref`: name of branch/tag/commit
    pub fn repo_list_statuses_by_ref(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
        query: RepoListStatusesByRefQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListStatusesByRef> {
        endpoints::RepoListStatusesByRef {
            owner,
            repo,
            r#ref,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoListStatusesByRef>(self)
    }

    /// Get the pull request of the commit
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: SHA of the commit to get
    pub fn repo_get_commit_pull_request(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetCommitPullRequest> {
        endpoints::RepoGetCommitPullRequest { owner, repo, sha }
            .make_request()
            .wrap_sync::<endpoints::RepoGetCommitPullRequest>(self)
    }

    /// Get commit comparison information
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `basehead`: compare two branches or commits
    pub fn repo_compare_diff(
        &self,
        owner: &str,
        repo: &str,
        basehead: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCompareDiff> {
        endpoints::RepoCompareDiff {
            owner,
            repo,
            basehead,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCompareDiff>(self)
    }

    /// Gets the metadata of all the entries of the root dir
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_contents_list(
        &self,
        owner: &str,
        repo: &str,
        query: RepoGetContentsListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetContentsList> {
        endpoints::RepoGetContentsList { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoGetContentsList>(self)
    }

    /// Modify multiple files in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`ChangeFilesOptions`]
    pub fn repo_change_files(
        &self,
        owner: &str,
        repo: &str,
        body: ChangeFilesOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoChangeFiles> {
        endpoints::RepoChangeFiles {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoChangeFiles>(self)
    }

    /// Gets the metadata and contents (if a file) of an entry in a repository, or a list of entries if a dir
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the dir, file, symlink or submodule in the repo
    pub fn repo_get_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetContentsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetContents> {
        endpoints::RepoGetContents {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetContents>(self)
    }

    /// Update a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the file to update
    /// - `body`: See [`UpdateFileOptions`]
    pub fn repo_update_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: UpdateFileOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoUpdateFile> {
        endpoints::RepoUpdateFile {
            owner,
            repo,
            filepath,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoUpdateFile>(self)
    }

    /// Create a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the file to create
    /// - `body`: See [`CreateFileOptions`]
    pub fn repo_create_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: CreateFileOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateFile> {
        endpoints::RepoCreateFile {
            owner,
            repo,
            filepath,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateFile>(self)
    }

    /// Delete a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: path of the file to delete
    /// - `body`: See [`DeleteFileOptions`]
    pub fn repo_delete_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: DeleteFileOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteFile> {
        endpoints::RepoDeleteFile {
            owner,
            repo,
            filepath,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeleteFile>(self)
    }

    /// Apply diff patch to repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`UpdateFileOptions`]
    pub fn repo_apply_diff_patch(
        &self,
        owner: &str,
        repo: &str,
        body: UpdateFileOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoApplyDiffPatch> {
        endpoints::RepoApplyDiffPatch {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoApplyDiffPatch>(self)
    }

    /// Get the EditorConfig definitions of a file in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: filepath of file to get
    pub fn repo_get_editor_config(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetEditorConfigQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetEditorConfig> {
        endpoints::RepoGetEditorConfig {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetEditorConfig>(self)
    }

    /// List a repository's flags
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_flags(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListFlags> {
        endpoints::RepoListFlags { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListFlags>(self)
    }

    /// Replace all flags of a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`ReplaceFlagsOption`]
    pub fn repo_replace_all_flags(
        &self,
        owner: &str,
        repo: &str,
        body: ReplaceFlagsOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoReplaceAllFlags> {
        endpoints::RepoReplaceAllFlags {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoReplaceAllFlags>(self)
    }

    /// Remove all flags from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_delete_all_flags(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteAllFlags> {
        endpoints::RepoDeleteAllFlags { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteAllFlags>(self)
    }

    /// Check if a repository has a given flag
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `flag`: name of the flag
    pub fn repo_check_flag(
        &self,
        owner: &str,
        repo: &str,
        flag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCheckFlag> {
        endpoints::RepoCheckFlag { owner, repo, flag }
            .make_request()
            .wrap_sync::<endpoints::RepoCheckFlag>(self)
    }

    /// Add a flag to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `flag`: name of the flag
    pub fn repo_add_flag(
        &self,
        owner: &str,
        repo: &str,
        flag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoAddFlag> {
        endpoints::RepoAddFlag { owner, repo, flag }
            .make_request()
            .wrap_sync::<endpoints::RepoAddFlag>(self)
    }

    /// Remove a flag from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `flag`: name of the flag
    pub fn repo_delete_flag(
        &self,
        owner: &str,
        repo: &str,
        flag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteFlag> {
        endpoints::RepoDeleteFlag { owner, repo, flag }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteFlag>(self)
    }

    /// List a repository's forks
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn list_forks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::ListForks> {
        endpoints::ListForks { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::ListForks>(self)
    }

    /// Fork a repository
    ///
    /// - `owner`: owner of the repo to fork
    /// - `repo`: name of the repo to fork
    /// - `body`: See [`CreateForkOption`]
    pub fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        body: CreateForkOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateFork> {
        endpoints::CreateFork {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::CreateFork>(self)
    }

    /// Gets the blob of a repository.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    pub fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetBlob> {
        endpoints::GetBlob { owner, repo, sha }
            .make_request()
            .wrap_sync::<endpoints::GetBlob>(self)
    }

    /// Get a single commit from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: a git ref or commit sha
    pub fn repo_get_single_commit(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: RepoGetSingleCommitQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetSingleCommit> {
        endpoints::RepoGetSingleCommit {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetSingleCommit>(self)
    }

    /// Get a commit's diff or patch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: SHA of the commit to get
    /// - `diffType`: whether the output is diff or patch
    pub fn repo_download_commit_diff_or_patch(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        diff_type: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDownloadCommitDiffOrPatch> {
        endpoints::RepoDownloadCommitDiffOrPatch {
            owner,
            repo,
            sha,
            diff_type,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDownloadCommitDiffOrPatch>(self)
    }

    /// Get a note corresponding to a single commit from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: a git ref or commit sha
    pub fn repo_get_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: RepoGetNoteQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetNote> {
        endpoints::RepoGetNote {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetNote>(self)
    }

    /// Set a note corresponding to a single commit from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: a git ref or commit sha
    /// - `body`: See [`NoteOptions`]
    pub fn repo_set_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: NoteOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoSetNote> {
        endpoints::RepoSetNote {
            owner,
            repo,
            sha,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoSetNote>(self)
    }

    /// Removes a note corresponding to a single commit from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: a git ref or commit sha
    pub fn repo_remove_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoRemoveNote> {
        endpoints::RepoRemoveNote { owner, repo, sha }
            .make_request()
            .wrap_sync::<endpoints::RepoRemoveNote>(self)
    }

    /// Get specified ref or filtered repository's refs
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_all_git_refs(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListAllGitRefs> {
        endpoints::RepoListAllGitRefs { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListAllGitRefs>(self)
    }

    /// Get specified ref or filtered repository's refs
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `ref`: part or full name of the ref
    pub fn repo_list_git_refs(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListGitRefs> {
        endpoints::RepoListGitRefs { owner, repo, r#ref }
            .make_request()
            .wrap_sync::<endpoints::RepoListGitRefs>(self)
    }

    /// Gets the tag object of an annotated tag (not lightweight tags)
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the tag. The Git tags API only supports annotated tag objects, not lightweight tags.
    pub fn get_annotated_tag(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetAnnotatedTag> {
        endpoints::GetAnnotatedTag { owner, repo, sha }
            .make_request()
            .wrap_sync::<endpoints::GetAnnotatedTag>(self)
    }

    /// Gets the tree of a repository.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    pub fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: GetTreeQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetTree> {
        endpoints::GetTree {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::GetTree>(self)
    }

    /// List the hooks in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_hooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListHooks> {
        endpoints::RepoListHooks { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListHooks>(self)
    }

    /// Create a hook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateHookOption`]
    pub fn repo_create_hook(
        &self,
        owner: &str,
        repo: &str,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateHook> {
        endpoints::RepoCreateHook {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateHook>(self)
    }

    /// List the Git hooks in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_git_hooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListGitHooks> {
        endpoints::RepoListGitHooks { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListGitHooks>(self)
    }

    /// Get a Git hook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    pub fn repo_get_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetGitHook> {
        endpoints::RepoGetGitHook { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoGetGitHook>(self)
    }

    /// Delete a Git hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    pub fn repo_delete_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteGitHook> {
        endpoints::RepoDeleteGitHook { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteGitHook>(self)
    }

    /// Edit a Git hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    /// - `body`: See [`EditGitHookOption`]
    pub fn repo_edit_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        body: EditGitHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditGitHook> {
        endpoints::RepoEditGitHook {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditGitHook>(self)
    }

    /// Get a hook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to get
    pub fn repo_get_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetHook> {
        endpoints::RepoGetHook { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoGetHook>(self)
    }

    /// Delete a hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to delete
    pub fn repo_delete_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteHook> {
        endpoints::RepoDeleteHook { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteHook>(self)
    }

    /// Edit a hook in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: index of the hook
    /// - `body`: See [`EditHookOption`]
    pub fn repo_edit_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditHook> {
        endpoints::RepoEditHook {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditHook>(self)
    }

    /// Test a push webhook
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the hook to test
    pub fn repo_test_hook(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        query: RepoTestHookQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoTestHook> {
        endpoints::RepoTestHook {
            owner,
            repo,
            id,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoTestHook>(self)
    }

    /// Returns the issue config for a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_issue_config(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetIssueConfig> {
        endpoints::RepoGetIssueConfig { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetIssueConfig>(self)
    }

    /// Returns the validation information for a issue config
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_validate_issue_config(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoValidateIssueConfig> {
        endpoints::RepoValidateIssueConfig { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoValidateIssueConfig>(self)
    }

    /// Get available issue templates for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_issue_templates(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetIssueTemplates> {
        endpoints::RepoGetIssueTemplates { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetIssueTemplates>(self)
    }

    /// List a repository's issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn issue_list_issues(
        &self,
        owner: &str,
        repo: &str,
        query: IssueListIssuesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueListIssues> {
        endpoints::IssueListIssues { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::IssueListIssues>(self)
    }

    /// Create an issue. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateIssueOption`]
    pub fn issue_create_issue(
        &self,
        owner: &str,
        repo: &str,
        body: CreateIssueOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateIssue> {
        endpoints::IssueCreateIssue {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateIssue>(self)
    }

    /// List all comments in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn issue_get_repo_comments(
        &self,
        owner: &str,
        repo: &str,
        query: IssueGetRepoCommentsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetRepoComments> {
        endpoints::IssueGetRepoComments { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::IssueGetRepoComments>(self)
    }

    /// Get a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    pub fn issue_get_comment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetComment> {
        endpoints::IssueGetComment { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueGetComment>(self)
    }

    /// Delete a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of comment to delete
    pub fn issue_delete_comment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteComment> {
        endpoints::IssueDeleteComment { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueDeleteComment>(self)
    }

    /// Edit a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    /// - `body`: See [`EditIssueCommentOption`]
    pub fn issue_edit_comment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditIssueCommentOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditComment> {
        endpoints::IssueEditComment {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditComment>(self)
    }

    /// List comment's attachments
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    pub fn issue_list_issue_comment_attachments(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueListIssueCommentAttachments> {
        endpoints::IssueListIssueCommentAttachments { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueListIssueCommentAttachments>(self)
    }

    /// Create a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment`: attachment to upload
    pub fn issue_create_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment: &[u8],
        query: IssueCreateIssueCommentAttachmentQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateIssueCommentAttachment> {
        endpoints::IssueCreateIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment: &attachment,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateIssueCommentAttachment>(self)
    }

    /// Get a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment_id`: id of the attachment to get
    pub fn issue_get_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetIssueCommentAttachment> {
        endpoints::IssueGetIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueGetIssueCommentAttachment>(self)
    }

    /// Delete a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment_id`: id of the attachment to delete
    pub fn issue_delete_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteIssueCommentAttachment> {
        endpoints::IssueDeleteIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteIssueCommentAttachment>(self)
    }

    /// Edit a comment attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment
    /// - `attachment_id`: id of the attachment to edit
    /// - `body`: See [`EditAttachmentOptions`]
    pub fn issue_edit_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
        body: EditAttachmentOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditIssueCommentAttachment> {
        endpoints::IssueEditIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditIssueCommentAttachment>(self)
    }

    /// Get a list of reactions from a comment of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    pub fn issue_get_comment_reactions(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetCommentReactions> {
        endpoints::IssueGetCommentReactions { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueGetCommentReactions>(self)
    }

    /// Add a reaction to a comment of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    /// - `content`: See [`EditReactionOption`]
    pub fn issue_post_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssuePostCommentReaction> {
        endpoints::IssuePostCommentReaction {
            owner,
            repo,
            id,
            body: content,
        }
        .make_request()
        .wrap_sync::<endpoints::IssuePostCommentReaction>(self)
    }

    /// Remove a reaction from a comment of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the comment to edit
    /// - `content`: See [`EditReactionOption`]
    pub fn issue_delete_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteCommentReaction> {
        endpoints::IssueDeleteCommentReaction {
            owner,
            repo,
            id,
            body: content,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteCommentReaction>(self)
    }

    /// List a repo's pinned issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_pinned_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListPinnedIssues> {
        endpoints::RepoListPinnedIssues { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListPinnedIssues>(self)
    }

    /// Get an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to get
    pub fn issue_get_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetIssue> {
        endpoints::IssueGetIssue { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueGetIssue>(self)
    }

    /// Delete an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue to delete
    pub fn issue_delete(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDelete> {
        endpoints::IssueDelete { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueDelete>(self)
    }

    /// Edit an issue. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to edit
    /// - `body`: See [`EditIssueOption`]
    pub fn issue_edit_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: EditIssueOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditIssue> {
        endpoints::IssueEditIssue {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditIssue>(self)
    }

    /// List issue's attachments
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_list_issue_attachments(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueListIssueAttachments> {
        endpoints::IssueListIssueAttachments { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueListIssueAttachments>(self)
    }

    /// Create an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment`: attachment to upload
    pub fn issue_create_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment: &[u8],
        query: IssueCreateIssueAttachmentQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateIssueAttachment> {
        endpoints::IssueCreateIssueAttachment {
            owner,
            repo,
            index,
            attachment: &attachment,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateIssueAttachment>(self)
    }

    /// Get an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment_id`: id of the attachment to get
    pub fn issue_get_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment_id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetIssueAttachment> {
        endpoints::IssueGetIssueAttachment {
            owner,
            repo,
            index,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueGetIssueAttachment>(self)
    }

    /// Delete an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment_id`: id of the attachment to delete
    pub fn issue_delete_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment_id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteIssueAttachment> {
        endpoints::IssueDeleteIssueAttachment {
            owner,
            repo,
            index,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteIssueAttachment>(self)
    }

    /// Edit an issue attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `attachment_id`: id of the attachment to edit
    /// - `body`: See [`EditAttachmentOptions`]
    pub fn issue_edit_issue_attachment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        attachment_id: u64,
        body: EditAttachmentOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditIssueAttachment> {
        endpoints::IssueEditIssueAttachment {
            owner,
            repo,
            index,
            attachment_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditIssueAttachment>(self)
    }

    /// List issues that are blocked by this issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_list_blocks(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueListBlocks> {
        endpoints::IssueListBlocks { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueListBlocks>(self)
    }

    /// Block the issue given in the body by the issue in path
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub fn issue_create_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateIssueBlocking> {
        endpoints::IssueCreateIssueBlocking {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateIssueBlocking>(self)
    }

    /// Unblock the issue given in the body by the issue in path
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub fn issue_remove_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueRemoveIssueBlocking> {
        endpoints::IssueRemoveIssueBlocking {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueRemoveIssueBlocking>(self)
    }

    /// List all comments on an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_get_comments(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueGetCommentsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetComments> {
        endpoints::IssueGetComments {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueGetComments>(self)
    }

    /// Add a comment to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`CreateIssueCommentOption`]
    pub fn issue_create_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: CreateIssueCommentOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateComment> {
        endpoints::IssueCreateComment {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateComment>(self)
    }

    /// Delete a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: this parameter is ignored
    /// - `id`: id of comment to delete
    pub fn issue_delete_comment_deprecated(
        &self,
        owner: &str,
        repo: &str,
        index: u32,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteCommentDeprecated> {
        endpoints::IssueDeleteCommentDeprecated {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteCommentDeprecated>(self)
    }

    /// Edit a comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: this parameter is ignored
    /// - `id`: id of the comment to edit
    /// - `body`: See [`EditIssueCommentOption`]
    pub fn issue_edit_comment_deprecated(
        &self,
        owner: &str,
        repo: &str,
        index: u32,
        id: u64,
        body: EditIssueCommentOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditCommentDeprecated> {
        endpoints::IssueEditCommentDeprecated {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditCommentDeprecated>(self)
    }

    /// Set an issue deadline. If set to null, the deadline is deleted. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to create or update a deadline on
    /// - `body`: See [`EditDeadlineOption`]
    pub fn issue_edit_issue_deadline(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: EditDeadlineOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditIssueDeadline> {
        endpoints::IssueEditIssueDeadline {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditIssueDeadline>(self)
    }

    /// List an issue's dependencies, i.e all issues that block this issue.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_list_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueListIssueDependencies> {
        endpoints::IssueListIssueDependencies { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueListIssueDependencies>(self)
    }

    /// Make the issue in the url depend on the issue in the form.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub fn issue_create_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateIssueDependencies> {
        endpoints::IssueCreateIssueDependencies {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateIssueDependencies>(self)
    }

    /// Remove an issue dependency
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueMeta`]
    pub fn issue_remove_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: &str,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueRemoveIssueDependencies> {
        endpoints::IssueRemoveIssueDependencies {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueRemoveIssueDependencies>(self)
    }

    /// Get an issue's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_get_labels(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetLabels> {
        endpoints::IssueGetLabels { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueGetLabels>(self)
    }

    /// Replace an issue's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueLabelsOption`]
    pub fn issue_replace_labels(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: IssueLabelsOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueReplaceLabels> {
        endpoints::IssueReplaceLabels {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueReplaceLabels>(self)
    }

    /// Add a label to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`IssueLabelsOption`]
    pub fn issue_add_label(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: IssueLabelsOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueAddLabel> {
        endpoints::IssueAddLabel {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueAddLabel>(self)
    }

    /// Remove all labels from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`DeleteLabelsOption`]
    pub fn issue_clear_labels(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: DeleteLabelsOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueClearLabels> {
        endpoints::IssueClearLabels {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueClearLabels>(self)
    }

    /// Remove a label from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `id`: id of the label to remove
    /// - `body`: See [`DeleteLabelsOption`]
    pub fn issue_remove_label(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: DeleteLabelsOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueRemoveLabel> {
        endpoints::IssueRemoveLabel {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueRemoveLabel>(self)
    }

    /// Pin an Issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue to pin
    pub fn pin_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::PinIssue> {
        endpoints::PinIssue { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::PinIssue>(self)
    }

    /// Unpin an Issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue to unpin
    pub fn unpin_issue(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UnpinIssue> {
        endpoints::UnpinIssue { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::UnpinIssue>(self)
    }

    /// Moves the Pin to the given Position
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of issue
    /// - `position`: the new position
    pub fn move_issue_pin(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        position: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::MoveIssuePin> {
        endpoints::MoveIssuePin {
            owner,
            repo,
            index,
            position,
        }
        .make_request()
        .wrap_sync::<endpoints::MoveIssuePin>(self)
    }

    /// Get a list reactions of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_get_issue_reactions(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetIssueReactions> {
        endpoints::IssueGetIssueReactions { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueGetIssueReactions>(self)
    }

    /// Add a reaction to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `content`: See [`EditReactionOption`]
    pub fn issue_post_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssuePostIssueReaction> {
        endpoints::IssuePostIssueReaction {
            owner,
            repo,
            index,
            body: content,
        }
        .make_request()
        .wrap_sync::<endpoints::IssuePostIssueReaction>(self)
    }

    /// Remove a reaction from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `content`: See [`EditReactionOption`]
    pub fn issue_delete_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteIssueReaction> {
        endpoints::IssueDeleteIssueReaction {
            owner,
            repo,
            index,
            body: content,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteIssueReaction>(self)
    }

    /// Delete an issue's existing stopwatch.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to stop the stopwatch on
    pub fn issue_delete_stop_watch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteStopWatch> {
        endpoints::IssueDeleteStopWatch { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueDeleteStopWatch>(self)
    }

    /// Start stopwatch on an issue.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to create the stopwatch on
    pub fn issue_start_stop_watch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueStartStopWatch> {
        endpoints::IssueStartStopWatch { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueStartStopWatch>(self)
    }

    /// Stop an issue's existing stopwatch.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to stop the stopwatch on
    pub fn issue_stop_stop_watch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueStopStopWatch> {
        endpoints::IssueStopStopWatch { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueStopStopWatch>(self)
    }

    /// Get users who subscribed on an issue.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_subscriptions(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueSubscriptions> {
        endpoints::IssueSubscriptions { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueSubscriptions>(self)
    }

    /// Check if user is subscribed to an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_check_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCheckSubscription> {
        endpoints::IssueCheckSubscription { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueCheckSubscription>(self)
    }

    /// Subscribe user to issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `user`: user to subscribe
    pub fn issue_add_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        user: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueAddSubscription> {
        endpoints::IssueAddSubscription {
            owner,
            repo,
            index,
            user,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueAddSubscription>(self)
    }

    /// Unsubscribe user from issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `user`: user witch unsubscribe
    pub fn issue_delete_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        user: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteSubscription> {
        endpoints::IssueDeleteSubscription {
            owner,
            repo,
            index,
            user,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteSubscription>(self)
    }

    /// List all comments and events on an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_get_comments_and_timeline(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueGetCommentsAndTimelineQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetCommentsAndTimeline> {
        endpoints::IssueGetCommentsAndTimeline {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueGetCommentsAndTimeline>(self)
    }

    /// List an issue's tracked times
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    pub fn issue_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: IssueTrackedTimesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueTrackedTimes> {
        endpoints::IssueTrackedTimes {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueTrackedTimes>(self)
    }

    /// Add tracked time to a issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `body`: See [`AddTimeOption`]
    pub fn issue_add_time(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: AddTimeOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueAddTime> {
        endpoints::IssueAddTime {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueAddTime>(self)
    }

    /// Reset a tracked time of an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue to add tracked time to
    pub fn issue_reset_time(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueResetTime> {
        endpoints::IssueResetTime { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::IssueResetTime>(self)
    }

    /// Delete specific tracked time
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `id`: id of time to delete
    pub fn issue_delete_time(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteTime> {
        endpoints::IssueDeleteTime {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueDeleteTime>(self)
    }

    /// List a repository's keys
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_keys(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListKeysQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListKeys> {
        endpoints::RepoListKeys { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoListKeys>(self)
    }

    /// Add a key to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateKeyOption`]
    pub fn repo_create_key(
        &self,
        owner: &str,
        repo: &str,
        body: CreateKeyOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateKey> {
        endpoints::RepoCreateKey {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateKey>(self)
    }

    /// Get a repository's key by id
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the key to get
    pub fn repo_get_key(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetKey> {
        endpoints::RepoGetKey { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoGetKey>(self)
    }

    /// Delete a key from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the key to delete
    pub fn repo_delete_key(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteKey> {
        endpoints::RepoDeleteKey { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteKey>(self)
    }

    /// Get all of a repository's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn issue_list_labels(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueListLabels> {
        endpoints::IssueListLabels { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::IssueListLabels>(self)
    }

    /// Create a label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateLabelOption`]
    pub fn issue_create_label(
        &self,
        owner: &str,
        repo: &str,
        body: CreateLabelOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateLabel> {
        endpoints::IssueCreateLabel {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateLabel>(self)
    }

    /// Get a single label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the label to get
    pub fn issue_get_label(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetLabel> {
        endpoints::IssueGetLabel { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueGetLabel>(self)
    }

    /// Delete a label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the label to delete
    pub fn issue_delete_label(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteLabel> {
        endpoints::IssueDeleteLabel { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueDeleteLabel>(self)
    }

    /// Update a label
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the label to edit
    /// - `body`: See [`EditLabelOption`]
    pub fn issue_edit_label(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditLabelOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditLabel> {
        endpoints::IssueEditLabel {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditLabel>(self)
    }

    /// Get languages and number of bytes of code written
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetLanguages> {
        endpoints::RepoGetLanguages { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetLanguages>(self)
    }

    /// Get a file or it's LFS object from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: filepath of the file to get
    pub fn repo_get_raw_file_or_lfs(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetRawFileOrLfsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetRawFileOrLfs> {
        endpoints::RepoGetRawFileOrLfs {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetRawFileOrLfs>(self)
    }

    /// Get all of a repository's opened milestones
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn issue_get_milestones_list(
        &self,
        owner: &str,
        repo: &str,
        query: IssueGetMilestonesListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetMilestonesList> {
        endpoints::IssueGetMilestonesList { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::IssueGetMilestonesList>(self)
    }

    /// Create a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateMilestoneOption`]
    pub fn issue_create_milestone(
        &self,
        owner: &str,
        repo: &str,
        body: CreateMilestoneOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueCreateMilestone> {
        endpoints::IssueCreateMilestone {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueCreateMilestone>(self)
    }

    /// Get a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: the milestone to get, identified by ID and if not available by name
    pub fn issue_get_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueGetMilestone> {
        endpoints::IssueGetMilestone { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueGetMilestone>(self)
    }

    /// Delete a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: the milestone to delete, identified by ID and if not available by name
    pub fn issue_delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueDeleteMilestone> {
        endpoints::IssueDeleteMilestone { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::IssueDeleteMilestone>(self)
    }

    /// Update a milestone
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: the milestone to edit, identified by ID and if not available by name
    /// - `body`: See [`EditMilestoneOption`]
    pub fn issue_edit_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        body: EditMilestoneOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::IssueEditMilestone> {
        endpoints::IssueEditMilestone {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::IssueEditMilestone>(self)
    }

    /// Sync a mirrored repository
    ///
    /// - `owner`: owner of the repo to sync
    /// - `repo`: name of the repo to sync
    pub fn repo_mirror_sync(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoMirrorSync> {
        endpoints::RepoMirrorSync { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoMirrorSync>(self)
    }

    /// Returns if new Issue Pins are allowed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_new_pin_allowed(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoNewPinAllowed> {
        endpoints::RepoNewPinAllowed { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoNewPinAllowed>(self)
    }

    /// List users's notification threads on a specific repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn notify_get_repo_list(
        &self,
        owner: &str,
        repo: &str,
        query: NotifyGetRepoListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyGetRepoList> {
        endpoints::NotifyGetRepoList { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::NotifyGetRepoList>(self)
    }

    /// Mark notification threads as read, pinned or unread on a specific repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn notify_read_repo_list(
        &self,
        owner: &str,
        repo: &str,
        query: NotifyReadRepoListQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::NotifyReadRepoList> {
        endpoints::NotifyReadRepoList { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::NotifyReadRepoList>(self)
    }

    /// List a repo's pull requests
    ///
    /// - `owner`: Owner of the repo
    /// - `repo`: Name of the repo
    pub fn repo_list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListPullRequestsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListPullRequests> {
        endpoints::RepoListPullRequests { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoListPullRequests>(self)
    }

    /// Create a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreatePullRequestOption`]
    pub fn repo_create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        body: CreatePullRequestOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreatePullRequest> {
        endpoints::RepoCreatePullRequest {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreatePullRequest>(self)
    }

    /// List a repo's pinned pull requests
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_pinned_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListPinnedPullRequests> {
        endpoints::RepoListPinnedPullRequests { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListPinnedPullRequests>(self)
    }

    /// Get a pull request by base and head
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `base`: base of the pull request to get
    /// - `head`: head of the pull request to get
    pub fn repo_get_pull_request_by_base_head(
        &self,
        owner: &str,
        repo: &str,
        base: &str,
        head: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullRequestByBaseHead> {
        endpoints::RepoGetPullRequestByBaseHead {
            owner,
            repo,
            base,
            head,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetPullRequestByBaseHead>(self)
    }

    /// Get a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub fn repo_get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullRequest> {
        endpoints::RepoGetPullRequest { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::RepoGetPullRequest>(self)
    }

    /// Update a pull request. If using deadline only the date will be taken into account, and time of day ignored.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to edit
    /// - `body`: See [`EditPullRequestOption`]
    pub fn repo_edit_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: EditPullRequestOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditPullRequest> {
        endpoints::RepoEditPullRequest {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditPullRequest>(self)
    }

    /// Get a pull request diff or patch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    /// - `diffType`: whether the output is diff or patch
    pub fn repo_download_pull_diff_or_patch(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        diff_type: &str,
        query: RepoDownloadPullDiffOrPatchQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDownloadPullDiffOrPatch> {
        endpoints::RepoDownloadPullDiffOrPatch {
            owner,
            repo,
            index,
            diff_type,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDownloadPullDiffOrPatch>(self)
    }

    /// Get commits for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub fn repo_get_pull_request_commits(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoGetPullRequestCommitsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullRequestCommits> {
        endpoints::RepoGetPullRequestCommits {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetPullRequestCommits>(self)
    }

    /// Get changed files for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub fn repo_get_pull_request_files(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoGetPullRequestFilesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullRequestFiles> {
        endpoints::RepoGetPullRequestFiles {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetPullRequestFiles>(self)
    }

    /// Check if a pull request has been merged
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    pub fn repo_pull_request_is_merged(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoPullRequestIsMerged> {
        endpoints::RepoPullRequestIsMerged { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::RepoPullRequestIsMerged>(self)
    }

    /// Merge a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to merge
    /// - `body`: See [`MergePullRequestOption`]
    pub fn repo_merge_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: MergePullRequestOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoMergePullRequest> {
        endpoints::RepoMergePullRequest {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoMergePullRequest>(self)
    }

    /// Cancel the scheduled auto merge for the given pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to merge
    pub fn repo_cancel_scheduled_auto_merge(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCancelScheduledAutoMerge> {
        endpoints::RepoCancelScheduledAutoMerge { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::RepoCancelScheduledAutoMerge>(self)
    }

    /// create review requests for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`PullReviewRequestOptions`]
    pub fn repo_create_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: PullReviewRequestOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreatePullReviewRequests> {
        endpoints::RepoCreatePullReviewRequests {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreatePullReviewRequests>(self)
    }

    /// cancel review requests for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`PullReviewRequestOptions`]
    pub fn repo_delete_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: PullReviewRequestOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeletePullReviewRequests> {
        endpoints::RepoDeletePullReviewRequests {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeletePullReviewRequests>(self)
    }

    /// List all reviews for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    pub fn repo_list_pull_reviews(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListPullReviews> {
        endpoints::RepoListPullReviews { owner, repo, index }
            .make_request()
            .wrap_sync::<endpoints::RepoListPullReviews>(self)
    }

    /// Create a review to an pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`CreatePullReviewOptions`]
    pub fn repo_create_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        body: CreatePullReviewOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreatePullReview> {
        endpoints::RepoCreatePullReview {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreatePullReview>(self)
    }

    /// Get a specific review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub fn repo_get_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullReview> {
        endpoints::RepoGetPullReview {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetPullReview>(self)
    }

    /// Submit a pending review to an pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `body`: See [`SubmitPullReviewOptions`]
    pub fn repo_submit_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: SubmitPullReviewOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoSubmitPullReview> {
        endpoints::RepoSubmitPullReview {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoSubmitPullReview>(self)
    }

    /// Delete a specific review from a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub fn repo_delete_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeletePullReview> {
        endpoints::RepoDeletePullReview {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeletePullReview>(self)
    }

    /// Get a specific review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub fn repo_get_pull_review_comments(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullReviewComments> {
        endpoints::RepoGetPullReviewComments {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetPullReviewComments>(self)
    }

    /// Add a new comment to a pull request review
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `body`: See [`serde_json::Value`]
    pub fn repo_create_pull_review_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: serde_json::Value,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreatePullReviewComment> {
        endpoints::RepoCreatePullReviewComment {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreatePullReviewComment>(self)
    }

    /// Get a pull review comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `comment`: id of the comment
    pub fn repo_get_pull_review_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        comment: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPullReviewComment> {
        endpoints::RepoGetPullReviewComment {
            owner,
            repo,
            index,
            id,
            comment,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetPullReviewComment>(self)
    }

    /// Delete a pull review comment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `comment`: id of the comment
    pub fn repo_delete_pull_review_comment(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        comment: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeletePullReviewComment> {
        endpoints::RepoDeletePullReviewComment {
            owner,
            repo,
            index,
            id,
            comment,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeletePullReviewComment>(self)
    }

    /// Dismiss a review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    /// - `body`: See [`DismissPullReviewOptions`]
    pub fn repo_dismiss_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
        body: DismissPullReviewOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDismissPullReview> {
        endpoints::RepoDismissPullReview {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDismissPullReview>(self)
    }

    /// Cancel to dismiss a review for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `id`: id of the review
    pub fn repo_un_dismiss_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoUnDismissPullReview> {
        endpoints::RepoUnDismissPullReview {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoUnDismissPullReview>(self)
    }

    /// Merge PR's baseBranch into headBranch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request to get
    pub fn repo_update_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: u64,
        query: RepoUpdatePullRequestQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoUpdatePullRequest> {
        endpoints::RepoUpdatePullRequest {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoUpdatePullRequest>(self)
    }

    /// Get all push mirrors of the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_push_mirrors(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListPushMirrors> {
        endpoints::RepoListPushMirrors { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListPushMirrors>(self)
    }

    /// add a push mirror to the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreatePushMirrorOption`]
    pub fn repo_add_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        body: CreatePushMirrorOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoAddPushMirror> {
        endpoints::RepoAddPushMirror {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoAddPushMirror>(self)
    }

    /// Sync all push mirrored repository
    ///
    /// - `owner`: owner of the repo to sync
    /// - `repo`: name of the repo to sync
    pub fn repo_push_mirror_sync(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoPushMirrorSync> {
        endpoints::RepoPushMirrorSync { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoPushMirrorSync>(self)
    }

    /// Get push mirror of the repository by remoteName
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: remote name of push mirror
    pub fn repo_get_push_mirror_by_remote_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetPushMirrorByRemoteName> {
        endpoints::RepoGetPushMirrorByRemoteName { owner, repo, name }
            .make_request()
            .wrap_sync::<endpoints::RepoGetPushMirrorByRemoteName>(self)
    }

    /// deletes a push mirror from a repository by remoteName
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: remote name of the pushMirror
    pub fn repo_delete_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeletePushMirror> {
        endpoints::RepoDeletePushMirror { owner, repo, name }
            .make_request()
            .wrap_sync::<endpoints::RepoDeletePushMirror>(self)
    }

    /// Get a file from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `filepath`: filepath of the file to get
    pub fn repo_get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        query: RepoGetRawFileQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetRawFile> {
        endpoints::RepoGetRawFile {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetRawFile>(self)
    }

    /// List a repo's releases
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_releases(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListReleasesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListReleases> {
        endpoints::RepoListReleases { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoListReleases>(self)
    }

    /// Create a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateReleaseOption`]
    pub fn repo_create_release(
        &self,
        owner: &str,
        repo: &str,
        body: CreateReleaseOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateRelease> {
        endpoints::RepoCreateRelease {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateRelease>(self)
    }

    /// Gets the most recent non-prerelease, non-draft release of a repository, sorted by created_at
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetLatestRelease> {
        endpoints::RepoGetLatestRelease { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetLatestRelease>(self)
    }

    /// Get a release by tag name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: tag name of the release to get
    pub fn repo_get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetReleaseByTag> {
        endpoints::RepoGetReleaseByTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<endpoints::RepoGetReleaseByTag>(self)
    }

    /// Delete a release by tag name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: tag name of the release to delete
    pub fn repo_delete_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteReleaseByTag> {
        endpoints::RepoDeleteReleaseByTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteReleaseByTag>(self)
    }

    /// Get a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release to get
    pub fn repo_get_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetRelease> {
        endpoints::RepoGetRelease { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoGetRelease>(self)
    }

    /// Delete a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release to delete
    pub fn repo_delete_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteRelease> {
        endpoints::RepoDeleteRelease { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteRelease>(self)
    }

    /// Update a release
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release to edit
    /// - `body`: See [`EditReleaseOption`]
    pub fn repo_edit_release(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        body: EditReleaseOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditRelease> {
        endpoints::RepoEditRelease {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditRelease>(self)
    }

    /// List release's attachments
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    pub fn repo_list_release_attachments(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListReleaseAttachments> {
        endpoints::RepoListReleaseAttachments { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoListReleaseAttachments>(self)
    }

    /// Create a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment`: attachment to upload (this parameter is incompatible with `external_url`)
    /// - `external_url`: url to external asset (this parameter is incompatible with `attachment`)
    pub fn repo_create_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment: Option<&[u8]>,
        external_url: Option<&str>,
        query: RepoCreateReleaseAttachmentQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateReleaseAttachment> {
        endpoints::RepoCreateReleaseAttachment {
            owner,
            repo,
            id,
            attachment: attachment.as_deref(),
            external_url: external_url.as_deref(),
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateReleaseAttachment>(self)
    }

    /// Get a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment_id`: id of the attachment to get
    pub fn repo_get_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetReleaseAttachment> {
        endpoints::RepoGetReleaseAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetReleaseAttachment>(self)
    }

    /// Delete a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment_id`: id of the attachment to delete
    pub fn repo_delete_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteReleaseAttachment> {
        endpoints::RepoDeleteReleaseAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeleteReleaseAttachment>(self)
    }

    /// Edit a release attachment
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the release
    /// - `attachment_id`: id of the attachment to edit
    /// - `body`: See [`EditAttachmentOptions`]
    pub fn repo_edit_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: u64,
        attachment_id: u64,
        body: EditAttachmentOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditReleaseAttachment> {
        endpoints::RepoEditReleaseAttachment {
            owner,
            repo,
            id,
            attachment_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditReleaseAttachment>(self)
    }

    /// Return all users that can be requested to review in this repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_reviewers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetReviewers> {
        endpoints::RepoGetReviewers { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetReviewers>(self)
    }

    /// Get signing-key.gpg for given repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_signing_key(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoSigningKey> {
        endpoints::RepoSigningKey { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoSigningKey>(self)
    }

    /// List a repo's stargazers
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_stargazers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListStargazers> {
        endpoints::RepoListStargazers { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListStargazers>(self)
    }

    /// Get a commit's statuses
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    pub fn repo_list_statuses(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        query: RepoListStatusesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListStatuses> {
        endpoints::RepoListStatuses {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoListStatuses>(self)
    }

    /// Create a commit status
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the commit
    /// - `body`: See [`CreateStatusOption`]
    pub fn repo_create_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: CreateStatusOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateStatus> {
        endpoints::RepoCreateStatus {
            owner,
            repo,
            sha,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateStatus>(self)
    }

    /// List a repo's watchers
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_subscribers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListSubscribers> {
        endpoints::RepoListSubscribers { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListSubscribers>(self)
    }

    /// Check if the current user is watching a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_check_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentCheckSubscription> {
        endpoints::UserCurrentCheckSubscription { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentCheckSubscription>(self)
    }

    /// Watch a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_put_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentPutSubscription> {
        endpoints::UserCurrentPutSubscription { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentPutSubscription>(self)
    }

    /// Unwatch a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_delete_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentDeleteSubscription> {
        endpoints::UserCurrentDeleteSubscription { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentDeleteSubscription>(self)
    }

    /// List tag protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_tag_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListTagProtection> {
        endpoints::RepoListTagProtection { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListTagProtection>(self)
    }

    /// Create a tag protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateTagProtectionOption`]
    pub fn repo_create_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        body: CreateTagProtectionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateTagProtection> {
        endpoints::RepoCreateTagProtection {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateTagProtection>(self)
    }

    /// Get a specific tag protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of the tag protect to get
    pub fn repo_get_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: u32,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetTagProtection> {
        endpoints::RepoGetTagProtection { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoGetTagProtection>(self)
    }

    /// Delete a specific tag protection for the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of protected tag
    pub fn repo_delete_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: u32,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteTagProtection> {
        endpoints::RepoDeleteTagProtection { owner, repo, id }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteTagProtection>(self)
    }

    /// Edit a tag protections for a repository. Only fields that are set will be changed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `id`: id of protected tag
    /// - `body`: See [`EditTagProtectionOption`]
    pub fn repo_edit_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: u32,
        body: EditTagProtectionOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditTagProtection> {
        endpoints::RepoEditTagProtection {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditTagProtection>(self)
    }

    /// List a repository's tags
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_tags(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListTags> {
        endpoints::RepoListTags { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListTags>(self)
    }

    /// Create a new git tag in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateTagOption`]
    pub fn repo_create_tag(
        &self,
        owner: &str,
        repo: &str,
        body: CreateTagOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateTag> {
        endpoints::RepoCreateTag {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateTag>(self)
    }

    /// Get the tag of a repository by tag name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: name of tag
    pub fn repo_get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetTag> {
        endpoints::RepoGetTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<endpoints::RepoGetTag>(self)
    }

    /// Delete a repository's tag by name
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `tag`: name of tag to delete
    pub fn repo_delete_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteTag> {
        endpoints::RepoDeleteTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteTag>(self)
    }

    /// List a repository's teams
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListTeams> {
        endpoints::RepoListTeams { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListTeams>(self)
    }

    /// Check if a team is assigned to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `team`: team name
    pub fn repo_check_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCheckTeam> {
        endpoints::RepoCheckTeam { owner, repo, team }
            .make_request()
            .wrap_sync::<endpoints::RepoCheckTeam>(self)
    }

    /// Add a team to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `team`: team name
    pub fn repo_add_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoAddTeam> {
        endpoints::RepoAddTeam { owner, repo, team }
            .make_request()
            .wrap_sync::<endpoints::RepoAddTeam>(self)
    }

    /// Delete a team from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `team`: team name
    pub fn repo_delete_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteTeam> {
        endpoints::RepoDeleteTeam { owner, repo, team }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteTeam>(self)
    }

    /// List a repo's tracked times
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        query: RepoTrackedTimesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoTrackedTimes> {
        endpoints::RepoTrackedTimes { owner, repo, query }
            .make_request()
            .wrap_sync::<endpoints::RepoTrackedTimes>(self)
    }

    /// List a user's tracked times in a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `user`: username of user
    pub fn user_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        user: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserTrackedTimes> {
        endpoints::UserTrackedTimes { owner, repo, user }
            .make_request()
            .wrap_sync::<endpoints::UserTrackedTimes>(self)
    }

    /// Get list of topics that a repository has
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_topics(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoListTopics> {
        endpoints::RepoListTopics { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoListTopics>(self)
    }

    /// Replace list of topics for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`RepoTopicOptions`]
    pub fn repo_update_topics(
        &self,
        owner: &str,
        repo: &str,
        body: RepoTopicOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoUpdateTopics> {
        endpoints::RepoUpdateTopics {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoUpdateTopics>(self)
    }

    /// Add a topic to a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `topic`: name of the topic to add
    pub fn repo_add_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoAddTopic> {
        endpoints::RepoAddTopic { owner, repo, topic }
            .make_request()
            .wrap_sync::<endpoints::RepoAddTopic>(self)
    }

    /// Delete a topic from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `topic`: name of the topic to delete
    pub fn repo_delete_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteTopic> {
        endpoints::RepoDeleteTopic { owner, repo, topic }
            .make_request()
            .wrap_sync::<endpoints::RepoDeleteTopic>(self)
    }

    /// Transfer a repo ownership
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    /// - `body`: Transfer Options

    ///   See [`TransferRepoOption`]
    pub fn repo_transfer(
        &self,
        owner: &str,
        repo: &str,
        body: TransferRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoTransfer> {
        endpoints::RepoTransfer {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoTransfer>(self)
    }

    /// Accept a repo transfer
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    pub fn accept_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::AcceptRepoTransfer> {
        endpoints::AcceptRepoTransfer { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::AcceptRepoTransfer>(self)
    }

    /// Reject a repo transfer
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    pub fn reject_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RejectRepoTransfer> {
        endpoints::RejectRepoTransfer { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RejectRepoTransfer>(self)
    }

    /// Create a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreateWikiPageOptions`]
    pub fn repo_create_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        body: CreateWikiPageOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoCreateWikiPage> {
        endpoints::RepoCreateWikiPage {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoCreateWikiPage>(self)
    }

    /// Get a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    pub fn repo_get_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetWikiPage> {
        endpoints::RepoGetWikiPage {
            owner,
            repo,
            page_name,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetWikiPage>(self)
    }

    /// Delete a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    pub fn repo_delete_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoDeleteWikiPage> {
        endpoints::RepoDeleteWikiPage {
            owner,
            repo,
            page_name,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoDeleteWikiPage>(self)
    }

    /// Edit a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    /// - `body`: See [`CreateWikiPageOptions`]
    pub fn repo_edit_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
        body: CreateWikiPageOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoEditWikiPage> {
        endpoints::RepoEditWikiPage {
            owner,
            repo,
            page_name,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoEditWikiPage>(self)
    }

    /// Get all wiki pages
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_wiki_pages(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetWikiPages> {
        endpoints::RepoGetWikiPages { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::RepoGetWikiPages>(self)
    }

    /// Get revisions of a wiki page
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `pageName`: name of the page
    pub fn repo_get_wiki_page_revisions(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetWikiPageRevisions> {
        endpoints::RepoGetWikiPageRevisions {
            owner,
            repo,
            page_name,
        }
        .make_request()
        .wrap_sync::<endpoints::RepoGetWikiPageRevisions>(self)
    }

    /// Create a repository using a template
    ///
    /// - `template_owner`: name of the template repository owner
    /// - `template_repo`: name of the template repository
    /// - `body`: See [`GenerateRepoOption`]
    pub fn generate_repo(
        &self,
        template_owner: &str,
        template_repo: &str,
        body: GenerateRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GenerateRepo> {
        endpoints::GenerateRepo {
            template_owner,
            template_repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::GenerateRepo>(self)
    }

    /// Get a repository by id
    ///
    /// - `id`: id of the repo to get
    pub fn repo_get_by_id(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::RepoGetById> {
        endpoints::RepoGetById { id }
            .make_request()
            .wrap_sync::<endpoints::RepoGetById>(self)
    }

    /// Get instance's global settings for api
    pub fn get_general_api_settings(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetGeneralApiSettings> {
        endpoints::GetGeneralApiSettings {}
            .make_request()
            .wrap_sync::<endpoints::GetGeneralApiSettings>(self)
    }

    /// Get instance's global settings for Attachment
    pub fn get_general_attachment_settings(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetGeneralAttachmentSettings> {
        endpoints::GetGeneralAttachmentSettings {}
            .make_request()
            .wrap_sync::<endpoints::GetGeneralAttachmentSettings>(self)
    }

    /// Get instance's global settings for repositories
    pub fn get_general_repository_settings(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetGeneralRepositorySettings> {
        endpoints::GetGeneralRepositorySettings {}
            .make_request()
            .wrap_sync::<endpoints::GetGeneralRepositorySettings>(self)
    }

    /// Get instance's global settings for ui
    pub fn get_general_ui_settings(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetGeneralUiSettings> {
        endpoints::GetGeneralUiSettings {}
            .make_request()
            .wrap_sync::<endpoints::GetGeneralUiSettings>(self)
    }

    /// Get default signing-key.gpg
    pub fn get_signing_key(&self) -> crate::sync::WrappedRequest<'_, endpoints::GetSigningKey> {
        endpoints::GetSigningKey {}
            .make_request()
            .wrap_sync::<endpoints::GetSigningKey>(self)
    }

    /// Get a team
    ///
    /// - `id`: id of the team to get
    pub fn org_get_team(&self, id: u64) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetTeam> {
        endpoints::OrgGetTeam { id }
            .make_request()
            .wrap_sync::<endpoints::OrgGetTeam>(self)
    }

    /// Delete a team
    ///
    /// - `id`: id of the team to delete
    pub fn org_delete_team(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgDeleteTeam> {
        endpoints::OrgDeleteTeam { id }
            .make_request()
            .wrap_sync::<endpoints::OrgDeleteTeam>(self)
    }

    /// Edit a team
    ///
    /// - `id`: id of the team to edit
    /// - `body`: See [`EditTeamOption`]
    pub fn org_edit_team(
        &self,
        id: u32,
        body: EditTeamOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgEditTeam> {
        endpoints::OrgEditTeam { id, body: body }
            .make_request()
            .wrap_sync::<endpoints::OrgEditTeam>(self)
    }

    /// List a team's activity feeds
    ///
    /// - `id`: id of the team
    pub fn org_list_team_activity_feeds(
        &self,
        id: u64,
        query: OrgListTeamActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListTeamActivityFeeds> {
        endpoints::OrgListTeamActivityFeeds { id, query }
            .make_request()
            .wrap_sync::<endpoints::OrgListTeamActivityFeeds>(self)
    }

    /// List a team's members
    ///
    /// - `id`: id of the team
    pub fn org_list_team_members(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListTeamMembers> {
        endpoints::OrgListTeamMembers { id }
            .make_request()
            .wrap_sync::<endpoints::OrgListTeamMembers>(self)
    }

    /// List a particular member of team
    ///
    /// - `id`: id of the team
    /// - `username`: username of the member to list
    pub fn org_list_team_member(
        &self,
        id: u64,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListTeamMember> {
        endpoints::OrgListTeamMember { id, username }
            .make_request()
            .wrap_sync::<endpoints::OrgListTeamMember>(self)
    }

    /// Add a team member
    ///
    /// - `id`: id of the team
    /// - `username`: username of the user to add
    pub fn org_add_team_member(
        &self,
        id: u64,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgAddTeamMember> {
        endpoints::OrgAddTeamMember { id, username }
            .make_request()
            .wrap_sync::<endpoints::OrgAddTeamMember>(self)
    }

    /// Remove a team member
    ///
    /// - `id`: id of the team
    /// - `username`: username of the user to remove
    pub fn org_remove_team_member(
        &self,
        id: u64,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgRemoveTeamMember> {
        endpoints::OrgRemoveTeamMember { id, username }
            .make_request()
            .wrap_sync::<endpoints::OrgRemoveTeamMember>(self)
    }

    /// List a team's repos
    ///
    /// - `id`: id of the team
    pub fn org_list_team_repos(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListTeamRepos> {
        endpoints::OrgListTeamRepos { id }
            .make_request()
            .wrap_sync::<endpoints::OrgListTeamRepos>(self)
    }

    /// List a particular repo of team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to list
    /// - `repo`: name of the repo to list
    pub fn org_list_team_repo(
        &self,
        id: u64,
        org: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListTeamRepo> {
        endpoints::OrgListTeamRepo { id, org, repo }
            .make_request()
            .wrap_sync::<endpoints::OrgListTeamRepo>(self)
    }

    /// Add a repository to a team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to add
    /// - `repo`: name of the repo to add
    pub fn org_add_team_repository(
        &self,
        id: u64,
        org: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgAddTeamRepository> {
        endpoints::OrgAddTeamRepository { id, org, repo }
            .make_request()
            .wrap_sync::<endpoints::OrgAddTeamRepository>(self)
    }

    /// Remove a repository from a team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to remove
    /// - `repo`: name of the repo to remove
    pub fn org_remove_team_repository(
        &self,
        id: u64,
        org: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgRemoveTeamRepository> {
        endpoints::OrgRemoveTeamRepository { id, org, repo }
            .make_request()
            .wrap_sync::<endpoints::OrgRemoveTeamRepository>(self)
    }

    /// search topics via keyword
    ///
    pub fn topic_search(
        &self,
        query: TopicSearchQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::TopicSearch> {
        endpoints::TopicSearch { query }
            .make_request()
            .wrap_sync::<endpoints::TopicSearch>(self)
    }

    /// Get the authenticated user
    pub fn user_get_current(&self) -> crate::sync::WrappedRequest<'_, endpoints::UserGetCurrent> {
        endpoints::UserGetCurrent {}
            .make_request()
            .wrap_sync::<endpoints::UserGetCurrent>(self)
    }

    /// Get an user's actions runner registration token
    pub fn user_get_runner_registration_token(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetRunnerRegistrationToken> {
        endpoints::UserGetRunnerRegistrationToken {}
            .make_request()
            .wrap_sync::<endpoints::UserGetRunnerRegistrationToken>(self)
    }

    /// Create or Update a secret value in a user scope
    ///
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub fn update_user_secret(
        &self,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateUserSecret> {
        endpoints::UpdateUserSecret {
            secretname,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UpdateUserSecret>(self)
    }

    /// Delete a secret in a user scope
    ///
    /// - `secretname`: name of the secret
    pub fn delete_user_secret(
        &self,
        secretname: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeleteUserSecret> {
        endpoints::DeleteUserSecret { secretname }
            .make_request()
            .wrap_sync::<endpoints::DeleteUserSecret>(self)
    }

    /// Get the user-level list of variables which is created by current doer
    ///
    pub fn get_user_variables_list(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetUserVariablesList> {
        endpoints::GetUserVariablesList {}
            .make_request()
            .wrap_sync::<endpoints::GetUserVariablesList>(self)
    }

    /// Get a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    pub fn get_user_variable(
        &self,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetUserVariable> {
        endpoints::GetUserVariable { variablename }
            .make_request()
            .wrap_sync::<endpoints::GetUserVariable>(self)
    }

    /// Update a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub fn update_user_variable(
        &self,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateUserVariable> {
        endpoints::UpdateUserVariable {
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UpdateUserVariable>(self)
    }

    /// Create a user-level variable
    ///
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub fn create_user_variable(
        &self,
        variablename: &str,
        body: CreateVariableOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateUserVariable> {
        endpoints::CreateUserVariable {
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::CreateUserVariable>(self)
    }

    /// Delete a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    pub fn delete_user_variable(
        &self,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::DeleteUserVariable> {
        endpoints::DeleteUserVariable { variablename }
            .make_request()
            .wrap_sync::<endpoints::DeleteUserVariable>(self)
    }

    /// List the authenticated user's oauth2 applications
    ///
    pub fn user_get_oauth2_applications(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetOAuth2Applications> {
        endpoints::UserGetOAuth2Applications {}
            .make_request()
            .wrap_sync::<endpoints::UserGetOAuth2Applications>(self)
    }

    /// creates a new OAuth2 application
    ///
    /// - `body`: See [`CreateOAuth2ApplicationOptions`]
    pub fn user_create_oauth2_application(
        &self,
        body: CreateOAuth2ApplicationOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCreateOAuth2Application> {
        endpoints::UserCreateOAuth2Application { body: body }
            .make_request()
            .wrap_sync::<endpoints::UserCreateOAuth2Application>(self)
    }

    /// get an OAuth2 Application
    ///
    /// - `id`: Application ID to be found
    pub fn user_get_oauth2_application(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetOAuth2Application> {
        endpoints::UserGetOAuth2Application { id }
            .make_request()
            .wrap_sync::<endpoints::UserGetOAuth2Application>(self)
    }

    /// delete an OAuth2 Application
    ///
    /// - `id`: token to be deleted
    pub fn user_delete_oauth2_application(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserDeleteOAuth2Application> {
        endpoints::UserDeleteOAuth2Application { id }
            .make_request()
            .wrap_sync::<endpoints::UserDeleteOAuth2Application>(self)
    }

    /// update an OAuth2 Application, this includes regenerating the client secret
    ///
    /// - `id`: application to be updated
    /// - `body`: See [`CreateOAuth2ApplicationOptions`]
    pub fn user_update_oauth2_application(
        &self,
        id: u64,
        body: CreateOAuth2ApplicationOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserUpdateOAuth2Application> {
        endpoints::UserUpdateOAuth2Application { id, body: body }
            .make_request()
            .wrap_sync::<endpoints::UserUpdateOAuth2Application>(self)
    }

    /// Update Avatar
    ///
    /// - `body`: See [`UpdateUserAvatarOption`]
    pub fn user_update_avatar(
        &self,
        body: UpdateUserAvatarOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserUpdateAvatar> {
        endpoints::UserUpdateAvatar { body: body }
            .make_request()
            .wrap_sync::<endpoints::UserUpdateAvatar>(self)
    }

    /// Delete Avatar
    pub fn user_delete_avatar(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserDeleteAvatar> {
        endpoints::UserDeleteAvatar {}
            .make_request()
            .wrap_sync::<endpoints::UserDeleteAvatar>(self)
    }

    /// Blocks a user from the doer.
    ///
    /// - `username`: username of the user
    pub fn user_block_user(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserBlockUser> {
        endpoints::UserBlockUser { username }
            .make_request()
            .wrap_sync::<endpoints::UserBlockUser>(self)
    }

    /// List the authenticated user's email addresses
    pub fn user_list_emails(&self) -> crate::sync::WrappedRequest<'_, endpoints::UserListEmails> {
        endpoints::UserListEmails {}
            .make_request()
            .wrap_sync::<endpoints::UserListEmails>(self)
    }

    /// Add email addresses
    ///
    /// - `body`: See [`CreateEmailOption`]
    pub fn user_add_email(
        &self,
        body: CreateEmailOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserAddEmail> {
        endpoints::UserAddEmail { body: body }
            .make_request()
            .wrap_sync::<endpoints::UserAddEmail>(self)
    }

    /// Delete email addresses
    ///
    /// - `body`: See [`DeleteEmailOption`]
    pub fn user_delete_email(
        &self,
        body: DeleteEmailOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserDeleteEmail> {
        endpoints::UserDeleteEmail { body: body }
            .make_request()
            .wrap_sync::<endpoints::UserDeleteEmail>(self)
    }

    /// List the authenticated user's followers
    ///
    pub fn user_current_list_followers(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListFollowers> {
        endpoints::UserCurrentListFollowers {}
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListFollowers>(self)
    }

    /// List the users that the authenticated user is following
    ///
    pub fn user_current_list_following(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListFollowing> {
        endpoints::UserCurrentListFollowing {}
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListFollowing>(self)
    }

    /// Check whether a user is followed by the authenticated user
    ///
    /// - `username`: username of followed user
    pub fn user_current_check_following(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentCheckFollowing> {
        endpoints::UserCurrentCheckFollowing { username }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentCheckFollowing>(self)
    }

    /// Follow a user
    ///
    /// - `username`: username of user to follow
    pub fn user_current_put_follow(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentPutFollow> {
        endpoints::UserCurrentPutFollow { username }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentPutFollow>(self)
    }

    /// Unfollow a user
    ///
    /// - `username`: username of user to unfollow
    pub fn user_current_delete_follow(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentDeleteFollow> {
        endpoints::UserCurrentDeleteFollow { username }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentDeleteFollow>(self)
    }

    /// Get a Token to verify
    pub fn get_verification_token(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::GetVerificationToken> {
        endpoints::GetVerificationToken {}
            .make_request()
            .wrap_sync::<endpoints::GetVerificationToken>(self)
    }

    /// Verify a GPG key
    pub fn user_verify_gpg_key(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserVerifyGpgKey> {
        endpoints::UserVerifyGpgKey {}
            .make_request()
            .wrap_sync::<endpoints::UserVerifyGpgKey>(self)
    }

    /// List the authenticated user's GPG keys
    ///
    pub fn user_current_list_gpg_keys(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListGpgKeys> {
        endpoints::UserCurrentListGpgKeys {}
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListGpgKeys>(self)
    }

    /// Create a GPG key
    ///
    /// - `Form`: See [`CreateGPGKeyOption`]
    pub fn user_current_post_gpg_key(
        &self,
        form: CreateGPGKeyOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentPostGpgKey> {
        endpoints::UserCurrentPostGpgKey { body: form }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentPostGpgKey>(self)
    }

    /// Get a GPG key
    ///
    /// - `id`: id of key to get
    pub fn user_current_get_gpg_key(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentGetGpgKey> {
        endpoints::UserCurrentGetGpgKey { id }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentGetGpgKey>(self)
    }

    /// Remove a GPG key
    ///
    /// - `id`: id of key to delete
    pub fn user_current_delete_gpg_key(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentDeleteGpgKey> {
        endpoints::UserCurrentDeleteGpgKey { id }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentDeleteGpgKey>(self)
    }

    /// List the authenticated user's webhooks
    ///
    pub fn user_list_hooks(&self) -> crate::sync::WrappedRequest<'_, endpoints::UserListHooks> {
        endpoints::UserListHooks {}
            .make_request()
            .wrap_sync::<endpoints::UserListHooks>(self)
    }

    /// Create a hook
    ///
    /// - `body`: See [`CreateHookOption`]
    pub fn user_create_hook(
        &self,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCreateHook> {
        endpoints::UserCreateHook { body: body }
            .make_request()
            .wrap_sync::<endpoints::UserCreateHook>(self)
    }

    /// Get a hook
    ///
    /// - `id`: id of the hook to get
    pub fn user_get_hook(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetHook> {
        endpoints::UserGetHook { id }
            .make_request()
            .wrap_sync::<endpoints::UserGetHook>(self)
    }

    /// Delete a hook
    ///
    /// - `id`: id of the hook to delete
    pub fn user_delete_hook(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserDeleteHook> {
        endpoints::UserDeleteHook { id }
            .make_request()
            .wrap_sync::<endpoints::UserDeleteHook>(self)
    }

    /// Update a hook
    ///
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub fn user_edit_hook(
        &self,
        id: u64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserEditHook> {
        endpoints::UserEditHook { id, body: body }
            .make_request()
            .wrap_sync::<endpoints::UserEditHook>(self)
    }

    /// List the authenticated user's public keys
    ///
    pub fn user_current_list_keys(
        &self,
        query: UserCurrentListKeysQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListKeys> {
        endpoints::UserCurrentListKeys { query }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListKeys>(self)
    }

    /// Create a public key
    ///
    /// - `body`: See [`CreateKeyOption`]
    pub fn user_current_post_key(
        &self,
        body: CreateKeyOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentPostKey> {
        endpoints::UserCurrentPostKey { body: body }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentPostKey>(self)
    }

    /// Get a public key
    ///
    /// - `id`: id of key to get
    pub fn user_current_get_key(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentGetKey> {
        endpoints::UserCurrentGetKey { id }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentGetKey>(self)
    }

    /// Delete a public key
    ///
    /// - `id`: id of key to delete
    pub fn user_current_delete_key(
        &self,
        id: u64,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentDeleteKey> {
        endpoints::UserCurrentDeleteKey { id }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentDeleteKey>(self)
    }

    /// List the authenticated user's blocked users
    ///
    pub fn user_list_blocked_users(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListBlockedUsers> {
        endpoints::UserListBlockedUsers {}
            .make_request()
            .wrap_sync::<endpoints::UserListBlockedUsers>(self)
    }

    /// List the current user's organizations
    ///
    pub fn org_list_current_user_orgs(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListCurrentUserOrgs> {
        endpoints::OrgListCurrentUserOrgs {}
            .make_request()
            .wrap_sync::<endpoints::OrgListCurrentUserOrgs>(self)
    }

    /// Get quota information for the authenticated user
    pub fn user_get_quota(&self) -> crate::sync::WrappedRequest<'_, endpoints::UserGetQuota> {
        endpoints::UserGetQuota {}
            .make_request()
            .wrap_sync::<endpoints::UserGetQuota>(self)
    }

    /// List the artifacts affecting the authenticated user's quota
    ///
    pub fn user_list_quota_artifacts(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListQuotaArtifacts> {
        endpoints::UserListQuotaArtifacts {}
            .make_request()
            .wrap_sync::<endpoints::UserListQuotaArtifacts>(self)
    }

    /// List the attachments affecting the authenticated user's quota
    ///
    pub fn user_list_quota_attachments(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListQuotaAttachments> {
        endpoints::UserListQuotaAttachments {}
            .make_request()
            .wrap_sync::<endpoints::UserListQuotaAttachments>(self)
    }

    /// Check if the authenticated user is over quota for a given subject
    pub fn user_check_quota(&self) -> crate::sync::WrappedRequest<'_, endpoints::UserCheckQuota> {
        endpoints::UserCheckQuota {}
            .make_request()
            .wrap_sync::<endpoints::UserCheckQuota>(self)
    }

    /// List the packages affecting the authenticated user's quota
    ///
    pub fn user_list_quota_packages(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListQuotaPackages> {
        endpoints::UserListQuotaPackages {}
            .make_request()
            .wrap_sync::<endpoints::UserListQuotaPackages>(self)
    }

    /// List the repos that the authenticated user owns
    ///
    pub fn user_current_list_repos(
        &self,
        query: UserCurrentListReposQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListRepos> {
        endpoints::UserCurrentListRepos { query }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListRepos>(self)
    }

    /// Create a repository
    ///
    /// - `body`: See [`CreateRepoOption`]
    pub fn create_current_user_repo(
        &self,
        body: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::CreateCurrentUserRepo> {
        endpoints::CreateCurrentUserRepo { body: body }
            .make_request()
            .wrap_sync::<endpoints::CreateCurrentUserRepo>(self)
    }

    /// Get user settings
    pub fn get_user_settings(&self) -> crate::sync::WrappedRequest<'_, endpoints::GetUserSettings> {
        endpoints::GetUserSettings {}
            .make_request()
            .wrap_sync::<endpoints::GetUserSettings>(self)
    }

    /// Update user settings
    ///
    /// - `body`: See [`UserSettingsOptions`]
    pub fn update_user_settings(
        &self,
        body: UserSettingsOptions,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UpdateUserSettings> {
        endpoints::UpdateUserSettings { body: body }
            .make_request()
            .wrap_sync::<endpoints::UpdateUserSettings>(self)
    }

    /// The repos that the authenticated user has starred
    ///
    pub fn user_current_list_starred(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListStarred> {
        endpoints::UserCurrentListStarred {}
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListStarred>(self)
    }

    /// Whether the authenticated is starring the repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_check_starring(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentCheckStarring> {
        endpoints::UserCurrentCheckStarring { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentCheckStarring>(self)
    }

    /// Star the given repo
    ///
    /// - `owner`: owner of the repo to star
    /// - `repo`: name of the repo to star
    pub fn user_current_put_star(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentPutStar> {
        endpoints::UserCurrentPutStar { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentPutStar>(self)
    }

    /// Unstar the given repo
    ///
    /// - `owner`: owner of the repo to unstar
    /// - `repo`: name of the repo to unstar
    pub fn user_current_delete_star(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentDeleteStar> {
        endpoints::UserCurrentDeleteStar { owner, repo }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentDeleteStar>(self)
    }

    /// Get list of all existing stopwatches
    ///
    pub fn user_get_stop_watches(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetStopWatches> {
        endpoints::UserGetStopWatches {}
            .make_request()
            .wrap_sync::<endpoints::UserGetStopWatches>(self)
    }

    /// List repositories watched by the authenticated user
    ///
    pub fn user_current_list_subscriptions(
        &self,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentListSubscriptions> {
        endpoints::UserCurrentListSubscriptions {}
            .make_request()
            .wrap_sync::<endpoints::UserCurrentListSubscriptions>(self)
    }

    /// List all the teams a user belongs to
    ///
    pub fn user_list_teams(&self) -> crate::sync::WrappedRequest<'_, endpoints::UserListTeams> {
        endpoints::UserListTeams {}
            .make_request()
            .wrap_sync::<endpoints::UserListTeams>(self)
    }

    /// List the current user's tracked times
    ///
    pub fn user_current_tracked_times(
        &self,
        query: UserCurrentTrackedTimesQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCurrentTrackedTimes> {
        endpoints::UserCurrentTrackedTimes { query }
            .make_request()
            .wrap_sync::<endpoints::UserCurrentTrackedTimes>(self)
    }

    /// Unblocks a user from the doer.
    ///
    /// - `username`: username of the user
    pub fn user_unblock_user(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserUnblockUser> {
        endpoints::UserUnblockUser { username }
            .make_request()
            .wrap_sync::<endpoints::UserUnblockUser>(self)
    }

    /// Search for users
    ///
    pub fn user_search(
        &self,
        query: UserSearchQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserSearch> {
        endpoints::UserSearch { query }
            .make_request()
            .wrap_sync::<endpoints::UserSearch>(self)
    }

    /// Get a user
    ///
    /// - `username`: username of user to get
    pub fn user_get(&self, username: &str) -> crate::sync::WrappedRequest<'_, endpoints::UserGet> {
        endpoints::UserGet { username }
            .make_request()
            .wrap_sync::<endpoints::UserGet>(self)
    }

    /// List a user's activity feeds
    ///
    /// - `username`: username of user
    pub fn user_list_activity_feeds(
        &self,
        username: &str,
        query: UserListActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListActivityFeeds> {
        endpoints::UserListActivityFeeds { username, query }
            .make_request()
            .wrap_sync::<endpoints::UserListActivityFeeds>(self)
    }

    /// List the given user's followers
    ///
    /// - `username`: username of user
    pub fn user_list_followers(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListFollowers> {
        endpoints::UserListFollowers { username }
            .make_request()
            .wrap_sync::<endpoints::UserListFollowers>(self)
    }

    /// List the users that the given user is following
    ///
    /// - `username`: username of user
    pub fn user_list_following(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListFollowing> {
        endpoints::UserListFollowing { username }
            .make_request()
            .wrap_sync::<endpoints::UserListFollowing>(self)
    }

    /// Check if one user is following another user
    ///
    /// - `username`: username of following user
    /// - `target`: username of followed user
    pub fn user_check_following(
        &self,
        username: &str,
        target: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCheckFollowing> {
        endpoints::UserCheckFollowing { username, target }
            .make_request()
            .wrap_sync::<endpoints::UserCheckFollowing>(self)
    }

    /// List the given user's GPG keys
    ///
    /// - `username`: username of user
    pub fn user_list_gpg_keys(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListGpgKeys> {
        endpoints::UserListGpgKeys { username }
            .make_request()
            .wrap_sync::<endpoints::UserListGpgKeys>(self)
    }

    /// Get a user's heatmap
    ///
    /// - `username`: username of user to get
    pub fn user_get_heatmap_data(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetHeatmapData> {
        endpoints::UserGetHeatmapData { username }
            .make_request()
            .wrap_sync::<endpoints::UserGetHeatmapData>(self)
    }

    /// List the given user's public keys
    ///
    /// - `username`: username of user
    pub fn user_list_keys(
        &self,
        username: &str,
        query: UserListKeysQuery,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListKeys> {
        endpoints::UserListKeys { username, query }
            .make_request()
            .wrap_sync::<endpoints::UserListKeys>(self)
    }

    /// List a user's organizations
    ///
    /// - `username`: username of user
    pub fn org_list_user_orgs(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgListUserOrgs> {
        endpoints::OrgListUserOrgs { username }
            .make_request()
            .wrap_sync::<endpoints::OrgListUserOrgs>(self)
    }

    /// Get user permissions in organization
    ///
    /// - `username`: username of user
    /// - `org`: name of the organization
    pub fn org_get_user_permissions(
        &self,
        username: &str,
        org: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::OrgGetUserPermissions> {
        endpoints::OrgGetUserPermissions { username, org }
            .make_request()
            .wrap_sync::<endpoints::OrgGetUserPermissions>(self)
    }

    /// List the repos owned by the given user
    ///
    /// - `username`: username of user
    pub fn user_list_repos(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListRepos> {
        endpoints::UserListRepos { username }
            .make_request()
            .wrap_sync::<endpoints::UserListRepos>(self)
    }

    /// The repos that the given user has starred
    ///
    /// - `username`: username of user
    pub fn user_list_starred(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListStarred> {
        endpoints::UserListStarred { username }
            .make_request()
            .wrap_sync::<endpoints::UserListStarred>(self)
    }

    /// List the repositories watched by a user
    ///
    /// - `username`: username of the user
    pub fn user_list_subscriptions(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserListSubscriptions> {
        endpoints::UserListSubscriptions { username }
            .make_request()
            .wrap_sync::<endpoints::UserListSubscriptions>(self)
    }

    /// List the authenticated user's access tokens
    ///
    /// - `username`: username of user
    pub fn user_get_tokens(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserGetTokens> {
        endpoints::UserGetTokens { username }
            .make_request()
            .wrap_sync::<endpoints::UserGetTokens>(self)
    }

    /// Create an access token
    ///
    /// - `username`: username of user
    /// - `body`: See [`CreateAccessTokenOption`]
    pub fn user_create_token(
        &self,
        username: &str,
        body: CreateAccessTokenOption,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserCreateToken> {
        endpoints::UserCreateToken {
            username,
            body: body,
        }
        .make_request()
        .wrap_sync::<endpoints::UserCreateToken>(self)
    }

    /// delete an access token
    ///
    /// - `username`: username of user
    /// - `token`: token to be deleted, identified by ID and if not available by name
    pub fn user_delete_access_token(
        &self,
        username: &str,
        token: &str,
    ) -> crate::sync::WrappedRequest<'_, endpoints::UserDeleteAccessToken> {
        endpoints::UserDeleteAccessToken { username, token }
            .make_request()
            .wrap_sync::<endpoints::UserDeleteAccessToken>(self)
    }

    /// Returns the version of the Forgejo application
    pub fn get_version(&self) -> crate::sync::WrappedRequest<'_, endpoints::GetVersion> {
        endpoints::GetVersion {}
            .make_request()
            .wrap_sync::<endpoints::GetVersion>(self)
    }
}
