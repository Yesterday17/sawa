use sawa_core::{
    errors::RepositoryResult,
    models::{
        product::ProductInstanceId,
        transfer::{ProductInstanceTransferHistory, TransferReason},
    },
};
use sea_orm::{ActiveValue, entity::prelude::*};

use crate::traits::TryIntoDomainModelSimple;

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
    #[sea_orm(belongs_to, from = "product_instance_id", to = "id", skip_fk)]
    pub product_instance: HasOne<super::product_instance::Entity>,

    /// The original owner
    pub from_owner_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "from_owner_id",
        to = "id",
        relation_enum = "FromOwner",
        skip_fk
    )]
    pub from_owner: HasOne<super::user::Entity>,

    /// The original holder
    pub to_owner_id: Uuid,
    #[sea_orm(
        belongs_to,
        from = "to_owner_id",
        to = "id",
        relation_enum = "ToOwner",
        skip_fk
    )]
    pub to_owner: HasOne<super::user::Entity>,

    /// The new owner
    pub from_holder_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "from_holder_id",
        to = "id",
        relation_enum = "FromHolder",
        skip_fk
    )]
    pub from_holder: HasOne<super::user::Entity>,
    /// The new holder
    pub to_holder_id: Uuid,
    #[sea_orm(
        belongs_to,
        from = "to_holder_id",
        to = "id",
        relation_enum = "ToHolder",
        skip_fk
    )]
    pub to_holder: HasOne<super::user::Entity>,

    /// Transfer reason
    pub reason: DBTransferReason,

    /// When the transfer happened
    pub transferred_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DBTransferReason {
    Purchase,
    Delivery,
    Trade,
    Gift,
    AdminTransfer,
}

impl From<TransferReason> for DBTransferReason {
    fn from(reason: TransferReason) -> Self {
        match reason {
            TransferReason::Purchase => DBTransferReason::Purchase,
            TransferReason::Delivery => DBTransferReason::Delivery,
            TransferReason::Trade => DBTransferReason::Trade,
            TransferReason::Gift => DBTransferReason::Gift,
            TransferReason::AdminTransfer => DBTransferReason::AdminTransfer,
        }
    }
}

impl From<DBTransferReason> for TransferReason {
    fn from(db_reason: DBTransferReason) -> Self {
        match db_reason {
            DBTransferReason::Purchase => TransferReason::Purchase,
            DBTransferReason::Delivery => TransferReason::Delivery,
            DBTransferReason::Trade => TransferReason::Trade,
            DBTransferReason::Gift => TransferReason::Gift,
            DBTransferReason::AdminTransfer => TransferReason::AdminTransfer,
        }
    }
}

impl TryIntoDomainModelSimple<ProductInstanceTransferHistory> for ModelEx {
    fn try_into_domain_model_simple(self) -> RepositoryResult<ProductInstanceTransferHistory> {
        Ok(ProductInstanceTransferHistory {
            id: self.id.try_into()?,
            from_owner_id: self.from_owner_id.map(TryInto::try_into).transpose()?,
            from_holder_id: self.from_holder_id.map(TryInto::try_into).transpose()?,
            to_owner_id: self.to_owner_id.try_into()?,
            to_holder_id: self.to_holder_id.try_into()?,
            reason: self.reason.into(),
            transferred_at: self.transferred_at,
        })
    }
}

impl From<(&ProductInstanceTransferHistory, ProductInstanceId)> for ActiveModel {
    fn from(
        (transfer, product_instance_id): (&ProductInstanceTransferHistory, ProductInstanceId),
    ) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::from(transfer.id.0)),
            product_instance_id: ActiveValue::Set(Uuid::from(product_instance_id)),
            from_owner_id: ActiveValue::Set(transfer.from_owner_id.map(Into::into)),
            from_holder_id: ActiveValue::Set(transfer.from_holder_id.map(Into::into)),
            to_owner_id: ActiveValue::Set(transfer.to_owner_id.into()),
            to_holder_id: ActiveValue::Set(transfer.to_holder_id.into()),
            reason: ActiveValue::Set(transfer.reason.into()),
            transferred_at: ActiveValue::Set(transfer.transferred_at),
        }
    }
}
