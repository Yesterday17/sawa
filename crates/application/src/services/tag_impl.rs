use sawa_core::{
    errors::RepositoryError,
    models::misc::{NonEmptyString, Tag, TagId},
    repositories::*,
    services::{CreateTagError, CreateTagRequest, GetTagError, GetTagRequest, TagService},
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> TagService for Service<P, PV, PI, PO, UT, U, T, M>
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
    async fn get_tag(&self, req: GetTagRequest) -> Result<Tag, GetTagError> {
        self.tag
            .find_by_id(&req.id)
            .await?
            .ok_or(GetTagError::NotFound)
    }

    async fn create_tag(&self, req: CreateTagRequest) -> Result<Tag, CreateTagError> {
        Ok(self
            .get_or_create_tag_by_name(req.name, Some(req.description), req.parent_id)
            .await?)
    }
}

/// Extension methods for TagService to support lazy tag creation.
///
/// These methods provide convenience functions for common tag operations.
impl<P, PV, PI, PO, UT, U, T, M> Service<P, PV, PI, PO, UT, U, T, M>
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
    /// Get or create a tag by name (lazy creation).
    ///
    /// This method:
    /// 1. Searches for an existing tag with the given name (case-sensitive)
    /// 2. Returns the existing tag if found
    /// 3. Creates a new tag if not found
    ///
    /// This is useful when creating product variants where you want to specify
    /// tag names directly without explicitly creating tags first.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Instead of:
    /// let tag = service.create_tag(CreateTagRequest { name: "Hatsune Miku".into(), ... }).await?;
    /// variant.add_tag(tag.id);
    ///
    /// // You can do:
    /// let tag_id = service.get_or_create_tag_by_name("Hatsune Miku".try_into()?, None).await?.id;
    /// variant.add_tag(tag_id);
    /// ```
    pub async fn get_or_create_tag_by_name(
        &self,
        name: NonEmptyString,
        description: Option<String>,
        parent_id: Option<TagId>,
    ) -> Result<Tag, RepositoryError> {
        // Search for existing tag by name (case-insensitive)
        let existing_tags = self.tag.find_by_name(name.as_str()).await?;
        if let Some(existing) = existing_tags {
            return Ok(existing);
        }

        // Create new tag
        let mut tag = if let Some(parent_id) = parent_id {
            Tag::with_parent(name, parent_id)
        } else {
            Tag::new(name)
        };
        if let Some(description) = description {
            tag.set_description(description);
        }

        self.tag.save(&tag).await?;

        Ok(tag)
    }
}
