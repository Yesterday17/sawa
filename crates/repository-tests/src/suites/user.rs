use sawa_core::{
    models::{
        misc::NonEmptyString,
        user::{Email, User, UserId, Username},
    },
    repositories::UserRepository,
};

fn make_string(s: &str) -> NonEmptyString {
    unsafe { NonEmptyString::new_unchecked(s.to_string()) }
}

fn create_test_user(username: &str, email: &str) -> User {
    User {
        id: UserId::new(),
        username: Username(username.to_string()),
        email: Email(email.to_string()),
        password_hash: make_string("hash"),
        avatar: None,
        created_at: chrono::Utc::now(),
    }
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: UserRepository>(repo: R) {
    let user = create_test_user("testuser", "test@example.com");
    let user_id = user.id;

    repo.save(user).await.unwrap();

    let found = repo.find_by_id(&user_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_id);
}

/// Test find_by_email finds user by email.
pub async fn test_find_by_email<R: UserRepository>(repo: R) {
    let user = create_test_user("testuser", "test@example.com");
    let user_id = user.id;

    let user = repo.save(user).await.unwrap();

    let by_email = repo.find_by_email(&user.email).await.unwrap();
    assert!(by_email.is_some());
    assert_eq!(by_email.unwrap().id, user_id);
}

/// Test find_by_username finds user by username.
pub async fn test_find_by_username<R: UserRepository>(repo: R) {
    let user = create_test_user("testuser", "test@example.com");
    let user_id = user.id;

    let user = repo.save(user).await.unwrap();

    let by_username = repo.find_by_username(&user.username).await.unwrap();
    assert!(by_username.is_some());
    assert_eq!(by_username.unwrap().id, user_id);
}

/// Test find_by_email returns None for different user's email.
pub async fn test_find_by_email_returns_correct_user<R: UserRepository>(repo: R) {
    let user_a = create_test_user("user_a", "a@example.com");
    let user_b = create_test_user("user_b", "b@example.com");

    let user_a = repo.save(user_a).await.unwrap();
    let user_b = repo.save(user_b).await.unwrap();

    // Find by user A's email should return user A
    let found = repo.find_by_email(&user_a.email).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_a.id);

    // Find by user B's email should return user B
    let found = repo.find_by_email(&user_b.email).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_b.id);
}

/// Test find_by_username returns correct user.
pub async fn test_find_by_username_returns_correct_user<R: UserRepository>(repo: R) {
    let user_a = create_test_user("user_a", "a@example.com");
    let user_b = create_test_user("user_b", "b@example.com");

    let user_a = repo.save(user_a).await.unwrap();
    let user_b = repo.save(user_b).await.unwrap();

    // Find by user A's username should return user A
    let found = repo.find_by_username(&user_a.username).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_a.id);

    // Find by user B's username should return user B
    let found = repo.find_by_username(&user_b.username).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_b.id);
}

/// Test delete removes user.
pub async fn test_delete<R: UserRepository>(repo: R) {
    let user = create_test_user("testuser", "test@example.com");
    let user_id = user.id;

    repo.save(user).await.unwrap();
    repo.delete(&user_id).await.unwrap();

    let after_delete = repo.find_by_id(&user_id).await.unwrap();
    assert!(after_delete.is_none());
}
