use sawa_core::models::{product::ProductInstanceId, transfer::UserTransactionId};
use sea_orm::{ActiveValue, entity::prelude::*};

///
/// UserTransactionItem entity (junction table for transaction items)
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_transaction_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub transaction_id: Uuid,
    #[sea_orm(belongs_to, from = "transaction_id", to = "id", skip_fk)]
    pub transaction: HasOne<super::user_transaction::Entity>,

    #[sea_orm(primary_key, auto_increment = false)]
    pub product_instance_id: Uuid,
    #[sea_orm(belongs_to, from = "product_instance_id", to = "id", skip_fk)]
    pub product_instance: HasOne<super::product_instance::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<(&UserTransactionId, &ProductInstanceId)>
    for crate::entities::user_transaction_item::ActiveModel
{
    fn from(
        (transaction_id, product_instance_id): (&UserTransactionId, &ProductInstanceId),
    ) -> Self {
        Self {
            transaction_id: ActiveValue::Set(Uuid::from(transaction_id.0)),
            product_instance_id: ActiveValue::Set(Uuid::from(product_instance_id)),
        }
    }
}
