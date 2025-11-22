use sawa_core::{errors::RepositoryError, models::user::*};
use sea_orm::{ActiveValue, entity::prelude::*};

/// User entity
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The username of the user.
    #[sea_orm(unique)]
    pub username: String,

    /// The email address of the user.
    #[sea_orm(unique)]
    pub email: String,

    /// The password hash of the user.
    pub password_hash: String,

    /// The avatar media id of the user.
    pub avatar_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "avatar_id", to = "id", skip_fk)]
    pub avatar: HasOne<super::media::Entity>,

    /// The timestamp when the user was created.
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Model> for User {
    type Error = RepositoryError;

    fn try_from(model: Model) -> Result<Self, Self::Error> {
        Ok(User {
            id: model.id.try_into()?,
            username: Username(model.username),
            email: Email(model.email),
            password_hash: model.password_hash.try_into()?,
            avatar: model.avatar_id.map(|id| id.try_into()).transpose()?.into(),
            created_at: model.created_at,
        })
    }
}

impl From<User> for crate::entities::user::ActiveModel {
    fn from(user: User) -> Self {
        Self {
            id: ActiveValue::Set(user.id.into()),
            username: ActiveValue::Set(user.username.0),
            email: ActiveValue::Set(user.email.0),
            password_hash: ActiveValue::Set(user.password_hash.into_string()),
            avatar_id: ActiveValue::Set(user.avatar.map(Into::into)),
            created_at: ActiveValue::Set(user.created_at),
        }
    }
}

impl From<UserUpdate> for crate::entities::user::ActiveModel {
    fn from(user: UserUpdate) -> Self {
        Self {
            id: ActiveValue::unchanged(user.id.into()),
            username: user
                .username
                .map(|username| ActiveValue::Set(username.0))
                .unwrap_or(ActiveValue::NotSet),
            email: user
                .email
                .map(|email| ActiveValue::Set(email.0))
                .unwrap_or(ActiveValue::NotSet),
            password_hash: user
                .password_hash
                .map(|password_hash| ActiveValue::Set(password_hash.into_string()))
                .unwrap_or(ActiveValue::NotSet),
            avatar_id: ActiveValue::Set(user.avatar.map(Into::into)),
            created_at: ActiveValue::NotSet,
        }
    }
}
