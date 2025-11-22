use sawa_core::{
    errors::RepositoryError,
    models::{
        transfer::{UserTransaction, UserTransactionId, UserTransactionStatus},
        user::UserId,
    },
    repositories::UserTransactionRepository,
};
use sea_orm::{QueryFilter, prelude::*, sea_query::OnConflict};

use crate::{
    entities::user_transaction, error::DatabaseError, traits::TryIntoDomainModelSimple,
    user_transaction_item,
};

pub struct PostgresUserTransactionRepository {
    db: DatabaseConnection,
}

impl PostgresUserTransactionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl UserTransactionRepository for PostgresUserTransactionRepository {
    async fn find_by_id(
        &self,
        id: &UserTransactionId,
    ) -> Result<Option<UserTransaction>, RepositoryError> {
        let entity = user_transaction::Entity::load()
            .filter(user_transaction::Column::Id.eq(Uuid::from(id.0)))
            .with(user_transaction_item::Entity)
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_by_from_user(
        &self,
        from_user_id: &UserId,
        status: Option<UserTransactionStatus>,
    ) -> Result<Vec<UserTransaction>, RepositoryError> {
        let mut query = user_transaction::Entity::load()
            .with(user_transaction_item::Entity)
            .filter(user_transaction::Column::FromUserId.eq(Uuid::from(from_user_id.0)));

        if let Some(status) = status {
            let db_status: crate::entities::user_transaction::DBUserTransactionStatus =
                status.into();
            query = query.filter(user_transaction::Column::Status.eq(db_status));
        }

        let entities = query.all(&self.db).await.map_err(DatabaseError)?;
        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_to_user(
        &self,
        to_user_id: &UserId,
        status: Option<UserTransactionStatus>,
    ) -> Result<Vec<UserTransaction>, RepositoryError> {
        let mut query = user_transaction::Entity::load()
            .with(user_transaction_item::Entity)
            .filter(user_transaction::Column::ToUserId.eq(Uuid::from(to_user_id.0)));

        if let Some(status) = status {
            let db_status: crate::entities::user_transaction::DBUserTransactionStatus =
                status.into();
            query = query.filter(user_transaction::Column::Status.eq(db_status));
        }

        let entities = query.all(&self.db).await.map_err(DatabaseError)?;
        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, transaction: &UserTransaction) -> Result<(), RepositoryError> {
        // TODO: Save transaction with items

        let active_model: crate::entities::user_transaction::ActiveModel = transaction.into();
        user_transaction::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(user_transaction::Column::Id)
                    .update_columns([
                        user_transaction::Column::Status,
                        user_transaction::Column::CompletedAt,
                        user_transaction::Column::CancelledAt,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        user_transaction_item::Entity::delete_many()
            .filter(user_transaction_item::Column::TransactionId.eq(Uuid::from(transaction.id.0)))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;
        for item in transaction.items.iter() {
            let active_model: crate::entities::user_transaction_item::ActiveModel =
                (&transaction.id, item).into();
            user_transaction_item::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(DatabaseError)?;
        }
        Ok(())
    }

    async fn save_batch(&self, transactions: &[UserTransaction]) -> Result<(), RepositoryError> {
        // TODO: Save batch of transactions with items
        for transaction in transactions {
            self.save(transaction).await?;
        }
        Ok(())
    }

    async fn delete(&self, id: &UserTransactionId) -> Result<(), RepositoryError> {
        user_transaction::Entity::delete_by_id(Uuid::from(id.0))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;
        user_transaction_item::Entity::delete_many()
            .filter(user_transaction_item::Column::TransactionId.eq(Uuid::from(id.0)))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}
