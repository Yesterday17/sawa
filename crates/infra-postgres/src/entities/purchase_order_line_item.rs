use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{errors::RepositoryError, models::purchase::PurchaseOrderLineItem};
use sea_orm::{ActiveValue::Set, entity::prelude::*};

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

impl TryIntoDomainModelSimple<PurchaseOrderLineItem> for ModelEx {
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

impl From<&PurchaseOrderLineItem> for ActiveModel {
    fn from(line_item: &PurchaseOrderLineItem) -> Self {
        Self {
            id: Set(Uuid::from(line_item.id.0)),
            purchase_order_item_id: Set(Uuid::from(line_item.purchase_order_item_id.0)),
            variant_id: Set(Uuid::from(line_item.variant_id.0)),
            owner_id: Set(Uuid::from(line_item.owner_id.0)),
            instance_id: Set(line_item.instance_id.map(|id| Uuid::from(id.0))),
            fulfilled_at: Set(line_item.fulfilled_at),
        }
    }
}
