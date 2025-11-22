use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{errors::RepositoryError, models::purchase::PurchaseOrderLineItem};
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
    #[sea_orm(belongs_to, from = "purchase_order_item_id", to = "id", skip_fk)]
    pub purchase_order_item: HasOne<super::purchase_order_item::Entity>,

    /// The variant to create instance for
    pub variant_id: Uuid,
    #[sea_orm(belongs_to, from = "variant_id", to = "id", skip_fk)]
    pub variant: HasOne<super::product_variant::Entity>,

    /// The ultimate owner of instance created for this line item
    pub owner_id: Uuid,
    #[sea_orm(belongs_to, from = "owner_id", to = "id", skip_fk)]
    pub owner: HasOne<super::user::Entity>,

    /// The created instance (None until fulfilled)
    pub instance_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "instance_id", to = "id", skip_fk)]
    pub instance: HasOne<super::product_instance::Entity>,

    /// The timestamp when the instance was created
    pub fulfilled_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryIntoDomainModelSimple<PurchaseOrderLineItem> for Model {
    fn try_into_domain_model_simple(self) -> Result<PurchaseOrderLineItem, RepositoryError> {
        Ok(PurchaseOrderLineItem {
            id: self.id.try_into()?,
            purchase_order_item_id: self.purchase_order_item_id.try_into()?,
            variant_id: self.variant_id.try_into()?,
            owner_id: self.owner_id.try_into()?,
            instance_id: self.instance_id.map(TryInto::try_into).transpose()?,
            fulfilled_at: self.fulfilled_at,
        })
    }
}
