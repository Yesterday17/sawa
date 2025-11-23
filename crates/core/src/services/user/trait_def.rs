use super::*;
use crate::models::user::User;

/// Service for managing users (Port).
///
/// This service handles user account operations:
/// - Creating new users
/// - Retrieving user profiles
pub trait UserService: Send + Sync + 'static {
    /// Get a user by their ID.
    fn get_user(
        &self,
        req: GetUserRequest,
    ) -> impl Future<Output = Result<User, GetUserError>> + Send;

    /// Login a user.
    fn login_user(
        &self,
        req: LoginRequest,
    ) -> impl Future<Output = Result<User, LoginError>> + Send;

    /// Create a new user.
    fn create_user(
        &self,
        req: CreateUserRequest,
    ) -> impl Future<Output = Result<User, CreateUserError>> + Send;
}
