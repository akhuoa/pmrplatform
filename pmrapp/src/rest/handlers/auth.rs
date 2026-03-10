use axum::{
    extract::State,
    http::StatusCode,
    Extension,
    Json,
};
use axum_login::AuthSession;
use pmrcore::ac::{
    user::User,
    workflow::State as WorkflowState,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    enforcement::PolicyState,
    error::{AppError, AuthError},
    server::{
        ac::Session,
        platform::PlatformRef,
    },
    workflow::state::TRANSITIONS,
};

// Request/Response types
#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct WorkflowTransitionRequest {
    pub resource: String,
    pub target: String,
}

// POST /api/auth/login
pub async fn login(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {
    let mut session: Session = auth_session.into();
    session
        .sign_in_with_login_password(payload.login, payload.password)
        .await?;

    Ok(Json(LoginResponse {
        message: "Successfully logged in".to_string(),
    }))
}

// POST /api/auth/logout
pub async fn logout(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
) -> Result<StatusCode, AuthError> {
    let mut session: Session = auth_session.into();
    session.sign_out().await?;
    Ok(StatusCode::OK)
}

// GET /api/auth/current-user
pub async fn current_user(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
) -> Json<Option<User>> {
    let session: Session = auth_session.into();
    Json(session.current_user())
}

// POST /api/workflow/transition
pub async fn workflow_transition(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Json(payload): Json<WorkflowTransitionRequest>,
) -> Result<Json<PolicyState>, AppError> {
    let session: Session = auth_session.into();

    if let Some(user) = session.current_user() {
        let target_state = WorkflowState::from_str(&payload.target)
            .map_err(|_| AppError::BadRequest)?;

        let state = platform
            .ac_platform
            .get_wf_state_for_res(&payload.resource)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let roles = platform
            .ac_platform
            .generate_policy_for_agent_res(&user.clone().into(), payload.resource.clone())
            .await
            .map_err(|_| AppError::InternalServerError)?
            .to_roles();

        if TRANSITIONS.validate(roles, state, target_state) {
            platform
                .ac_platform
                .set_wf_state_for_res(&payload.resource, target_state)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            let policy = platform
                .ac_platform
                .generate_policy_for_agent_res(&user.into(), payload.resource)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(Json(PolicyState::new(Some(policy), target_state)))
        } else {
            Err(AppError::Forbidden)
        }
    } else {
        Err(AppError::Forbidden)
    }
}
