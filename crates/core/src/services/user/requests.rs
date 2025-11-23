use crate::models::misc::{MediaId, NonEmptyString};
use crate::models::user::{Email, UserId, Username};

/// Request to get a user by ID.
pub enum GetUserRequest {
    ById(UserId),
    ByUsername(Username),
    ByEmail(Email),
}

/// Request to create a new user.
pub struct CreateUserRequest {
    pub email: Email,
    pub username: Username,
    pub password: NonEmptyString,
    pub avatar: Option<MediaId>,
}

/// Request to login a user.
pub struct LoginRequest {
    pub username: Username,
    pub password: NonEmptyString,
}
