use sawa_core::errors::RepositoryError;

pub trait TryIntoDomainModelSimple<DomainModel> {
    fn try_into_domain_model_simple(self) -> Result<DomainModel, RepositoryError>;
}
