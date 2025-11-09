use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::misc::{Tag, TagId},
    repositories::TagRepository,
};

/// In-memory implementation of TagRepository.
#[derive(Clone)]
pub struct InMemoryTagRepository {
    tags: Arc<RwLock<HashMap<TagId, Tag>>>,
}

impl InMemoryTagRepository {
    pub fn new() -> Self {
        Self {
            tags: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryTagRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TagRepository for InMemoryTagRepository {
    async fn find_by_id(&self, id: &TagId) -> Result<Option<Tag>, RepositoryError> {
        let tags = self.tags.read().unwrap();
        Ok(tags.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Tag>, RepositoryError> {
        let tags = self.tags.read().unwrap();
        Ok(tags.values().cloned().collect())
    }

    async fn find_by_name_prefix(&self, prefix: &str) -> Result<Vec<Tag>, RepositoryError> {
        let tags = self.tags.read().unwrap();
        let prefix_lower = prefix.to_lowercase();
        Ok(tags
            .values()
            .filter(|t| t.name.as_str().to_lowercase().starts_with(&prefix_lower))
            .cloned()
            .collect())
    }

    async fn find_by_parent(&self, parent_id: &TagId) -> Result<Vec<Tag>, RepositoryError> {
        let tags = self.tags.read().unwrap();
        Ok(tags
            .values()
            .filter(|t| t.parent_tag_id == Some(*parent_id))
            .cloned()
            .collect())
    }

    async fn find_roots(&self) -> Result<Vec<Tag>, RepositoryError> {
        let tags = self.tags.read().unwrap();
        Ok(tags
            .values()
            .filter(|t| t.parent_tag_id.is_none())
            .cloned()
            .collect())
    }

    async fn save(&self, tag: &Tag) -> Result<(), RepositoryError> {
        let mut tags = self.tags.write().unwrap();
        tags.insert(tag.id, tag.clone());
        Ok(())
    }

    async fn delete(&self, id: &TagId) -> Result<(), RepositoryError> {
        let mut tags = self.tags.write().unwrap();
        tags.remove(id);
        Ok(())
    }
}
