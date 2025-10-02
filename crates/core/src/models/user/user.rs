use chrono::{DateTime, Utc};
use uuid::NonNilUuid;

use crate::models::misc::{MediaId, NonEmptyString};

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

    /// The timestamp when the user was last updated.
    pub updated_at: DateTime<Utc>,
}

pub struct UserId(pub NonNilUuid);

pub struct Username(pub String);

pub struct Email(pub String);
