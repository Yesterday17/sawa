use crate::models::misc::{MediaId, NonEmptyString};
use crate::models::user::{Email, UserId, Username};

/// Request to get a user by ID.
pub struct GetUserRequest {
    pub id: UserId,
}

/// Request to create a new user.
pub struct CreateUserRequest {
    pub email: Email,
    pub username: Username,
    pub password: NonEmptyString,
    pub avatar: Option<MediaId>,
}
