use sawa_core::{
    models::{
        product::ProductInstanceId,
        purchase::PurchaseOrderId,
        transfer::{TransactionStatus, UserTransaction, UserTransactionId},
        user::UserId,
    },
    repositories::UserTransactionRepository,
};

fn create_test_transaction(
    from_user_id: UserId,
    to_user_id: UserId,
    status: TransactionStatus,
) -> UserTransaction {
    let (completed_at, cancelled_at) = match status {
        TransactionStatus::Completed => (Some(chrono::Utc::now()), None),
        TransactionStatus::Cancelled => (None, Some(chrono::Utc::now())),
        _ => (None, None),
    };

    UserTransaction {
        id: UserTransactionId::new(),
        from_user_id,
        to_user_id,
        items: vec![ProductInstanceId::new()],
        price: None,
        status,
        source_order_id: Some(PurchaseOrderId::new()),
        created_at: chrono::Utc::now(),
        completed_at,
        cancelled_at,
        updated_at: chrono::Utc::now(),
    }
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: UserTransactionRepository>(repo: R) {
    let tx = create_test_transaction(UserId::new(), UserId::new(), TransactionStatus::Pending);
    let tx_id = tx.id;

    repo.save(&tx).await.unwrap();

    let found = repo.find_by_id(&tx_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, tx_id);
}

/// Test find_by_from_user returns sender's transactions (permission check).
pub async fn test_find_by_from_user_permission<R: UserTransactionRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();
    let user_c = UserId::new();

    // User A sends to User C
    let tx_a_to_c = create_test_transaction(user_a, user_c, TransactionStatus::Pending);
    // User B sends to User C
    let tx_b_to_c = create_test_transaction(user_b, user_c, TransactionStatus::Pending);

    repo.save(&tx_a_to_c).await.unwrap();
    repo.save(&tx_b_to_c).await.unwrap();

    // User A queries their sent transactions - should only see tx_a_to_c
    let user_a_sent = repo.find_by_from_user(&user_a, None).await.unwrap();
    assert_eq!(user_a_sent.len(), 1);
    assert_eq!(user_a_sent[0].from_user_id, user_a);
    assert!(user_a_sent.iter().all(|t| t.from_user_id != user_b));

    // User B queries their sent transactions - should only see tx_b_to_c
    let user_b_sent = repo.find_by_from_user(&user_b, None).await.unwrap();
    assert_eq!(user_b_sent.len(), 1);
    assert_eq!(user_b_sent[0].from_user_id, user_b);
}

/// Test find_by_to_user returns receiver's transactions (permission check).
pub async fn test_find_by_to_user_permission<R: UserTransactionRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();
    let user_c = UserId::new();

    // User A receives from User C
    let tx_c_to_a = create_test_transaction(user_c, user_a, TransactionStatus::Pending);
    // User B receives from User C
    let tx_c_to_b = create_test_transaction(user_c, user_b, TransactionStatus::Pending);

    repo.save(&tx_c_to_a).await.unwrap();
    repo.save(&tx_c_to_b).await.unwrap();

    // User A queries their received transactions - should only see tx_c_to_a
    let user_a_received = repo.find_by_to_user(&user_a, None).await.unwrap();
    assert_eq!(user_a_received.len(), 1);
    assert_eq!(user_a_received[0].to_user_id, user_a);
    assert!(user_a_received.iter().all(|t| t.to_user_id != user_b));

    // User B queries their received transactions - should only see tx_c_to_b
    let user_b_received = repo.find_by_to_user(&user_b, None).await.unwrap();
    assert_eq!(user_b_received.len(), 1);
    assert_eq!(user_b_received[0].to_user_id, user_b);
}

/// Test find_by_from_user with status filter.
pub async fn test_find_by_from_user_with_status<R: UserTransactionRepository>(repo: R) {
    let user_id = UserId::new();
    let other_user = UserId::new();

    let pending_tx = create_test_transaction(user_id, other_user, TransactionStatus::Pending);
    let completed_tx = create_test_transaction(user_id, other_user, TransactionStatus::Completed);

    repo.save(&pending_tx).await.unwrap();
    repo.save(&completed_tx).await.unwrap();

    // Query all sent transactions
    let all = repo.find_by_from_user(&user_id, None).await.unwrap();
    assert_eq!(all.len(), 2);

    // Query only pending
    let pending = repo
        .find_by_from_user(&user_id, Some(TransactionStatus::Pending))
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].status, TransactionStatus::Pending);

    // Query only completed
    let completed = repo
        .find_by_from_user(&user_id, Some(TransactionStatus::Completed))
        .await
        .unwrap();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].status, TransactionStatus::Completed);
}

/// Test find_by_to_user with status filter.
pub async fn test_find_by_to_user_with_status<R: UserTransactionRepository>(repo: R) {
    let user_id = UserId::new();
    let other_user = UserId::new();

    let pending_tx = create_test_transaction(other_user, user_id, TransactionStatus::Pending);
    let completed_tx = create_test_transaction(other_user, user_id, TransactionStatus::Completed);

    repo.save(&pending_tx).await.unwrap();
    repo.save(&completed_tx).await.unwrap();

    // Query all received transactions
    let all = repo.find_by_to_user(&user_id, None).await.unwrap();
    assert_eq!(all.len(), 2);

    // Query only pending
    let pending = repo
        .find_by_to_user(&user_id, Some(TransactionStatus::Pending))
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].status, TransactionStatus::Pending);
}

/// Test find_by_source_order returns related transactions.
pub async fn test_find_by_source_order<R: UserTransactionRepository>(repo: R) {
    let order_id = PurchaseOrderId::new();
    let user_a = UserId::new();
    let user_b = UserId::new();

    let mut tx1 = create_test_transaction(user_a, user_b, TransactionStatus::Pending);
    tx1.source_order_id = Some(order_id);

    let mut tx2 = create_test_transaction(user_a, user_b, TransactionStatus::Pending);
    tx2.source_order_id = Some(order_id);

    let tx3 = create_test_transaction(user_a, user_b, TransactionStatus::Pending);
    // tx3 has different source_order_id

    repo.save(&tx1).await.unwrap();
    repo.save(&tx2).await.unwrap();
    repo.save(&tx3).await.unwrap();

    let related = repo.find_by_source_order(&order_id).await.unwrap();
    assert_eq!(related.len(), 2);
    assert!(related.iter().all(|t| t.source_order_id == Some(order_id)));
}

/// Test delete removes transaction.
pub async fn test_delete<R: UserTransactionRepository>(repo: R) {
    let tx = create_test_transaction(UserId::new(), UserId::new(), TransactionStatus::Pending);
    let tx_id = tx.id;

    repo.save(&tx).await.unwrap();
    repo.delete(&tx_id).await.unwrap();

    let after_delete = repo.find_by_id(&tx_id).await.unwrap();
    assert!(after_delete.is_none());
}
