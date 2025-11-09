use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::product::{Product, ProductId},
    repositories::ProductRepository,
};

/// In-memory implementation of ProductRepository.
pub struct InMemoryProductRepository {
    products: Arc<RwLock<HashMap<ProductId, Product>>>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductRepository for InMemoryProductRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError> {
        let products = self.products.read().unwrap();
        Ok(products.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let products = self.products.read().unwrap();
        Ok(products.values().cloned().collect())
    }

    async fn save(&self, product: &Product) -> Result<(), RepositoryError> {
        let mut products = self.products.write().unwrap();
        products.insert(product.id, product.clone());
        Ok(())
    }

    async fn delete(&self, id: &ProductId) -> Result<(), RepositoryError> {
        let mut products = self.products.write().unwrap();
        products.remove(id);
        Ok(())
    }
}

