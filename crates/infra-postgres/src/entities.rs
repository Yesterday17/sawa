pub mod media;
pub mod product;
pub mod product_instance;
pub mod product_instance_status_history;
pub mod product_instance_transfer_history;
pub mod product_variant;
pub mod product_variant_tag;
pub mod purchase_order;
pub mod purchase_order_item;
pub mod purchase_order_line_item;
pub mod tag;
pub mod user;
pub mod user_transaction;
pub mod user_transaction_item;

pub mod prelude {
    pub use super::media::Entity as Media;
    pub use super::product::Entity as Product;
    pub use super::product_instance::Entity as ProductInstance;
    pub use super::product_instance_status_history::Entity as ProductInstanceStatusHistory;
    pub use super::product_instance_transfer_history::Entity as ProductInstanceTransferHistory;
    pub use super::product_variant::Entity as ProductVariant;
    pub use super::product_variant_tag::Entity as ProductVariantTag;
    pub use super::purchase_order::Entity as PurchaseOrder;
    pub use super::purchase_order_item::Entity as PurchaseOrderItem;
    pub use super::purchase_order_line_item::Entity as PurchaseOrderLineItem;
    pub use super::tag::Entity as Tag;
    pub use super::user::Entity as User;
    pub use super::user_transaction::Entity as UserTransaction;
    pub use super::user_transaction_item::Entity as UserTransactionItem;
}

pub async fn sync_schema(db: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    db.get_schema_builder()
        .register(prelude::Media)
        .register(prelude::Product)
        .register(prelude::ProductInstance)
        .register(prelude::ProductInstanceStatusHistory)
        .register(prelude::ProductInstanceTransferHistory)
        .register(prelude::ProductVariant)
        .register(prelude::ProductVariantTag)
        .register(prelude::PurchaseOrder)
        .register(prelude::PurchaseOrderItem)
        .register(prelude::PurchaseOrderLineItem)
        .register(prelude::Tag)
        .register(prelude::User)
        .register(prelude::UserTransaction)
        .register(prelude::UserTransactionItem)
        .sync(db)
        .await
}
