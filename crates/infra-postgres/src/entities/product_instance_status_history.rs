use sea_orm::entity::prelude::*;

///
/// ProductInstanceStatusHistory entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_instance_status_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub product_instance_id: Uuid,
    #[sea_orm(belongs_to, from = "product_instance_id", to = "id")]
    pub product_instance: HasOne<super::product_instance::Entity>,

    /// The new status after this change
    pub status: super::product_instance::DBProductInstanceStatus,

    /// When the status was changed
    pub changed_at: DateTimeUtc,

    /// Reason for the status change
    pub reason: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}
