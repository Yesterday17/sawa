use aide::NoApi;
use axum_login::{AuthUser, AuthnBackend};
use sawa_core::{
    models::{
        misc::NonEmptyString,
        user::{UserId, Username},
    },
    services::{GetUserError, GetUserRequest, LoginError, LoginRequest, UserService},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiUser {
    id: UserId,
    pub username: Username,
    password: NonEmptyString,
}

impl From<sawa_core::models::user::User> for ApiUser {
    fn from(user: sawa_core::models::user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            password: user.password_hash,
        }
    }
}

impl std::fmt::Debug for ApiUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for ApiUser {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct Credentials {
    pub username: Username,
    pub password: NonEmptyString,
}

#[derive(Debug, Clone)]
pub struct AuthBackend<S> {
    service: S,
}

impl<S> AuthBackend<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> AuthnBackend for AuthBackend<S>
where
    S: UserService + Clone,
{
    type User = ApiUser;
    type Credentials = Credentials;
    type Error = LoginError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self
            .service
            .login_user(LoginRequest {
                username: creds.username,
                password: creds.password,
            })
            .await;

        match user {
            Ok(user) => Ok(Some(user.into())),
            Err(LoginError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.service.get_user(GetUserRequest::ById(*user_id)).await;
        match user {
            Ok(user) => Ok(Some(user.into())),
            Err(GetUserError::NotFound) => return Ok(None),
            Err(GetUserError::Repository(e)) => return Err(LoginError::Repository(e)),
        }
    }
}

pub type AuthSession<S> = NoApi<axum_login::AuthSession<AuthBackend<S>>>;
