use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::{
        purchase::PurchaseOrderId,
        transfer::{TransactionStatus, UserTransaction, UserTransactionId},
        user::UserId,
    },
    repositories::UserTransactionRepository,
};

/// In-memory implementation of UserTransactionRepository.
#[derive(Clone)]
pub struct InMemoryUserTransactionRepository {
    transactions: Arc<RwLock<HashMap<UserTransactionId, UserTransaction>>>,
}

impl InMemoryUserTransactionRepository {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryUserTransactionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserTransactionRepository for InMemoryUserTransactionRepository {
    async fn find_by_id(
        &self,
        id: &UserTransactionId,
    ) -> Result<Option<UserTransaction>, RepositoryError> {
        let transactions = self.transactions.read().unwrap();
        Ok(transactions.get(id).cloned())
    }

    async fn find_by_from_user(
        &self,
        from_user_id: &UserId,
        status: Option<TransactionStatus>,
    ) -> Result<Vec<UserTransaction>, RepositoryError> {
        let transactions = self.transactions.read().unwrap();
        Ok(transactions
            .values()
            .filter(|t| t.from_user_id == *from_user_id && status.map_or(true, |s| t.status == s))
            .cloned()
            .collect())
    }

    async fn find_by_to_user(
        &self,
        to_user_id: &UserId,
        status: Option<TransactionStatus>,
    ) -> Result<Vec<UserTransaction>, RepositoryError> {
        let transactions = self.transactions.read().unwrap();
        Ok(transactions
            .values()
            .filter(|t| t.to_user_id == *to_user_id && status.map_or(true, |s| t.status == s))
            .cloned()
            .collect())
    }

    async fn find_by_source_order(
        &self,
        order_id: &PurchaseOrderId,
    ) -> Result<Vec<UserTransaction>, RepositoryError> {
        let transactions = self.transactions.read().unwrap();
        Ok(transactions
            .values()
            .filter(|t| t.source_order_id == Some(*order_id))
            .cloned()
            .collect())
    }

    async fn save(&self, transaction: &UserTransaction) -> Result<(), RepositoryError> {
        let mut transactions = self.transactions.write().unwrap();
        transactions.insert(transaction.id, transaction.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserTransactionId) -> Result<(), RepositoryError> {
        let mut transactions = self.transactions.write().unwrap();
        transactions.remove(id);
        Ok(())
    }
}
