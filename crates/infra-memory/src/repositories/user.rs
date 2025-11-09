use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::user::{Email, User, UserId, Username},
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

    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        let mut users = self.users.write().unwrap();
        users.insert(user.id, user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError> {
        let mut users = self.users.write().unwrap();
        users.remove(id);
        Ok(())
    }
}
