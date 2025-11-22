use crate::services::Service;
use chrono::Utc;
use sawa_core::models::product::ProductInstanceStatus;
use sawa_core::models::transfer::{UserTransaction, UserTransactionId, UserTransactionStatus};
use sawa_core::repositories::{ProductInstanceRepository, UserTransactionRepository};
use sawa_core::services::*;

impl<P, PV, PI, PO, UT, U, T, M> TransactionLifecycleService for Service<P, PV, PI, PO, UT, U, T, M>
where
    P: sawa_core::repositories::ProductRepository,
    PV: sawa_core::repositories::ProductVariantRepository,
    PI: ProductInstanceRepository,
    PO: sawa_core::repositories::PurchaseOrderRepository,
    UT: UserTransactionRepository,
    U: sawa_core::repositories::UserRepository,
    T: sawa_core::repositories::TagRepository,
    M: sawa_core::repositories::MediaRepository,
{
    async fn create_transaction(
        &self,
        req: CreateTransactionRequest,
    ) -> Result<UserTransaction, CreateTransactionError> {
        // 1. Verify items
        let mut instances = Vec::new();
        for item_id in &req.items {
            let instance = self
                .product_instance
                .find_by_id(item_id)
                .await?
                .ok_or(CreateTransactionError::ItemNotFound)?;

            if instance.owner_id != req.from_user_id {
                return Err(CreateTransactionError::ItemNotOwned);
            }

            if instance.status != ProductInstanceStatus::Active {
                return Err(CreateTransactionError::ItemNotActive);
            }

            instances.push(instance);
        }

        // 2. Lock items
        for instance in &mut instances {
            instance.status = ProductInstanceStatus::Locked;
        }
        self.product_instance.save_batch(&instances).await?;

        // 3. Create transaction
        let transaction = UserTransaction {
            id: UserTransactionId::new(),
            from_user_id: req.from_user_id,
            to_user_id: req.to_user_id,
            items: req.items,
            status: UserTransactionStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
            cancelled_at: None,
        };

        if let Err(e) = self.transaction.save(&transaction).await {
            // Rollback: Unlock items
            for instance in &mut instances {
                instance.status = ProductInstanceStatus::Active;
            }
            // Try to revert changes. If this fails, we have an inconsistency.
            // Ideally this should be in a DB transaction.
            let _ = self.product_instance.save_batch(&instances).await;
            return Err(e.into());
        }

        Ok(transaction)
    }

    async fn complete_transaction(
        &self,
        req: CompleteTransactionRequest,
    ) -> Result<UserTransaction, CompleteTransactionError> {
        let mut transaction = self
            .transaction
            .find_by_id(&req.transaction_id)
            .await?
            .ok_or(CompleteTransactionError::NotFound)?;

        if transaction.status == UserTransactionStatus::Completed {
            return Err(CompleteTransactionError::AlreadyCompleted);
        }
        if transaction.status == UserTransactionStatus::Cancelled {
            return Err(CompleteTransactionError::Cancelled);
        }

        if transaction.to_user_id != req.user_id {
            return Err(CompleteTransactionError::PermissionDenied);
        }

        // Transfer ownership
        let mut instances = Vec::new();
        for item_id in &transaction.items {
            // We assume items exist because they were checked at creation
            // But we should handle the case where they might have been deleted (though unlikely in Locked state)
            if let Some(mut instance) = self.product_instance.find_by_id(item_id).await? {
                instance.owner_id = transaction.to_user_id;
                instance.holder_id = transaction.to_user_id;
                instance.status = ProductInstanceStatus::Active;
                instances.push(instance);
            }
        }

        self.product_instance.save_batch(&instances).await?;

        transaction.status = UserTransactionStatus::Completed;
        transaction.completed_at = Some(Utc::now());

        if let Err(e) = self.transaction.save(&transaction).await {
            // Rollback: Revert ownership transfer
            for instance in &mut instances {
                instance.owner_id = transaction.from_user_id;
                instance.holder_id = transaction.from_user_id; // Assuming holder was owner
                instance.status = ProductInstanceStatus::Locked;
            }
            let _ = self.product_instance.save_batch(&instances).await;
            return Err(e.into());
        }

        Ok(transaction)
    }

    async fn cancel_transaction(
        &self,
        req: CancelTransactionRequest,
    ) -> Result<UserTransaction, CancelTransactionError> {
        let mut transaction = self
            .transaction
            .find_by_id(&req.transaction_id)
            .await?
            .ok_or(CancelTransactionError::NotFound)?;

        if transaction.status == UserTransactionStatus::Completed {
            return Err(CancelTransactionError::AlreadyCompleted);
        }
        if transaction.status == UserTransactionStatus::Cancelled {
            return Err(CancelTransactionError::AlreadyCancelled);
        }

        if transaction.from_user_id != req.user_id && transaction.to_user_id != req.user_id {
            return Err(CancelTransactionError::PermissionDenied);
        }

        // Unlock items
        let mut instances = Vec::new();
        for item_id in &transaction.items {
            if let Some(mut instance) = self.product_instance.find_by_id(item_id).await? {
                instance.status = ProductInstanceStatus::Active;
                instances.push(instance);
            }
        }
        self.product_instance.save_batch(&instances).await?;

        transaction.status = UserTransactionStatus::Cancelled;
        transaction.cancelled_at = Some(Utc::now());

        if let Err(e) = self.transaction.save(&transaction).await {
            // Rollback: Lock items again
            for instance in &mut instances {
                instance.status = ProductInstanceStatus::Locked;
            }
            let _ = self.product_instance.save_batch(&instances).await;
            return Err(e.into());
        }

        Ok(transaction)
    }
}
