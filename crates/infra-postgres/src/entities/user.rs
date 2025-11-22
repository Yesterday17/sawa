use sawa_core::{errors::RepositoryError, models::user::*};
use sea_orm::{ActiveValue::Set, entity::prelude::*};

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
            id: Set(user.id.into()),
            username: Set(user.username.0),
            email: Set(user.email.0),
            password_hash: Set(user.password_hash.into_string()),
            avatar_id: Set(user.avatar.map(Into::into)),
            created_at: Set(user.created_at),
        }
    }
}
