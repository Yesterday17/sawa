use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::user::{Email, User, UserId, UserUpdate, Username},
    repositories::UserRepository,
};

/// In-memory implementation of UserRepository.
#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<UserId, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError> {
        let users = self.users.read().unwrap();
        Ok(users.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| u.email.0 == email.0).cloned())
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, RepositoryError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| u.username.0 == username.0).cloned())
    }

    async fn create(&self, user: User) -> Result<User, RepositoryError> {
        let mut users = self.users.write().unwrap();
        if users.contains_key(&user.id) {
            return Err(RepositoryError::Duplicated(format!(
                "User with ID {:?} already exists",
                user.id
            )));
        }
        if users.values().any(|u| u.username.0 == user.username.0) {
            return Err(RepositoryError::Duplicated(format!(
                "User with username {} already exists",
                user.username.0
            )));
        }
        if users.values().any(|u| u.email.0 == user.email.0) {
            return Err(RepositoryError::Duplicated(format!(
                "User with email {} already exists",
                user.email.0
            )));
        }
        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn update(&self, user: UserUpdate) -> Result<User, RepositoryError> {
        let mut users = self.users.write().unwrap();
        let existing_user = users.remove(&user.id).ok_or(RepositoryError::NotFound)?;
        let updated_user = User {
            id: existing_user.id,
            username: user.username.unwrap_or(existing_user.username),
            email: user.email.unwrap_or(existing_user.email),
            password_hash: user.password_hash.unwrap_or(existing_user.password_hash),
            avatar: user.avatar,
            created_at: existing_user.created_at,
        };
        users.insert(user.id, updated_user.clone());
        Ok(updated_user)
    }

    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError> {
        let mut users = self.users.write().unwrap();
        users.remove(id);
        Ok(())
    }
}
