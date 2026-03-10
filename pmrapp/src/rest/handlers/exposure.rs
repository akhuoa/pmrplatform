use axum::{
    extract::Path,
    Extension,
    Json,
};
use axum_login::AuthSession;
use pmrcore::{
    alias::AliasEntry,
    exposure::{
        traits::{Exposure as _, ExposureBackend, ExposureFile as _, ExposureFileView as _},
        Exposure, ExposureFile, ExposureFileView,
    },
    workspace::Workspace,
};
use pmrctrl::error::CtrlError;
use serde::{Deserialize, Serialize};

use crate::{
    enforcement::EnforcedOk,
    error::AppError,
    server::{
        ac::Session,
        platform::PlatformRef,
    },
};

pub type Exposures = Vec<AliasEntry<Exposure>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct ExposureInfo {
    pub exposure: Exposure,
    pub exposure_alias: Option<String>,
    pub files: Vec<(String, bool)>,
    pub workspace: Workspace,
    pub workspace_alias: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ResolvedExposurePath {
    Target(
        ExposureFile,
        Result<(ExposureFileView, Option<String>), Vec<String>>,
    ),
    Redirect(String),
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
        .resolve_alias("exposure", id_str)
        .await
        .map_err(|_| AppError::InternalServerError)?
        .ok_or(AppError::NotFound)
}

// GET /api/exposures
pub async fn list(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
) -> Result<Json<EnforcedOk<Exposures>>, AppError> {
    let session: Session = auth_session.into();
    let policy_state = session
        .enforcer_and_policy_state("/exposure/", "")
        .await?;

    let exposures = ExposureBackend::list(platform.mc_platform.as_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?
        .into_iter()
        .map(|exposure| AliasEntry {
            alias: exposure.id.to_string(),
            entity: exposure,
        })
        .collect();

    Ok(Json(policy_state.to_enforced_ok(exposures)))
}

// GET /api/exposures/aliased
pub async fn list_aliased(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
) -> Result<Json<EnforcedOk<Exposures>>, AppError> {
    let session: Session = auth_session.into();
    let policy_state = session
        .enforcer_and_policy_state("/exposure/", "")
        .await?;

    let exposures = platform
        .mc_platform
        .list_aliased_exposures()
        .await
        .map_err(|_| AppError::InternalServerError)?
        .into_iter()
        .map(|exposure| exposure.map(|entity| entity.into_inner()))
        .collect();

    Ok(Json(policy_state.to_enforced_ok(exposures)))
}

// GET /api/exposures/workspace/:workspace_id
pub async fn list_aliased_for_workspace(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Path(workspace_id_str): Path<String>,
) -> Result<Json<Vec<AliasEntry<Exposure>>>, AppError> {
    let session: Session = auth_session.into();

    // Resolve workspace ID
    let id = if let Ok(num) = workspace_id_str.parse::<i64>() {
        num
    } else {
        platform
            .mc_platform
            .resolve_alias("workspace", &workspace_id_str)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::NotFound)?
    };

    session
        .enforcer(format!("/workspace/{id}/"), "")
        .await?;
    session.enforcer("/exposure/".to_string(), "").await?;

    let exposures = platform
        .mc_platform
        .list_aliased_exposures_for_workspace(id)
        .await
        .map_err(|_| AppError::InternalServerError)?
        .into_iter()
        .map(|exposure| exposure.map(|entity| entity.into_inner()))
        .collect();

    Ok(Json(exposures))
}

// GET /api/exposure/:id
pub async fn get_exposure_info(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Path(id_str): Path<String>,
) -> Result<Json<EnforcedOk<ExposureInfo>>, AppError> {
    let session: Session = auth_session.into();
    let id = resolve_id(&id_str, &platform).await?;

    let policy_state = session
        .enforcer_and_policy_state(format!("/exposure/{id}/"), "")
        .await?;

    let ctrl = platform
        .get_exposure(id)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let files = ctrl
        .pair_files_info()
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let exposure = ctrl.exposure().clone_inner();
    let workspace = platform
        .mc_platform
        .get_workspace(exposure.workspace_id)
        .await
        .map_err(|_| AppError::InternalServerError)?
        .into_inner();

    let exposure_alias = ctrl
        .alias()
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let workspace_alias = platform
        .mc_platform
        .get_alias("workspace", exposure.workspace_id)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(policy_state.to_enforced_ok(ExposureInfo {
        exposure,
        exposure_alias,
        files,
        workspace,
        workspace_alias,
    })))
}

// GET /api/exposure/:id/resolve/*path
pub async fn resolve_exposure_path(
    Extension(auth_session): Extension<AuthSession<pmrac::Platform>>,
    Extension(platform): Extension<PlatformRef>,
    Path((id_str, path)): Path<(String, String)>,
) -> Result<Json<EnforcedOk<ResolvedExposurePath>>, AppError> {
    let session: Session = auth_session.into();
    let id = resolve_id(&id_str, &platform).await?;

    let policy_state = session
        .enforcer_and_policy_state(format!("/exposure/{id}/"), "")
        .await?;

    let ec = platform
        .get_exposure(id)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    match ec.resolve_file_view(path.as_ref()).await {
        (Ok(efc), Ok(efvc)) => {
            // to ensure the views is populated
            efc.exposure_file()
                .views()
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(Json(policy_state.to_enforced_ok(
                ResolvedExposurePath::Target(
                    efc.exposure_file().clone_inner(),
                    Ok((
                        efvc.exposure_file_view().clone_inner(),
                        efvc.view_path().map(str::to_string),
                    )),
                ),
            )))
        }
        (_, Err(CtrlError::None)) => {
            // since the request path has a direct hit on file, doesn't
            // matter if ExposureFileCtrl found or not.
            let exposure = ec.exposure();
            let path = platform
                .get_workspace(exposure.workspace_id())
                .await
                .map_err(|_| AppError::InternalServerError)?
                .alias()
                .await
                .map_err(|_| AppError::InternalServerError)?
                .map_or_else(
                    || {
                        format!(
                            "/workspace/:/id/{}/rawfile/{}/{path}",
                            exposure.workspace_id(),
                            exposure.commit_id(),
                        )
                    },
                    |alias| {
                        format!(
                            "/workspace/{alias}/rawfile/{}/{path}",
                            exposure.commit_id(),
                        )
                    },
                );

            Ok(Json(
                policy_state.to_enforced_ok(ResolvedExposurePath::Redirect(path)),
            ))
        }
        (Ok(efc), Err(CtrlError::EFVCNotFound(viewstr))) if viewstr == "" => {
            // to ensure the views is populated
            efc.exposure_file()
                .views()
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(Json(policy_state.to_enforced_ok(
                ResolvedExposurePath::Target(
                    efc.exposure_file().clone_inner(),
                    Err(efc
                        .exposure_file()
                        .views()
                        .await
                        .map_err(|_| AppError::InternalServerError)?
                        .iter()
                        .filter_map(|v| v.view_key().map(str::to_string))
                        .collect::<Vec<_>>()),
                ),
            )))
        }
        // CtrlError::UnknownPath(_) | CtrlError::EFVCNotFound(_)
        _ => Err(AppError::NotFound),
    }
}
