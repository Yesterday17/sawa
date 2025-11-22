use crate::services::Service;
use sawa_core::models::transfer::UserTransaction;
use sawa_core::repositories::*;
use sawa_core::services::*;

impl<P, PV, PI, PO, UT, U, T, M> TransactionService for Service<P, PV, PI, PO, UT, U, T, M>
where
    P: ProductRepository,
    PV: ProductVariantRepository,
    PI: ProductInstanceRepository,
    PO: PurchaseOrderRepository,
    UT: UserTransactionRepository,
    U: UserRepository,
    T: TagRepository,
    M: MediaRepository,
{
    async fn get_transaction(
        &self,
        req: GetTransactionRequest,
    ) -> Result<UserTransaction, GetTransactionError> {
        let transaction = self
            .transaction
            .find_by_id(&req.transaction_id)
            .await?
            .ok_or(GetTransactionError::NotFound)?;
        if transaction.from_user_id != req.user_id && transaction.to_user_id != req.user_id {
            return Err(GetTransactionError::NotFound);
        }
        Ok(transaction)
    }
}
