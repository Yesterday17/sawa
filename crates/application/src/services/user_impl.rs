use chrono::Utc;
use sawa_core::{
    models::{
        misc::NonEmptyString,
        user::{User, UserId},
    },
    repositories::*,
    services::{CreateUserError, CreateUserRequest, GetUserError, GetUserRequest, UserService},
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
        self.user
            .find_by_id(&req.id)
            .await?
            .ok_or(GetUserError::NotFound)
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
