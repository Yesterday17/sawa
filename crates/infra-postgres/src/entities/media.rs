use sea_orm::entity::prelude::*;

///
/// Media entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The publicly accessible URL of the media.
    pub url: String,
}

impl ActiveModelBehavior for ActiveModel {}
