use crate::models::misc::{MediaId, NonEmptyString};
use chrono::{DateTime, Utc};

crate::create_entity_id!(UserId);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,

    /// The username of the user.
    pub username: Username,

    /// The email of the user.
    pub email: Email,

    /// The password hash of the user.
    ///
    /// This is a hash of the user's password.
    /// It is used to verify the user's password when they log in.
    pub password_hash: NonEmptyString,

    /// The avatar media id of the user.
    pub avatar: Option<MediaId>,

    /// The timestamp when the user was created.
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Username(pub String);

#[derive(Debug, Clone)]
pub struct Email(pub String);
