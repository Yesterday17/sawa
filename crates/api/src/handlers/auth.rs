use aide::axum::IntoApiResponse;
use axum::{Json, extract::State, http::StatusCode};
use sawa_core::{
    models::{
        misc::{MediaId, NonEmptyString},
        user::{Email, UserId, Username},
    },
    services::{CreateUserRequest, UserService},
};

use crate::{
    auth::{ApiUser, AuthSession, Credentials},
    error::AppError,
    state::AppState,
};

/// Login a user.
///
/// Authenticates a user with username and password and creates a session.
pub async fn login<S>(
    mut auth_session: AuthSession<S>,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: Clone + UserService,
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
    S: Clone + UserService,
{
    if auth_session.logout().await.is_err() {
        return Err(AppError::InternalServerError);
    }

    Ok(StatusCode::OK)
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct RegisterBody {
    pub email: Email,
    pub username: Username,
    pub password: NonEmptyString,
}

#[derive(serde::Serialize, schemars::JsonSchema)]
pub struct PublicUser {
    pub id: UserId,
    pub username: Username,
    pub email: Option<Email>,
    pub avatar: Option<MediaId>,
}

pub async fn register<S>(
    mut auth_session: AuthSession<S>,
    State(state): State<AppState<S>>,
    Json(body): Json<RegisterBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: Clone + UserService,
{
    let req = CreateUserRequest {
        username: body.username.clone(),
        password: body.password.clone(),
        email: body.email,
        avatar: None,
    };

    let user = state
        .service
        .create_user(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    auth_session
        .login(&user.clone().into())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((
        StatusCode::CREATED,
        Json(PublicUser {
            id: user.id,
            username: user.username,
            email: Some(user.email),
            avatar: user.avatar,
        }),
    ))
}
