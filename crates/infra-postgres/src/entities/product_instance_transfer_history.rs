use sea_orm::entity::prelude::*;

///
/// ProductInstanceTransferHistory entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "product_instance_transfer_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The product instance this transfer belongs to
    pub product_instance_id: Uuid,
    #[sea_orm(belongs_to, from = "product_instance_id", to = "id")]
    pub product_instance: HasOne<super::product_instance::Entity>,

    /// The original owner
    pub from_owner_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "from_owner_id",
        to = "id",
        relation_enum = "FromOwner"
    )]
    pub from_owner: HasOne<super::user::Entity>,

    /// The original holder
    pub to_owner_id: Uuid,
    #[sea_orm(belongs_to, from = "to_owner_id", to = "id", relation_enum = "ToOwner")]
    pub to_owner: HasOne<super::user::Entity>,

    /// The new owner
    pub from_holder_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "from_holder_id",
        to = "id",
        relation_enum = "FromHolder"
    )]
    pub from_holder: HasOne<super::user::Entity>,
    /// The new holder
    pub to_holder_id: Uuid,
    #[sea_orm(
        belongs_to,
        from = "to_holder_id",
        to = "id",
        relation_enum = "ToHolder"
    )]
    pub to_holder: HasOne<super::user::Entity>,

    /// Transfer reason
    pub reason: String,

    /// When the transfer happened
    pub transferred_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
