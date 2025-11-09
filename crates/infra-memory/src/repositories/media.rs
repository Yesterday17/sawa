use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::misc::{Media, MediaId},
    repositories::MediaRepository,
};

/// In-memory implementation of MediaRepository.
pub struct InMemoryMediaRepository {
    media: Arc<RwLock<HashMap<MediaId, Media>>>,
}

impl InMemoryMediaRepository {
    pub fn new() -> Self {
        Self {
            media: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryMediaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaRepository for InMemoryMediaRepository {
    async fn find_by_id(&self, id: &MediaId) -> Result<Option<Media>, RepositoryError> {
        let media = self.media.read().unwrap();
        Ok(media.get(id).cloned())
    }

    async fn find_by_ids(&self, ids: &[MediaId]) -> Result<Vec<Media>, RepositoryError> {
        let media = self.media.read().unwrap();
        Ok(ids.iter().filter_map(|id| media.get(id).cloned()).collect())
    }

    async fn save(&self, media_item: &Media) -> Result<(), RepositoryError> {
        let mut media = self.media.write().unwrap();
        media.insert(media_item.id, media_item.clone());
        Ok(())
    }

    async fn delete(&self, id: &MediaId) -> Result<(), RepositoryError> {
        let mut media = self.media.write().unwrap();
        media.remove(id);
        Ok(())
    }
}

