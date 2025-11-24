use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::{
        misc::TagId,
        product::{ProductId, ProductVariant, ProductVariantId},
    },
    repositories::ProductVariantRepository,
};

/// In-memory implementation of ProductVariantRepository.
#[derive(Clone)]
pub struct InMemoryProductVariantRepository {
    variants: Arc<RwLock<HashMap<ProductVariantId, ProductVariant>>>,
}

impl InMemoryProductVariantRepository {
    pub fn new() -> Self {
        Self {
            variants: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryProductVariantRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductVariantRepository for InMemoryProductVariantRepository {
    async fn find_by_id(
        &self,
        id: &ProductVariantId,
    ) -> Result<Option<ProductVariant>, RepositoryError> {
        let variants = self.variants.read().unwrap();
        Ok(variants.get(id).cloned())
    }

    async fn load_by_ids(
        &self,
        ids: &[ProductVariantId],
    ) -> Result<Vec<Option<ProductVariant>>, RepositoryError> {
        let variants = self.variants.read().unwrap();
        let result = ids.iter().map(|id| variants.get(id).cloned()).collect();
        Ok(result)
    }

    async fn find_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<ProductVariant>, RepositoryError> {
        let variants = self.variants.read().unwrap();
        Ok(variants
            .values()
            .filter(|v| v.product_id == *product_id)
            .cloned()
            .collect())
    }

    async fn find_by_tags_all(
        &self,
        tag_ids: &[TagId],
    ) -> Result<Vec<ProductVariant>, RepositoryError> {
        let variants = self.variants.read().unwrap();
        Ok(variants
            .values()
            .filter(|v| tag_ids.iter().all(|tag| v.tags.contains(tag)))
            .cloned()
            .collect())
    }

    async fn find_by_tags_any(
        &self,
        tag_ids: &[TagId],
    ) -> Result<Vec<ProductVariant>, RepositoryError> {
        let variants = self.variants.read().unwrap();
        Ok(variants
            .values()
            .filter(|v| tag_ids.iter().any(|tag| v.tags.contains(tag)))
            .cloned()
            .collect())
    }

    async fn find_all(&self) -> Result<Vec<ProductVariant>, RepositoryError> {
        let variants = self.variants.read().unwrap();
        Ok(variants.values().cloned().collect())
    }

    async fn save(&self, variant: &ProductVariant) -> Result<(), RepositoryError> {
        let mut variants = self.variants.write().unwrap();
        variants.insert(variant.id, variant.clone());
        Ok(())
    }

    async fn delete(&self, id: &ProductVariantId) -> Result<(), RepositoryError> {
        let mut variants = self.variants.write().unwrap();
        variants.remove(id);
        Ok(())
    }
}
