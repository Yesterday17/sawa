use sea_orm::entity::prelude::*;

///
/// PurchaseOrderLineItem entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "purchase_order_line_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The purchase order item this order item belongs to
    pub purchase_order_item_id: Uuid,
    #[sea_orm(belongs_to, from = "purchase_order_item_id", to = "id")]
    pub purchase_order_item: HasOne<super::purchase_order_item::Entity>,

    /// The variant to create instance for
    pub variant_id: Uuid,
    #[sea_orm(belongs_to, from = "variant_id", to = "id")]
    pub variant: HasOne<super::product_variant::Entity>,

    /// The ultimate owner of instance created for this line item
    pub owner_id: Uuid,
    #[sea_orm(belongs_to, from = "owner_id", to = "id")]
    pub owner: HasOne<super::user::Entity>,

    /// The created instance (None until fulfilled)
    pub instance_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "instance_id", to = "id")]
    pub instance: HasOne<super::product_instance::Entity>,

    /// The timestamp when the instance was created
    pub fulfilled_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}
