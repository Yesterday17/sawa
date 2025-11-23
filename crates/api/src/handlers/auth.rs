use aide::axum::IntoApiResponse;
use axum::{Json, http::StatusCode};
use sawa_core::services::UserService;

use crate::{
    auth::{AuthSession, Credentials},
    error::AppError,
};

/// Login a user.
///
/// Authenticates a user with username and password and creates a session.
pub async fn login<S>(
    mut auth_session: AuthSession<S>,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: Clone + Send + Sync + 'static + UserService,
{
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(AppError::InternalServerError),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(AppError::InternalServerError);
    }

    Ok(StatusCode::OK)
}

/// Logout a user.
///
/// Destroys the current session.
pub async fn logout<S>(mut auth_session: AuthSession<S>) -> Result<impl IntoApiResponse, AppError>
where
    S: Clone + Send + Sync + 'static + UserService,
{
    if auth_session.logout().await.is_err() {
        return Err(AppError::InternalServerError);
    }

    Ok(StatusCode::OK)
}
