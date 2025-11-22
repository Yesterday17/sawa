use sawa_core::{
    models::user::{Email, User, UserId, Username},
    repositories::UserRepository,
};

fn create_test_user(username: &str, email: &str) -> User {
    User {
        id: UserId::new(),
        username: Username(username.to_string()),
        email: Email(email.to_string()),
        password_hash: "hash".try_into().unwrap(),
        avatar: None,
        created_at: chrono::Utc::now(),
    }
}

fn create_random_test_user() -> User {
    let unique = uuid::Uuid::new_v4().to_string();
    create_test_user(
        &format!("user_{}", &unique[..8]),
        &format!("{}@example.com", unique),
    )
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: UserRepository>(repo: R) {
    let user = create_random_test_user();
    let user = repo.create(user).await.unwrap();

    let found = repo.find_by_id(&user.id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);

    // Clean up
    repo.delete(&user.id).await.unwrap();
}

/// Test find_by_email finds user by email.
pub async fn test_find_by_email<R: UserRepository>(repo: R) {
    let user = create_random_test_user();
    let user = repo.create(user).await.unwrap();

    let by_email = repo.find_by_email(&user.email).await.unwrap();
    assert!(by_email.is_some());
    assert_eq!(by_email.unwrap().id, user.id);

    // Clean up
    repo.delete(&user.id).await.unwrap();
}

/// Test find_by_username finds user by username.
pub async fn test_find_by_username<R: UserRepository>(repo: R) {
    let user = create_random_test_user();
    let user = repo.create(user).await.unwrap();

    let by_username = repo.find_by_username(&user.username).await.unwrap();
    assert!(by_username.is_some());
    assert_eq!(by_username.unwrap().id, user.id);

    // Clean up
    repo.delete(&user.id).await.unwrap();
}

/// Test find_by_email returns None for different user's email.
pub async fn test_find_by_email_returns_correct_user<R: UserRepository>(repo: R) {
    let user_a = create_random_test_user();
    let user_b = create_random_test_user();

    let user_a = repo.create(user_a).await.unwrap();
    let user_b = repo.create(user_b).await.unwrap();

    // Find by user A's email should return user A
    let found = repo.find_by_email(&user_a.email).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_a.id);

    // Find by user B's email should return user B
    let found = repo.find_by_email(&user_b.email).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_b.id);

    // Clean up
    repo.delete(&user_a.id).await.unwrap();
    repo.delete(&user_b.id).await.unwrap();
}

/// Test find_by_username returns correct user.
pub async fn test_find_by_username_returns_correct_user<R: UserRepository>(repo: R) {
    let user_a = create_random_test_user();
    let user_b = create_random_test_user();

    let user_a = repo.create(user_a).await.unwrap();
    let user_b = repo.create(user_b).await.unwrap();

    // Find by user A's username should return user A
    let found = repo.find_by_username(&user_a.username).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_a.id);

    // Find by user B's username should return user B
    let found = repo.find_by_username(&user_b.username).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_b.id);

    // Clean up
    repo.delete(&user_a.id).await.unwrap();
    repo.delete(&user_b.id).await.unwrap();
}

/// Test delete removes user.
pub async fn test_delete<R: UserRepository>(repo: R) {
    let user = create_random_test_user();
    let user = repo.create(user).await.unwrap();

    repo.delete(&user.id).await.unwrap();

    let after_delete = repo.find_by_id(&user.id).await.unwrap();
    assert!(after_delete.is_none());
}

/// Test duplicate creation and not found update.
pub async fn test_duplicate_and_not_found<R: UserRepository>(repo: R) {
    let user = create_test_user("duplicate_test", "duplicate@example.com");

    // 1. Create user
    repo.create(user.clone()).await.unwrap();

    // 2. Try to create duplicate user (same ID)
    let result = repo.create(user.clone()).await;
    assert!(matches!(
        result,
        Err(sawa_core::errors::RepositoryError::Duplicated(_))
    ));

    // 3. Try to create duplicate user (same username)
    let mut duplicate_username = create_test_user("duplicate_test", "other@example.com");
    // Ensure ID is different
    duplicate_username.id = UserId::new();
    let result = repo.create(duplicate_username).await;
    assert!(matches!(
        result,
        Err(sawa_core::errors::RepositoryError::Duplicated(_))
    ));

    // 4. Try to create duplicate user (same email)
    let mut duplicate_email = create_test_user("other_user", "duplicate@example.com");
    // Ensure ID is different
    duplicate_email.id = UserId::new();
    let result = repo.create(duplicate_email).await;
    assert!(matches!(
        result,
        Err(sawa_core::errors::RepositoryError::Duplicated(_))
    ));

    // 5. Try to update unknown user
    let unknown_user = create_test_user("unknown", "unknown@example.com");
    let update = sawa_core::models::user::UserUpdate {
        id: unknown_user.id,
        username: None,
        email: None,
        password_hash: None,
        avatar: None,
    };
    let result = repo.update(update).await;
    eprintln!("{:?}", result);
    assert!(matches!(
        result,
        Err(sawa_core::errors::RepositoryError::NotFound)
    ));

    // Clean up
    repo.delete(&user.id).await.unwrap();
}
