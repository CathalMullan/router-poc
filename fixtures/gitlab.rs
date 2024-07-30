#[rustfmt::skip]
pub const ROUTES: [&str; 1612] = [
  // abuse_reports
  "/-/abuse_reports",
  "/-/abuse_reports/add_category",

  // acme_challenges
  "/-/acme-challenge",

  // activity_pub/projects/releases
  "/{namespace_id:*}/{project_id}/-/releases/inbox",
  "/{namespace_id:*}/{project_id}/-/releases/outbox",

  // admin/abuse_reports
  "/admin/abuse_reports",
  "/admin/abuse_reports/{id}",
  "/admin/abuse_reports/{id}/moderate_user",

  // admin/ai/feature_settings
  "/admin/ai/feature_settings",
  "/admin/ai/feature_settings/{id}",
  "/admin/ai/feature_settings/{id}/edit",

  // admin/ai/self_hosted_models
  "/admin/ai/self_hosted_models",
  "/admin/ai/self_hosted_models/{id}",
  "/admin/ai/self_hosted_models/{id}/edit",
  "/admin/ai/self_hosted_models/new",

  // admin/ai/terms_and_conditions
  "/admin/ai/self_hosted_models/terms_and_conditions",

  // admin/application_settings
  "/admin/application_settings",
  "/admin/application_settings/advanced_search",
  "/admin/application_settings/analytics",
  "/admin/application_settings/ci_cd",
  "/admin/application_settings/clear_repository_check_states",
  "/admin/application_settings/general",
  "/admin/application_settings/integrations",
  "/admin/application_settings/lets_encrypt_terms_of_service",
  "/admin/application_settings/metrics_and_profiling",
  "/admin/application_settings/namespace_storage",
  "/admin/application_settings/network",
  "/admin/application_settings/preferences",
  "/admin/application_settings/reporting",
  "/admin/application_settings/repository",
  "/admin/application_settings/reset_error_tracking_access_token",
  "/admin/application_settings/reset_health_check_token",
  "/admin/application_settings/reset_registration_token",
  "/admin/application_settings/seat_link_payload",
  "/admin/application_settings/security_and_compliance",
  "/admin/application_settings/slack_app_manifest_download",
  "/admin/application_settings/slack_app_manifest_share",
  "/admin/application_settings/templates",
  "/admin/application_settings/update_microsoft_application",
  "/admin/application_settings/usage_data",

  // admin/application_settings/appearances
  "/admin/application_settings/appearance",
  "/admin/application_settings/appearance/favicon",
  "/admin/application_settings/appearance/header_logos",
  "/admin/application_settings/appearance/logo",
  "/admin/application_settings/appearance/preview_sign_in",
  "/admin/application_settings/appearance/pwa_icon",

  // admin/application_settings/roles_and_permissions
  "/admin/application_settings/roles_and_permissions",
  "/admin/application_settings/roles_and_permissions/{id}/edit",
  "/admin/application_settings/roles_and_permissions/new",

  // admin/application_settings/scim_oauth
  "/admin/application_settings/scim_oauth",

  // admin/applications
  "/admin/applications",
  "/admin/applications/{id}",
  "/admin/applications/{id}/edit",
  "/admin/applications/{id}/renew",
  "/admin/applications/new",

  // admin/audit_log_reports
  "/admin/audit_log_reports",

  // admin/audit_logs
  "/admin/audit_logs",

  // admin/background_jobs
  "/admin/background_jobs",

  // admin/background_migrations
  "/admin/background_migrations",
  "/admin/background_migrations/{id}",
  "/admin/background_migrations/{id}/pause",
  "/admin/background_migrations/{id}/resume",
  "/admin/background_migrations/{id}/retry",

  // admin/batched_jobs
  "/admin/background_migrations/{background_migration_id}/batched_jobs/{id}",

  // admin/broadcast_messages
  "/admin/broadcast_messages",
  "/admin/broadcast_messages/{id}",
  "/admin/broadcast_messages/{id}/edit",
  "/admin/broadcast_messages/preview",

  // admin/ci/variables
  "/admin/ci/variables",

  // admin/clusters
  "/admin/clusters",
  "/admin/clusters/{id}",
  "/admin/clusters/{id}/clear_cache",
  "/admin/clusters/{id}/cluster_status",
  "/admin/clusters/{id}/environments",
  "/admin/clusters/{id}/metrics",
  "/admin/clusters/{id}/metrics_dashboard",
  "/admin/clusters/connect",
  "/admin/clusters/create_user",
  "/admin/clusters/new_cluster_docs",

  // admin/clusters/integrations
  "/admin/clusters/{cluster_id}/integration/create_or_update",

  // admin/code_suggestions
  "/admin/code_suggestions",

  // admin/cohorts
  "/admin/cohorts",

  // admin/credentials
  "/admin/credentials",
  "/admin/credentials/{credential_id}/resources/{resource_id}/revoke",
  "/admin/credentials/{id}",
  "/admin/credentials/{id}/revoke",

  // admin/dashboard
  "/admin",
  "/admin/dashboard/stats",

  // admin/deploy_keys
  "/admin/deploy_keys",
  "/admin/deploy_keys/{id}",
  "/admin/deploy_keys/{id}/edit",
  "/admin/deploy_keys/new",

  // admin/dev_ops_report
  "/admin/dev_ops_reports",

  // admin/elasticsearch
  "/admin/elasticsearch/cancel_index_deletion",
  "/admin/elasticsearch/enqueue_index",
  "/admin/elasticsearch/retry_migration",
  "/admin/elasticsearch/trigger_reindexing",

  // admin/emails
  "/admin/email",

  // admin/geo/nodes
  "/admin/geo",
  "/admin/geo/sites",
  "/admin/geo/sites/{id}",
  "/admin/geo/sites/{id}/edit",
  "/admin/geo/sites/{id}/replication",
  "/admin/geo/sites/new",

  // admin/geo/replicables
  "/admin/geo/replication/{replicable_name_plural}",
  "/admin/geo/sites/{id}/replication/{replicable_name_plural}",

  // admin/geo/settings
  "/admin/application_settings/geo",
  "/admin/geo/settings",

  // admin/gitaly_servers
  "/admin/gitaly_servers",

  // admin/groups
  "/admin/groups",
  "/admin/groups/{id:*}",
  "/admin/groups/{id:*}/edit",
  "/admin/groups/{id:*}/members_update",
  "/admin/groups/{id:*}/reset_runners_minutes",
  "/admin/groups/new",

  // admin/health_check
  "/admin/health_check",

  // admin/hook_logs
  "/admin/hooks/{hook_id}/hook_logs/{id}",
  "/admin/hooks/{hook_id}/hook_logs/{id}/retry",

  // admin/hooks
  "/admin/hooks",
  "/admin/hooks/{id}",
  "/admin/hooks/{id}/edit",
  "/admin/hooks/{id}/test",

  // admin/identities
  "/admin/users/{user_id}/identities",
  "/admin/users/{user_id}/identities/{id}",
  "/admin/users/{user_id}/identities/{id}/edit",
  "/admin/users/{user_id}/identities/new",

  // admin/impersonation_tokens
  "/admin/users/{user_id}/impersonation_tokens",
  "/admin/users/{user_id}/impersonation_tokens/{id}/revoke",

  // admin/impersonations
  "/admin/impersonation",

  // admin/initial_setup
  "/admin/initial_setup",
  "/admin/initial_setup/new",

  // admin/instance_review
  "/admin/instance_review",

  // admin/integrations
  "/admin/application_settings/integrations/{id}",
  "/admin/application_settings/integrations/{id}/edit",
  "/admin/application_settings/integrations/{id}/overrides",
  "/admin/application_settings/integrations/{id}/reset",
  "/admin/application_settings/integrations/{id}/test",

  // admin/jobs
  "/admin/jobs",
  "/admin/jobs/cancel_all",

  // admin/keys
  "/admin/users/{user_id}/keys/{id}",

  // admin/labels
  "/admin/labels",
  "/admin/labels/{id}",
  "/admin/labels/{id}/edit",
  "/admin/labels/new",

  // admin/licenses
  "/admin/license",
  "/admin/license/download",
  "/admin/license/sync_seat_link",

  // admin/licenses/usage_exports
  "/admin/license/usage_export",

  // admin/namespace_limits
  "/admin/namespace_limits",
  "/admin/namespace_limits/export_usage",

  // admin/organizations
  "/admin/organizations",

  // admin/plan_limits
  "/admin/plan_limits",

  // admin/projects
  "/admin/projects",
  "/admin/projects/{namespace_id:*}/{id}",
  "/admin/projects/{namespace_id:*}/{id}/edit",
  "/admin/projects/{namespace_id:*}/{id}/repository_check",
  "/admin/projects/{namespace_id:*}/{id}/transfer",

  // admin/push_rules
  "/admin/push_rule",

  // admin/role_promotion_requests
  "/admin/role_promotion_requests",

  // admin/runner_projects
  "/admin/projects/{namespace_id:*}/{project_id}/runner_projects",
  "/admin/projects/{namespace_id:*}/{project_id}/runner_projects/{id}",

  // admin/runners
  "/admin/runners",
  "/admin/runners/{id}",
  "/admin/runners/{id}/edit",
  "/admin/runners/{id}/pause",
  "/admin/runners/{id}/register",
  "/admin/runners/{id}/resume",
  "/admin/runners/dashboard",
  "/admin/runners/new",
  "/admin/runners/runner_setup_scripts",
  "/admin/runners/tag_list",

  // admin/sessions
  "/admin/session",
  "/admin/session/destroy",
  "/admin/session/new",

  // admin/slacks
  "/admin/application_settings/slack",
  "/admin/application_settings/slack/slack_auth",

  // admin/spam_logs
  "/admin/spam_logs",
  "/admin/spam_logs/{id}",
  "/admin/spam_logs/{id}/mark_as_ham",

  // admin/subscriptions
  "/admin/subscription",

  // admin/system_info
  "/admin/system_info",

  // admin/topics
  "/admin/topics",
  "/admin/topics/{id}",
  "/admin/topics/{id}/edit",
  "/admin/topics/merge",
  "/admin/topics/new",
  "/admin/topics/preview_markdown",

  // admin/topics/avatars
  "/admin/topics/{topic_id}/avatar",

  // admin/usage_trends
  "/admin/usage_trends",

  // admin/user_permission_exports
  "/admin/user_permission_exports",

  // admin/users
  "/admin/users",
  "/admin/users/{id}",
  "/admin/users/{id}/activate",
  "/admin/users/{id}/approve",
  "/admin/users/{id}/ban",
  "/admin/users/{id}/block",
  "/admin/users/{id}/card_match",
  "/admin/users/{id}/confirm",
  "/admin/users/{id}/deactivate",
  "/admin/users/{id}/destroy_identity_verification_exemption",
  "/admin/users/{id}/disable_two_factor",
  "/admin/users/{id}/edit",
  "/admin/users/{id}/identity_verification_exemption",
  "/admin/users/{id}/impersonate",
  "/admin/users/{id}/keys",
  "/admin/users/{id}/phone_match",
  "/admin/users/{id}/projects",
  "/admin/users/{id}/reject",
  "/admin/users/{id}/remove/{email_id}",
  "/admin/users/{id}/reset_runners_minutes",
  "/admin/users/{id}/trust",
  "/admin/users/{id}/unban",
  "/admin/users/{id}/unblock",
  "/admin/users/{id}/unlock",
  "/admin/users/{id}/untrust",
  "/admin/users/new",

  // admin/version_check
  "/admin/version_check",

  // application
  "/{namespace_id:*}/{project_id}",
  "/{namespace_id:*}/{project_id}/{all:*}",
  "/{unmatched_route:*}",

  // autocomplete
  "/-/autocomplete/award_emojis",
  "/-/autocomplete/deploy_keys_with_owners",
  "/-/autocomplete/group_subgroups",
  "/-/autocomplete/merge_request_source_branches",
  "/-/autocomplete/merge_request_target_branches",
  "/-/autocomplete/namespace_routes",
  "/-/autocomplete/project_groups",
  "/-/autocomplete/project_routes",
  "/-/autocomplete/projects",
  "/-/autocomplete/users",
  "/-/autocomplete/users/{id}",

  // banzai/uploads
  "/-/{model}/{model_id}/uploads/{secret}/{filename}",

  // chaos
  "/-/chaos/cpu_spin",
  "/-/chaos/db_spin",
  "/-/chaos/gc",
  "/-/chaos/kill",
  "/-/chaos/leakmem",
  "/-/chaos/quit",
  "/-/chaos/sleep",

  // clusters/agents/dashboard
  "/-/kubernetes",
  "/-/kubernetes/{agent_id}",
  "/-/kubernetes/{agent_id}/{vueroute:*}",

  // confirmations
  "/-/profile/emails/confirmation",
  "/-/profile/emails/confirmation/new",
  "/users/almost_there",
  "/users/confirmation",
  "/users/confirmation/new",

  // countries
  "/-/countries",

  // country_states
  "/-/country_states",

  // customers_dot/proxy
  "/-/customers_dot/proxy/graphql",

  // dashboard
  "/dashboard/activity",
  "/dashboard/issues",
  "/dashboard/merge_requests",

  // dashboard/groups
  "/dashboard/groups",

  // dashboard/labels
  "/dashboard/labels",

  // dashboard/milestones
  "/dashboard/milestones",

  // dashboard/projects
  "/dashboard",
  "/dashboard/projects",
  "/dashboard/projects/removed",
  "/dashboard/projects/starred",

  // dashboard/snippets
  "/dashboard/snippets",

  // dashboard/todos
  "/dashboard/todos",
  "/dashboard/todos/{id}",
  "/dashboard/todos/{id}/restore",
  "/dashboard/todos/bulk_restore",
  "/dashboard/todos/destroy_all",

  // devise/unlocks
  "/users/unlock",
  "/users/unlock/new",

  // doorkeeper/openid_connect/userinfo
  "/oauth/userinfo",

  // explore/catalog
  "/explore/catalog",
  "/explore/catalog/{full_path:*}",

  // explore/dependencies
  "/explore/dependencies",

  // explore/groups
  "/explore/groups",

  // explore/projects
  "/explore",
  "/explore/projects",
  "/explore/projects/starred",
  "/explore/projects/topics",
  "/explore/projects/topics/{topic_name}",
  "/explore/projects/trending",
  "/public",
  "/public/projects",

  // explore/snippets
  "/explore/snippets",

  // external_redirect/external_redirect
  "/-/external_redirect",

  // google_api/authorizations
  "/-/google_api/auth/callback",

  // graphql
  "/api/graphql",
  "/api/v4/geo/graphql",

  // groups
  "/{id:*}",
  "/groups",
  "/groups/{group_id:*}/-/preview_markdown",
  "/groups/{group_id:*}/-/restore",
  "/groups/{id:*}",
  "/groups/{id:*}/-/activity",
  "/groups/{id:*}/-/details",
  "/groups/{id:*}/-/download_export",
  "/groups/{id:*}/-/edit",
  "/groups/{id:*}/-/export",
  "/groups/{id:*}/-/inactive",
  "/groups/{id:*}/-/issues",
  "/groups/{id:*}/-/merge_requests",
  "/groups/{id:*}/-/projects",
  "/groups/{id:*}/-/shared",
  "/groups/{id:*}/-/transfer",
  "/groups/{id:*}/-/unfoldered_environment_names",
  "/groups/{group_id}/preview_markdown",
  "/groups/new",

  // groups/achievements
  "/groups/{group_id:*}/-/achievements",
  "/groups/{group_id:*}/-/achievements/{id}/edit",
  "/groups/{group_id:*}/-/achievements/new",

  // groups/add_ons/discover_duo_pro
  "/groups/{group_id:*}/-/add_ons/discover_duo_pro",

  // groups/analytics/ci_cd_analytics
  "/groups/{group_id:*}/-/analytics/ci_cd",

  // groups/analytics/coverage_reports
  "/groups/{group_id:*}/-/analytics/coverage_reports",

  // groups/analytics/cycle_analytics
  "/groups/{group_id:*}/-/analytics/value_stream_analytics",

  // groups/analytics/cycle_analytics/stages
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average_duration_chart",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/count",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/median",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/records",

  // groups/analytics/cycle_analytics/summary
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/cycle_times",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/lead_times",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/summary",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/time_summary",

  // groups/analytics/cycle_analytics/value_streams
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{id}",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/{id}/edit",
  "/groups/{group_id:*}/-/analytics/value_stream_analytics/value_streams/new",

  // groups/analytics/dashboards
  "/groups/{group_id:*}/-/analytics/dashboards",
  "/groups/{group_id:*}/-/analytics/dashboards/{vueroute:*}",

  // groups/analytics/devops_adoption
  "/groups/{group_id:*}/-/analytics/devops_adoption",

  // groups/analytics/merge_request_analytics
  "/groups/{group_id:*}/-/analytics/merge_request_analytics",

  // groups/analytics/productivity_analytics
  "/groups/{group_id:*}/-/analytics/productivity_analytics",

  // groups/analytics/repository_analytics
  "/groups/{group_id:*}/-/analytics/repository_analytics",

  // groups/analytics/tasks_by_type
  "/groups/{group_id:*}/-/analytics/type_of_work/tasks_by_type",
  "/groups/{group_id:*}/-/analytics/type_of_work/tasks_by_type/top_labels",

  // groups/audit_events
  "/groups/{group_id:*}/-/audit_events",

  // groups/autocomplete_sources
  "/groups/{group_id:*}/-/autocomplete_sources/commands",
  "/groups/{group_id:*}/-/autocomplete_sources/epics",
  "/groups/{group_id:*}/-/autocomplete_sources/issues",
  "/groups/{group_id:*}/-/autocomplete_sources/iterations",
  "/groups/{group_id:*}/-/autocomplete_sources/labels",
  "/groups/{group_id:*}/-/autocomplete_sources/members",
  "/groups/{group_id:*}/-/autocomplete_sources/merge_requests",
  "/groups/{group_id:*}/-/autocomplete_sources/milestones",
  "/groups/{group_id:*}/-/autocomplete_sources/vulnerabilities",
  "/groups/{group_id:*}/-/autocomplete_sources/wikis",

  // groups/avatars
  "/groups/{group_id:*}/-/avatar",

  // groups/billings
  "/groups/{group_id:*}/-/billings",
  "/groups/{group_id:*}/-/billings/refresh_seats",

  // groups/boards
  "/groups/{group_id:*}/-/boards",
  "/groups/{group_id:*}/-/boards/{id}",

  // groups/children
  "/groups/{group_id:*}/-/children",

  // groups/clusters
  "/groups/{group_id:*}/-/clusters",
  "/groups/{group_id:*}/-/clusters/{id}",
  "/groups/{group_id:*}/-/clusters/{id}/clear_cache",
  "/groups/{group_id:*}/-/clusters/{id}/cluster_status",
  "/groups/{group_id:*}/-/clusters/{id}/environments",
  "/groups/{group_id:*}/-/clusters/{id}/metrics",
  "/groups/{group_id:*}/-/clusters/{id}/metrics_dashboard",
  "/groups/{group_id:*}/-/clusters/connect",
  "/groups/{group_id:*}/-/clusters/create_user",
  "/groups/{group_id:*}/-/clusters/new_cluster_docs",

  // groups/clusters/integrations
  "/groups/{group_id:*}/-/clusters/{cluster_id}/integration/create_or_update",

  // groups/comment_templates
  "/groups/{group_id:*}/-/comment_templates",
  "/groups/{group_id:*}/-/comment_templates/{id}",

  // groups/confluences
  "/groups/{group_id:*}/-/wikis/-/confluence",

  // groups/contribution_analytics
  "/groups/{group_id:*}/-/contribution_analytics",

  // groups/crm/contacts
  "/groups/{group_id:*}/-/crm/contacts",
  "/groups/{group_id:*}/-/crm/contacts/{id}/edit",
  "/groups/{group_id:*}/-/crm/contacts/new",

  // groups/crm/organizations
  "/groups/{group_id:*}/-/crm/organizations",
  "/groups/{group_id:*}/-/crm/organizations/{id}/edit",
  "/groups/{group_id:*}/-/crm/organizations/new",

  // groups/custom_emoji
  "/groups/{group_id:*}/-/custom_emoji",
  "/groups/{group_id:*}/-/custom_emoji/new",

  // groups/dependencies
  "/groups/{group_id:*}/-/dependencies",
  "/groups/{group_id:*}/-/dependencies/licenses",
  "/groups/{group_id:*}/-/dependencies/locations",

  // groups/dependency_proxies
  "/groups/{group_id:*}/-/dependency_proxy",

  // groups/dependency_proxy_auth
  "/v2",

  // groups/dependency_proxy_for_containers
  "/v2/{group_id:*}/dependency_proxy/containers/{image:*}/blobs/{sha}",
  "/v2/{group_id:*}/dependency_proxy/containers/{image:*}/blobs/{sha}/upload",
  "/v2/{group_id:*}/dependency_proxy/containers/{image:*}/blobs/{sha}/upload/authorize",
  "/v2/{group_id:*}/dependency_proxy/containers/{image:*}/manifests/{tag:*}",
  "/v2/{group_id:*}/dependency_proxy/containers/{image:*}/manifests/{tag:*}/upload",
  "/v2/{group_id:*}/dependency_proxy/containers/{image:*}/manifests/{tag:*}/upload/authorize",

  // groups/deploy_tokens
  "/groups/{group_id:*}/-/deploy_tokens/{id}/revoke",

  // groups/discovers
  "/groups/{group_id:*}/-/discover",

  // groups/epic_boards
  "/groups/{group_id:*}/-/epic_boards",
  "/groups/{group_id:*}/-/epic_boards/{id}",

  // groups/epic_issues
  "/groups/{group_id:*}/-/epics/{epic_id}/issues",
  "/groups/{group_id:*}/-/epics/{epic_id}/issues/{id}",

  // groups/epics
  "/groups/{group_id:*}/-/epics",
  "/groups/{group_id:*}/-/epics/{id}",
  "/groups/{group_id:*}/-/epics/{id}/descriptions/{version_id}",
  "/groups/{group_id:*}/-/epics/{id}/descriptions/{version_id}/diff",
  "/groups/{group_id:*}/-/epics/{id}/discussions",
  "/groups/{group_id:*}/-/epics/{id}/edit",
  "/groups/{group_id:*}/-/epics/{id}/realtime_changes",
  "/groups/{group_id:*}/-/epics/{id}/toggle_award_emoji",
  "/groups/{group_id:*}/-/epics/{id}/toggle_subscription",
  "/groups/{group_id:*}/-/epics/bulk_update",
  "/groups/{group_id:*}/-/epics/new",

  // groups/epics/epic_links
  "/groups/{group_id:*}/-/epics/{epic_id}/links",
  "/groups/{group_id:*}/-/epics/{epic_id}/links/{id}",

  // groups/epics/notes
  "/groups/{group_id:*}/-/epics/{epic_id}/notes",
  "/groups/{group_id:*}/-/epics/{epic_id}/notes/{id}",
  "/groups/{group_id:*}/-/epics/{epic_id}/notes/{id}/toggle_award_emoji",

  // groups/epics/related_epic_links
  "/groups/{group_id:*}/-/epics/{epic_id}/related_epic_links",
  "/groups/{group_id:*}/-/epics/{epic_id}/related_epic_links/{id}",

  // groups/group_links
  "/groups/{group_id:*}/-/group_links/{id}",

  // groups/group_members
  "/groups/{group_id:*}/-/group_members",
  "/groups/{group_id:*}/-/group_members/{id}",
  "/groups/{group_id:*}/-/group_members/{id}/approve_access_request",
  "/groups/{group_id:*}/-/group_members/{id}/ban",
  "/groups/{group_id:*}/-/group_members/{id}/override",
  "/groups/{group_id:*}/-/group_members/{id}/resend_invite",
  "/groups/{group_id:*}/-/group_members/{id}/unban",
  "/groups/{group_id:*}/-/group_members/export_csv",
  "/groups/{group_id:*}/-/group_members/leave",
  "/groups/{group_id:*}/-/group_members/request_access",

  // groups/harbor/artifacts
  "/groups/{group_id:*}/-/harbor/repositories/{repository_id}/artifacts",

  // groups/harbor/repositories
  "/groups/{group_id:*}/-/harbor/repositories",
  "/groups/{group_id:*}/-/harbor/repositories/{id}",

  // groups/harbor/tags
  "/groups/{group_id:*}/-/harbor/repositories/{repository_id}/artifacts/{artifact_id}/tags",

  // groups/hook_logs
  "/groups/{group_id:*}/-/hooks/{hook_id}/hook_logs/{id}",
  "/groups/{group_id:*}/-/hooks/{hook_id}/hook_logs/{id}/retry",

  // groups/hooks
  "/groups/{group_id:*}/-/hooks",
  "/groups/{group_id:*}/-/hooks/{id}",
  "/groups/{group_id:*}/-/hooks/{id}/edit",
  "/groups/{group_id:*}/-/hooks/{id}/test",

  // groups/imports
  "/groups/{group_id:*}/-/import",

  // groups/infrastructure_registry
  "/groups/{group_id:*}/-/terraform_module_registry",

  // groups/insights
  "/groups/{group_id:*}/-/insights",
  "/groups/{group_id:*}/-/insights/query",

  // groups/issues
  "/groups/{group_id:*}/-/issues/bulk_update",

  // groups/issues_analytics
  "/groups/{group_id:*}/-/issues_analytics",

  // groups/iteration_cadences
  "/groups/{group_id:*}/-/cadences",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/{id}",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/{id}/edit",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/{iteration_cadence_id}/iterations",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/{iteration_cadence_id}/iterations/{id}",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/{iteration_cadence_id}/iterations/{id}/edit",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/{iteration_cadence_id}/iterations/new",
  "/groups/{group_id:*}/-/cadences/{vueroute:*}/new",
  "/groups/{group_id:*}/-/cadences/{id}",
  "/groups/{group_id:*}/-/cadences/{id}/edit",
  "/groups/{group_id:*}/-/cadences/{iteration_cadence_id}/iterations",
  "/groups/{group_id:*}/-/cadences/{iteration_cadence_id}/iterations/{id}",
  "/groups/{group_id:*}/-/cadences/{iteration_cadence_id}/iterations/{id}/edit",
  "/groups/{group_id:*}/-/cadences/{iteration_cadence_id}/iterations/new",
  "/groups/{group_id:*}/-/cadences/new",

  // groups/iterations
  "/groups/{group_id:*}/-/iterations",
  "/groups/{group_id:*}/-/iterations/{id}",
  "/groups/{group_id:*}/-/iterations/{id}/edit",
  "/groups/{group_id:*}/-/iterations/new",

  // groups/labels
  "/groups/{group_id:*}/-/labels",
  "/groups/{group_id:*}/-/labels/{id}",
  "/groups/{group_id:*}/-/labels/{id}/edit",
  "/groups/{group_id:*}/-/labels/{id}/toggle_subscription",
  "/groups/{group_id:*}/-/labels/new",

  // groups/ldap_group_links
  "/groups/{group_id:*}/-/ldap_group_links",
  "/groups/{group_id:*}/-/ldap_group_links/{id}",

  // groups/ldaps
  "/groups/{group_id:*}/-/ldap/sync",

  // groups/merge_requests
  "/groups/{group_id:*}/-/merge_requests/bulk_update",

  // groups/milestones
  "/groups/{group_id:*}/-/milestones",
  "/groups/{group_id:*}/-/milestones/{id}",
  "/groups/{group_id:*}/-/milestones/{id}/edit",
  "/groups/{group_id:*}/-/milestones/{id}/issues",
  "/groups/{group_id:*}/-/milestones/{id}/labels",
  "/groups/{group_id:*}/-/milestones/{id}/merge_requests",
  "/groups/{group_id:*}/-/milestones/{id}/participants",
  "/groups/{group_id:*}/-/milestones/new",

  // groups/notification_settings
  "/groups/{group_id:*}/-/notification_setting",

  // groups/omniauth_callbacks
  "/groups/{group_id:*}/-/saml/callback",

  // groups/packages
  "/groups/{group_id:*}/-/packages",
  "/groups/{group_id:*}/-/packages/{id}",

  // groups/protected_branches
  "/groups/{group_id:*}/-/protected_branches",
  "/groups/{group_id:*}/-/protected_branches/{id}",

  // groups/protected_environments
  "/groups/{group_id:*}/-/protected_environments",
  "/groups/{group_id:*}/-/protected_environments/{id}",

  // groups/push_rules
  "/groups/{group_id:*}/-/push_rules",

  // groups/registry/repositories
  "/groups/{group_id:*}/-/container_registries",
  "/groups/{group_id:*}/-/container_registries/{id}",

  // groups/releases
  "/groups/{group_id:*}/-/releases",

  // groups/roadmap
  "/groups/{group_id:*}/-/roadmap",

  // groups/runners
  "/groups/{group_id:*}/-/runners",
  "/groups/{group_id:*}/-/runners/{id}",
  "/groups/{group_id:*}/-/runners/{id}/edit",
  "/groups/{group_id:*}/-/runners/{id}/pause",
  "/groups/{group_id:*}/-/runners/{id}/register",
  "/groups/{group_id:*}/-/runners/{id}/resume",
  "/groups/{group_id:*}/-/runners/dashboard",
  "/groups/{group_id:*}/-/runners/new",

  // groups/saml_group_links
  "/groups/{group_id:*}/-/saml_group_links",
  "/groups/{group_id:*}/-/saml_group_links/{id}",

  // groups/saml_providers
  "/groups/{group_id:*}/-/saml",
  "/groups/{group_id:*}/-/saml/update_microsoft_application",

  // groups/scim_oauth
  "/groups/{group_id:*}/-/scim_oauth",

  // groups/seat_usage
  "/groups/{group_id:*}/-/seat_usage",

  // groups/security/compliance_dashboards
  "/groups/{group_id:*}/-/security/compliance_dashboard",
  "/groups/{group_id:*}/-/security/compliance_dashboard/{vueroute:*}",

  // groups/security/compliance_framework_reports
  "/groups/{group_id:*}/-/security/compliance_framework_reports",

  // groups/security/compliance_project_framework_reports
  "/groups/{group_id:*}/-/security/compliance_project_framework_reports",

  // groups/security/compliance_standards_adherence_reports
  "/groups/{group_id:*}/-/security/compliance_standards_adherence_reports",

  // groups/security/compliance_violation_reports
  "/groups/{group_id:*}/-/security/compliance_violation_reports",

  // groups/security/credentials
  "/groups/{group_id:*}/-/security/credentials",
  "/groups/{group_id:*}/-/security/credentials/{id}",
  "/groups/{group_id:*}/-/security/credentials/{id}/revoke",

  // groups/security/dashboard
  "/groups/{group_id:*}/-/security/dashboard",

  // groups/security/discover
  "/groups/{group_id:*}/-/security/discover",

  // groups/security/merge_commit_reports
  "/groups/{group_id:*}/-/security/merge_commit_reports",

  // groups/security/policies
  "/groups/{group_id:*}/-/security/policies",
  "/groups/{group_id:*}/-/security/policies/{id}/edit",
  "/groups/{group_id:*}/-/security/policies/new",
  "/groups/{group_id:*}/-/security/policies/schema",

  // groups/security/vulnerabilities
  "/groups/{group_id:*}/-/security/vulnerabilities",

  // groups/service_accounts
  "/groups/{group_id:*}/-/service_accounts",
  "/groups/{group_id:*}/-/service_accounts/{vueroute:*}",
  "/groups/{group_id:*}/-/service_accounts/{vueroute:*}/{id}",
  "/groups/{group_id:*}/-/service_accounts/{vueroute:*}/{id}/edit",
  "/groups/{group_id:*}/-/service_accounts/{vueroute:*}/new",
  "/groups/{group_id:*}/-/service_accounts/{id}",
  "/groups/{group_id:*}/-/service_accounts/{id}/edit",
  "/groups/{group_id:*}/-/service_accounts/new",

  // groups/settings/access_tokens
  "/groups/{group_id:*}/-/settings/access_tokens",
  "/groups/{group_id:*}/-/settings/access_tokens/{id}/revoke",

  // groups/settings/analytics
  "/groups/{group_id:*}/-/settings/analytics",

  // groups/settings/applications
  "/groups/{group_id:*}/-/settings/applications",
  "/groups/{group_id:*}/-/settings/applications/{id}",
  "/groups/{group_id:*}/-/settings/applications/{id}/edit",
  "/groups/{group_id:*}/-/settings/applications/{id}/renew",
  "/groups/{group_id:*}/-/settings/applications/new",

  // groups/settings/ci_cd
  "/groups/{group_id:*}/-/settings/ci_cd",
  "/groups/{group_id:*}/-/settings/ci_cd/reset_registration_token",
  "/groups/{group_id:*}/-/settings/ci_cd/runner_setup_scripts",
  "/groups/{group_id:*}/-/settings/ci_cd/update_auto_devops",

  // groups/settings/domain_verification
  "/groups/{group_id:*}/-/settings/domain_verification",
  "/groups/{group_id:*}/-/settings/domain_verification/{id}",
  "/groups/{group_id:*}/-/settings/domain_verification/{id}/clean_certificate",
  "/groups/{group_id:*}/-/settings/domain_verification/{id}/retry_auto_ssl",
  "/groups/{group_id:*}/-/settings/domain_verification/{id}/verify",
  "/groups/{group_id:*}/-/settings/domain_verification/new",

  // groups/settings/integrations
  "/groups/{group_id:*}/-/settings/integrations",
  "/groups/{group_id:*}/-/settings/integrations/{id}",
  "/groups/{group_id:*}/-/settings/integrations/{id}/edit",
  "/groups/{group_id:*}/-/settings/integrations/{id}/reset",
  "/groups/{group_id:*}/-/settings/integrations/{id}/test",

  // groups/settings/merge_requests
  "/groups/{group_id:*}/-/settings/merge_requests",

  // groups/settings/packages_and_registries
  "/groups/{group_id:*}/-/settings/packages_and_registries",

  // groups/settings/remote_development/workspaces
  "/groups/{group_id:*}/-/settings/workspaces",

  // groups/settings/reporting
  "/groups/{group_id:*}/-/settings/reporting",

  // groups/settings/repository
  "/groups/{group_id:*}/-/settings/ci_cd/deploy_token/create",
  "/groups/{group_id:*}/-/settings/repository",
  "/groups/{group_id:*}/-/settings/repository/deploy_token/create",

  // groups/settings/roles_and_permissions
  "/groups/{group_id:*}/-/settings/roles_and_permissions",
  "/groups/{group_id:*}/-/settings/roles_and_permissions/{id}/edit",
  "/groups/{group_id:*}/-/settings/roles_and_permissions/new",

  // groups/settings/slacks
  "/groups/{group_id:*}/-/settings/slack",
  "/groups/{group_id:*}/-/settings/slack/slack_auth",

  // groups/shared_projects
  "/groups/{group_id:*}/-/shared_projects",

  // groups/sso
  "/groups/{group_id:*}/-/saml/sso",
  "/groups/{group_id:*}/-/saml/unlink",

  // groups/todos
  "/groups/{group_id:*}/-/todos",

  // groups/two_factor_auths
  "/groups/{group_id:*}/-/two_factor_auth",

  // groups/uploads
  "/groups/{group_id:*}/-/uploads",
  "/groups/{group_id:*}/-/uploads/{secret}/{filename}",
  "/groups/{group_id:*}/-/uploads/authorize",

  // groups/usage_quotas
  "/groups/{group_id:*}/-/usage_quotas",
  "/groups/{group_id:*}/-/usage_quotas/pending_members",

  // groups/variables
  "/groups/{group_id:*}/-/variables",

  // groups/wikis
  "/groups/{group_id:*}/-/wikis",
  "/groups/{group_id:*}/-/wikis/{id:*}",
  "/groups/{group_id:*}/-/wikis/{id:*}/diff",
  "/groups/{group_id:*}/-/wikis/{id:*}/edit",
  "/groups/{group_id:*}/-/wikis/{id:*}/history",
  "/groups/{group_id:*}/-/wikis/{id:*}/preview_markdown",
  "/groups/{group_id:*}/-/wikis/{id:*}/raw",
  "/groups/{group_id:*}/-/wikis/git_access",
  "/groups/{group_id:*}/-/wikis/new",
  "/groups/{group_id:*}/-/wikis/pages",
  "/groups/{group_id:*}/-/wikis/templates",

  // groups/work_items
  "/groups/{group_id:*}/-/work_items",
  "/groups/{group_id:*}/-/work_items/{iid}",
  "/groups/{group_id:*}/-/work_items/{iid}/descriptions/{version_id}",
  "/groups/{group_id:*}/-/work_items/{iid}/descriptions/{version_id}/diff",

  // health
  "/-/liveness",
  "/-/readiness",

  // health_check
  "/health_check",
  "/health_check/{checks}",

  // help
  "/help",
  "/help/{path:*}",
  "/help/docs",
  "/help/drawers/{markdown_file:*}",
  "/help/instance_configuration",
  "/help/shortcuts",

  // ide
  "/-/ide",
  "/-/ide/oauth_redirect",
  "/-/ide/project",
  "/-/ide/project/{project_id}",
  "/-/ide/project/{project_id}/blob",
  "/-/ide/project/{project_id}/blob/{branch:*}",
  "/-/ide/project/{project_id}/blob/{branch:*}/-",
  "/-/ide/project/{project_id}/blob/{branch:*}/-/{path:*}",
  "/-/ide/project/{project_id}/edit",
  "/-/ide/project/{project_id}/edit/{branch:*}",
  "/-/ide/project/{project_id}/edit/{branch:*}/-",
  "/-/ide/project/{project_id}/edit/{branch:*}/-/{path:*}",
  "/-/ide/project/{project_id}/merge_requests/{merge_request_id}",
  "/-/ide/project/{project_id}/tree",
  "/-/ide/project/{project_id}/tree/{branch:*}",
  "/-/ide/project/{project_id}/tree/{branch:*}/-",
  "/-/ide/project/{project_id}/tree/{branch:*}/-/{path:*}",

  // import/bitbucket
  "/import/bitbucket",
  "/import/bitbucket/callback",
  "/import/bitbucket/realtime_changes",
  "/import/bitbucket/status",

  // import/bitbucket_server
  "/import/bitbucket_server",
  "/import/bitbucket_server/callback",
  "/import/bitbucket_server/configure",
  "/import/bitbucket_server/new",
  "/import/bitbucket_server/realtime_changes",
  "/import/bitbucket_server/status",

  // import/bulk_imports
  "/import/bulk_imports",
  "/import/bulk_imports/{id}/history",
  "/import/bulk_imports/{id}/history/{entity_id}/failures",
  "/import/bulk_imports/configure",
  "/import/bulk_imports/history",
  "/import/bulk_imports/realtime_changes",
  "/import/bulk_imports/status",

  // import/fogbugz
  "/import/fogbugz",
  "/import/fogbugz/callback",
  "/import/fogbugz/new",
  "/import/fogbugz/realtime_changes",
  "/import/fogbugz/status",
  "/import/fogbugz/user_map",

  // import/gitea
  "/import/gitea",
  "/import/gitea/new",
  "/import/gitea/personal_access_token",
  "/import/gitea/realtime_changes",
  "/import/gitea/status",

  // import/github
  "/import/github",
  "/import/github/callback",
  "/import/github/cancel",
  "/import/github/cancel_all",
  "/import/github/counts",
  "/import/github/details",
  "/import/github/failures",
  "/import/github/new",
  "/import/github/personal_access_token",
  "/import/github/realtime_changes",
  "/import/github/status",

  // import/github_groups
  "/import/github_group/status",

  // import/gitlab_groups
  "/import/gitlab_group",
  "/import/gitlab_group/authorize",

  // import/gitlab_projects
  "/import/gitlab_project",
  "/import/gitlab_project/authorize",
  "/import/gitlab_project/new",

  // import/google_oauth2
  "/users/auth/-/import/google_oauth2/callback",

  // import/history
  "/import/history",

  // import/manifest
  "/import/manifest",
  "/import/manifest/new",
  "/import/manifest/realtime_changes",
  "/import/manifest/status",
  "/import/manifest/upload",

  // import/source_users
  "/import/source_users/{id}",
  "/import/source_users/{id}/accept",
  "/import/source_users/{id}/decline",

  // import/url
  "/import/url/validate",

  // invites
  "/-/invites/{id}",
  "/-/invites/{id}/accept",
  "/-/invites/{id}/decline",

  // jira_connect/app_descriptor
  "/-/jira_connect/app_descriptor",

  // jira_connect/branches
  "/-/jira_connect/branches/new",
  "/-/jira_connect/branches/route",

  // jira_connect/events
  "/-/jira_connect/events/installed",
  "/-/jira_connect/events/uninstalled",

  // jira_connect/installations
  "/-/jira_connect/installations",

  // jira_connect/oauth_application_ids
  "/-/jira_connect/oauth_application_id",

  // jira_connect/oauth_callbacks
  "/-/jira_connect/oauth_callbacks",

  // jira_connect/public_keys
  "/-/jira_connect/public_keys/{id}",

  // jira_connect/repositories
  "/-/jira_connect/repositories/associate",
  "/-/jira_connect/repositories/search",

  // jira_connect/subscriptions
  "/-/jira_connect/subscriptions",
  "/-/jira_connect/subscriptions/{id}",

  // jira_connect/workspaces
  "/-/jira_connect/workspaces/search",

  // jwks
  "/-/jwks",
  "/.well-known/oauth-authorization-server",
  "/.well-known/openid-configuration",
  "/.well-known/webfinger",
  "/oauth/discovery/keys",

  // jwt
  "/jwt/auth",

  // mailgun/webhooks
  "/-/mailgun/webhooks",
  "/-/members/mailgun/permanent_failures",

  // metrics
  "/-/metrics",
  "/-/metrics/system",

  // oauth/applications
  "/-/user_settings/applications",
  "/oauth/applications",
  "/oauth/applications/{id}",
  "/oauth/applications/{id}/edit",
  "/oauth/applications/{id}/renew",
  "/oauth/applications/new",

  // oauth/authorizations
  "/oauth/authorize",
  "/oauth/authorize/native",

  // oauth/authorized_applications
  "/oauth/authorized_applications",
  "/oauth/authorized_applications/{id}",

  // oauth/geo_auth
  "/oauth/geo/auth",
  "/oauth/geo/callback",
  "/oauth/geo/logout",

  // oauth/token_info
  "/oauth/token/info",

  // oauth/tokens
  "/oauth/introspect",
  "/oauth/revoke",
  "/oauth/token",

  // omniauth_callbacks
  "/users/auth/google_oauth2",
  "/users/auth/google_oauth2/callback",

  // omniauth_kerberos
  "/users/auth/kerberos/negotiate",

  // operations
  "/-/operations",
  "/-/operations/environments",

  // organizations/groups
  "/-/organizations/{organization_path}/groups",
  "/-/organizations/{organization_path}/groups/{id:*}/edit",
  "/-/organizations/{organization_path}/groups/new",

  // organizations/organizations
  "/-/organizations",
  "/-/organizations/{organization_path}",
  "/-/organizations/{organization_path}/activity",
  "/-/organizations/{organization_path}/groups_and_projects",
  "/-/organizations/{organization_path}/users",
  "/-/organizations/new",
  "/-/organizations/preview_markdown",

  // organizations/projects
  "/-/organizations/{organization_path}/projects/{namespace_id:*}/{id}/edit",

  // organizations/settings
  "/-/organizations/{organization_path}/settings/general",

  // passwords
  "/users/password",
  "/users/password/edit",
  "/users/password/new",

  // phone_verification/telesign_callbacks
  "/-/phone_verification/telesign_callback",

  // profiles
  "/-/profile/reset_feed_token",
  "/-/profile/reset_incoming_email_token",
  "/-/profile/reset_static_object_token",
  "/-/profile/update_username",

  // profiles/accounts
  "/-/profile/account",
  "/-/profile/account/unlink",

  // profiles/avatars
  "/-/profile/avatar",

  // profiles/billings
  "/-/profile/billings",

  // profiles/chat_names
  "/-/profile/chat_names",
  "/-/profile/chat_names/{id}",
  "/-/profile/chat_names/deny",
  "/-/profile/chat_names/new",

  // profiles/comment_templates
  "/-/profile/comment_templates",
  "/-/profile/comment_templates/{id}",

  // profiles/emails
  "/-/profile/emails",
  "/-/profile/emails/{id}",
  "/-/profile/emails/{id}/resend_confirmation_instructions",

  // profiles/groups
  "/-/profile/groups/{id:*}/notifications",

  // profiles/notifications
  "/-/profile/notifications",

  // profiles/preferences
  "/-/profile/preferences",

  // profiles/slacks
  "/-/profile/slack/edit",
  "/-/profile/slack/slack_link",

  // profiles/two_factor_auths
  "/-/profile/two_factor_auth",
  "/-/profile/two_factor_auth/codes",
  "/-/profile/two_factor_auth/create_webauthn",
  "/-/profile/two_factor_auth/skip",

  // profiles/usage_quotas
  "/-/profile/usage_quotas",

  // profiles/webauthn_registrations
  "/-/profile/webauthn_registrations/{id}",

  // projects
  "/{namespace_id:*}/{id}",
  "/{namespace_id:*}/{id}/activity",
  "/{namespace_id:*}/{id}/archive",
  "/{namespace_id:*}/{id}/download_export",
  "/{namespace_id:*}/{id}/edit",
  "/{namespace_id:*}/{id}/export",
  "/{namespace_id:*}/{id}/generate_new_export",
  "/{namespace_id:*}/{id}/housekeeping",
  "/{namespace_id:*}/{id}/new_issuable_address",
  "/{namespace_id:*}/{id}/preview_markdown",
  "/{namespace_id:*}/{id}/refs",
  "/{namespace_id:*}/{id}/remove_export",
  "/{namespace_id:*}/{id}/remove_fork",
  "/{namespace_id:*}/{id}/toggle_star",
  "/{namespace_id:*}/{id}/transfer",
  "/{namespace_id:*}/{id}/unarchive",
  "/{namespace_id:*}/{id}/unfoldered_environment_names",
  "/{namespace_id:*}/{project_id}/-/planning_hierarchy",
  "/{namespace_id:*}/{project_id}/-/preview_markdown",
  "/{namespace_id:*}/{project_id}/restore",
  "/projects",
  "/projects/new",

  // projects/alert_management
  "/{namespace_id:*}/{project_id}/-/alert_management",
  "/{namespace_id:*}/{project_id}/-/alert_management/{id}",
  "/{namespace_id:*}/{project_id}/-/alert_management/{id}/details",
  "/{namespace_id:*}/{project_id}/-/alert_management/{id}/details/{page:*}",

  // projects/alerting/notifications
  "/{namespace_id:*}/{project_id}/alerts/notify",
  "/{namespace_id:*}/{project_id}/alerts/notify/{name}/{endpoint_identifier}",
  "/{namespace_id:*}/{project_id}/prometheus/alerts/notify",

  // projects/alerts
  "/{namespace_id:*}/{project_id}/prometheus/alerts/{id}/metrics_dashboard",

  // projects/analytics/code_reviews
  "/{namespace_id:*}/{project_id}/-/analytics/code_reviews",

  // projects/analytics/cycle_analytics
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics",

  // projects/analytics/cycle_analytics/stages
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/average_duration_chart",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/count",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/median",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{value_stream_id}/stages/{id}/records",

  // projects/analytics/cycle_analytics/summary
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/summary",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/time_summary",

  // projects/analytics/cycle_analytics/value_streams
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/analytics/value_stream_analytics/value_streams/new",

  // projects/analytics/dashboards
  "/{namespace_id:*}/{project_id}/-/analytics/dashboards",
  "/{namespace_id:*}/{project_id}/-/analytics/dashboards/{vueroute:*}",

  // projects/analytics/issues_analytics
  "/{namespace_id:*}/{project_id}/-/analytics/issues_analytics",

  // projects/analytics/merge_request_analytics
  "/{namespace_id:*}/{project_id}/-/analytics/merge_request_analytics",

  // projects/approver_groups
  "/{namespace_id:*}/{project_id}/-/approver_groups/{id}",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/approver_groups/{id}",

  // projects/approvers
  "/{namespace_id:*}/{project_id}/-/approvers/{id}",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/approvers",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/approvers/{id}",

  // projects/artifacts
  "/{namespace_id:*}/{project_id}/-/artifacts",
  "/{namespace_id:*}/{project_id}/-/artifacts/{id}",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/browse",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/browse/{path:*}",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/download",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/external_file/{path:*}",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/file/{path:*}",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/keep",
  "/{namespace_id:*}/{project_id}/-/jobs/{job_id}/artifacts/raw/{path:*}",
  "/{namespace_id:*}/{project_id}/-/jobs/artifacts/{ref_name_and_path:*}",

  // projects/audit_events
  "/{namespace_id:*}/{project_id}/-/audit_events",

  // projects/autocomplete_sources
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/commands",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/contacts",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/epics",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/issues",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/iterations",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/labels",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/members",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/merge_requests",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/milestones",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/snippets",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/vulnerabilities",
  "/{namespace_id:*}/{project_id}/-/autocomplete_sources/wikis",

  // projects/automations
  "/{namespace_id:*}/{project_id}/-/automations",

  // projects/avatars
  "/{namespace_id:*}/{project_id}/-/avatar",

  // projects/aws/configuration
  "/{namespace_id:*}/{project_id}/-/aws/configuration",

  // projects/badges
  "/{namespace_id:*}/{project_id}/-/badges/release",
  "/{namespace_id:*}/{project_id}/badges",
  "/{namespace_id:*}/{project_id}/badges/{ref:*}/coverage",
  "/{namespace_id:*}/{project_id}/badges/{ref:*}/pipeline",

  // projects/blame
  "/{namespace_id:*}/{project_id}/-/blame_page/{id:*}",
  "/{namespace_id:*}/{project_id}/-/blame/{id:*}",
  "/{namespace_id:*}/{project_id}/-/blame/{id:*}/streaming",
  "/{namespace_id:*}/{project_id}/blame/{id:*}",

  // projects/blob
  "/{namespace_id:*}/{project_id}/-/blob/{id:*}",
  "/{namespace_id:*}/{project_id}/-/blob/{id:*}/diff",
  "/{namespace_id:*}/{project_id}/-/create/{id:*}",
  "/{namespace_id:*}/{project_id}/-/edit/{id:*}",
  "/{namespace_id:*}/{project_id}/-/new/{id:*}",
  "/{namespace_id:*}/{project_id}/-/preview/{id:*}",
  "/{namespace_id:*}/{project_id}/-/update/{id:*}",
  "/{namespace_id:*}/{project_id}/blob/{id:*}",

  // projects/boards
  "/{namespace_id:*}/{project_id}/-/boards",
  "/{namespace_id:*}/{project_id}/-/boards/{id}",

  // projects/branches
  "/{namespace_id:*}/{project_id}/-/branches",
  "/{namespace_id:*}/{project_id}/-/branches/{id}",
  "/{namespace_id:*}/{project_id}/-/branches/{state}",
  "/{namespace_id:*}/{project_id}/-/branches/diverging_commit_counts",
  "/{namespace_id:*}/{project_id}/-/branches/new",
  "/{namespace_id:*}/{project_id}/-/merged_branches",

  // projects/build_artifacts
  "/{namespace_id:*}/{project_id}/builds/{build_id}/artifacts/browse",
  "/{namespace_id:*}/{project_id}/builds/{build_id}/artifacts/browse/{path:*}",
  "/{namespace_id:*}/{project_id}/builds/{build_id}/artifacts/download",
  "/{namespace_id:*}/{project_id}/builds/{build_id}/artifacts/file/{path:*}",
  "/{namespace_id:*}/{project_id}/builds/{build_id}/artifacts/raw/{path:*}",
  "/{namespace_id:*}/{project_id}/builds/artifacts/{ref_name_and_path:*}",

  // projects/builds
  "/{namespace_id:*}/{project_id}/builds",
  "/{namespace_id:*}/{project_id}/builds/{id}",
  "/{namespace_id:*}/{project_id}/builds/{id}/raw",

  // projects/ci/daily_build_group_report_results
  "/{namespace_id:*}/{project_id}/-/ci/daily_build_group_report_results",

  // projects/ci/lints
  "/{namespace_id:*}/{project_id}/-/ci/lint",

  // projects/ci/pipeline_editor
  "/{namespace_id:*}/{project_id}/-/ci/editor",

  // projects/ci/prometheus_metrics/histograms
  "/{namespace_id:*}/{project_id}/-/ci/prometheus_metrics/histograms",

  // projects/cluster_agents
  "/{namespace_id:*}/{project_id}/-/cluster_agents/{name}",

  // projects/clusters
  "/{namespace_id:*}/{project_id}/-/clusters",
  "/{namespace_id:*}/{project_id}/-/clusters/{id}",
  "/{namespace_id:*}/{project_id}/-/clusters/{id}/clear_cache",
  "/{namespace_id:*}/{project_id}/-/clusters/{id}/cluster_status",
  "/{namespace_id:*}/{project_id}/-/clusters/{id}/environments",
  "/{namespace_id:*}/{project_id}/-/clusters/{id}/metrics",
  "/{namespace_id:*}/{project_id}/-/clusters/{id}/metrics_dashboard",
  "/{namespace_id:*}/{project_id}/-/clusters/connect",
  "/{namespace_id:*}/{project_id}/-/clusters/create_user",
  "/{namespace_id:*}/{project_id}/-/clusters/new_cluster_docs",

  // projects/clusters/integrations
  "/{namespace_id:*}/{project_id}/-/clusters/{cluster_id}/integration/create_or_update",

  // projects/comment_templates
  "/{namespace_id:*}/{project_id}/-/comment_templates",
  "/{namespace_id:*}/{project_id}/-/comment_templates/{id}",

  // projects/commit
  "/{namespace_id:*}/{project_id}/-/commit/{id}",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/branches",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/cherry_pick",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/diff_files",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/diff_for_path",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/merge_requests",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/pipelines",
  "/{namespace_id:*}/{project_id}/-/commit/{id}/revert",

  // projects/commits
  "/{namespace_id:*}/{project_id}/-/commits",
  "/{namespace_id:*}/{project_id}/-/commits/{id:*}",
  "/{namespace_id:*}/{project_id}/-/commits/{id:*}/signatures",

  // projects/compare
  "/{namespace_id:*}/{project_id}/-/compare",
  "/{namespace_id:*}/{project_id}/-/compare/{from}...{to}",
  "/{namespace_id:*}/{project_id}/-/compare/diff_for_path",
  "/{namespace_id:*}/{project_id}/-/compare/signatures",

  // projects/compliance_frameworks
  "/{namespace_id:*}/{project_id}/-/compliance_frameworks",

  // projects/confluences
  "/{namespace_id:*}/{project_id}/-/wikis/-/confluence",

  // projects/cycle_analytics
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics",

  // projects/cycle_analytics/events
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/code",
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/issue",
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/plan",
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/production",
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/review",
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/staging",
  "/{namespace_id:*}/{project_id}/-/value_stream_analytics/events/test",

  // projects/dependencies
  "/{namespace_id:*}/{project_id}/-/dependencies",

  // projects/deploy_keys
  "/{namespace_id:*}/{project_id}/-/deploy_keys",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/{id}",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/{id}/disable",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/{id}/enable",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/available_project_keys",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/available_public_keys",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/enabled_keys",
  "/{namespace_id:*}/{project_id}/-/deploy_keys/new",

  // projects/deploy_tokens
  "/{namespace_id:*}/{project_id}/-/deploy_tokens/{id}/revoke",

  // projects/deployments
  "/{namespace_id:*}/{project_id}/-/environments/{environment_id}/deployments",
  "/{namespace_id:*}/{project_id}/-/environments/{environment_id}/deployments/{id}",
  "/{namespace_id:*}/{project_id}/-/environments/{environment_id}/deployments/{id}/additional_metrics",
  "/{namespace_id:*}/{project_id}/-/environments/{environment_id}/deployments/{id}/metrics",

  // projects/design_management/designs/raw_images
  "/{namespace_id:*}/{project_id}/-/design_management/designs/{design_id}/{sha}/raw_image",
  "/{namespace_id:*}/{project_id}/-/design_management/designs/{design_id}/raw_image",

  // projects/design_management/designs/resized_image
  "/{namespace_id:*}/{project_id}/-/design_management/designs/{design_id}/{sha}/resized_image/{id}",
  "/{namespace_id:*}/{project_id}/-/design_management/designs/{design_id}/resized_image/{id}",

  // projects/discussions
  "/{namespace_id:*}/{project_id}/-/{noteable_type}/{noteable_id}/discussions/{id}",
  "/{namespace_id:*}/{project_id}/-/{noteable_type}/{noteable_id}/discussions/{id}/resolve",

  // projects/environments
  "/{namespace_id:*}/{project_id}/-/environments",
  "/{namespace_id:*}/{project_id}/-/environments/{id}",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/cancel_auto_stop",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/k8s",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/k8s/{vueroute:*}",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/stop",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/terminal",
  "/{namespace_id:*}/{project_id}/-/environments/{id}/terminal.ws/authorize",
  "/{namespace_id:*}/{project_id}/-/environments/folders/{id:*}",
  "/{namespace_id:*}/{project_id}/-/environments/new",
  "/{namespace_id:*}/{project_id}/-/environments/search",

  // projects/environments/prometheus_api
  "/{namespace_id:*}/{project_id}/-/environments/{id}/prometheus/api/v1/{proxy_path:*}",

  // projects/error_tracking
  "/{namespace_id:*}/{project_id}/-/error_tracking",
  "/{namespace_id:*}/{project_id}/-/error_tracking/{issue_id}",
  "/{namespace_id:*}/{project_id}/-/error_tracking/{issue_id}/details",

  // projects/error_tracking/projects
  "/{namespace_id:*}/{project_id}/-/error_tracking/projects",

  // projects/error_tracking/stack_traces
  "/{namespace_id:*}/{project_id}/-/error_tracking/{issue_id}/stack_trace",

  // projects/feature_flag_issues
  "/{namespace_id:*}/{project_id}/-/feature_flags/{feature_flag_iid}/issues",
  "/{namespace_id:*}/{project_id}/-/feature_flags/{feature_flag_iid}/issues/{id}",

  // projects/feature_flags
  "/{namespace_id:*}/{project_id}/-/feature_flags",
  "/{namespace_id:*}/{project_id}/-/feature_flags/{iid}",
  "/{namespace_id:*}/{project_id}/-/feature_flags/{iid}/edit",
  "/{namespace_id:*}/{project_id}/-/feature_flags/new",

  // projects/feature_flags_clients
  "/{namespace_id:*}/{project_id}/-/feature_flags_client/reset_token",

  // projects/feature_flags_user_lists
  "/{namespace_id:*}/{project_id}/-/feature_flags_user_lists",
  "/{namespace_id:*}/{project_id}/-/feature_flags_user_lists/{iid}",
  "/{namespace_id:*}/{project_id}/-/feature_flags_user_lists/{iid}/edit",
  "/{namespace_id:*}/{project_id}/-/feature_flags_user_lists/new",

  // projects/find_file
  "/{namespace_id:*}/{project_id}/-/files/{id:*}",
  "/{namespace_id:*}/{project_id}/-/find_file/{id:*}",

  // projects/forks
  "/{namespace_id:*}/{project_id}/-/forks",
  "/{namespace_id:*}/{project_id}/-/forks/new",

  // projects/google_cloud/artifact_registry
  "/{namespace_id:*}/{project_id}/-/google_cloud/artifact_registry",
  "/{namespace_id:*}/{project_id}/-/google_cloud/artifact_registry/projects/{project}/locations/{location}/repositories/{repository}/dockerImages/{image}",

  // projects/google_cloud/configuration
  "/{namespace_id:*}/{project_id}/-/google_cloud/configuration",

  // projects/google_cloud/databases
  "/{namespace_id:*}/{project_id}/-/google_cloud/databases",
  "/{namespace_id:*}/{project_id}/-/google_cloud/databases/new/{product}",

  // projects/google_cloud/deployments
  "/{namespace_id:*}/{project_id}/-/google_cloud/deployments",
  "/{namespace_id:*}/{project_id}/-/google_cloud/deployments/cloud_run",
  "/{namespace_id:*}/{project_id}/-/google_cloud/deployments/cloud_storage",

  // projects/google_cloud/gcp_regions
  "/{namespace_id:*}/{project_id}/-/google_cloud/gcp_regions",

  // projects/google_cloud/revoke_oauth
  "/{namespace_id:*}/{project_id}/-/google_cloud/revoke_oauth",

  // projects/google_cloud/service_accounts
  "/{namespace_id:*}/{project_id}/-/google_cloud/service_accounts",

  // projects/graphs
  "/{namespace_id:*}/{project_id}/-/graphs/{id}",
  "/{namespace_id:*}/{project_id}/-/graphs/{id}/charts",
  "/{namespace_id:*}/{project_id}/-/graphs/{id}/ci",
  "/{namespace_id:*}/{project_id}/-/graphs/{id}/commits",
  "/{namespace_id:*}/{project_id}/-/graphs/{id}/languages",

  // projects/group_links
  "/{namespace_id:*}/{project_id}/-/group_links/{id}",

  // projects/harbor/artifacts
  "/{namespace_id:*}/{project_id}/-/harbor/repositories/{repository_id}/artifacts",

  // projects/harbor/repositories
  "/{namespace_id:*}/{project_id}/-/harbor/repositories",
  "/{namespace_id:*}/{project_id}/-/harbor/repositories/{id}",

  // projects/harbor/tags
  "/{namespace_id:*}/{project_id}/-/harbor/repositories/{repository_id}/artifacts/{artifact_id}/tags",

  // projects/hook_logs
  "/{namespace_id:*}/{project_id}/-/hooks/{hook_id}/hook_logs/{id}",
  "/{namespace_id:*}/{project_id}/-/hooks/{hook_id}/hook_logs/{id}/retry",

  // projects/hooks
  "/{namespace_id:*}/{project_id}/-/hooks",
  "/{namespace_id:*}/{project_id}/-/hooks/{id}",
  "/{namespace_id:*}/{project_id}/-/hooks/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/hooks/{id}/test",

  // projects/import/jira
  "/{namespace_id:*}/{project_id}/-/import/jira",

  // projects/imports
  "/{namespace_id:*}/{project_id}/-/import",
  "/{namespace_id:*}/{project_id}/-/import/new",

  // projects/incident_management/escalation_policies
  "/{namespace_id:*}/{project_id}/-/escalation_policies",

  // projects/incident_management/oncall_schedules
  "/{namespace_id:*}/{project_id}/-/oncall_schedules",

  // projects/incident_management/pager_duty_incidents
  "/{namespace_id:*}/{project_id}/-/incidents/integrations/pagerduty",

  // projects/incident_management/timeline_events
  "/{namespace_id:*}/{project_id}/-/incident_management/timeline_events/preview_markdown",

  // projects/incidents
  "/{namespace_id:*}/{project_id}/-/incidents",
  "/{namespace_id:*}/{project_id}/-/issues/incident/{id}",
  "/{namespace_id:*}/{project_id}/-/issues/incident/{id}/{incident_tab}",

  // projects/insights
  "/{namespace_id:*}/{project_id}/insights",
  "/{namespace_id:*}/{project_id}/insights/query",

  // projects/integrations/jira/issues
  "/{namespace_id:*}/{project_id}/-/integrations/jira/issues",
  "/{namespace_id:*}/{project_id}/-/integrations/jira/issues/{id}",

  // projects/integrations/slash_commands
  "/{namespace_id:*}/{project_id}/-/integrations/slash_commands",
  "/{namespace_id:*}/{project_id}/-/integrations/slash_commands/confirm",

  // projects/integrations/zentao/issues
  "/{namespace_id:*}/{project_id}/-/integrations/zentao/issues",
  "/{namespace_id:*}/{project_id}/-/integrations/zentao/issues/{id}",

  // projects/issue_feature_flags
  "/{namespace_id:*}/{project_id}/-/issues/{issue_id}/feature_flags",
  "/{namespace_id:*}/{project_id}/-/issues/{issue_id}/feature_flags/{id}",

  // projects/issue_links
  "/{namespace_id:*}/{project_id}/-/issues/{issue_id}/links",
  "/{namespace_id:*}/{project_id}/-/issues/{issue_id}/links/{id}",

  // projects/issues
  "/{namespace_id:*}/{project_id}/-/issues",
  "/{namespace_id:*}/{project_id}/-/issues/{id}",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/{incident_tab}",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/can_create_branch",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/create_merge_request",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/descriptions/{version_id}",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/descriptions/{version_id}/diff",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/designs",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/designs/{vueroute:*}",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/discussions",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/mark_as_spam",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/move",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/realtime_changes",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/related_branches",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/reorder",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/toggle_award_emoji",
  "/{namespace_id:*}/{project_id}/-/issues/{id}/toggle_subscription",
  "/{namespace_id:*}/{project_id}/-/issues/bulk_update",
  "/{namespace_id:*}/{project_id}/-/issues/export_csv",
  "/{namespace_id:*}/{project_id}/-/issues/import_csv",
  "/{namespace_id:*}/{project_id}/-/issues/new",
  "/{namespace_id:*}/{project_id}/-/issues/service_desk",

  // projects/iteration_cadences
  "/{namespace_id:*}/{project_id}/-/cadences",
  "/{namespace_id:*}/{project_id}/-/cadences/{vueroute:*}",
  "/{namespace_id:*}/{project_id}/-/cadences/{vueroute:*}/{id}",
  "/{namespace_id:*}/{project_id}/-/cadences/{vueroute:*}/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/cadences/{vueroute:*}/{iteration_cadence_id}/iterations",
  "/{namespace_id:*}/{project_id}/-/cadences/{vueroute:*}/{iteration_cadence_id}/iterations/{id}",
  "/{namespace_id:*}/{project_id}/-/cadences/{vueroute:*}/new",
  "/{namespace_id:*}/{project_id}/-/cadences/{id}",
  "/{namespace_id:*}/{project_id}/-/cadences/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/cadences/{iteration_cadence_id}/iterations",
  "/{namespace_id:*}/{project_id}/-/cadences/{iteration_cadence_id}/iterations/{id}",
  "/{namespace_id:*}/{project_id}/-/cadences/new",

  // projects/iterations
  "/{namespace_id:*}/{project_id}/-/iterations",
  "/{namespace_id:*}/{project_id}/-/iterations/{id}",

  // projects/jobs
  "/{namespace_id:*}/{project_id}/-/jobs",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/cancel",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/erase",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/play",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/proxy",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/proxy.ws/authorize",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/raw",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/retry",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/status",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/terminal",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/terminal.ws/authorize",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/test_report_summary",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/trace",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/unschedule",
  "/{namespace_id:*}/{project_id}/-/jobs/{id}/viewer",

  // projects/labels
  "/{namespace_id:*}/{project_id}/-/labels",
  "/{namespace_id:*}/{project_id}/-/labels/{id}",
  "/{namespace_id:*}/{project_id}/-/labels/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/labels/{id}/promote",
  "/{namespace_id:*}/{project_id}/-/labels/{id}/remove_priority",
  "/{namespace_id:*}/{project_id}/-/labels/{id}/toggle_subscription",
  "/{namespace_id:*}/{project_id}/-/labels/generate",
  "/{namespace_id:*}/{project_id}/-/labels/new",
  "/{namespace_id:*}/{project_id}/-/labels/set_priorities",

  // projects/learn_gitlab
  "/{namespace_id:*}/{project_id}/-/learn_gitlab",

  // projects/logs
  "/{namespace_id:*}/{project_id}/-/logs",

  // projects/mattermosts
  "/{namespace_id:*}/{project_id}/-/mattermost",
  "/{namespace_id:*}/{project_id}/-/mattermost/new",

  // projects/merge_requests
  "/{namespace_id:*}/{project_id}/-/merge_requests",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/accessibility_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/api_fuzzing_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/assign_related_issues",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/cancel_auto_merge",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/ci_environments_status",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/codequality_mr_diff_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/codequality_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/commit_change_content",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/commits",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/container_scanning_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/context_commits",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/coverage_fuzzing_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/coverage_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/dast_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/dependency_scanning_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/descriptions/{version_id}",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/descriptions/{version_id}/diff",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/discussions",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/exposed_artifacts",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/license_scanning_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/license_scanning_reports_collapsed",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/merge",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/metrics_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/pipeline_status",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/pipelines",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/rebase",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/remove_wip",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/sast_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/secret_detection_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/security_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/terraform_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/test_reports",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/toggle_award_emoji",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/toggle_subscription",
  "/{namespace_id:*}/{project_id}/-/merge_requests/bulk_update",
  "/{namespace_id:*}/{project_id}/-/merge_requests/diff_for_path",
  "/{namespace_id:*}/{project_id}/-/merge_requests/export_csv",

  // projects/merge_requests/conflicts
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/conflict_for_path",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/conflicts",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/resolve_conflicts",

  // projects/merge_requests/content
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/cached_widget",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/widget",

  // projects/merge_requests/creations
  "/{namespace_id:*}/{project_id}/-/merge_requests/new",
  "/{namespace_id:*}/{project_id}/-/merge_requests/new/branch_from",
  "/{namespace_id:*}/{project_id}/-/merge_requests/new/branch_to",
  "/{namespace_id:*}/{project_id}/-/merge_requests/new/diff_for_path",
  "/{namespace_id:*}/{project_id}/-/merge_requests/new/diffs",
  "/{namespace_id:*}/{project_id}/-/merge_requests/new/pipelines",
  "/{namespace_id:*}/{project_id}/-/merge_requests/new/target_projects",

  // projects/merge_requests/diffs
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/diff_by_file_hash/{file_hash}",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/diff_for_path",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/diffs",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/diffs_batch",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/diffs_metadata",

  // projects/merge_requests/diffs_stream
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/diffs_stream",

  // projects/merge_requests/drafts
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/drafts",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/drafts/{id}",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/drafts/discard",
  "/{namespace_id:*}/{project_id}/-/merge_requests/{merge_request_id}/drafts/publish",

  // projects/merge_requests/saml_approvals
  "/{namespace_id:*}/{project_id}/-/merge_requests/{id}/saml_approval",

  // projects/merge_trains
  "/{namespace_id:*}/{project_id}/-/merge_trains",

  // projects/metrics
  "/{namespace_id:*}/{project_id}/-/metrics",
  "/{namespace_id:*}/{project_id}/-/metrics/{id}",

  // projects/milestones
  "/{namespace_id:*}/{project_id}/-/milestones",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}/issues",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}/labels",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}/merge_requests",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}/participants",
  "/{namespace_id:*}/{project_id}/-/milestones/{id}/promote",
  "/{namespace_id:*}/{project_id}/-/milestones/new",

  // projects/mirrors
  "/{namespace_id:*}/{project_id}/-/mirror",
  "/{namespace_id:*}/{project_id}/-/mirror/ssh_host_keys",
  "/{namespace_id:*}/{project_id}/-/mirror/update_now",

  // projects/ml/agents
  "/{namespace_id:*}/{project_id}/-/ml/agents",
  "/{namespace_id:*}/{project_id}/-/ml/agents/{vueroute:*}",
  "/{namespace_id:*}/{project_id}/-/ml/agents/{vueroute:*}/{id}",
  "/{namespace_id:*}/{project_id}/-/ml/agents/{vueroute:*}/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/ml/agents/{vueroute:*}/new",
  "/{namespace_id:*}/{project_id}/-/ml/agents/{id}",
  "/{namespace_id:*}/{project_id}/-/ml/agents/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/ml/agents/new",

  // projects/ml/candidates
  "/{namespace_id:*}/{project_id}/-/ml/candidates/{iid}",

  // projects/ml/experiments
  "/{namespace_id:*}/{project_id}/-/ml/experiments",
  "/{namespace_id:*}/{project_id}/-/ml/experiments/{iid}",

  // projects/ml/model_versions
  "/{namespace_id:*}/{project_id}/-/ml/models/{model_model_id}/versions/{model_version_id}",

  // projects/ml/models
  "/{namespace_id:*}/{project_id}/-/ml/models",
  "/{namespace_id:*}/{project_id}/-/ml/models/{model_id}",
  "/{namespace_id:*}/{project_id}/-/ml/models/new",

  // projects/network
  "/{namespace_id:*}/{project_id}/-/network/{id}",

  // projects/notes
  "/{namespace_id:*}/{project_id}/noteable/{target_type}/{target_id}/notes",
  "/{namespace_id:*}/{project_id}/notes",
  "/{namespace_id:*}/{project_id}/notes/{id}",
  "/{namespace_id:*}/{project_id}/notes/{id}/delete_attachment",
  "/{namespace_id:*}/{project_id}/notes/{id}/outdated_line_change",
  "/{namespace_id:*}/{project_id}/notes/{id}/resolve",
  "/{namespace_id:*}/{project_id}/notes/{id}/toggle_award_emoji",

  // projects/on_demand_scans
  "/{namespace_id:*}/{project_id}/-/on_demand_scans",
  "/{namespace_id:*}/{project_id}/-/on_demand_scans/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/on_demand_scans/new",

  // projects/packages/infrastructure_registry
  "/{namespace_id:*}/{project_id}/-/terraform_module_registry",
  "/{namespace_id:*}/{project_id}/-/terraform_module_registry/{id}",

  // projects/packages/package_files
  "/{namespace_id:*}/{project_id}/-/package_files/{id}/download",

  // projects/packages/packages
  "/{namespace_id:*}/{project_id}/-/packages",
  "/{namespace_id:*}/{project_id}/-/packages/{id}",

  // projects/pages
  "/{namespace_id:*}/{project_id}/pages",
  "/{namespace_id:*}/{project_id}/pages/new",

  // projects/pages_domains
  "/{namespace_id:*}/{project_id}/pages/domains",
  "/{namespace_id:*}/{project_id}/pages/domains/{id}",
  "/{namespace_id:*}/{project_id}/pages/domains/{id}/clean_certificate",
  "/{namespace_id:*}/{project_id}/pages/domains/{id}/edit",
  "/{namespace_id:*}/{project_id}/pages/domains/{id}/retry_auto_ssl",
  "/{namespace_id:*}/{project_id}/pages/domains/{id}/verify",
  "/{namespace_id:*}/{project_id}/pages/domains/new",

  // projects/path_locks
  "/{namespace_id:*}/{project_id}/path_locks",
  "/{namespace_id:*}/{project_id}/path_locks/{id}",
  "/{namespace_id:*}/{project_id}/path_locks/toggle",

  // projects/pipeline_schedules
  "/{namespace_id:*}/{project_id}/-/pipeline_schedules",
  "/{namespace_id:*}/{project_id}/-/pipeline_schedules/{id}",
  "/{namespace_id:*}/{project_id}/-/pipeline_schedules/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/pipeline_schedules/{id}/play",
  "/{namespace_id:*}/{project_id}/-/pipeline_schedules/{id}/take_ownership",
  "/{namespace_id:*}/{project_id}/-/pipeline_schedules/new",

  // projects/pipelines
  "/{namespace_id:*}/{project_id}/-/pipelines",
  "/{namespace_id:*}/{project_id}/-/pipelines/{ref:*}/latest",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/builds",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/cancel",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/codequality_report",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/dag",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/downloadable_artifacts",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/failures",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/licenses",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/retry",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/security",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/stage",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/status",
  "/{namespace_id:*}/{project_id}/-/pipelines/{id}/test_report",
  "/{namespace_id:*}/{project_id}/-/pipelines/charts",
  "/{namespace_id:*}/{project_id}/-/pipelines/latest",
  "/{namespace_id:*}/{project_id}/-/pipelines/new",

  // projects/pipelines/email_campaigns
  "/{namespace_id:*}/{project_id}/-/pipelines/{pipeline_id}/validate_account",

  // projects/pipelines/stages
  "/{namespace_id:*}/{project_id}/-/pipelines/{pipeline_id}/stages/{stage_name}/play_manual",

  // projects/pipelines/tests
  "/{namespace_id:*}/{project_id}/-/pipelines/{pipeline_id}/tests/{suite_name}",
  "/{namespace_id:*}/{project_id}/-/pipelines/{pipeline_id}/tests/summary",

  // projects/pipelines_settings
  "/{namespace_id:*}/{project_id}/-/pipelines/settings",

  // projects/project_members
  "/{namespace_id:*}/{project_id}/-/project_members",
  "/{namespace_id:*}/{project_id}/-/project_members/{id}",
  "/{namespace_id:*}/{project_id}/-/project_members/{id}/approve_access_request",
  "/{namespace_id:*}/{project_id}/-/project_members/{id}/resend_invite",
  "/{namespace_id:*}/{project_id}/-/project_members/leave",
  "/{namespace_id:*}/{project_id}/-/project_members/request_access",

  // projects/prometheus/metrics
  "/{namespace_id:*}/{project_id}/prometheus/metrics",
  "/{namespace_id:*}/{project_id}/prometheus/metrics/{id}",
  "/{namespace_id:*}/{project_id}/prometheus/metrics/{id}/edit",
  "/{namespace_id:*}/{project_id}/prometheus/metrics/active_common",
  "/{namespace_id:*}/{project_id}/prometheus/metrics/new",
  "/{namespace_id:*}/{project_id}/prometheus/metrics/validate_query",

  // projects/protected_branches
  "/{namespace_id:*}/{project_id}/-/protected_branches",
  "/{namespace_id:*}/{project_id}/-/protected_branches/{id}",

  // projects/protected_environments
  "/{namespace_id:*}/{project_id}/-/protected_environments",
  "/{namespace_id:*}/{project_id}/-/protected_environments/{id}",
  "/{namespace_id:*}/{project_id}/-/protected_environments/search",

  // projects/protected_tags
  "/{namespace_id:*}/{project_id}/-/protected_tags",
  "/{namespace_id:*}/{project_id}/-/protected_tags/{id}",

  // projects/push_rules
  "/{namespace_id:*}/{project_id}/-/push_rules/{id}",

  // projects/quality/test_cases
  "/{namespace_id:*}/{project_id}/-/quality/test_cases",
  "/{namespace_id:*}/{project_id}/-/quality/test_cases/{id}",
  "/{namespace_id:*}/{project_id}/-/quality/test_cases/new",

  // projects/raw
  "/{namespace_id:*}/{project_id}/-/raw/{id:*}",
  "/{namespace_id:*}/{project_id}/raw/{id:*}",

  // projects/redirect
  "/projects/{id}",

  // projects/refs
  "/{namespace_id:*}/{project_id}/-/refs/{id}/logs_tree",
  "/{namespace_id:*}/{project_id}/-/refs/{id}/logs_tree/{path:*}",
  "/{namespace_id:*}/{project_id}/-/refs/switch",

  // projects/registry/repositories
  "/{namespace_id:*}/{project_id}/container_registry",
  "/{namespace_id:*}/{project_id}/container_registry/{id}",

  // projects/registry/tags
  "/{namespace_id:*}/{project_id}/registry/repository/{repository_id}/tags",
  "/{namespace_id:*}/{project_id}/registry/repository/{repository_id}/tags/{id}",
  "/{namespace_id:*}/{project_id}/registry/repository/{repository_id}/tags/bulk_destroy",

  // projects/releases
  "/{namespace_id:*}/{project_id}/-/releases",
  "/{namespace_id:*}/{project_id}/-/releases/{tag}",
  "/{namespace_id:*}/{project_id}/-/releases/{tag}/downloads/{filepath:*}",
  "/{namespace_id:*}/{project_id}/-/releases/{tag}/edit",
  "/{namespace_id:*}/{project_id}/-/releases/new",
  "/{namespace_id:*}/{project_id}/-/releases/permalink/latest",
  "/{namespace_id:*}/{project_id}/-/releases/permalink/latest/{suffix_path:*}",

  // projects/releases/evidences
  "/{namespace_id:*}/{project_id}/-/releases/{tag}/evidences/{id}",

  // projects/repositories
  "/{namespace_id:*}/{project_id}/-/archive/{id:*}.{format}",
  "/{namespace_id:*}/{project_id}/-/repository",
  "/{namespace_id:*}/{project_id}/repository",

  // projects/requirements_management/requirements
  "/{namespace_id:*}/{project_id}/-/requirements_management/requirements",
  "/{namespace_id:*}/{project_id}/-/requirements_management/requirements/import_csv",
  "/{namespace_id:*}/{project_id}/-/requirements_management/requirements/import_csv/authorize",

  // projects/runner_projects
  "/{namespace_id:*}/{project_id}/runner_projects",
  "/{namespace_id:*}/{project_id}/runner_projects/{id}",

  // projects/runners
  "/{namespace_id:*}/{project_id}/-/runners",
  "/{namespace_id:*}/{project_id}/-/runners/{id}",
  "/{namespace_id:*}/{project_id}/-/runners/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/runners/{id}/pause",
  "/{namespace_id:*}/{project_id}/-/runners/{id}/register",
  "/{namespace_id:*}/{project_id}/-/runners/{id}/resume",
  "/{namespace_id:*}/{project_id}/-/runners/new",
  "/{namespace_id:*}/{project_id}/-/runners/toggle_group_runners",
  "/{namespace_id:*}/{project_id}/-/runners/toggle_shared_runners",

  // projects/secrets
  "/{namespace_id:*}/{project_id}/-/secrets",
  "/{namespace_id:*}/{project_id}/-/secrets/{vueroute:*}",

  // projects/security/api_fuzzing_configuration
  "/{namespace_id:*}/{project_id}/-/security/configuration/api_fuzzing",

  // projects/security/configuration
  "/{namespace_id:*}/{project_id}/-/security/configuration",

  // projects/security/corpus_management
  "/{namespace_id:*}/{project_id}/-/security/configuration/corpus_management",

  // projects/security/dashboard
  "/{namespace_id:*}/{project_id}/-/security/dashboard",

  // projects/security/dast_configuration
  "/{namespace_id:*}/{project_id}/-/security/configuration/dast",

  // projects/security/dast_profiles
  "/{namespace_id:*}/{project_id}/-/security/configuration/profile_library",

  // projects/security/dast_scanner_profiles
  "/{namespace_id:*}/{project_id}/-/security/configuration/profile_library/dast_scanner_profiles/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/security/configuration/profile_library/dast_scanner_profiles/new",

  // projects/security/dast_site_profiles
  "/{namespace_id:*}/{project_id}/-/security/configuration/profile_library/dast_site_profiles/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/security/configuration/profile_library/dast_site_profiles/new",

  // projects/security/discover
  "/{namespace_id:*}/{project_id}/-/security/discover",

  // projects/security/policies
  "/{namespace_id:*}/{project_id}/-/security/policies",
  "/{namespace_id:*}/{project_id}/-/security/policies/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/security/policies/new",
  "/{namespace_id:*}/{project_id}/-/security/policies/schema",

  // projects/security/sast_configuration
  "/{namespace_id:*}/{project_id}/-/security/configuration/sast",

  // projects/security/scanned_resources
  "/{namespace_id:*}/{project_id}/-/security/scanned_resources",

  // projects/security/vulnerabilities
  "/{namespace_id:*}/{project_id}/-/security/vulnerabilities/{id}",
  "/{namespace_id:*}/{project_id}/-/security/vulnerabilities/{id}/discussions",
  "/{namespace_id:*}/{project_id}/-/security/vulnerabilities/new",

  // projects/security/vulnerabilities/notes
  "/{namespace_id:*}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes",
  "/{namespace_id:*}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes/{id}",
  "/{namespace_id:*}/{project_id}/-/security/vulnerabilities/{vulnerability_id}/notes/{id}/toggle_award_emoji",

  // projects/security/vulnerability_report
  "/{namespace_id:*}/{project_id}/-/security/vulnerability_report",

  // projects/service_desk
  "/{namespace_id:*}/{project_id}/service_desk",

  // projects/service_desk/custom_email
  "/{namespace_id:*}/{project_id}/-/service_desk/custom_email",

  // projects/service_ping
  "/{namespace_id:*}/{project_id}/service_ping/web_ide_pipelines_count",

  // projects/settings/access_tokens
  "/{namespace_id:*}/{project_id}/-/settings/access_tokens",
  "/{namespace_id:*}/{project_id}/-/settings/access_tokens/{id}/revoke",

  // projects/settings/analytics
  "/{namespace_id:*}/{project_id}/-/settings/analytics",

  // projects/settings/branch_rules
  "/{namespace_id:*}/{project_id}/-/settings/repository/branch_rules",

  // projects/settings/ci_cd
  "/{namespace_id:*}/{project_id}/-/settings/ci_cd",
  "/{namespace_id:*}/{project_id}/-/settings/ci_cd/reset_cache",
  "/{namespace_id:*}/{project_id}/-/settings/ci_cd/reset_registration_token",
  "/{namespace_id:*}/{project_id}/-/settings/ci_cd/runner_setup_scripts",

  // projects/settings/integration_hook_logs
  "/{namespace_id:*}/{project_id}/-/settings/integrations/{integration_id}/hook_logs/{id}",
  "/{namespace_id:*}/{project_id}/-/settings/integrations/{integration_id}/hook_logs/{id}/retry",

  // projects/settings/integrations
  "/{namespace_id:*}/{project_id}/-/settings/integrations",
  "/{namespace_id:*}/{project_id}/-/settings/integrations/{id}",
  "/{namespace_id:*}/{project_id}/-/settings/integrations/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/settings/integrations/{id}/test",

  // projects/settings/merge_requests
  "/{namespace_id:*}/{project_id}/-/settings/merge_requests",

  // projects/settings/operations
  "/{namespace_id:*}/{project_id}/-/settings/operations",
  "/{namespace_id:*}/{project_id}/-/settings/operations/reset_alerting_token",
  "/{namespace_id:*}/{project_id}/-/settings/operations/reset_pagerduty_token",

  // projects/settings/packages_and_registries
  "/{namespace_id:*}/{project_id}/-/settings/packages_and_registries",
  "/{namespace_id:*}/{project_id}/-/settings/packages_and_registries/cleanup_image_tags",

  // projects/settings/repository
  "/{namespace_id:*}/{project_id}/-/settings/ci_cd/deploy_token/create",
  "/{namespace_id:*}/{project_id}/-/settings/repository",
  "/{namespace_id:*}/{project_id}/-/settings/repository/cleanup",
  "/{namespace_id:*}/{project_id}/-/settings/repository/deploy_token/create",

  // projects/settings/slacks
  "/{namespace_id:*}/{project_id}/-/settings/slack",
  "/{namespace_id:*}/{project_id}/-/settings/slack/edit",
  "/{namespace_id:*}/{project_id}/-/settings/slack/slack_auth",

  // projects/snippets
  "/{namespace_id:*}/{project_id}/-/snippets",
  "/{namespace_id:*}/{project_id}/-/snippets/{id}",
  "/{namespace_id:*}/{project_id}/-/snippets/{id}/edit",
  "/{namespace_id:*}/{project_id}/-/snippets/{id}/mark_as_spam",
  "/{namespace_id:*}/{project_id}/-/snippets/{id}/raw",
  "/{namespace_id:*}/{project_id}/-/snippets/{id}/toggle_award_emoji",
  "/{namespace_id:*}/{project_id}/-/snippets/new",
  "/{namespace_id:*}/{project_id}/snippets/{id}/raw",

  // projects/snippets/blobs
  "/{namespace_id:*}/{project_id}/-/snippets/{snippet_id}/raw/{ref}/{path:*}",

  // projects/starrers
  "/{namespace_id:*}/{project_id}/-/starrers",

  // projects/subscriptions
  "/{namespace_id:*}/{project_id}/-/subscriptions",
  "/{namespace_id:*}/{project_id}/-/subscriptions/{id}",

  // projects/tags
  "/{namespace_id:*}/{project_id}/-/tags",
  "/{namespace_id:*}/{project_id}/-/tags/{id}",
  "/{namespace_id:*}/{project_id}/-/tags/new",

  // projects/target_branch_rules
  "/{namespace_id:*}/{project_id}/-/target_branch_rules",
  "/{namespace_id:*}/{project_id}/-/target_branch_rules/{id}",

  // projects/templates
  "/{namespace_id:*}/{project_id}/description_templates/names/{template_type}",
  "/{namespace_id:*}/{project_id}/templates/{template_type}",
  "/{namespace_id:*}/{project_id}/templates/{template_type}/{key}",

  // projects/terraform
  "/{namespace_id:*}/{project_id}/-/terraform",

  // projects/todos
  "/{namespace_id:*}/{project_id}/todos",

  // projects/tracing
  "/{namespace_id:*}/{project_id}/-/tracing",
  "/{namespace_id:*}/{project_id}/-/tracing/{id}",

  // projects/tree
  "/{namespace_id:*}/{project_id}/-/create_dir/{id:*}",
  "/{namespace_id:*}/{project_id}/-/tree/{id:*}",
  "/{namespace_id:*}/{project_id}/tree/{id:*}",

  // projects/triggers
  "/{namespace_id:*}/{project_id}/-/triggers",
  "/{namespace_id:*}/{project_id}/-/triggers/{id}",

  // projects/uploads
  "/{namespace_id:*}/{project_id}/uploads",
  "/{namespace_id:*}/{project_id}/uploads/{secret}/{filename}",
  "/{namespace_id:*}/{project_id}/uploads/authorize",

  // projects/usage_quotas
  "/{namespace_id:*}/{project_id}/-/usage_quotas",

  // projects/variables
  "/{namespace_id:*}/{project_id}/-/variables",

  // projects/vulnerability_feedback
  "/{namespace_id:*}/{project_id}/-/vulnerability_feedback",
  "/{namespace_id:*}/{project_id}/-/vulnerability_feedback/{id}",
  "/{namespace_id:*}/{project_id}/-/vulnerability_feedback/count",

  // projects/web_ide_schemas
  "/{namespace_id:*}/{project_id}/-/schema/{branch}/{filename:*}",

  // projects/web_ide_terminals
  "/{namespace_id:*}/{project_id}/ide_terminals",
  "/{namespace_id:*}/{project_id}/ide_terminals/{id}",
  "/{namespace_id:*}/{project_id}/ide_terminals/{id}/cancel",
  "/{namespace_id:*}/{project_id}/ide_terminals/{id}/retry",
  "/{namespace_id:*}/{project_id}/ide_terminals/check_config",

  // projects/wikis
  "/{namespace_id:*}/{project_id}/-/wikis",
  "/{namespace_id:*}/{project_id}/-/wikis/{id:*}",
  "/{namespace_id:*}/{project_id}/-/wikis/{id:*}/diff",
  "/{namespace_id:*}/{project_id}/-/wikis/{id:*}/edit",
  "/{namespace_id:*}/{project_id}/-/wikis/{id:*}/history",
  "/{namespace_id:*}/{project_id}/-/wikis/{id:*}/preview_markdown",
  "/{namespace_id:*}/{project_id}/-/wikis/{id:*}/raw",
  "/{namespace_id:*}/{project_id}/-/wikis/git_access",
  "/{namespace_id:*}/{project_id}/-/wikis/new",
  "/{namespace_id:*}/{project_id}/-/wikis/pages",
  "/{namespace_id:*}/{project_id}/-/wikis/templates",

  // projects/work_items
  "/{namespace_id:*}/{project_id}/-/work_items/{iid}",
  "/{namespace_id:*}/{project_id}/-/work_items/{iid}/designs",
  "/{namespace_id:*}/{project_id}/-/work_items/{iid}/designs/{vueroute:*}",
  "/{namespace_id:*}/{project_id}/-/work_items/import_csv",
  "/{namespace_id:*}/{project_id}/-/work_items/import_csv/authorize",

  // pwa
  "/-/manifest",
  "/-/offline",

  // rails/info
  "/rails/info",
  "/rails/info/properties",
  "/rails/info/routes",

  // rails/mailers
  "/rails/mailers",
  "/rails/mailers/{path}",

  // registrations
  "/users",
  "/users/cancel",
  "/users/edit",
  "/users/sign_up",

  // registrations/company
  "/users/sign_up/company",
  "/users/sign_up/company/new",

  // registrations/groups
  "/users/sign_up/groups",
  "/users/sign_up/groups/new",

  // registrations/welcome
  "/users/sign_up/welcome",

  // remote_development/workspaces
  "/-/remote_development/workspaces",
  "/-/remote_development/workspaces/{vueroute:*}",
  "/-/remote_development/workspaces/{vueroute:*}/{id}",
  "/-/remote_development/workspaces/{vueroute:*}/{id}/edit",
  "/-/remote_development/workspaces/{vueroute:*}/{workspace_id}/workspaces",
  "/-/remote_development/workspaces/{vueroute:*}/{workspace_id}/workspaces/new",
  "/-/remote_development/workspaces/{vueroute:*}/new",
  "/-/remote_development/workspaces/{id}",
  "/-/remote_development/workspaces/{id}/edit",
  "/-/remote_development/workspaces/{workspace_id}/workspaces",
  "/-/remote_development/workspaces/{workspace_id}/workspaces/new",
  "/-/remote_development/workspaces/new",

  // remote_development/workspaces_feature_flag
  "/-/remote_development/workspaces_feature_flag",

  // repositories/git_http
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/git-receive-pack",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/git-upload-pack",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/refs",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/ssh-upload-pack",
  "/{repository_path:*}/git-receive-pack",
  "/{repository_path:*}/git-upload-pack",
  "/{repository_path:*}/info/refs",
  "/{repository_path:*}/ssh-upload-pack",

  // repositories/lfs_api
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/objects",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/objects/{oid:*}",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/objects/batch",
  "/{repository_path:*}/info/lfs/objects",
  "/{repository_path:*}/info/lfs/objects/{oid:*}",
  "/{repository_path:*}/info/lfs/objects/batch",

  // repositories/lfs_locks_api
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/locks",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/locks/{id}",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/locks/{id}/edit",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/locks/{id}/unlock",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/locks/new",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/info/lfs/locks/verify",
  "/{repository_path:*}/info/lfs/locks",
  "/{repository_path:*}/info/lfs/locks/{id}",
  "/{repository_path:*}/info/lfs/locks/{id}/edit",
  "/{repository_path:*}/info/lfs/locks/{id}/unlock",
  "/{repository_path:*}/info/lfs/locks/new",
  "/{repository_path:*}/info/lfs/locks/verify",

  // repositories/lfs_storage
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/gitlab-lfs/objects/{oid:*}",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/gitlab-lfs/objects/{oid:*}/{size:*}",
  "/-/push_from_secondary/{geo_node_id}/{repository_path:*}/gitlab-lfs/objects/{oid:*}/{size:*}/authorize",
  "/{repository_path:*}/gitlab-lfs/objects/{oid:*}",
  "/{repository_path:*}/gitlab-lfs/objects/{oid:*}/{size:*}",
  "/{repository_path:*}/gitlab-lfs/objects/{oid:*}/{size:*}/authorize",

  // root
  "/",

  // runner_setup
  "/-/runner_setup/platforms",

  // sandbox
  "/-/sandbox/mermaid",
  "/-/sandbox/swagger",

  // search
  "/search",
  "/search/aggregations",
  "/search/autocomplete",
  "/search/count",
  "/search/opensearch",
  "/search/settings",

  // security/dashboard
  "/-/security/dashboard",
  "/-/security/dashboard/settings",

  // security/projects
  "/-/security/projects",
  "/-/security/projects/{id}",

  // security/vulnerabilities
  "/-/security/vulnerabilities",

  // sent_notifications
  "/-/sent_notifications/{id}/unsubscribe",

  // sessions
  "/users/auth/geo/sign_in",
  "/users/auth/geo/sign_out",
  "/users/resend_verification_code",
  "/users/sign_in",
  "/users/sign_out",
  "/users/successful_verification",
  "/users/update_email",

  // sitemap
  "/sitemap",

  // smartcard
  "/-/smartcard/auth",
  "/-/smartcard/extract_certificate",
  "/-/smartcard/verify_certificate",

  // snippets
  "/-/snippets",
  "/-/snippets/{id}",
  "/-/snippets/{id}/edit",
  "/-/snippets/{id}/mark_as_spam",
  "/-/snippets/{id}/raw",
  "/-/snippets/{id}/toggle_award_emoji",
  "/-/snippets/new",
  "/-/snippets/preview_markdown",
  "/snippets/{id}/raw",

  // snippets/blobs
  "/-/snippets/{snippet_id}/raw/{ref}/{path:*}",

  // snippets/notes
  "/-/snippets/{snippet_id}/notes",
  "/-/snippets/{snippet_id}/notes/{id}",
  "/-/snippets/{snippet_id}/notes/{id}/delete_attachment",
  "/-/snippets/{snippet_id}/notes/{id}/toggle_award_emoji",

  // subscriptions
  "/-/subscriptions",
  "/-/subscriptions/buy_minutes",
  "/-/subscriptions/buy_storage",
  "/-/subscriptions/new",
  "/-/subscriptions/payment_form",
  "/-/subscriptions/payment_method",
  "/-/subscriptions/validate_payment_method",

  // subscriptions/groups
  "/-/subscriptions/groups",
  "/-/subscriptions/groups/{id}",
  "/-/subscriptions/groups/{id}/edit",
  "/-/subscriptions/groups/new",

  // subscriptions/hand_raise_leads
  "/-/subscriptions/hand_raise_leads",

  // subscriptions/trials
  "/-/trials",
  "/-/trials/new",

  // subscriptions/trials/duo_pro
  "/-/trials/duo_pro",
  "/-/trials/duo_pro/new",

  // terraform/services
  "/.well-known/terraform.json",

  // time_tracking/timelogs
  "/-/timelogs",

  // trial_registrations
  "/-/trial_registrations",
  "/-/trial_registrations/new",

  // uploads
  "/uploads/-/system/{model}/{id}/{secret}/{filename}",
  "/uploads/-/system/{model}/{mounted_as}/{id}/{filename}",
  "/uploads/-/system/temp/{secret}/{filename}",
  "/uploads/{model}",
  "/uploads/{model}/authorize",

  // user_settings/active_sessions
  "/-/profile/active_sessions/{id}",
  "/-/user_settings/active_sessions",
  "/-/user_settings/active_sessions/{id}",
  "/-/user_settings/active_sessions/saml",

  // user_settings/gpg_keys
  "/-/user_settings/gpg_keys",
  "/-/user_settings/gpg_keys/{id}",
  "/-/user_settings/gpg_keys/{id}/revoke",

  // user_settings/identities
  "/-/user_settings/identities",
  "/-/user_settings/identities/new",

  // user_settings/passwords
  "/-/user_settings/password",
  "/-/user_settings/password/edit",
  "/-/user_settings/password/new",
  "/-/user_settings/password/reset",

  // user_settings/personal_access_tokens
  "/-/profile/personal_access_tokens/{id}/revoke",
  "/-/user_settings/personal_access_tokens",
  "/-/user_settings/personal_access_tokens/{id}/revoke",

  // user_settings/profiles
  "/-/profile",
  "/-/user_settings/profile",

  // user_settings/ssh_keys
  "/-/user_settings/ssh_keys",
  "/-/user_settings/ssh_keys/{id}",
  "/-/user_settings/ssh_keys/{id}/revoke",

  // user_settings/user_settings
  "/-/user_settings/authentication_log",

  // users
  "/{username}",
  "/{username}.gpg",
  "/{username}.keys",
  "/users/{username}/activity",
  "/users/{username}/available_group_templates",
  "/users/{username}/available_project_templates",
  "/users/{username}/calendar",
  "/users/{username}/calendar_activities",
  "/users/{username}/contributed",
  "/users/{username}/exists",
  "/users/{username}/follow",
  "/users/{username}/followers",
  "/users/{username}/following",
  "/users/{username}/groups",
  "/users/{username}/projects",
  "/users/{username}/snippets",
  "/users/{username}/starred",
  "/users/{username}/unfollow",

  // users/broadcast_message_dismissals
  "/-/users/broadcast_message_dismissals",

  // users/callouts
  "/-/users/callouts",

  // users/group_callouts
  "/-/users/group_callouts",

  // users/identity_verification
  "/-/identity_verification",
  "/-/identity_verification/send_phone_verification_code",
  "/-/identity_verification/success",
  "/-/identity_verification/toggle_phone_exemption",
  "/-/identity_verification/verification_state",
  "/-/identity_verification/verify_credit_card",
  "/-/identity_verification/verify_credit_card_captcha",
  "/-/identity_verification/verify_phone_verification_code",

  // users/namespace_visits
  "/-/track_namespace_visits",

  // users/pins
  "/-/users/pins",

  // users/project_callouts
  "/-/users/project_callouts",

  // users/registrations_identity_verification
  "/users/identity_verification",
  "/users/identity_verification/arkose_labs_challenge",
  "/users/identity_verification/resend_email_code",
  "/users/identity_verification/restricted",
  "/users/identity_verification/send_phone_verification_code",
  "/users/identity_verification/success",
  "/users/identity_verification/toggle_phone_exemption",
  "/users/identity_verification/verification_state",
  "/users/identity_verification/verify_arkose_labs_session",
  "/users/identity_verification/verify_credit_card",
  "/users/identity_verification/verify_credit_card_captcha",
  "/users/identity_verification/verify_email_code",
  "/users/identity_verification/verify_phone_verification_code",

  // users/terms
  "/-/users/terms",
  "/-/users/terms/{id}/accept",
  "/-/users/terms/{id}/decline",

  // users/unsubscribes
  "/unsubscribes/{email}",

  // web_ide/remote_ide
  "/-/ide/remote/{remote_host}",
  "/-/ide/remote/{remote_host}/{remote_path:*}",

  // well_known
  "/.well-known/security.txt",

  // whats_new
  "/-/whats_new"
];
