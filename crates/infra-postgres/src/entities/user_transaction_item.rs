use sea_orm::entity::prelude::*;

///
/// UserTransactionItem entity (junction table for transaction items)
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_transaction_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique_key = "item")]
    pub transaction_id: Uuid,
    #[sea_orm(belongs_to, from = "transaction_id", to = "id")]
    pub transaction: HasOne<super::user_transaction::Entity>,

    #[sea_orm(unique_key = "item")]
    pub product_instance_id: Uuid,
    #[sea_orm(belongs_to, from = "product_instance_id", to = "id")]
    pub product_instance: HasOne<super::product_instance::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
