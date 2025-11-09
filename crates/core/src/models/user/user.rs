use chrono::{DateTime, Utc};
use uuid::{NonNilUuid, Uuid};

use crate::models::misc::{MediaId, NonEmptyString};

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

    /// The timestamp when the user was last updated.
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub NonNilUuid);

impl UserId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

#[derive(Debug, Clone)]
pub struct Username(pub String);

#[derive(Debug, Clone)]
pub struct Email(pub String);
