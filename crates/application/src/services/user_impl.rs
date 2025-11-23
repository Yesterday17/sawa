use chrono::Utc;
use sawa_core::{
    models::{
        misc::NonEmptyString,
        user::{User, UserId},
    },
    repositories::*,
    services::{
        CreateUserError, CreateUserRequest, GetUserError, GetUserRequest, LoginError, LoginRequest,
        UserService,
    },
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> UserService for Service<P, PV, PI, PO, UT, U, T, M>
where
    P: ProductRepository,
    PV: ProductVariantRepository,
    PI: ProductInstanceRepository,
    PO: PurchaseOrderRepository,
    UT: UserTransactionRepository,
    U: UserRepository,
    T: TagRepository,
    M: MediaRepository,
{
    async fn get_user(&self, req: GetUserRequest) -> Result<User, GetUserError> {
        match req {
            GetUserRequest::ById(user_id) => self
                .user
                .find_by_id(&user_id)
                .await?
                .ok_or(GetUserError::NotFound),
            GetUserRequest::ByUsername(username) => self
                .user
                .find_by_username(&username)
                .await?
                .ok_or(GetUserError::NotFound),
            GetUserRequest::ByEmail(email) => self
                .user
                .find_by_email(&email)
                .await?
                .ok_or(GetUserError::NotFound),
        }
    }

    async fn login_user(&self, req: LoginRequest) -> Result<User, LoginError> {
        // Find user by username
        let user = self
            .user
            .find_by_username(&req.username)
            .await?
            .ok_or(sawa_core::services::LoginError::NotFound)?;

        let password_hash = user.password_hash.clone();
        // Verify password
        let is_valid = tokio::task::spawn_blocking(move || {
            bcrypt::verify(req.password.as_str(), &password_hash)
        })
        .await
        .map_err(|e| LoginError::FailedToVerifyPassword(e.to_string()))?
        .map_err(|e| LoginError::FailedToVerifyPassword(e.to_string()))?;

        if !is_valid {
            return Err(LoginError::InvalidPassword);
        }

        Ok(user)
    }

    async fn create_user(&self, req: CreateUserRequest) -> Result<User, CreateUserError> {
        // Check if user with same email already exists
        if self.user.find_by_email(&req.email).await?.is_some() {
            return Err(CreateUserError::AlreadyExists);
        }

        // Check if user with same username already exists
        if self.user.find_by_username(&req.username).await?.is_some() {
            return Err(CreateUserError::AlreadyExists);
        }

        // Hash password using bcrypt
        // Default cost factor is 12, which provides a good balance between security and performance
        let password_hash: NonEmptyString =
            bcrypt::hash(req.password.as_str(), bcrypt::DEFAULT_COST)
                .map_err(|e| CreateUserError::FailedToHashPassword(e.to_string()))?
                .try_into()
                .expect("Hashed password could never be empty");

        // Create new user
        let user = User {
            id: UserId::new(),
            username: req.username,
            email: req.email,
            password_hash,
            avatar: req.avatar,
            created_at: Utc::now(),
        };

        // Save user
        let saved_user = self.user.create(user).await?;

        Ok(saved_user)
    }
}
