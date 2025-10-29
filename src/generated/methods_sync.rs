use super::structs::*;
use crate::generated::endpoints;
use crate::Endpoint;
use crate::WrappedRequest;

impl crate::sync::Forgejo {
    /// Returns the instance's Actor
    pub fn activitypub_instance_actor(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubInstanceActor,
        <endpoints::ActivitypubInstanceActor as Endpoint>::Response,
    > {
        endpoints::ActivitypubInstanceActor {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Send to the inbox
    pub fn activitypub_instance_actor_inbox(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubInstanceActorInbox,
        <endpoints::ActivitypubInstanceActorInbox as Endpoint>::Response,
    > {
        endpoints::ActivitypubInstanceActorInbox {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns the Repository actor for a repo
    ///
    /// - `repository-id`: repository ID of the repo
    pub fn activitypub_repository(
        &self,
        repository_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubRepository,
        <endpoints::ActivitypubRepository as Endpoint>::Response,
    > {
        endpoints::ActivitypubRepository { repository_id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Send to the inbox
    ///
    /// - `repository-id`: repository ID of the repo
    /// - `body`: See [`ForgeLike`]
    pub fn activitypub_repository_inbox(
        &self,
        repository_id: i64,
        body: ForgeLike,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubRepositoryInbox,
        <endpoints::ActivitypubRepositoryInbox as Endpoint>::Response,
    > {
        endpoints::ActivitypubRepositoryInbox {
            repository_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Returns the Person actor for a user
    ///
    /// - `user-id`: user ID of the user
    pub fn activitypub_person(
        &self,
        user_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubPerson,
        <endpoints::ActivitypubPerson as Endpoint>::Response,
    > {
        endpoints::ActivitypubPerson { user_id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a specific activity object of the user
    ///
    /// - `user-id`: user ID of the user
    /// - `activity-id`: activity ID of the sought activity
    pub fn activitypub_person_activity_note(
        &self,
        user_id: u32,
        activity_id: u32,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubPersonActivityNote,
        <endpoints::ActivitypubPersonActivityNote as Endpoint>::Response,
    > {
        endpoints::ActivitypubPersonActivityNote {
            user_id,
            activity_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Get a specific activity of the user
    ///
    /// - `user-id`: user ID of the user
    /// - `activity-id`: activity ID of the sought activity
    pub fn activitypub_person_activity(
        &self,
        user_id: u32,
        activity_id: u32,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubPersonActivity,
        <endpoints::ActivitypubPersonActivity as Endpoint>::Response,
    > {
        endpoints::ActivitypubPersonActivity {
            user_id,
            activity_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Send to the inbox
    ///
    /// - `user-id`: user ID of the user
    pub fn activitypub_person_inbox(
        &self,
        user_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubPersonInbox,
        <endpoints::ActivitypubPersonInbox as Endpoint>::Response,
    > {
        endpoints::ActivitypubPersonInbox { user_id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the user's recorded activity
    ///
    /// - `user-id`: user ID of the user
    pub fn activitypub_person_feed(
        &self,
        user_id: u32,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ActivitypubPersonFeed,
        <endpoints::ActivitypubPersonFeed as Endpoint>::Response,
    > {
        endpoints::ActivitypubPersonFeed { user_id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List cron tasks
    ///
    pub fn admin_cron_list(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCronList,
        <endpoints::AdminCronList as Endpoint>::Response,
    > {
        endpoints::AdminCronList {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Run cron task
    ///
    /// - `task`: task to run
    pub fn admin_cron_run(
        &self,
        task: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCronRun<'_>,
        <endpoints::AdminCronRun<'_> as Endpoint>::Response,
    > {
        endpoints::AdminCronRun { task }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List all users' email addresses
    ///
    pub fn admin_get_all_emails(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetAllEmails,
        <endpoints::AdminGetAllEmails as Endpoint>::Response,
    > {
        endpoints::AdminGetAllEmails {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search users' email addresses
    ///
    pub fn admin_search_emails(
        &self,
        query: AdminSearchEmailsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminSearchEmails,
        <endpoints::AdminSearchEmails as Endpoint>::Response,
    > {
        endpoints::AdminSearchEmails { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List global (system) webhooks
    ///
    pub fn admin_list_hooks(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminListHooks,
        <endpoints::AdminListHooks as Endpoint>::Response,
    > {
        endpoints::AdminListHooks {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a hook
    ///
    /// - `body`: See [`CreateHookOption`]
    pub fn admin_create_hook(
        &self,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreateHook,
        <endpoints::AdminCreateHook as Endpoint>::Response,
    > {
        endpoints::AdminCreateHook { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a hook
    ///
    /// - `id`: id of the hook to get
    pub fn admin_get_hook(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetHook,
        <endpoints::AdminGetHook as Endpoint>::Response,
    > {
        endpoints::AdminGetHook { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a hook
    ///
    /// - `id`: id of the hook to delete
    pub fn admin_delete_hook(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminDeleteHook,
        <endpoints::AdminDeleteHook as Endpoint>::Response,
    > {
        endpoints::AdminDeleteHook { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update a hook
    ///
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub fn admin_edit_hook(
        &self,
        id: i64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminEditHook,
        <endpoints::AdminEditHook as Endpoint>::Response,
    > {
        endpoints::AdminEditHook { id, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List all organizations
    ///
    pub fn admin_get_all_orgs(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetAllOrgs,
        <endpoints::AdminGetAllOrgs as Endpoint>::Response,
    > {
        endpoints::AdminGetAllOrgs {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the available quota groups
    pub fn admin_list_quota_groups(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminListQuotaGroups,
        <endpoints::AdminListQuotaGroups as Endpoint>::Response,
    > {
        endpoints::AdminListQuotaGroups {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a new quota group
    ///
    /// - `group`: Definition of the quota group

    ///   See [`CreateQuotaGroupOptions`]
    pub fn admin_create_quota_group(
        &self,
        group: CreateQuotaGroupOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreateQuotaGroup,
        <endpoints::AdminCreateQuotaGroup as Endpoint>::Response,
    > {
        endpoints::AdminCreateQuotaGroup { body: group }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get information about the quota group
    ///
    /// - `quotagroup`: quota group to query
    pub fn admin_get_quota_group(
        &self,
        quotagroup: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetQuotaGroup<'_>,
        <endpoints::AdminGetQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminGetQuotaGroup { quotagroup }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a quota group
    ///
    /// - `quotagroup`: quota group to delete
    pub fn admin_delete_quota_group(
        &self,
        quotagroup: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminDeleteQuotaGroup<'_>,
        <endpoints::AdminDeleteQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminDeleteQuotaGroup { quotagroup }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Adds a rule to a quota group
    ///
    /// - `quotagroup`: quota group to add a rule to
    /// - `quotarule`: the name of the quota rule to add to the group
    pub fn admin_add_rule_to_quota_group(
        &self,
        quotagroup: &str,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminAddRuleToQuotaGroup<'_>,
        <endpoints::AdminAddRuleToQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminAddRuleToQuotaGroup {
            quotagroup,
            quotarule,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Removes a rule from a quota group
    ///
    /// - `quotagroup`: quota group to remove a rule from
    /// - `quotarule`: the name of the quota rule to remove from the group
    pub fn admin_remove_rule_from_quota_group(
        &self,
        quotagroup: &str,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminRemoveRuleFromQuotaGroup<'_>,
        <endpoints::AdminRemoveRuleFromQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminRemoveRuleFromQuotaGroup {
            quotagroup,
            quotarule,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List users in a quota group
    ///
    /// - `quotagroup`: quota group to list members of
    pub fn admin_list_users_in_quota_group(
        &self,
        quotagroup: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminListUsersInQuotaGroup<'_>,
        <endpoints::AdminListUsersInQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminListUsersInQuotaGroup { quotagroup }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Add a user to a quota group
    ///
    /// - `quotagroup`: quota group to add the user to
    /// - `username`: username of the user to add to the quota group
    pub fn admin_add_user_to_quota_group(
        &self,
        quotagroup: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminAddUserToQuotaGroup<'_>,
        <endpoints::AdminAddUserToQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminAddUserToQuotaGroup {
            quotagroup,
            username,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Remove a user from a quota group
    ///
    /// - `quotagroup`: quota group to remove a user from
    /// - `username`: username of the user to remove from the quota group
    pub fn admin_remove_user_from_quota_group(
        &self,
        quotagroup: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminRemoveUserFromQuotaGroup<'_>,
        <endpoints::AdminRemoveUserFromQuotaGroup<'_> as Endpoint>::Response,
    > {
        endpoints::AdminRemoveUserFromQuotaGroup {
            quotagroup,
            username,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List the available quota rules
    pub fn admin_list_quota_rules(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminListQuotaRules,
        <endpoints::AdminListQuotaRules as Endpoint>::Response,
    > {
        endpoints::AdminListQuotaRules {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a new quota rule
    ///
    /// - `rule`: Definition of the quota rule

    ///   See [`CreateQuotaRuleOptions`]
    pub fn admin_create_quota_rule(
        &self,
        rule: CreateQuotaRuleOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreateQuotaRule,
        <endpoints::AdminCreateQuotaRule as Endpoint>::Response,
    > {
        endpoints::AdminCreateQuotaRule { body: rule }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get information about a quota rule
    ///
    /// - `quotarule`: quota rule to query
    pub fn admin_get_quota_rule(
        &self,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetQuotaRule<'_>,
        <endpoints::AdminGetQuotaRule<'_> as Endpoint>::Response,
    > {
        endpoints::AdminGetQuotaRule { quotarule }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Deletes a quota rule
    ///
    /// - `quotarule`: quota rule to delete
    pub fn admin_d_elete_quota_rule(
        &self,
        quotarule: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminDEleteQuotaRule<'_>,
        <endpoints::AdminDEleteQuotaRule<'_> as Endpoint>::Response,
    > {
        endpoints::AdminDEleteQuotaRule { quotarule }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Change an existing quota rule
    ///
    /// - `quotarule`: Quota rule to change
    /// - `rule`: See [`EditQuotaRuleOptions`]
    pub fn admin_edit_quota_rule(
        &self,
        quotarule: &str,
        rule: EditQuotaRuleOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminEditQuotaRule<'_>,
        <endpoints::AdminEditQuotaRule<'_> as Endpoint>::Response,
    > {
        endpoints::AdminEditQuotaRule {
            quotarule,
            body: rule,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Search action jobs according filter conditions
    ///
    pub fn admin_search_run_jobs(
        &self,
        query: AdminSearchRunJobsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminSearchRunJobs,
        <endpoints::AdminSearchRunJobs as Endpoint>::Response,
    > {
        endpoints::AdminSearchRunJobs { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get an global actions runner registration token
    pub fn admin_get_runner_registration_token(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetRunnerRegistrationToken,
        <endpoints::AdminGetRunnerRegistrationToken as Endpoint>::Response,
    > {
        endpoints::AdminGetRunnerRegistrationToken {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List unadopted repositories
    ///
    pub fn admin_unadopted_list(
        &self,
        query: AdminUnadoptedListQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminUnadoptedList,
        <endpoints::AdminUnadoptedList as Endpoint>::Response,
    > {
        endpoints::AdminUnadoptedList { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Adopt unadopted files as a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn admin_adopt_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminAdoptRepository<'_>,
        <endpoints::AdminAdoptRepository<'_> as Endpoint>::Response,
    > {
        endpoints::AdminAdoptRepository { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete unadopted files
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn admin_delete_unadopted_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminDeleteUnadoptedRepository<'_>,
        <endpoints::AdminDeleteUnadoptedRepository<'_> as Endpoint>::Response,
    > {
        endpoints::AdminDeleteUnadoptedRepository { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search users according filter conditions
    ///
    pub fn admin_search_users(
        &self,
        query: AdminSearchUsersQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminSearchUsers,
        <endpoints::AdminSearchUsers as Endpoint>::Response,
    > {
        endpoints::AdminSearchUsers { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a user account
    ///
    /// - `body`: See [`CreateUserOption`]
    pub fn admin_create_user(
        &self,
        body: CreateUserOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreateUser,
        <endpoints::AdminCreateUser as Endpoint>::Response,
    > {
        endpoints::AdminCreateUser { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete user account
    ///
    /// - `username`: username of user to delete
    pub fn admin_delete_user(
        &self,
        username: &str,
        query: AdminDeleteUserQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminDeleteUser<'_>,
        <endpoints::AdminDeleteUser<'_> as Endpoint>::Response,
    > {
        endpoints::AdminDeleteUser { username, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Edit an existing user
    ///
    /// - `username`: username of user to edit
    /// - `body`: See [`EditUserOption`]
    pub fn admin_edit_user(
        &self,
        username: &str,
        body: EditUserOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminEditUser<'_>,
        <endpoints::AdminEditUser<'_> as Endpoint>::Response,
    > {
        endpoints::AdminEditUser {
            username,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Add an SSH public key to user's account
    ///
    /// - `username`: username of the user
    /// - `key`: See [`CreateKeyOption`]
    pub fn admin_create_public_key(
        &self,
        username: &str,
        key: CreateKeyOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreatePublicKey<'_>,
        <endpoints::AdminCreatePublicKey<'_> as Endpoint>::Response,
    > {
        endpoints::AdminCreatePublicKey {
            username,
            body: key,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Remove a public key from user's account
    ///
    /// - `username`: username of user
    /// - `id`: id of the key to delete
    pub fn admin_delete_user_public_key(
        &self,
        username: &str,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminDeleteUserPublicKey<'_>,
        <endpoints::AdminDeleteUserPublicKey<'_> as Endpoint>::Response,
    > {
        endpoints::AdminDeleteUserPublicKey { username, id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create an organization
    ///
    /// - `username`: username of the user that will own the created organization
    /// - `organization`: See [`CreateOrgOption`]
    pub fn admin_create_org(
        &self,
        username: &str,
        organization: CreateOrgOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreateOrg<'_>,
        <endpoints::AdminCreateOrg<'_> as Endpoint>::Response,
    > {
        endpoints::AdminCreateOrg {
            username,
            body: organization,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Get the user's quota info
    ///
    /// - `username`: username of user to query
    pub fn admin_get_user_quota(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminGetUserQuota<'_>,
        <endpoints::AdminGetUserQuota<'_> as Endpoint>::Response,
    > {
        endpoints::AdminGetUserQuota { username }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminSetUserQuotaGroups<'_>,
        <endpoints::AdminSetUserQuotaGroups<'_> as Endpoint>::Response,
    > {
        endpoints::AdminSetUserQuotaGroups {
            username,
            body: groups,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Rename a user
    ///
    /// - `username`: existing username of user
    /// - `body`: See [`RenameUserOption`]
    pub fn admin_rename_user(
        &self,
        username: &str,
        body: RenameUserOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminRenameUser<'_>,
        <endpoints::AdminRenameUser<'_> as Endpoint>::Response,
    > {
        endpoints::AdminRenameUser {
            username,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Create a repository on behalf of a user
    ///
    /// - `username`: username of the user. This user will own the created repository
    /// - `repository`: See [`CreateRepoOption`]
    pub fn admin_create_repo(
        &self,
        username: &str,
        repository: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AdminCreateRepo<'_>,
        <endpoints::AdminCreateRepo<'_> as Endpoint>::Response,
    > {
        endpoints::AdminCreateRepo {
            username,
            body: repository,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Returns a list of all gitignore templates
    pub fn list_gitignores_templates(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListGitignoresTemplates,
        <endpoints::ListGitignoresTemplates as Endpoint>::Response,
    > {
        endpoints::ListGitignoresTemplates {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns information about a gitignore template
    ///
    /// - `name`: name of the template
    pub fn get_gitignore_template_info(
        &self,
        name: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetGitignoreTemplateInfo<'_>,
        <endpoints::GetGitignoreTemplateInfo<'_> as Endpoint>::Response,
    > {
        endpoints::GetGitignoreTemplateInfo { name }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns a list of all label templates
    pub fn list_label_templates(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListLabelTemplates,
        <endpoints::ListLabelTemplates as Endpoint>::Response,
    > {
        endpoints::ListLabelTemplates {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns all labels in a template
    ///
    /// - `name`: name of the template
    pub fn get_label_template_info(
        &self,
        name: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetLabelTemplateInfo<'_>,
        <endpoints::GetLabelTemplateInfo<'_> as Endpoint>::Response,
    > {
        endpoints::GetLabelTemplateInfo { name }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns a list of all license templates
    pub fn list_license_templates(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListLicenseTemplates,
        <endpoints::ListLicenseTemplates as Endpoint>::Response,
    > {
        endpoints::ListLicenseTemplates {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns information about a license template
    ///
    /// - `name`: name of the license
    pub fn get_license_template_info(
        &self,
        name: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetLicenseTemplateInfo<'_>,
        <endpoints::GetLicenseTemplateInfo<'_> as Endpoint>::Response,
    > {
        endpoints::GetLicenseTemplateInfo { name }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Render a markdown document as HTML
    ///
    /// - `body`: See [`MarkdownOption`]
    pub fn render_markdown(
        &self,
        body: MarkdownOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RenderMarkdown,
        <endpoints::RenderMarkdown as Endpoint>::Response,
    > {
        endpoints::RenderMarkdown { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Render raw markdown as HTML
    ///
    /// - `body`: Request body to render

    ///   See [`String`]
    pub fn render_markdown_raw(
        &self,
        body: String,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RenderMarkdownRaw,
        <endpoints::RenderMarkdownRaw as Endpoint>::Response,
    > {
        endpoints::RenderMarkdownRaw { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Render a markup document as HTML
    ///
    /// - `body`: See [`MarkupOption`]
    pub fn render_markup(
        &self,
        body: MarkupOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RenderMarkup,
        <endpoints::RenderMarkup as Endpoint>::Response,
    > {
        endpoints::RenderMarkup { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns the nodeinfo of the Forgejo application
    pub fn get_node_info(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetNodeInfo,
        <endpoints::GetNodeInfo as Endpoint>::Response,
    > {
        endpoints::GetNodeInfo {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List users's notification threads
    ///
    pub fn notify_get_list(
        &self,
        query: NotifyGetListQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyGetList,
        <endpoints::NotifyGetList as Endpoint>::Response,
    > {
        endpoints::NotifyGetList { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Mark notification threads as read, pinned or unread
    ///
    pub fn notify_read_list(
        &self,
        query: NotifyReadListQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyReadList,
        <endpoints::NotifyReadList as Endpoint>::Response,
    > {
        endpoints::NotifyReadList { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if unread notifications exist
    pub fn notify_new_available(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyNewAvailable,
        <endpoints::NotifyNewAvailable as Endpoint>::Response,
    > {
        endpoints::NotifyNewAvailable {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get notification thread by ID
    ///
    /// - `id`: id of notification thread
    pub fn notify_get_thread(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyGetThread,
        <endpoints::NotifyGetThread as Endpoint>::Response,
    > {
        endpoints::NotifyGetThread { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Mark notification thread as read by ID
    ///
    /// - `id`: id of notification thread
    pub fn notify_read_thread(
        &self,
        id: i64,
        query: NotifyReadThreadQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyReadThread,
        <endpoints::NotifyReadThread as Endpoint>::Response,
    > {
        endpoints::NotifyReadThread { id, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a repository in an organization
    ///
    /// - `org`: name of organization
    /// - `body`: See [`CreateRepoOption`]
    pub fn create_org_repo_deprecated(
        &self,
        org: &str,
        body: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateOrgRepoDeprecated<'_>,
        <endpoints::CreateOrgRepoDeprecated<'_> as Endpoint>::Response,
    > {
        endpoints::CreateOrgRepoDeprecated { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List all organizations
    ///
    pub fn org_get_all(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetAll,
        <endpoints::OrgGetAll as Endpoint>::Response,
    > {
        endpoints::OrgGetAll {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create an organization
    ///
    /// - `organization`: See [`CreateOrgOption`]
    pub fn org_create(
        &self,
        organization: CreateOrgOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgCreate,
        <endpoints::OrgCreate as Endpoint>::Response,
    > {
        endpoints::OrgCreate { body: organization }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get an organization
    ///
    /// - `org`: name of the organization to get
    pub fn org_get(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGet<'_>,
        <endpoints::OrgGet<'_> as Endpoint>::Response,
    > {
        endpoints::OrgGet { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete an organization
    ///
    /// - `org`: organization that is to be deleted
    pub fn org_delete(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgDelete<'_>,
        <endpoints::OrgDelete<'_> as Endpoint>::Response,
    > {
        endpoints::OrgDelete { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Edit an organization
    ///
    /// - `org`: name of the organization to edit
    /// - `body`: See [`EditOrgOption`]
    pub fn org_edit(
        &self,
        org: &str,
        body: EditOrgOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgEdit<'_>,
        <endpoints::OrgEdit<'_> as Endpoint>::Response,
    > {
        endpoints::OrgEdit { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search for organization's action jobs according filter conditions
    ///
    /// - `org`: name of the organization
    pub fn org_search_run_jobs(
        &self,
        org: &str,
        query: OrgSearchRunJobsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgSearchRunJobs<'_>,
        <endpoints::OrgSearchRunJobs<'_> as Endpoint>::Response,
    > {
        endpoints::OrgSearchRunJobs { org, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get an organization's actions runner registration token
    ///
    /// - `org`: name of the organization
    pub fn org_get_runner_registration_token(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetRunnerRegistrationToken<'_>,
        <endpoints::OrgGetRunnerRegistrationToken<'_> as Endpoint>::Response,
    > {
        endpoints::OrgGetRunnerRegistrationToken { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List actions secrets of an organization
    ///
    /// - `org`: name of the organization
    pub fn org_list_actions_secrets(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListActionsSecrets<'_>,
        <endpoints::OrgListActionsSecrets<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListActionsSecrets { org }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateOrgSecret<'_>,
        <endpoints::UpdateOrgSecret<'_> as Endpoint>::Response,
    > {
        endpoints::UpdateOrgSecret {
            org,
            secretname,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Delete a secret in an organization
    ///
    /// - `org`: name of organization
    /// - `secretname`: name of the secret
    pub fn delete_org_secret(
        &self,
        org: &str,
        secretname: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeleteOrgSecret<'_>,
        <endpoints::DeleteOrgSecret<'_> as Endpoint>::Response,
    > {
        endpoints::DeleteOrgSecret { org, secretname }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List variables of an organization
    ///
    /// - `org`: name of the organization
    pub fn get_org_variables_list(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetOrgVariablesList<'_>,
        <endpoints::GetOrgVariablesList<'_> as Endpoint>::Response,
    > {
        endpoints::GetOrgVariablesList { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get organization's variable by name
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    pub fn get_org_variable(
        &self,
        org: &str,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetOrgVariable<'_>,
        <endpoints::GetOrgVariable<'_> as Endpoint>::Response,
    > {
        endpoints::GetOrgVariable { org, variablename }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update variable in organization
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub fn update_org_variable(
        &self,
        org: &str,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateOrgVariable<'_>,
        <endpoints::UpdateOrgVariable<'_> as Endpoint>::Response,
    > {
        endpoints::UpdateOrgVariable {
            org,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Create a new variable in organization
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub fn create_org_variable(
        &self,
        org: &str,
        variablename: &str,
        body: CreateVariableOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateOrgVariable<'_>,
        <endpoints::CreateOrgVariable<'_> as Endpoint>::Response,
    > {
        endpoints::CreateOrgVariable {
            org,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Delete organization's variable by name
    ///
    /// - `org`: name of the organization
    /// - `variablename`: name of the variable
    pub fn delete_org_variable(
        &self,
        org: &str,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeleteOrgVariable<'_>,
        <endpoints::DeleteOrgVariable<'_> as Endpoint>::Response,
    > {
        endpoints::DeleteOrgVariable { org, variablename }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List an organization's activity feeds
    ///
    /// - `org`: name of the org
    pub fn org_list_activity_feeds(
        &self,
        org: &str,
        query: OrgListActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListActivityFeeds<'_>,
        <endpoints::OrgListActivityFeeds<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListActivityFeeds { org, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update an organization's avatar
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`UpdateUserAvatarOption`]
    pub fn org_update_avatar(
        &self,
        org: &str,
        body: UpdateUserAvatarOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgUpdateAvatar<'_>,
        <endpoints::OrgUpdateAvatar<'_> as Endpoint>::Response,
    > {
        endpoints::OrgUpdateAvatar { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete an organization's avatar. It will be replaced by a default one
    ///
    /// - `org`: name of the organization
    pub fn org_delete_avatar(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgDeleteAvatar<'_>,
        <endpoints::OrgDeleteAvatar<'_> as Endpoint>::Response,
    > {
        endpoints::OrgDeleteAvatar { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Blocks a user from the organization
    ///
    /// - `org`: name of the org
    /// - `username`: username of the user
    pub fn org_block_user(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgBlockUser<'_>,
        <endpoints::OrgBlockUser<'_> as Endpoint>::Response,
    > {
        endpoints::OrgBlockUser { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List an organization's webhooks
    ///
    /// - `org`: name of the organization
    pub fn org_list_hooks(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListHooks<'_>,
        <endpoints::OrgListHooks<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListHooks { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a hook
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateHookOption`]
    pub fn org_create_hook(
        &self,
        org: &str,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgCreateHook<'_>,
        <endpoints::OrgCreateHook<'_> as Endpoint>::Response,
    > {
        endpoints::OrgCreateHook { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to get
    pub fn org_get_hook(
        &self,
        org: &str,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetHook<'_>,
        <endpoints::OrgGetHook<'_> as Endpoint>::Response,
    > {
        endpoints::OrgGetHook { org, id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to delete
    pub fn org_delete_hook(
        &self,
        org: &str,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgDeleteHook<'_>,
        <endpoints::OrgDeleteHook<'_> as Endpoint>::Response,
    > {
        endpoints::OrgDeleteHook { org, id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update a hook
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub fn org_edit_hook(
        &self,
        org: &str,
        id: i64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgEditHook<'_>,
        <endpoints::OrgEditHook<'_> as Endpoint>::Response,
    > {
        endpoints::OrgEditHook {
            org,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List an organization's labels
    ///
    /// - `org`: name of the organization
    pub fn org_list_labels(
        &self,
        org: &str,
        query: OrgListLabelsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListLabels<'_>,
        <endpoints::OrgListLabels<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListLabels { org, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a label for an organization
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateLabelOption`]
    pub fn org_create_label(
        &self,
        org: &str,
        body: CreateLabelOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgCreateLabel<'_>,
        <endpoints::OrgCreateLabel<'_> as Endpoint>::Response,
    > {
        endpoints::OrgCreateLabel { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a single label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to get
    pub fn org_get_label(
        &self,
        org: &str,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetLabel<'_>,
        <endpoints::OrgGetLabel<'_> as Endpoint>::Response,
    > {
        endpoints::OrgGetLabel { org, id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to delete
    pub fn org_delete_label(
        &self,
        org: &str,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgDeleteLabel<'_>,
        <endpoints::OrgDeleteLabel<'_> as Endpoint>::Response,
    > {
        endpoints::OrgDeleteLabel { org, id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update a label
    ///
    /// - `org`: name of the organization
    /// - `id`: id of the label to edit
    /// - `body`: See [`EditLabelOption`]
    pub fn org_edit_label(
        &self,
        org: &str,
        id: i64,
        body: EditLabelOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgEditLabel<'_>,
        <endpoints::OrgEditLabel<'_> as Endpoint>::Response,
    > {
        endpoints::OrgEditLabel {
            org,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List the organization's blocked users
    ///
    /// - `org`: name of the org
    pub fn org_list_blocked_users(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListBlockedUsers<'_>,
        <endpoints::OrgListBlockedUsers<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListBlockedUsers { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List an organization's members
    ///
    /// - `org`: name of the organization
    pub fn org_list_members(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListMembers<'_>,
        <endpoints::OrgListMembers<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListMembers { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if a user is a member of an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_is_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgIsMember<'_>,
        <endpoints::OrgIsMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgIsMember { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Remove a member from an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_delete_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgDeleteMember<'_>,
        <endpoints::OrgDeleteMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgDeleteMember { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List an organization's public members
    ///
    /// - `org`: name of the organization
    pub fn org_list_public_members(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListPublicMembers<'_>,
        <endpoints::OrgListPublicMembers<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListPublicMembers { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if a user is a public member of an organization
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_is_public_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgIsPublicMember<'_>,
        <endpoints::OrgIsPublicMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgIsPublicMember { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Publicize a user's membership
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_publicize_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgPublicizeMember<'_>,
        <endpoints::OrgPublicizeMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgPublicizeMember { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Conceal a user's membership
    ///
    /// - `org`: name of the organization
    /// - `username`: username of the user
    pub fn org_conceal_member(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgConcealMember<'_>,
        <endpoints::OrgConcealMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgConcealMember { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get quota information for an organization
    ///
    /// - `org`: name of the organization
    pub fn org_get_quota(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetQuota<'_>,
        <endpoints::OrgGetQuota<'_> as Endpoint>::Response,
    > {
        endpoints::OrgGetQuota { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the artifacts affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub fn org_list_quota_artifacts(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListQuotaArtifacts<'_>,
        <endpoints::OrgListQuotaArtifacts<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListQuotaArtifacts { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the attachments affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub fn org_list_quota_attachments(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListQuotaAttachments<'_>,
        <endpoints::OrgListQuotaAttachments<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListQuotaAttachments { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if the organization is over quota for a given subject
    ///
    /// - `org`: name of the organization
    pub fn org_check_quota(
        &self,
        org: &str,
        query: OrgCheckQuotaQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgCheckQuota<'_>,
        <endpoints::OrgCheckQuota<'_> as Endpoint>::Response,
    > {
        endpoints::OrgCheckQuota { org, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the packages affecting the organization's quota
    ///
    /// - `org`: name of the organization
    pub fn org_list_quota_packages(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListQuotaPackages<'_>,
        <endpoints::OrgListQuotaPackages<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListQuotaPackages { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Rename an organization
    ///
    /// - `org`: existing org name
    /// - `body`: See [`RenameOrgOption`]
    pub fn rename_org(
        &self,
        org: &str,
        body: RenameOrgOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RenameOrg<'_>,
        <endpoints::RenameOrg<'_> as Endpoint>::Response,
    > {
        endpoints::RenameOrg { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List an organization's repos
    ///
    /// - `org`: name of the organization
    pub fn org_list_repos(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListRepos<'_>,
        <endpoints::OrgListRepos<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListRepos { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a repository in an organization
    ///
    /// - `org`: name of organization
    /// - `body`: See [`CreateRepoOption`]
    pub fn create_org_repo(
        &self,
        org: &str,
        body: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateOrgRepo<'_>,
        <endpoints::CreateOrgRepo<'_> as Endpoint>::Response,
    > {
        endpoints::CreateOrgRepo { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List an organization's teams
    ///
    /// - `org`: name of the organization
    pub fn org_list_teams(
        &self,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListTeams<'_>,
        <endpoints::OrgListTeams<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListTeams { org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a team
    ///
    /// - `org`: name of the organization
    /// - `body`: See [`CreateTeamOption`]
    pub fn org_create_team(
        &self,
        org: &str,
        body: CreateTeamOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgCreateTeam<'_>,
        <endpoints::OrgCreateTeam<'_> as Endpoint>::Response,
    > {
        endpoints::OrgCreateTeam { org, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search for teams within an organization
    ///
    /// - `org`: name of the organization
    pub fn team_search(
        &self,
        org: &str,
        query: TeamSearchQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::TeamSearch<'_>,
        <endpoints::TeamSearch<'_> as Endpoint>::Response,
    > {
        endpoints::TeamSearch { org, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Unblock a user from the organization
    ///
    /// - `org`: name of the org
    /// - `username`: username of the user
    pub fn org_unblock_user(
        &self,
        org: &str,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgUnblockUser<'_>,
        <endpoints::OrgUnblockUser<'_> as Endpoint>::Response,
    > {
        endpoints::OrgUnblockUser { org, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Gets all packages of an owner
    ///
    /// - `owner`: owner of the packages
    pub fn list_packages(
        &self,
        owner: &str,
        query: ListPackagesQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListPackages<'_>,
        <endpoints::ListPackages<'_> as Endpoint>::Response,
    > {
        endpoints::ListPackages { owner, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Link a package to a repository
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    /// - `repo_name`: name of the repository to link.
    pub fn link_package(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
        repo_name: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::LinkPackage<'_>,
        <endpoints::LinkPackage<'_> as Endpoint>::Response,
    > {
        endpoints::LinkPackage {
            owner,
            r#type,
            name,
            repo_name,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Unlink a package from a repository
    ///
    /// - `owner`: owner of the package
    /// - `type`: type of the package
    /// - `name`: name of the package
    pub fn unlink_package(
        &self,
        owner: &str,
        r#type: &str,
        name: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UnlinkPackage<'_>,
        <endpoints::UnlinkPackage<'_> as Endpoint>::Response,
    > {
        endpoints::UnlinkPackage {
            owner,
            r#type,
            name,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetPackage<'_>,
        <endpoints::GetPackage<'_> as Endpoint>::Response,
    > {
        endpoints::GetPackage {
            owner,
            r#type,
            name,
            version,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeletePackage<'_>,
        <endpoints::DeletePackage<'_> as Endpoint>::Response,
    > {
        endpoints::DeletePackage {
            owner,
            r#type,
            name,
            version,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListPackageFiles<'_>,
        <endpoints::ListPackageFiles<'_> as Endpoint>::Response,
    > {
        endpoints::ListPackageFiles {
            owner,
            r#type,
            name,
            version,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Search for issues across the repositories that the user has access to
    ///
    pub fn issue_search_issues(
        &self,
        query: IssueSearchIssuesQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueSearchIssues,
        <endpoints::IssueSearchIssues as Endpoint>::Response,
    > {
        endpoints::IssueSearchIssues { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Migrate a remote git repository
    ///
    /// - `body`: See [`MigrateRepoOptions`]
    pub fn repo_migrate(
        &self,
        body: MigrateRepoOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoMigrate,
        <endpoints::RepoMigrate as Endpoint>::Response,
    > {
        endpoints::RepoMigrate { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search for repositories
    ///
    pub fn repo_search(
        &self,
        query: RepoSearchQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSearch,
        <endpoints::RepoSearch as Endpoint>::Response,
    > {
        endpoints::RepoSearch { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGet<'_>,
        <endpoints::RepoGet<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGet { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a repository
    ///
    /// - `owner`: owner of the repo to delete
    /// - `repo`: name of the repo to delete
    pub fn repo_delete(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDelete<'_>,
        <endpoints::RepoDelete<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDelete { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEdit<'_>,
        <endpoints::RepoEdit<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEdit {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Search for repository's action jobs according filter conditions
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_search_run_jobs(
        &self,
        owner: &str,
        repo: &str,
        query: RepoSearchRunJobsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSearchRunJobs<'_>,
        <endpoints::RepoSearchRunJobs<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSearchRunJobs { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a repository's actions runner registration token
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_runner_registration_token(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetRunnerRegistrationToken<'_>,
        <endpoints::RepoGetRunnerRegistrationToken<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetRunnerRegistrationToken { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a repository's action runs
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn list_action_runs(
        &self,
        owner: &str,
        repo: &str,
        query: ListActionRunsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListActionRuns<'_>,
        <endpoints::ListActionRuns<'_> as Endpoint>::Response,
    > {
        endpoints::ListActionRuns { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get an action run
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `run_id`: id of the action run
    pub fn get_action_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetActionRun<'_>,
        <endpoints::GetActionRun<'_> as Endpoint>::Response,
    > {
        endpoints::GetActionRun {
            owner,
            repo,
            run_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List an repo's actions secrets
    ///
    /// - `owner`: owner of the repository
    /// - `repo`: name of the repository
    pub fn repo_list_actions_secrets(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListActionsSecrets<'_>,
        <endpoints::RepoListActionsSecrets<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListActionsSecrets { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateRepoSecret<'_>,
        <endpoints::UpdateRepoSecret<'_> as Endpoint>::Response,
    > {
        endpoints::UpdateRepoSecret {
            owner,
            repo,
            secretname,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeleteRepoSecret<'_>,
        <endpoints::DeleteRepoSecret<'_> as Endpoint>::Response,
    > {
        endpoints::DeleteRepoSecret {
            owner,
            repo,
            secretname,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repository's action tasks
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn list_action_tasks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListActionTasks<'_>,
        <endpoints::ListActionTasks<'_> as Endpoint>::Response,
    > {
        endpoints::ListActionTasks { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get repo-level variables list
    ///
    /// - `owner`: name of the owner
    /// - `repo`: name of the repository
    pub fn get_repo_variables_list(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetRepoVariablesList<'_>,
        <endpoints::GetRepoVariablesList<'_> as Endpoint>::Response,
    > {
        endpoints::GetRepoVariablesList { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetRepoVariable<'_>,
        <endpoints::GetRepoVariable<'_> as Endpoint>::Response,
    > {
        endpoints::GetRepoVariable {
            owner,
            repo,
            variablename,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateRepoVariable<'_>,
        <endpoints::UpdateRepoVariable<'_> as Endpoint>::Response,
    > {
        endpoints::UpdateRepoVariable {
            owner,
            repo,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateRepoVariable<'_>,
        <endpoints::CreateRepoVariable<'_> as Endpoint>::Response,
    > {
        endpoints::CreateRepoVariable {
            owner,
            repo,
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeleteRepoVariable<'_>,
        <endpoints::DeleteRepoVariable<'_> as Endpoint>::Response,
    > {
        endpoints::DeleteRepoVariable {
            owner,
            repo,
            variablename,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Dispatches a workflow
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `workflowfilename`: name of the workflow
    /// - `body`: See [`DispatchWorkflowOption`]
    pub fn dispatch_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflowfilename: &str,
        body: DispatchWorkflowOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DispatchWorkflow<'_>,
        <endpoints::DispatchWorkflow<'_> as Endpoint>::Response,
    > {
        endpoints::DispatchWorkflow {
            owner,
            repo,
            workflowfilename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListActivityFeeds<'_>,
        <endpoints::RepoListActivityFeeds<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListActivityFeeds { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetArchive<'_>,
        <endpoints::RepoGetArchive<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetArchive {
            owner,
            repo,
            archive,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Return all users that have write access and can be assigned to issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetAssignees<'_>,
        <endpoints::RepoGetAssignees<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetAssignees { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update a repository's avatar
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`UpdateRepoAvatarOption`]
    pub fn repo_update_avatar(
        &self,
        owner: &str,
        repo: &str,
        body: UpdateRepoAvatarOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoUpdateAvatar<'_>,
        <endpoints::RepoUpdateAvatar<'_> as Endpoint>::Response,
    > {
        endpoints::RepoUpdateAvatar {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Delete a repository's avatar
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_delete_avatar(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteAvatar<'_>,
        <endpoints::RepoDeleteAvatar<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteAvatar { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List branch protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_branch_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListBranchProtection<'_>,
        <endpoints::RepoListBranchProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListBranchProtection { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateBranchProtection<'_>,
        <endpoints::RepoCreateBranchProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateBranchProtection {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetBranchProtection<'_>,
        <endpoints::RepoGetBranchProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetBranchProtection { owner, repo, name }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteBranchProtection<'_>,
        <endpoints::RepoDeleteBranchProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteBranchProtection { owner, repo, name }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditBranchProtection<'_>,
        <endpoints::RepoEditBranchProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditBranchProtection {
            owner,
            repo,
            name,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repository's branches
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_branches(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListBranches<'_>,
        <endpoints::RepoListBranches<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListBranches { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateBranch<'_>,
        <endpoints::RepoCreateBranch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateBranch {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetBranch<'_>,
        <endpoints::RepoGetBranch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetBranch {
            owner,
            repo,
            branch,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteBranch<'_>,
        <endpoints::RepoDeleteBranch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteBranch {
            owner,
            repo,
            branch,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoUpdateBranch<'_>,
        <endpoints::RepoUpdateBranch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoUpdateBranch {
            owner,
            repo,
            branch,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repository's collaborators
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_collaborators(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListCollaborators<'_>,
        <endpoints::RepoListCollaborators<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListCollaborators { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCheckCollaborator<'_>,
        <endpoints::RepoCheckCollaborator<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCheckCollaborator {
            owner,
            repo,
            collaborator,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoAddCollaborator<'_>,
        <endpoints::RepoAddCollaborator<'_> as Endpoint>::Response,
    > {
        endpoints::RepoAddCollaborator {
            owner,
            repo,
            collaborator,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteCollaborator<'_>,
        <endpoints::RepoDeleteCollaborator<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteCollaborator {
            owner,
            repo,
            collaborator,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetRepoPermissions<'_>,
        <endpoints::RepoGetRepoPermissions<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetRepoPermissions {
            owner,
            repo,
            collaborator,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetAllCommits<'_>,
        <endpoints::RepoGetAllCommits<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetAllCommits { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetCombinedStatusByRef<'_>,
        <endpoints::RepoGetCombinedStatusByRef<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetCombinedStatusByRef { owner, repo, r#ref }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListStatusesByRef<'_>,
        <endpoints::RepoListStatusesByRef<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListStatusesByRef {
            owner,
            repo,
            r#ref,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetCommitPullRequest<'_>,
        <endpoints::RepoGetCommitPullRequest<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetCommitPullRequest { owner, repo, sha }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCompareDiff<'_>,
        <endpoints::RepoCompareDiff<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCompareDiff {
            owner,
            repo,
            basehead,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetContentsList<'_>,
        <endpoints::RepoGetContentsList<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetContentsList { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoChangeFiles<'_>,
        <endpoints::RepoChangeFiles<'_> as Endpoint>::Response,
    > {
        endpoints::RepoChangeFiles {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetContents<'_>,
        <endpoints::RepoGetContents<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetContents {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoUpdateFile<'_>,
        <endpoints::RepoUpdateFile<'_> as Endpoint>::Response,
    > {
        endpoints::RepoUpdateFile {
            owner,
            repo,
            filepath,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateFile<'_>,
        <endpoints::RepoCreateFile<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateFile {
            owner,
            repo,
            filepath,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteFile<'_>,
        <endpoints::RepoDeleteFile<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteFile {
            owner,
            repo,
            filepath,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Convert a mirror repo to a normal repo.
    ///
    /// - `owner`: owner of the repo to convert
    /// - `repo`: name of the repo to convert
    pub fn repo_convert(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoConvert<'_>,
        <endpoints::RepoConvert<'_> as Endpoint>::Response,
    > {
        endpoints::RepoConvert { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoApplyDiffPatch<'_>,
        <endpoints::RepoApplyDiffPatch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoApplyDiffPatch {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetEditorConfig<'_>,
        <endpoints::RepoGetEditorConfig<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetEditorConfig {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repository's flags
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_flags(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListFlags<'_>,
        <endpoints::RepoListFlags<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListFlags { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoReplaceAllFlags<'_>,
        <endpoints::RepoReplaceAllFlags<'_> as Endpoint>::Response,
    > {
        endpoints::RepoReplaceAllFlags {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Remove all flags from a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_delete_all_flags(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteAllFlags<'_>,
        <endpoints::RepoDeleteAllFlags<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteAllFlags { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCheckFlag<'_>,
        <endpoints::RepoCheckFlag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCheckFlag { owner, repo, flag }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoAddFlag<'_>,
        <endpoints::RepoAddFlag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoAddFlag { owner, repo, flag }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteFlag<'_>,
        <endpoints::RepoDeleteFlag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteFlag { owner, repo, flag }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a repository's forks
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn list_forks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::ListForks<'_>,
        <endpoints::ListForks<'_> as Endpoint>::Response,
    > {
        endpoints::ListForks { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateFork<'_>,
        <endpoints::CreateFork<'_> as Endpoint>::Response,
    > {
        endpoints::CreateFork {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Gets multiplbe blobs of a repository.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn get_blobs(
        &self,
        owner: &str,
        repo: &str,
        query: GetBlobsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetBlobs<'_>,
        <endpoints::GetBlobs<'_> as Endpoint>::Response,
    > {
        endpoints::GetBlobs { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Gets the blob of a repository.
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `sha`: sha of the blob to retrieve
    pub fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetBlob<'_>,
        <endpoints::GetBlob<'_> as Endpoint>::Response,
    > {
        endpoints::GetBlob { owner, repo, sha }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetSingleCommit<'_>,
        <endpoints::RepoGetSingleCommit<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetSingleCommit {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDownloadCommitDiffOrPatch<'_>,
        <endpoints::RepoDownloadCommitDiffOrPatch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDownloadCommitDiffOrPatch {
            owner,
            repo,
            sha,
            diff_type,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetNote<'_>,
        <endpoints::RepoGetNote<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetNote {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSetNote<'_>,
        <endpoints::RepoSetNote<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSetNote {
            owner,
            repo,
            sha,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoRemoveNote<'_>,
        <endpoints::RepoRemoveNote<'_> as Endpoint>::Response,
    > {
        endpoints::RepoRemoveNote { owner, repo, sha }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get specified ref or filtered repository's refs
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_all_git_refs(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListAllGitRefs<'_>,
        <endpoints::RepoListAllGitRefs<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListAllGitRefs { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListGitRefs<'_>,
        <endpoints::RepoListGitRefs<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListGitRefs { owner, repo, r#ref }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetAnnotatedTag<'_>,
        <endpoints::GetAnnotatedTag<'_> as Endpoint>::Response,
    > {
        endpoints::GetAnnotatedTag { owner, repo, sha }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetTree<'_>,
        <endpoints::GetTree<'_> as Endpoint>::Response,
    > {
        endpoints::GetTree {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List the hooks in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_hooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListHooks<'_>,
        <endpoints::RepoListHooks<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListHooks { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateHook<'_>,
        <endpoints::RepoCreateHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateHook {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List the Git hooks in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_git_hooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListGitHooks<'_>,
        <endpoints::RepoListGitHooks<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListGitHooks { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetGitHook<'_>,
        <endpoints::RepoGetGitHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetGitHook { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteGitHook<'_>,
        <endpoints::RepoDeleteGitHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteGitHook { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditGitHook<'_>,
        <endpoints::RepoEditGitHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditGitHook {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetHook<'_>,
        <endpoints::RepoGetHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetHook { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteHook<'_>,
        <endpoints::RepoDeleteHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteHook { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditHook<'_>,
        <endpoints::RepoEditHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditHook {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        query: RepoTestHookQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoTestHook<'_>,
        <endpoints::RepoTestHook<'_> as Endpoint>::Response,
    > {
        endpoints::RepoTestHook {
            owner,
            repo,
            id,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Returns the issue config for a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_issue_config(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetIssueConfig<'_>,
        <endpoints::RepoGetIssueConfig<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetIssueConfig { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns the validation information for a issue config
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_validate_issue_config(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoValidateIssueConfig<'_>,
        <endpoints::RepoValidateIssueConfig<'_> as Endpoint>::Response,
    > {
        endpoints::RepoValidateIssueConfig { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get available issue templates for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_issue_templates(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetIssueTemplates<'_>,
        <endpoints::RepoGetIssueTemplates<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetIssueTemplates { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueListIssues<'_>,
        <endpoints::IssueListIssues<'_> as Endpoint>::Response,
    > {
        endpoints::IssueListIssues { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateIssue<'_>,
        <endpoints::IssueCreateIssue<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateIssue {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetRepoComments<'_>,
        <endpoints::IssueGetRepoComments<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetRepoComments { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetComment<'_>,
        <endpoints::IssueGetComment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetComment { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteComment<'_>,
        <endpoints::IssueDeleteComment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteComment { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditIssueCommentOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditComment<'_>,
        <endpoints::IssueEditComment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditComment {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueListIssueCommentAttachments<'_>,
        <endpoints::IssueListIssueCommentAttachments<'_> as Endpoint>::Response,
    > {
        endpoints::IssueListIssueCommentAttachments { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment: &[u8],
        query: IssueCreateIssueCommentAttachmentQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateIssueCommentAttachment<'_>,
        <endpoints::IssueCreateIssueCommentAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment: &attachment,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetIssueCommentAttachment<'_>,
        <endpoints::IssueGetIssueCommentAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteIssueCommentAttachment<'_>,
        <endpoints::IssueDeleteIssueCommentAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment_id: i64,
        body: EditAttachmentOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditIssueCommentAttachment<'_>,
        <endpoints::IssueEditIssueCommentAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditIssueCommentAttachment {
            owner,
            repo,
            id,
            attachment_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetCommentReactions<'_>,
        <endpoints::IssueGetCommentReactions<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetCommentReactions { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssuePostCommentReaction<'_>,
        <endpoints::IssuePostCommentReaction<'_> as Endpoint>::Response,
    > {
        endpoints::IssuePostCommentReaction {
            owner,
            repo,
            id,
            body: content,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteCommentReaction<'_>,
        <endpoints::IssueDeleteCommentReaction<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteCommentReaction {
            owner,
            repo,
            id,
            body: content,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repo's pinned issues
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_pinned_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListPinnedIssues<'_>,
        <endpoints::RepoListPinnedIssues<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListPinnedIssues { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetIssue<'_>,
        <endpoints::IssueGetIssue<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetIssue { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDelete<'_>,
        <endpoints::IssueDelete<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDelete { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: EditIssueOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditIssue<'_>,
        <endpoints::IssueEditIssue<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditIssue {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueListIssueAttachments<'_>,
        <endpoints::IssueListIssueAttachments<'_> as Endpoint>::Response,
    > {
        endpoints::IssueListIssueAttachments { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        attachment: &[u8],
        query: IssueCreateIssueAttachmentQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateIssueAttachment<'_>,
        <endpoints::IssueCreateIssueAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateIssueAttachment {
            owner,
            repo,
            index,
            attachment: &attachment,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        attachment_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetIssueAttachment<'_>,
        <endpoints::IssueGetIssueAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetIssueAttachment {
            owner,
            repo,
            index,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        attachment_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteIssueAttachment<'_>,
        <endpoints::IssueDeleteIssueAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteIssueAttachment {
            owner,
            repo,
            index,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        attachment_id: i64,
        body: EditAttachmentOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditIssueAttachment<'_>,
        <endpoints::IssueEditIssueAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditIssueAttachment {
            owner,
            repo,
            index,
            attachment_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueListBlocks<'_>,
        <endpoints::IssueListBlocks<'_> as Endpoint>::Response,
    > {
        endpoints::IssueListBlocks { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateIssueBlocking<'_>,
        <endpoints::IssueCreateIssueBlocking<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateIssueBlocking {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueRemoveIssueBlocking<'_>,
        <endpoints::IssueRemoveIssueBlocking<'_> as Endpoint>::Response,
    > {
        endpoints::IssueRemoveIssueBlocking {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        query: IssueGetCommentsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetComments<'_>,
        <endpoints::IssueGetComments<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetComments {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: CreateIssueCommentOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateComment<'_>,
        <endpoints::IssueCreateComment<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateComment {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteCommentDeprecated<'_>,
        <endpoints::IssueDeleteCommentDeprecated<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteCommentDeprecated {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditIssueCommentOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditCommentDeprecated<'_>,
        <endpoints::IssueEditCommentDeprecated<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditCommentDeprecated {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: EditDeadlineOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditIssueDeadline<'_>,
        <endpoints::IssueEditIssueDeadline<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditIssueDeadline {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueListIssueDependencies<'_>,
        <endpoints::IssueListIssueDependencies<'_> as Endpoint>::Response,
    > {
        endpoints::IssueListIssueDependencies { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateIssueDependencies<'_>,
        <endpoints::IssueCreateIssueDependencies<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateIssueDependencies {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: IssueMeta,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueRemoveIssueDependencies<'_>,
        <endpoints::IssueRemoveIssueDependencies<'_> as Endpoint>::Response,
    > {
        endpoints::IssueRemoveIssueDependencies {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetLabels<'_>,
        <endpoints::IssueGetLabels<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetLabels { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: IssueLabelsOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueReplaceLabels<'_>,
        <endpoints::IssueReplaceLabels<'_> as Endpoint>::Response,
    > {
        endpoints::IssueReplaceLabels {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: IssueLabelsOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueAddLabel<'_>,
        <endpoints::IssueAddLabel<'_> as Endpoint>::Response,
    > {
        endpoints::IssueAddLabel {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: DeleteLabelsOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueClearLabels<'_>,
        <endpoints::IssueClearLabels<'_> as Endpoint>::Response,
    > {
        endpoints::IssueClearLabels {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Remove a label from an issue
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the issue
    /// - `identifier`: name or id of the label to remove
    /// - `body`: See [`DeleteLabelsOption`]
    pub fn issue_remove_label(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        identifier: &str,
        body: DeleteLabelsOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueRemoveLabel<'_>,
        <endpoints::IssueRemoveLabel<'_> as Endpoint>::Response,
    > {
        endpoints::IssueRemoveLabel {
            owner,
            repo,
            index,
            identifier,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::PinIssue<'_>,
        <endpoints::PinIssue<'_> as Endpoint>::Response,
    > {
        endpoints::PinIssue { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UnpinIssue<'_>,
        <endpoints::UnpinIssue<'_> as Endpoint>::Response,
    > {
        endpoints::UnpinIssue { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        position: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::MoveIssuePin<'_>,
        <endpoints::MoveIssuePin<'_> as Endpoint>::Response,
    > {
        endpoints::MoveIssuePin {
            owner,
            repo,
            index,
            position,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetIssueReactions<'_>,
        <endpoints::IssueGetIssueReactions<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetIssueReactions { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssuePostIssueReaction<'_>,
        <endpoints::IssuePostIssueReaction<'_> as Endpoint>::Response,
    > {
        endpoints::IssuePostIssueReaction {
            owner,
            repo,
            index,
            body: content,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        content: EditReactionOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteIssueReaction<'_>,
        <endpoints::IssueDeleteIssueReaction<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteIssueReaction {
            owner,
            repo,
            index,
            body: content,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteStopWatch<'_>,
        <endpoints::IssueDeleteStopWatch<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteStopWatch { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueStartStopWatch<'_>,
        <endpoints::IssueStartStopWatch<'_> as Endpoint>::Response,
    > {
        endpoints::IssueStartStopWatch { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueStopStopWatch<'_>,
        <endpoints::IssueStopStopWatch<'_> as Endpoint>::Response,
    > {
        endpoints::IssueStopStopWatch { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueSubscriptions<'_>,
        <endpoints::IssueSubscriptions<'_> as Endpoint>::Response,
    > {
        endpoints::IssueSubscriptions { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCheckSubscription<'_>,
        <endpoints::IssueCheckSubscription<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCheckSubscription { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        user: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueAddSubscription<'_>,
        <endpoints::IssueAddSubscription<'_> as Endpoint>::Response,
    > {
        endpoints::IssueAddSubscription {
            owner,
            repo,
            index,
            user,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        user: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteSubscription<'_>,
        <endpoints::IssueDeleteSubscription<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteSubscription {
            owner,
            repo,
            index,
            user,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        query: IssueGetCommentsAndTimelineQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetCommentsAndTimeline<'_>,
        <endpoints::IssueGetCommentsAndTimeline<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetCommentsAndTimeline {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        query: IssueTrackedTimesQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueTrackedTimes<'_>,
        <endpoints::IssueTrackedTimes<'_> as Endpoint>::Response,
    > {
        endpoints::IssueTrackedTimes {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        body: AddTimeOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueAddTime<'_>,
        <endpoints::IssueAddTime<'_> as Endpoint>::Response,
    > {
        endpoints::IssueAddTime {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueResetTime<'_>,
        <endpoints::IssueResetTime<'_> as Endpoint>::Response,
    > {
        endpoints::IssueResetTime { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteTime<'_>,
        <endpoints::IssueDeleteTime<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteTime {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListKeys<'_>,
        <endpoints::RepoListKeys<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListKeys { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateKey<'_>,
        <endpoints::RepoCreateKey<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateKey {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetKey<'_>,
        <endpoints::RepoGetKey<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetKey { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteKey<'_>,
        <endpoints::RepoDeleteKey<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteKey { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get all of a repository's labels
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn issue_list_labels(
        &self,
        owner: &str,
        repo: &str,
        query: IssueListLabelsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueListLabels<'_>,
        <endpoints::IssueListLabels<'_> as Endpoint>::Response,
    > {
        endpoints::IssueListLabels { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateLabel<'_>,
        <endpoints::IssueCreateLabel<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateLabel {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetLabel<'_>,
        <endpoints::IssueGetLabel<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetLabel { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteLabel<'_>,
        <endpoints::IssueDeleteLabel<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteLabel { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditLabelOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditLabel<'_>,
        <endpoints::IssueEditLabel<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditLabel {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Get languages and number of bytes of code written
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetLanguages<'_>,
        <endpoints::RepoGetLanguages<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetLanguages { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetRawFileOrLfs<'_>,
        <endpoints::RepoGetRawFileOrLfs<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetRawFileOrLfs {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetMilestonesList<'_>,
        <endpoints::IssueGetMilestonesList<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetMilestonesList { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueCreateMilestone<'_>,
        <endpoints::IssueCreateMilestone<'_> as Endpoint>::Response,
    > {
        endpoints::IssueCreateMilestone {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueGetMilestone<'_>,
        <endpoints::IssueGetMilestone<'_> as Endpoint>::Response,
    > {
        endpoints::IssueGetMilestone { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueDeleteMilestone<'_>,
        <endpoints::IssueDeleteMilestone<'_> as Endpoint>::Response,
    > {
        endpoints::IssueDeleteMilestone { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditMilestoneOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::IssueEditMilestone<'_>,
        <endpoints::IssueEditMilestone<'_> as Endpoint>::Response,
    > {
        endpoints::IssueEditMilestone {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Sync a mirrored repository
    ///
    /// - `owner`: owner of the repo to sync
    /// - `repo`: name of the repo to sync
    pub fn repo_mirror_sync(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoMirrorSync<'_>,
        <endpoints::RepoMirrorSync<'_> as Endpoint>::Response,
    > {
        endpoints::RepoMirrorSync { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns if new Issue Pins are allowed
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_new_pin_allowed(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoNewPinAllowed<'_>,
        <endpoints::RepoNewPinAllowed<'_> as Endpoint>::Response,
    > {
        endpoints::RepoNewPinAllowed { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyGetRepoList<'_>,
        <endpoints::NotifyGetRepoList<'_> as Endpoint>::Response,
    > {
        endpoints::NotifyGetRepoList { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::NotifyReadRepoList<'_>,
        <endpoints::NotifyReadRepoList<'_> as Endpoint>::Response,
    > {
        endpoints::NotifyReadRepoList { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a repo's pull requests. If a pull request is selected but fails to be retrieved for any reason, it will be a null value in the list of results.
    ///
    /// - `owner`: Owner of the repo
    /// - `repo`: Name of the repo
    pub fn repo_list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        query: RepoListPullRequestsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListPullRequests<'_>,
        <endpoints::RepoListPullRequests<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListPullRequests { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreatePullRequest<'_>,
        <endpoints::RepoCreatePullRequest<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreatePullRequest {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repo's pinned pull requests
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_pinned_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListPinnedPullRequests<'_>,
        <endpoints::RepoListPinnedPullRequests<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListPinnedPullRequests { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullRequestByBaseHead<'_>,
        <endpoints::RepoGetPullRequestByBaseHead<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullRequestByBaseHead {
            owner,
            repo,
            base,
            head,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullRequest<'_>,
        <endpoints::RepoGetPullRequest<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullRequest { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: EditPullRequestOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditPullRequest<'_>,
        <endpoints::RepoEditPullRequest<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditPullRequest {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        diff_type: &str,
        query: RepoDownloadPullDiffOrPatchQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDownloadPullDiffOrPatch<'_>,
        <endpoints::RepoDownloadPullDiffOrPatch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDownloadPullDiffOrPatch {
            owner,
            repo,
            index,
            diff_type,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        query: RepoGetPullRequestCommitsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullRequestCommits<'_>,
        <endpoints::RepoGetPullRequestCommits<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullRequestCommits {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        query: RepoGetPullRequestFilesQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullRequestFiles<'_>,
        <endpoints::RepoGetPullRequestFiles<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullRequestFiles {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoPullRequestIsMerged<'_>,
        <endpoints::RepoPullRequestIsMerged<'_> as Endpoint>::Response,
    > {
        endpoints::RepoPullRequestIsMerged { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: MergePullRequestOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoMergePullRequest<'_>,
        <endpoints::RepoMergePullRequest<'_> as Endpoint>::Response,
    > {
        endpoints::RepoMergePullRequest {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCancelScheduledAutoMerge<'_>,
        <endpoints::RepoCancelScheduledAutoMerge<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCancelScheduledAutoMerge { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create review requests for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`PullReviewRequestOptions`]
    pub fn repo_create_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: PullReviewRequestOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreatePullReviewRequests<'_>,
        <endpoints::RepoCreatePullReviewRequests<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreatePullReviewRequests {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Cancel review requests for a pull request
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `index`: index of the pull request
    /// - `body`: See [`PullReviewRequestOptions`]
    pub fn repo_delete_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: PullReviewRequestOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeletePullReviewRequests<'_>,
        <endpoints::RepoDeletePullReviewRequests<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeletePullReviewRequests {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListPullReviews<'_>,
        <endpoints::RepoListPullReviews<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListPullReviews { owner, repo, index }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        index: i64,
        body: CreatePullReviewOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreatePullReview<'_>,
        <endpoints::RepoCreatePullReview<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreatePullReview {
            owner,
            repo,
            index,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullReview<'_>,
        <endpoints::RepoGetPullReview<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullReview {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
        body: SubmitPullReviewOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSubmitPullReview<'_>,
        <endpoints::RepoSubmitPullReview<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSubmitPullReview {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeletePullReview<'_>,
        <endpoints::RepoDeletePullReview<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeletePullReview {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullReviewComments<'_>,
        <endpoints::RepoGetPullReviewComments<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullReviewComments {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
        body: serde_json::Value,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreatePullReviewComment<'_>,
        <endpoints::RepoCreatePullReviewComment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreatePullReviewComment {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
        comment: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPullReviewComment<'_>,
        <endpoints::RepoGetPullReviewComment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPullReviewComment {
            owner,
            repo,
            index,
            id,
            comment,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
        comment: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeletePullReviewComment<'_>,
        <endpoints::RepoDeletePullReviewComment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeletePullReviewComment {
            owner,
            repo,
            index,
            id,
            comment,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
        body: DismissPullReviewOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDismissPullReview<'_>,
        <endpoints::RepoDismissPullReview<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDismissPullReview {
            owner,
            repo,
            index,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoUnDismissPullReview<'_>,
        <endpoints::RepoUnDismissPullReview<'_> as Endpoint>::Response,
    > {
        endpoints::RepoUnDismissPullReview {
            owner,
            repo,
            index,
            id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        index: i64,
        query: RepoUpdatePullRequestQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoUpdatePullRequest<'_>,
        <endpoints::RepoUpdatePullRequest<'_> as Endpoint>::Response,
    > {
        endpoints::RepoUpdatePullRequest {
            owner,
            repo,
            index,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Get all push mirrors of the repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_push_mirrors(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListPushMirrors<'_>,
        <endpoints::RepoListPushMirrors<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListPushMirrors { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Set up a new push mirror in a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `body`: See [`CreatePushMirrorOption`]
    pub fn repo_add_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        body: CreatePushMirrorOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoAddPushMirror<'_>,
        <endpoints::RepoAddPushMirror<'_> as Endpoint>::Response,
    > {
        endpoints::RepoAddPushMirror {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Sync all push mirrored repository
    ///
    /// - `owner`: owner of the repo to sync
    /// - `repo`: name of the repo to sync
    pub fn repo_push_mirror_sync(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoPushMirrorSync<'_>,
        <endpoints::RepoPushMirrorSync<'_> as Endpoint>::Response,
    > {
        endpoints::RepoPushMirrorSync { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetPushMirrorByRemoteName<'_>,
        <endpoints::RepoGetPushMirrorByRemoteName<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetPushMirrorByRemoteName { owner, repo, name }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Remove a push mirror from a repository by remoteName
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `name`: remote name of the pushMirror
    pub fn repo_delete_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeletePushMirror<'_>,
        <endpoints::RepoDeletePushMirror<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeletePushMirror { owner, repo, name }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetRawFile<'_>,
        <endpoints::RepoGetRawFile<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetRawFile {
            owner,
            repo,
            filepath,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListReleases<'_>,
        <endpoints::RepoListReleases<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListReleases { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateRelease<'_>,
        <endpoints::RepoCreateRelease<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateRelease {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Gets the most recent non-prerelease, non-draft release of a repository, sorted by created_at
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetLatestRelease<'_>,
        <endpoints::RepoGetLatestRelease<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetLatestRelease { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetReleaseByTag<'_>,
        <endpoints::RepoGetReleaseByTag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetReleaseByTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteReleaseByTag<'_>,
        <endpoints::RepoDeleteReleaseByTag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteReleaseByTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetRelease<'_>,
        <endpoints::RepoGetRelease<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetRelease { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteRelease<'_>,
        <endpoints::RepoDeleteRelease<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteRelease { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditReleaseOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditRelease<'_>,
        <endpoints::RepoEditRelease<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditRelease {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListReleaseAttachments<'_>,
        <endpoints::RepoListReleaseAttachments<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListReleaseAttachments { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment: Option<&[u8]>,
        external_url: Option<&str>,
        query: RepoCreateReleaseAttachmentQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateReleaseAttachment<'_>,
        <endpoints::RepoCreateReleaseAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateReleaseAttachment {
            owner,
            repo,
            id,
            attachment: attachment.as_deref(),
            external_url: external_url.as_deref(),
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetReleaseAttachment<'_>,
        <endpoints::RepoGetReleaseAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetReleaseAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment_id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteReleaseAttachment<'_>,
        <endpoints::RepoDeleteReleaseAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteReleaseAttachment {
            owner,
            repo,
            id,
            attachment_id,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
        attachment_id: i64,
        body: EditAttachmentOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditReleaseAttachment<'_>,
        <endpoints::RepoEditReleaseAttachment<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditReleaseAttachment {
            owner,
            repo,
            id,
            attachment_id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Return all users that can be requested to review in this repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_reviewers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetReviewers<'_>,
        <endpoints::RepoGetReviewers<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetReviewers { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get signing-key.gpg for given repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_signing_key(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSigningKey<'_>,
        <endpoints::RepoSigningKey<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSigningKey { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a repo's stargazers
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_stargazers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListStargazers<'_>,
        <endpoints::RepoListStargazers<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListStargazers { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListStatuses<'_>,
        <endpoints::RepoListStatuses<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListStatuses {
            owner,
            repo,
            sha,
            query,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateStatus<'_>,
        <endpoints::RepoCreateStatus<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateStatus {
            owner,
            repo,
            sha,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repo's watchers
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_subscribers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListSubscribers<'_>,
        <endpoints::RepoListSubscribers<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListSubscribers { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if the current user is watching a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_check_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentCheckSubscription<'_>,
        <endpoints::UserCurrentCheckSubscription<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentCheckSubscription { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Watch a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_put_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentPutSubscription<'_>,
        <endpoints::UserCurrentPutSubscription<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentPutSubscription { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Unwatch a repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_delete_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentDeleteSubscription<'_>,
        <endpoints::UserCurrentDeleteSubscription<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentDeleteSubscription { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Gets information about syncing the fork default branch with the base branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_sync_fork_default_info(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSyncForkDefaultInfo<'_>,
        <endpoints::RepoSyncForkDefaultInfo<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSyncForkDefaultInfo { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Syncs the default branch of a fork with the base branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_sync_fork_default(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSyncForkDefault<'_>,
        <endpoints::RepoSyncForkDefault<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSyncForkDefault { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Gets information about syncing a fork branch with the base branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: The branch
    pub fn repo_sync_fork_branch_info(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSyncForkBranchInfo<'_>,
        <endpoints::RepoSyncForkBranchInfo<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSyncForkBranchInfo {
            owner,
            repo,
            branch,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Syncs a fork branch with the base branch
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    /// - `branch`: The branch
    pub fn repo_sync_fork_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoSyncForkBranch<'_>,
        <endpoints::RepoSyncForkBranch<'_> as Endpoint>::Response,
    > {
        endpoints::RepoSyncForkBranch {
            owner,
            repo,
            branch,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List tag protections for a repository
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_tag_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListTagProtection<'_>,
        <endpoints::RepoListTagProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListTagProtection { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateTagProtection<'_>,
        <endpoints::RepoCreateTagProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateTagProtection {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetTagProtection<'_>,
        <endpoints::RepoGetTagProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetTagProtection { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteTagProtection<'_>,
        <endpoints::RepoDeleteTagProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteTagProtection { owner, repo, id }
            .make_request()
            .wrap_sync::<_, _>(self)
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
        id: i64,
        body: EditTagProtectionOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditTagProtection<'_>,
        <endpoints::RepoEditTagProtection<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditTagProtection {
            owner,
            repo,
            id,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// List a repository's tags
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_tags(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListTags<'_>,
        <endpoints::RepoListTags<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListTags { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateTag<'_>,
        <endpoints::RepoCreateTag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateTag {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetTag<'_>,
        <endpoints::RepoGetTag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteTag<'_>,
        <endpoints::RepoDeleteTag<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteTag { owner, repo, tag }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a repository's teams
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListTeams<'_>,
        <endpoints::RepoListTeams<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListTeams { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCheckTeam<'_>,
        <endpoints::RepoCheckTeam<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCheckTeam { owner, repo, team }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoAddTeam<'_>,
        <endpoints::RepoAddTeam<'_> as Endpoint>::Response,
    > {
        endpoints::RepoAddTeam { owner, repo, team }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteTeam<'_>,
        <endpoints::RepoDeleteTeam<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteTeam { owner, repo, team }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoTrackedTimes<'_>,
        <endpoints::RepoTrackedTimes<'_> as Endpoint>::Response,
    > {
        endpoints::RepoTrackedTimes { owner, repo, query }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserTrackedTimes<'_>,
        <endpoints::UserTrackedTimes<'_> as Endpoint>::Response,
    > {
        endpoints::UserTrackedTimes { owner, repo, user }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get list of topics that a repository has
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_list_topics(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoListTopics<'_>,
        <endpoints::RepoListTopics<'_> as Endpoint>::Response,
    > {
        endpoints::RepoListTopics { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoUpdateTopics<'_>,
        <endpoints::RepoUpdateTopics<'_> as Endpoint>::Response,
    > {
        endpoints::RepoUpdateTopics {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoAddTopic<'_>,
        <endpoints::RepoAddTopic<'_> as Endpoint>::Response,
    > {
        endpoints::RepoAddTopic { owner, repo, topic }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteTopic<'_>,
        <endpoints::RepoDeleteTopic<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteTopic { owner, repo, topic }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoTransfer<'_>,
        <endpoints::RepoTransfer<'_> as Endpoint>::Response,
    > {
        endpoints::RepoTransfer {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Accept a repo transfer
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    pub fn accept_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::AcceptRepoTransfer<'_>,
        <endpoints::AcceptRepoTransfer<'_> as Endpoint>::Response,
    > {
        endpoints::AcceptRepoTransfer { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Reject a repo transfer
    ///
    /// - `owner`: owner of the repo to transfer
    /// - `repo`: name of the repo to transfer
    pub fn reject_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RejectRepoTransfer<'_>,
        <endpoints::RejectRepoTransfer<'_> as Endpoint>::Response,
    > {
        endpoints::RejectRepoTransfer { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoCreateWikiPage<'_>,
        <endpoints::RepoCreateWikiPage<'_> as Endpoint>::Response,
    > {
        endpoints::RepoCreateWikiPage {
            owner,
            repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetWikiPage<'_>,
        <endpoints::RepoGetWikiPage<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetWikiPage {
            owner,
            repo,
            page_name,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoDeleteWikiPage<'_>,
        <endpoints::RepoDeleteWikiPage<'_> as Endpoint>::Response,
    > {
        endpoints::RepoDeleteWikiPage {
            owner,
            repo,
            page_name,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoEditWikiPage<'_>,
        <endpoints::RepoEditWikiPage<'_> as Endpoint>::Response,
    > {
        endpoints::RepoEditWikiPage {
            owner,
            repo,
            page_name,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Get all wiki pages
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn repo_get_wiki_pages(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetWikiPages<'_>,
        <endpoints::RepoGetWikiPages<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetWikiPages { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetWikiPageRevisions<'_>,
        <endpoints::RepoGetWikiPageRevisions<'_> as Endpoint>::Response,
    > {
        endpoints::RepoGetWikiPageRevisions {
            owner,
            repo,
            page_name,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
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
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GenerateRepo<'_>,
        <endpoints::GenerateRepo<'_> as Endpoint>::Response,
    > {
        endpoints::GenerateRepo {
            template_owner,
            template_repo,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Get a repository by id
    ///
    /// - `id`: id of the repo to get
    pub fn repo_get_by_id(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::RepoGetById,
        <endpoints::RepoGetById as Endpoint>::Response,
    > {
        endpoints::RepoGetById { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get instance's global settings for api
    pub fn get_general_api_settings(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetGeneralApiSettings,
        <endpoints::GetGeneralApiSettings as Endpoint>::Response,
    > {
        endpoints::GetGeneralApiSettings {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get instance's global settings for Attachment
    pub fn get_general_attachment_settings(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetGeneralAttachmentSettings,
        <endpoints::GetGeneralAttachmentSettings as Endpoint>::Response,
    > {
        endpoints::GetGeneralAttachmentSettings {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get instance's global settings for repositories
    pub fn get_general_repository_settings(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetGeneralRepositorySettings,
        <endpoints::GetGeneralRepositorySettings as Endpoint>::Response,
    > {
        endpoints::GetGeneralRepositorySettings {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get instance's global settings for ui
    pub fn get_general_ui_settings(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetGeneralUiSettings,
        <endpoints::GetGeneralUiSettings as Endpoint>::Response,
    > {
        endpoints::GetGeneralUiSettings {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get default signing-key.gpg
    pub fn get_signing_key(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetSigningKey,
        <endpoints::GetSigningKey as Endpoint>::Response,
    > {
        endpoints::GetSigningKey {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get default signing-key.ssh
    pub fn get_ssh_signing_key(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetSshSigningKey,
        <endpoints::GetSshSigningKey as Endpoint>::Response,
    > {
        endpoints::GetSshSigningKey {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a team
    ///
    /// - `id`: id of the team to get
    pub fn org_get_team(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetTeam,
        <endpoints::OrgGetTeam as Endpoint>::Response,
    > {
        endpoints::OrgGetTeam { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a team
    ///
    /// - `id`: id of the team to delete
    pub fn org_delete_team(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgDeleteTeam,
        <endpoints::OrgDeleteTeam as Endpoint>::Response,
    > {
        endpoints::OrgDeleteTeam { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Edit a team
    ///
    /// - `id`: id of the team to edit
    /// - `body`: See [`EditTeamOption`]
    pub fn org_edit_team(
        &self,
        id: i64,
        body: EditTeamOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgEditTeam,
        <endpoints::OrgEditTeam as Endpoint>::Response,
    > {
        endpoints::OrgEditTeam { id, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a team's activity feeds
    ///
    /// - `id`: id of the team
    pub fn org_list_team_activity_feeds(
        &self,
        id: i64,
        query: OrgListTeamActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListTeamActivityFeeds,
        <endpoints::OrgListTeamActivityFeeds as Endpoint>::Response,
    > {
        endpoints::OrgListTeamActivityFeeds { id, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a team's members
    ///
    /// - `id`: id of the team
    pub fn org_list_team_members(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListTeamMembers,
        <endpoints::OrgListTeamMembers as Endpoint>::Response,
    > {
        endpoints::OrgListTeamMembers { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a particular member of team
    ///
    /// - `id`: id of the team
    /// - `username`: username of the member to list
    pub fn org_list_team_member(
        &self,
        id: i64,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListTeamMember<'_>,
        <endpoints::OrgListTeamMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListTeamMember { id, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Add a team member
    ///
    /// - `id`: id of the team
    /// - `username`: username of the user to add
    pub fn org_add_team_member(
        &self,
        id: i64,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgAddTeamMember<'_>,
        <endpoints::OrgAddTeamMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgAddTeamMember { id, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Remove a team member
    ///
    /// - `id`: id of the team
    /// - `username`: username of the user to remove
    pub fn org_remove_team_member(
        &self,
        id: i64,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgRemoveTeamMember<'_>,
        <endpoints::OrgRemoveTeamMember<'_> as Endpoint>::Response,
    > {
        endpoints::OrgRemoveTeamMember { id, username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a team's repos
    ///
    /// - `id`: id of the team
    pub fn org_list_team_repos(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListTeamRepos,
        <endpoints::OrgListTeamRepos as Endpoint>::Response,
    > {
        endpoints::OrgListTeamRepos { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a particular repo of team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to list
    /// - `repo`: name of the repo to list
    pub fn org_list_team_repo(
        &self,
        id: i64,
        org: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListTeamRepo<'_>,
        <endpoints::OrgListTeamRepo<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListTeamRepo { id, org, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Add a repository to a team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to add
    /// - `repo`: name of the repo to add
    pub fn org_add_team_repository(
        &self,
        id: i64,
        org: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgAddTeamRepository<'_>,
        <endpoints::OrgAddTeamRepository<'_> as Endpoint>::Response,
    > {
        endpoints::OrgAddTeamRepository { id, org, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Remove a repository from a team
    ///
    /// - `id`: id of the team
    /// - `org`: organization that owns the repo to remove
    /// - `repo`: name of the repo to remove
    pub fn org_remove_team_repository(
        &self,
        id: i64,
        org: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgRemoveTeamRepository<'_>,
        <endpoints::OrgRemoveTeamRepository<'_> as Endpoint>::Response,
    > {
        endpoints::OrgRemoveTeamRepository { id, org, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search for topics by keyword
    ///
    pub fn topic_search(
        &self,
        query: TopicSearchQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::TopicSearch,
        <endpoints::TopicSearch as Endpoint>::Response,
    > {
        endpoints::TopicSearch { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get the authenticated user
    pub fn user_get_current(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetCurrent,
        <endpoints::UserGetCurrent as Endpoint>::Response,
    > {
        endpoints::UserGetCurrent {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search for user's action jobs according filter conditions
    ///
    pub fn user_search_run_jobs(
        &self,
        query: UserSearchRunJobsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserSearchRunJobs,
        <endpoints::UserSearchRunJobs as Endpoint>::Response,
    > {
        endpoints::UserSearchRunJobs { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get an user's actions runner registration token
    pub fn user_get_runner_registration_token(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetRunnerRegistrationToken,
        <endpoints::UserGetRunnerRegistrationToken as Endpoint>::Response,
    > {
        endpoints::UserGetRunnerRegistrationToken {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create or Update a secret value in a user scope
    ///
    /// - `secretname`: name of the secret
    /// - `body`: See [`CreateOrUpdateSecretOption`]
    pub fn update_user_secret(
        &self,
        secretname: &str,
        body: CreateOrUpdateSecretOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateUserSecret<'_>,
        <endpoints::UpdateUserSecret<'_> as Endpoint>::Response,
    > {
        endpoints::UpdateUserSecret {
            secretname,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Delete a secret in a user scope
    ///
    /// - `secretname`: name of the secret
    pub fn delete_user_secret(
        &self,
        secretname: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeleteUserSecret<'_>,
        <endpoints::DeleteUserSecret<'_> as Endpoint>::Response,
    > {
        endpoints::DeleteUserSecret { secretname }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get the user-level list of variables which is created by current doer
    ///
    pub fn get_user_variables_list(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetUserVariablesList,
        <endpoints::GetUserVariablesList as Endpoint>::Response,
    > {
        endpoints::GetUserVariablesList {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    pub fn get_user_variable(
        &self,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetUserVariable<'_>,
        <endpoints::GetUserVariable<'_> as Endpoint>::Response,
    > {
        endpoints::GetUserVariable { variablename }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    /// - `body`: See [`UpdateVariableOption`]
    pub fn update_user_variable(
        &self,
        variablename: &str,
        body: UpdateVariableOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateUserVariable<'_>,
        <endpoints::UpdateUserVariable<'_> as Endpoint>::Response,
    > {
        endpoints::UpdateUserVariable {
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Create a user-level variable
    ///
    /// - `variablename`: name of the variable
    /// - `body`: See [`CreateVariableOption`]
    pub fn create_user_variable(
        &self,
        variablename: &str,
        body: CreateVariableOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateUserVariable<'_>,
        <endpoints::CreateUserVariable<'_> as Endpoint>::Response,
    > {
        endpoints::CreateUserVariable {
            variablename,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Delete a user-level variable which is created by current doer
    ///
    /// - `variablename`: name of the variable
    pub fn delete_user_variable(
        &self,
        variablename: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::DeleteUserVariable<'_>,
        <endpoints::DeleteUserVariable<'_> as Endpoint>::Response,
    > {
        endpoints::DeleteUserVariable { variablename }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's oauth2 applications
    ///
    pub fn user_get_oauth2_applications(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetOAuth2Applications,
        <endpoints::UserGetOAuth2Applications as Endpoint>::Response,
    > {
        endpoints::UserGetOAuth2Applications {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Creates a new OAuth2 application
    ///
    /// - `body`: See [`CreateOAuth2ApplicationOptions`]
    pub fn user_create_oauth2_application(
        &self,
        body: CreateOAuth2ApplicationOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCreateOAuth2Application,
        <endpoints::UserCreateOAuth2Application as Endpoint>::Response,
    > {
        endpoints::UserCreateOAuth2Application { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get an OAuth2 application
    ///
    /// - `id`: Application ID to be found
    pub fn user_get_oauth2_application(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetOAuth2Application,
        <endpoints::UserGetOAuth2Application as Endpoint>::Response,
    > {
        endpoints::UserGetOAuth2Application { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete an OAuth2 application
    ///
    /// - `id`: token to be deleted
    pub fn user_delete_oauth2_application(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserDeleteOAuth2Application,
        <endpoints::UserDeleteOAuth2Application as Endpoint>::Response,
    > {
        endpoints::UserDeleteOAuth2Application { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update an OAuth2 application, this includes regenerating the client secret
    ///
    /// - `id`: application to be updated
    /// - `body`: See [`CreateOAuth2ApplicationOptions`]
    pub fn user_update_oauth2_application(
        &self,
        id: i64,
        body: CreateOAuth2ApplicationOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserUpdateOAuth2Application,
        <endpoints::UserUpdateOAuth2Application as Endpoint>::Response,
    > {
        endpoints::UserUpdateOAuth2Application { id, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update avatar of the current user
    ///
    /// - `body`: See [`UpdateUserAvatarOption`]
    pub fn user_update_avatar(
        &self,
        body: UpdateUserAvatarOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserUpdateAvatar,
        <endpoints::UserUpdateAvatar as Endpoint>::Response,
    > {
        endpoints::UserUpdateAvatar { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete avatar of the current user. It will be replaced by a default one
    pub fn user_delete_avatar(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserDeleteAvatar,
        <endpoints::UserDeleteAvatar as Endpoint>::Response,
    > {
        endpoints::UserDeleteAvatar {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Blocks a user from the doer
    ///
    /// - `username`: username of the user
    pub fn user_block_user(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserBlockUser<'_>,
        <endpoints::UserBlockUser<'_> as Endpoint>::Response,
    > {
        endpoints::UserBlockUser { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List all email addresses of the current user
    pub fn user_list_emails(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListEmails,
        <endpoints::UserListEmails as Endpoint>::Response,
    > {
        endpoints::UserListEmails {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Add an email addresses to the current user's account
    ///
    /// - `body`: See [`CreateEmailOption`]
    pub fn user_add_email(
        &self,
        body: CreateEmailOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserAddEmail,
        <endpoints::UserAddEmail as Endpoint>::Response,
    > {
        endpoints::UserAddEmail { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete email addresses from the current user's account
    ///
    /// - `body`: See [`DeleteEmailOption`]
    pub fn user_delete_email(
        &self,
        body: DeleteEmailOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserDeleteEmail,
        <endpoints::UserDeleteEmail as Endpoint>::Response,
    > {
        endpoints::UserDeleteEmail { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's followers
    ///
    pub fn user_current_list_followers(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListFollowers,
        <endpoints::UserCurrentListFollowers as Endpoint>::Response,
    > {
        endpoints::UserCurrentListFollowers {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the users that the authenticated user is following
    ///
    pub fn user_current_list_following(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListFollowing,
        <endpoints::UserCurrentListFollowing as Endpoint>::Response,
    > {
        endpoints::UserCurrentListFollowing {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check whether a user is followed by the authenticated user
    ///
    /// - `username`: username of followed user
    pub fn user_current_check_following(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentCheckFollowing<'_>,
        <endpoints::UserCurrentCheckFollowing<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentCheckFollowing { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Follow a user
    ///
    /// - `username`: username of user to follow
    pub fn user_current_put_follow(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentPutFollow<'_>,
        <endpoints::UserCurrentPutFollow<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentPutFollow { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Unfollow a user
    ///
    /// - `username`: username of user to unfollow
    pub fn user_current_delete_follow(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentDeleteFollow<'_>,
        <endpoints::UserCurrentDeleteFollow<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentDeleteFollow { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a Token to verify
    pub fn get_verification_token(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetVerificationToken,
        <endpoints::GetVerificationToken as Endpoint>::Response,
    > {
        endpoints::GetVerificationToken {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Verify a GPG key
    ///
    /// - `body`: See [`VerifyGPGKeyOption`]
    pub fn user_verify_gpg_key(
        &self,
        body: VerifyGPGKeyOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserVerifyGpgKey,
        <endpoints::UserVerifyGpgKey as Endpoint>::Response,
    > {
        endpoints::UserVerifyGpgKey { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's GPG keys
    ///
    pub fn user_current_list_gpg_keys(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListGpgKeys,
        <endpoints::UserCurrentListGpgKeys as Endpoint>::Response,
    > {
        endpoints::UserCurrentListGpgKeys {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Add a GPG public key to current user's account
    ///
    /// - `Form`: See [`CreateGPGKeyOption`]
    pub fn user_current_post_gpg_key(
        &self,
        form: CreateGPGKeyOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentPostGpgKey,
        <endpoints::UserCurrentPostGpgKey as Endpoint>::Response,
    > {
        endpoints::UserCurrentPostGpgKey { body: form }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a GPG key
    ///
    /// - `id`: id of key to get
    pub fn user_current_get_gpg_key(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentGetGpgKey,
        <endpoints::UserCurrentGetGpgKey as Endpoint>::Response,
    > {
        endpoints::UserCurrentGetGpgKey { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Remove a GPG public key from current user's account
    ///
    /// - `id`: id of key to delete
    pub fn user_current_delete_gpg_key(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentDeleteGpgKey,
        <endpoints::UserCurrentDeleteGpgKey as Endpoint>::Response,
    > {
        endpoints::UserCurrentDeleteGpgKey { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's webhooks
    ///
    pub fn user_list_hooks(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListHooks,
        <endpoints::UserListHooks as Endpoint>::Response,
    > {
        endpoints::UserListHooks {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a hook
    ///
    /// - `body`: See [`CreateHookOption`]
    pub fn user_create_hook(
        &self,
        body: CreateHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCreateHook,
        <endpoints::UserCreateHook as Endpoint>::Response,
    > {
        endpoints::UserCreateHook { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a hook
    ///
    /// - `id`: id of the hook to get
    pub fn user_get_hook(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetHook,
        <endpoints::UserGetHook as Endpoint>::Response,
    > {
        endpoints::UserGetHook { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a hook
    ///
    /// - `id`: id of the hook to delete
    pub fn user_delete_hook(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserDeleteHook,
        <endpoints::UserDeleteHook as Endpoint>::Response,
    > {
        endpoints::UserDeleteHook { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update a hook
    ///
    /// - `id`: id of the hook to update
    /// - `body`: See [`EditHookOption`]
    pub fn user_edit_hook(
        &self,
        id: i64,
        body: EditHookOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserEditHook,
        <endpoints::UserEditHook as Endpoint>::Response,
    > {
        endpoints::UserEditHook { id, body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's public keys
    ///
    pub fn user_current_list_keys(
        &self,
        query: UserCurrentListKeysQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListKeys,
        <endpoints::UserCurrentListKeys as Endpoint>::Response,
    > {
        endpoints::UserCurrentListKeys { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a public key
    ///
    /// - `body`: See [`CreateKeyOption`]
    pub fn user_current_post_key(
        &self,
        body: CreateKeyOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentPostKey,
        <endpoints::UserCurrentPostKey as Endpoint>::Response,
    > {
        endpoints::UserCurrentPostKey { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a public key
    ///
    /// - `id`: id of key to get
    pub fn user_current_get_key(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentGetKey,
        <endpoints::UserCurrentGetKey as Endpoint>::Response,
    > {
        endpoints::UserCurrentGetKey { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Delete a public key
    ///
    /// - `id`: id of key to delete
    pub fn user_current_delete_key(
        &self,
        id: i64,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentDeleteKey,
        <endpoints::UserCurrentDeleteKey as Endpoint>::Response,
    > {
        endpoints::UserCurrentDeleteKey { id }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's blocked users
    ///
    pub fn user_list_blocked_users(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListBlockedUsers,
        <endpoints::UserListBlockedUsers as Endpoint>::Response,
    > {
        endpoints::UserListBlockedUsers {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the current user's organizations
    ///
    pub fn org_list_current_user_orgs(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListCurrentUserOrgs,
        <endpoints::OrgListCurrentUserOrgs as Endpoint>::Response,
    > {
        endpoints::OrgListCurrentUserOrgs {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get quota information for the authenticated user
    pub fn user_get_quota(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetQuota,
        <endpoints::UserGetQuota as Endpoint>::Response,
    > {
        endpoints::UserGetQuota {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the artifacts affecting the authenticated user's quota
    ///
    pub fn user_list_quota_artifacts(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListQuotaArtifacts,
        <endpoints::UserListQuotaArtifacts as Endpoint>::Response,
    > {
        endpoints::UserListQuotaArtifacts {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the attachments affecting the authenticated user's quota
    ///
    pub fn user_list_quota_attachments(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListQuotaAttachments,
        <endpoints::UserListQuotaAttachments as Endpoint>::Response,
    > {
        endpoints::UserListQuotaAttachments {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if the authenticated user is over quota for a given subject
    ///
    pub fn user_check_quota(
        &self,
        query: UserCheckQuotaQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCheckQuota,
        <endpoints::UserCheckQuota as Endpoint>::Response,
    > {
        endpoints::UserCheckQuota { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the packages affecting the authenticated user's quota
    ///
    pub fn user_list_quota_packages(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListQuotaPackages,
        <endpoints::UserListQuotaPackages as Endpoint>::Response,
    > {
        endpoints::UserListQuotaPackages {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the repos that the authenticated user owns
    ///
    pub fn user_current_list_repos(
        &self,
        query: UserCurrentListReposQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListRepos,
        <endpoints::UserCurrentListRepos as Endpoint>::Response,
    > {
        endpoints::UserCurrentListRepos { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Create a repository
    ///
    /// - `body`: See [`CreateRepoOption`]
    pub fn create_current_user_repo(
        &self,
        body: CreateRepoOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::CreateCurrentUserRepo,
        <endpoints::CreateCurrentUserRepo as Endpoint>::Response,
    > {
        endpoints::CreateCurrentUserRepo { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get current user's account settings
    pub fn get_user_settings(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetUserSettings,
        <endpoints::GetUserSettings as Endpoint>::Response,
    > {
        endpoints::GetUserSettings {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Update settings in current user's account
    ///
    /// - `body`: See [`UserSettingsOptions`]
    pub fn update_user_settings(
        &self,
        body: UserSettingsOptions,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UpdateUserSettings,
        <endpoints::UpdateUserSettings as Endpoint>::Response,
    > {
        endpoints::UpdateUserSettings { body: body }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// The repos that the authenticated user has starred
    ///
    pub fn user_current_list_starred(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListStarred,
        <endpoints::UserCurrentListStarred as Endpoint>::Response,
    > {
        endpoints::UserCurrentListStarred {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Whether the authenticated is starring the repo
    ///
    /// - `owner`: owner of the repo
    /// - `repo`: name of the repo
    pub fn user_current_check_starring(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentCheckStarring<'_>,
        <endpoints::UserCurrentCheckStarring<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentCheckStarring { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Star the given repo
    ///
    /// - `owner`: owner of the repo to star
    /// - `repo`: name of the repo to star
    pub fn user_current_put_star(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentPutStar<'_>,
        <endpoints::UserCurrentPutStar<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentPutStar { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Unstar the given repo
    ///
    /// - `owner`: owner of the repo to unstar
    /// - `repo`: name of the repo to unstar
    pub fn user_current_delete_star(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentDeleteStar<'_>,
        <endpoints::UserCurrentDeleteStar<'_> as Endpoint>::Response,
    > {
        endpoints::UserCurrentDeleteStar { owner, repo }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get list of all existing stopwatches
    ///
    pub fn user_get_stop_watches(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetStopWatches,
        <endpoints::UserGetStopWatches as Endpoint>::Response,
    > {
        endpoints::UserGetStopWatches {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List repositories watched by the authenticated user
    ///
    pub fn user_current_list_subscriptions(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentListSubscriptions,
        <endpoints::UserCurrentListSubscriptions as Endpoint>::Response,
    > {
        endpoints::UserCurrentListSubscriptions {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List all the teams a user belongs to
    ///
    pub fn user_list_teams(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListTeams,
        <endpoints::UserListTeams as Endpoint>::Response,
    > {
        endpoints::UserListTeams {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the current user's tracked times
    ///
    pub fn user_current_tracked_times(
        &self,
        query: UserCurrentTrackedTimesQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCurrentTrackedTimes,
        <endpoints::UserCurrentTrackedTimes as Endpoint>::Response,
    > {
        endpoints::UserCurrentTrackedTimes { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Unblocks a user from the doer
    ///
    /// - `username`: username of the user
    pub fn user_unblock_user(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserUnblockUser<'_>,
        <endpoints::UserUnblockUser<'_> as Endpoint>::Response,
    > {
        endpoints::UserUnblockUser { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Search for users
    ///
    pub fn user_search(
        &self,
        query: UserSearchQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserSearch,
        <endpoints::UserSearch as Endpoint>::Response,
    > {
        endpoints::UserSearch { query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a user
    ///
    /// - `username`: username of user to get
    pub fn user_get(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGet<'_>,
        <endpoints::UserGet<'_> as Endpoint>::Response,
    > {
        endpoints::UserGet { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a user's activity feeds
    ///
    /// - `username`: username of user
    pub fn user_list_activity_feeds(
        &self,
        username: &str,
        query: UserListActivityFeedsQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListActivityFeeds<'_>,
        <endpoints::UserListActivityFeeds<'_> as Endpoint>::Response,
    > {
        endpoints::UserListActivityFeeds { username, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the given user's followers
    ///
    /// - `username`: username of user
    pub fn user_list_followers(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListFollowers<'_>,
        <endpoints::UserListFollowers<'_> as Endpoint>::Response,
    > {
        endpoints::UserListFollowers { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the users that the given user is following
    ///
    /// - `username`: username of user
    pub fn user_list_following(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListFollowing<'_>,
        <endpoints::UserListFollowing<'_> as Endpoint>::Response,
    > {
        endpoints::UserListFollowing { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Check if one user is following another user
    ///
    /// - `username`: username of following user
    /// - `target`: username of followed user
    pub fn user_check_following(
        &self,
        username: &str,
        target: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCheckFollowing<'_>,
        <endpoints::UserCheckFollowing<'_> as Endpoint>::Response,
    > {
        endpoints::UserCheckFollowing { username, target }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the given user's GPG keys
    ///
    /// - `username`: username of user
    pub fn user_list_gpg_keys(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListGpgKeys<'_>,
        <endpoints::UserListGpgKeys<'_> as Endpoint>::Response,
    > {
        endpoints::UserListGpgKeys { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get a user's heatmap
    ///
    /// - `username`: username of user to get
    pub fn user_get_heatmap_data(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetHeatmapData<'_>,
        <endpoints::UserGetHeatmapData<'_> as Endpoint>::Response,
    > {
        endpoints::UserGetHeatmapData { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the given user's public keys
    ///
    /// - `username`: username of user
    pub fn user_list_keys(
        &self,
        username: &str,
        query: UserListKeysQuery,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListKeys<'_>,
        <endpoints::UserListKeys<'_> as Endpoint>::Response,
    > {
        endpoints::UserListKeys { username, query }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List a user's organizations
    ///
    /// - `username`: username of user
    pub fn org_list_user_orgs(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgListUserOrgs<'_>,
        <endpoints::OrgListUserOrgs<'_> as Endpoint>::Response,
    > {
        endpoints::OrgListUserOrgs { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Get user permissions in organization
    ///
    /// - `username`: username of user
    /// - `org`: name of the organization
    pub fn org_get_user_permissions(
        &self,
        username: &str,
        org: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::OrgGetUserPermissions<'_>,
        <endpoints::OrgGetUserPermissions<'_> as Endpoint>::Response,
    > {
        endpoints::OrgGetUserPermissions { username, org }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the repos owned by the given user
    ///
    /// - `username`: username of user
    pub fn user_list_repos(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListRepos<'_>,
        <endpoints::UserListRepos<'_> as Endpoint>::Response,
    > {
        endpoints::UserListRepos { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// The repos that the given user has starred
    ///
    /// - `username`: username of user
    pub fn user_list_starred(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListStarred<'_>,
        <endpoints::UserListStarred<'_> as Endpoint>::Response,
    > {
        endpoints::UserListStarred { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the repositories watched by a user
    ///
    /// - `username`: username of the user
    pub fn user_list_subscriptions(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserListSubscriptions<'_>,
        <endpoints::UserListSubscriptions<'_> as Endpoint>::Response,
    > {
        endpoints::UserListSubscriptions { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// List the authenticated user's access tokens
    ///
    /// - `username`: username of user
    pub fn user_get_tokens(
        &self,
        username: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserGetTokens<'_>,
        <endpoints::UserGetTokens<'_> as Endpoint>::Response,
    > {
        endpoints::UserGetTokens { username }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Generate an access token for the current user
    ///
    /// - `username`: username of user
    /// - `body`: See [`CreateAccessTokenOption`]
    pub fn user_create_token(
        &self,
        username: &str,
        body: CreateAccessTokenOption,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserCreateToken<'_>,
        <endpoints::UserCreateToken<'_> as Endpoint>::Response,
    > {
        endpoints::UserCreateToken {
            username,
            body: body,
        }
        .make_request()
        .wrap_sync::<_, _>(self)
    }

    /// Delete an access token from current user's account
    ///
    /// - `username`: username of user
    /// - `token`: token to be deleted, identified by ID and if not available by name
    pub fn user_delete_access_token(
        &self,
        username: &str,
        token: &str,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::UserDeleteAccessToken<'_>,
        <endpoints::UserDeleteAccessToken<'_> as Endpoint>::Response,
    > {
        endpoints::UserDeleteAccessToken { username, token }
            .make_request()
            .wrap_sync::<_, _>(self)
    }

    /// Returns the version of the running application
    pub fn get_version(
        &self,
    ) -> crate::sync::WrappedRequest<
        '_,
        endpoints::GetVersion,
        <endpoints::GetVersion as Endpoint>::Response,
    > {
        endpoints::GetVersion {}
            .make_request()
            .wrap_sync::<_, _>(self)
    }
}
