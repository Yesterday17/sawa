use sawa_core::{
    errors::RepositoryResult,
    models::transfer::{UserTransaction, UserTransactionStatus},
};
use sea_orm::{ActiveValue, entity::prelude::*};

use crate::traits::TryIntoDomainModelSimple;

///
/// UserTransaction entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique)]
    pub id: Uuid,

    /// User transferring items out
    pub from_user_id: Uuid,
    #[sea_orm(
        belongs_to,
        derive_enum = "FromUser",
        from = "from_user_id",
        to = "id",
        skip_fk
    )]
    pub from_user: HasOne<super::user::Entity>,

    /// User receiving items
    pub to_user_id: Uuid,
    #[sea_orm(
        belongs_to,
        derive_enum = "ToUser",
        from = "to_user_id",
        to = "id",
        skip_fk
    )]
    pub to_user: HasOne<super::user::Entity>,

    /// Items being transferred
    #[sea_orm(has_many, skip_fk)]
    pub items: HasMany<super::user_transaction_item::Entity>,

    /// Transaction status
    pub status: DBUserTransactionStatus,

    /// The timestamp when the transaction was created.
    pub created_at: DateTimeUtc,
    /// The timestamp when the transaction was completed.
    pub completed_at: Option<DateTimeUtc>,
    /// The timestamp when the transaction was cancelled.
    pub cancelled_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DBUserTransactionStatus {
    Pending,
    Completed,
    Cancelled,
}

impl From<UserTransactionStatus> for DBUserTransactionStatus {
    fn from(status: UserTransactionStatus) -> Self {
        match status {
            UserTransactionStatus::Pending => DBUserTransactionStatus::Pending,
            UserTransactionStatus::Completed => DBUserTransactionStatus::Completed,
            UserTransactionStatus::Cancelled => DBUserTransactionStatus::Cancelled,
        }
    }
}

impl From<DBUserTransactionStatus> for UserTransactionStatus {
    fn from(status: DBUserTransactionStatus) -> Self {
        match status {
            DBUserTransactionStatus::Pending => UserTransactionStatus::Pending,
            DBUserTransactionStatus::Completed => UserTransactionStatus::Completed,
            DBUserTransactionStatus::Cancelled => UserTransactionStatus::Cancelled,
        }
    }
}

impl TryIntoDomainModelSimple<UserTransaction> for ModelEx {
    fn try_into_domain_model_simple(self) -> RepositoryResult<UserTransaction> {
        let items = self
            .items
            .into_iter()
            .map(|i| i.product_instance_id.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(UserTransaction {
            id: self.id.try_into()?,
            from_user_id: self.from_user_id.try_into()?,
            to_user_id: self.to_user_id.try_into()?,
            items,
            status: self.status.into(),
            created_at: self.created_at,
            completed_at: self.completed_at,
            cancelled_at: self.cancelled_at,
        })
    }
}

impl From<&UserTransaction> for crate::entities::user_transaction::ActiveModel {
    fn from(transaction: &UserTransaction) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::from(transaction.id.0)),
            from_user_id: ActiveValue::Set(Uuid::from(transaction.from_user_id.0)),
            to_user_id: ActiveValue::Set(Uuid::from(transaction.to_user_id.0)),
            status: ActiveValue::Set(transaction.status.into()),
            created_at: ActiveValue::Set(transaction.created_at),
            completed_at: ActiveValue::Set(transaction.completed_at),
            cancelled_at: ActiveValue::Set(transaction.cancelled_at),
        }
    }
}
