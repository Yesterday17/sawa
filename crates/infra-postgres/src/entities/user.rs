use sea_orm::entity::prelude::*;

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
    #[sea_orm(belongs_to, from = "avatar_id", to = "id")]
    pub avatar: HasOne<super::media::Entity>,

    /// The timestamp when the user was created.
    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
