use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{
    errors::RepositoryResult,
    models::product::{ProductInstanceId, ProductInstanceStatusHistory},
};
use sea_orm::{ActiveValue, entity::prelude::*};

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
    #[sea_orm(belongs_to, from = "product_instance_id", to = "id", skip_fk)]
    pub product_instance: HasOne<super::product_instance::Entity>,

    /// The new status after this change
    pub status: super::product_instance::DBProductInstanceStatus,

    /// When the status was changed
    pub changed_at: DateTimeUtc,

    /// Reason for the status change
    pub reason: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryIntoDomainModelSimple<ProductInstanceStatusHistory> for ModelEx {
    fn try_into_domain_model_simple(self) -> RepositoryResult<ProductInstanceStatusHistory> {
        Ok(ProductInstanceStatusHistory {
            id: self.id.try_into()?,
            status: self.status.into(),
            changed_at: self.changed_at,
            reason: self.reason,
        })
    }
}

impl From<(&ProductInstanceStatusHistory, ProductInstanceId)> for ActiveModel {
    fn from(
        (status, product_instance_id): (&ProductInstanceStatusHistory, ProductInstanceId),
    ) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::from(status.id.0)),
            product_instance_id: ActiveValue::Set(Uuid::from(product_instance_id)),
            status: ActiveValue::Set(status.status.into()),
            changed_at: ActiveValue::Set(status.changed_at),
            reason: ActiveValue::Set(status.reason.clone()),
        }
    }
}
