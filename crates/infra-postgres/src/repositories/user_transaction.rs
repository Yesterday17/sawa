use sawa_core::{
    errors::RepositoryError,
    models::{
        transfer::{UserTransaction, UserTransactionId, UserTransactionStatus},
        user::UserId,
    },
    repositories::UserTransactionRepository,
};
use sea_orm::{QueryFilter, TransactionTrait, prelude::*, sea_query::OnConflict};

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
        let transaction_id = Uuid::from(transaction.id.0);
        let transaction_active_model: user_transaction::ActiveModel = transaction.into();

        // Prepare item data outside the closure
        let item_models: Vec<user_transaction_item::ActiveModel> = transaction
            .items
            .iter()
            .map(|item| (&transaction.id, item).into())
            .collect();

        self.db
            .transaction(|db| {
                Box::pin(async move {
                    // Save or update the transaction
                    user_transaction::Entity::insert(transaction_active_model)
                        .on_conflict(
                            OnConflict::column(user_transaction::Column::Id)
                                .update_columns([
                                    user_transaction::Column::Status,
                                    user_transaction::Column::CompletedAt,
                                    user_transaction::Column::CancelledAt,
                                ])
                                .to_owned(),
                        )
                        .exec(db)
                        .await?;

                    // Delete existing items
                    user_transaction_item::Entity::delete_many()
                        .filter(user_transaction_item::Column::TransactionId.eq(transaction_id))
                        .exec(db)
                        .await?;

                    // Insert new items
                    if !item_models.is_empty() {
                        user_transaction_item::Entity::insert_many(item_models)
                            .exec(db)
                            .await?;
                    }

                    Ok(())
                })
            })
            .await
            .map_err(DatabaseError::from)?;

        Ok(())
    }

    async fn save_batch(&self, transactions: &[UserTransaction]) -> Result<(), RepositoryError> {
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
