use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension,
    Json,
};
use axum_login::AuthSession;
use pmrcore::{
    alias::AliasEntry,
    repo::{LogInfo, RepoResult},
    workspace::{
        traits::{Workspace as _, WorkspaceBackend},
        Workspace,
    },
};
use serde::Deserialize;

use crate::{
    enforcement::{EnforcedOk, PolicyState},
    error::AppError,
    server::{
        ac::Session,
        platform::PlatformRef,
    },
};

pub type Workspaces = Vec<AliasEntry<Workspace>>;

// Query parameters for workspace info
#[derive(Deserialize)]
pub struct WorkspaceInfoQuery {
    pub commit: Option<String>,
    pub path: Option<String>,
}

// Request body for create workspace
#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub uri: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
}

// Helper to resolve ID from path parameter
async fn resolve_id(id_str: &str, platform: &PlatformRef) -> Result<i64, AppError> {
    // Try to parse as number first
    if let Ok(num) = id_str.parse::<i64>() {
        return Ok(num);
    }

    // Otherwise treat as alias
    platform
        .mc_platform
        .resolve_alias("workspace", id_str)
        .await
        .map_err(|_| AppError::InternalServerError)?
        .ok_or(AppError::NotFound)
}

// GET /api/workspaces/policy-state
pub async fn workspace_root_policy_state(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
) -> Result<Json<PolicyState>, AppError> {
    let session: Session = auth_session.into();
    let policy_state = session
        .enforcer_and_policy_state("/workspace/", "")
        .await?;
    Ok(Json(policy_state))
}

// GET /api/workspaces
pub async fn list_workspaces(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
) -> Result<Json<EnforcedOk<Workspaces>>, AppError> {
    let session: Session = auth_session.into();
    let policy_state = session
        .enforcer_and_policy_state("/workspace/", "")
        .await?;

    let workspaces = WorkspaceBackend::list_workspaces(platform.mc_platform.as_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?
        .into_iter()
        .map(|workspace| AliasEntry {
            alias: workspace.id.to_string(),
            entity: workspace,
        })
        .collect();

    Ok(Json(policy_state.to_enforced_ok(workspaces)))
}

// GET /api/workspaces/aliased
pub async fn list_aliased_workspaces(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
) -> Result<Json<EnforcedOk<Workspaces>>, AppError> {
    let session: Session = auth_session.into();
    let policy_state = session
        .enforcer_and_policy_state("/workspace/", "")
        .await?;

    let workspaces = platform
        .mc_platform
        .list_aliased_workspaces()
        .await
        .map_err(|_| AppError::InternalServerError)?
        .into_iter()
        .map(|workspace| workspace.map(|entity| entity.into_inner()))
        .collect();

    Ok(Json(policy_state.to_enforced_ok(workspaces)))
}

// GET /api/workspace/:id
pub async fn get_workspace_info(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Path(id_str): Path<String>,
    Query(query): Query<WorkspaceInfoQuery>,
) -> Result<Json<EnforcedOk<RepoResult>>, AppError> {
    let session: Session = auth_session.into();
    let id = resolve_id(&id_str, &platform).await?;

    let policy_state = session
        .enforcer_and_policy_state(format!("/workspace/{id}/"), "")
        .await?;

    let handle = platform
        .repo_backend()
        .git_handle(id)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match (
        query.commit.as_ref(),
        query.path.as_ref(),
        handle.repo(),
    ) {
        (None, None, Err(_)) => Ok(Json(policy_state.to_enforced_ok(RepoResult {
            workspace: handle.workspace().clone_inner(),
            commit: None,
            path: None,
            target: None,
        }))),
        (_, _, Err(_)) => Err(AppError::InternalServerError),
        (_, _, Ok(_)) => {
            let result = handle
                .pathinfo(query.commit, query.path)
                .map_err(|_| AppError::InternalServerError)?
                .into();
            Ok(Json(policy_state.to_enforced_ok(result)))
        }
    }
}

// GET /api/workspace/:id/log
pub async fn get_log_info(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Path(id_str): Path<String>,
) -> Result<Json<EnforcedOk<LogInfo>>, AppError> {
    let session: Session = auth_session.into();
    let id = resolve_id(&id_str, &platform).await?;

    let policy_state = session
        .enforcer_and_policy_state(format!("/workspace/{id}/"), "")
        .await?;

    let handle = platform
        .repo_backend()
        .git_handle(id)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let log_info = handle
        .loginfo(None, None, Some(30))
        .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(policy_state.to_enforced_ok(log_info)))
}

// POST /api/workspace
pub async fn create_workspace(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> Result<Json<AliasEntry<Workspace>>, AppError> {
    use pmrcore::ac::{agent::Agent, role::Role, workflow::State as WorkflowState};

    let session: Session = auth_session.into();
    let policy_state = session
        .enforcer_and_policy_state("/workspace/", "create")
        .await?;

    // Create the workspace
    let entry = platform
        .mc_platform
        .create_aliased_workspace(
            &payload.uri,
            payload.description.as_deref(),
            payload.long_description.as_deref(),
        )
        .await
        .map_err(|_| AppError::InternalServerError)?
        .map(|entity| entity.into_inner());

    // Set the default workflow state to private
    let id = entry.entity.id();
    let resource = format!("/workspace/{id}/");
    platform
        .ac_platform
        .set_wf_state_for_res(&resource, WorkflowState::Private)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    // Grant the current user the owner permission
    if let Some(policy) = policy_state.policy {
        if let Agent::User(user) = policy.agent {
            platform
                .ac_platform
                .res_grant_role_to_agent(&resource, user, Role::Owner)
                .await
                .map_err(|_| AppError::InternalServerError)?;
        }
    }

    Ok(Json(entry))
}

// POST /api/workspace/:id/sync
pub async fn synchronize(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Path(id_str): Path<String>,
) -> Result<StatusCode, AppError> {
    let session: Session = auth_session.into();
    let id = resolve_id(&id_str, &platform).await?;

    session
        .enforcer(format!("/workspace/{id}/"), "protocol_write")
        .await?;

    platform
        .repo_backend()
        .sync_workspace(id)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(StatusCode::OK)
}
