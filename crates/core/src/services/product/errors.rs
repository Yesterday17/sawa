use crate::errors::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetProductError {
    #[error("Product not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CreateProductError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum ListProductsError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum GetProductVariantError {
    #[error("Product variant not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CreateProductVariantError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("Product not found")]
    ProductNotFound,
}

#[derive(Debug, Error)]
pub enum ListProductVariantsError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
