use sawa_core::{
    models::misc::{Media, MediaId},
    repositories::*,
    services::{
        CreateMediaBatchRequest, CreateMediaError, CreateMediaRequest, GetMediaError,
        GetMediaRequest, MediaService,
    },
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> MediaService for Service<P, PV, PI, PO, UT, U, T, M>
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
    async fn get_media(&self, req: GetMediaRequest) -> Result<Media, GetMediaError> {
        self.media
            .find_by_id(&req.id)
            .await?
            .ok_or(GetMediaError::NotFound)
    }

    async fn create_media(&self, req: CreateMediaRequest) -> Result<Media, CreateMediaError> {
        let media = Media {
            id: MediaId::new(),
            url: req.url,
        };

        self.media.save(&media).await?;

        Ok(media)
    }

    async fn create_media_batch(
        &self,
        req: CreateMediaBatchRequest,
    ) -> Result<Vec<Media>, CreateMediaError> {
        let mut medias = Vec::with_capacity(req.urls.len());

        for url in req.urls {
            let media = Media {
                id: MediaId::new(),
                url,
            };
            self.media.save(&media).await?;
            medias.push(media);
        }

        Ok(medias)
    }
}
