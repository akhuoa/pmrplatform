pub mod handlers;

use axum::{
    routing::{get, post},
    Router,
};

/// Creates the REST API router with all endpoints
/// Note: This router expects the auth session Extension and Platform Extension
/// to be provided by the parent router's layers
pub fn create_router() -> Router {
    Router::new()
        // Authentication & Workflow endpoints
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/current-user", get(handlers::auth::current_user))
        .route(
            "/workflow/transition",
            post(handlers::auth::workflow_transition),
        )
        // Workspace endpoints
        .route(
            "/workspaces/policy-state",
            get(handlers::workspace::workspace_root_policy_state),
        )
        .route("/workspaces", get(handlers::workspace::list_workspaces))
        .route(
            "/workspaces/aliased",
            get(handlers::workspace::list_aliased_workspaces),
        )
        .route(
            "/workspace/:id",
            get(handlers::workspace::get_workspace_info),
        )
        .route(
            "/workspace/:id/log",
            get(handlers::workspace::get_log_info),
        )
        .route("/workspace", post(handlers::workspace::create_workspace))
        .route(
            "/workspace/:id/sync",
            post(handlers::workspace::synchronize),
        )
        // Exposure endpoints
        .route("/exposures", get(handlers::exposure::list))
        .route("/exposures/aliased", get(handlers::exposure::list_aliased))
        .route(
            "/exposures/workspace/:workspace_id",
            get(handlers::exposure::list_aliased_for_workspace),
        )
        .route(
            "/exposure/:id",
            get(handlers::exposure::get_exposure_info),
        )
        .route(
            "/exposure/:id/resolve/*path",
            get(handlers::exposure::resolve_exposure_path),
        )
}
