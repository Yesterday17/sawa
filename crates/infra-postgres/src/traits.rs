use sawa_core::errors::RepositoryError;

pub trait TryIntoDomainModel<DomainModel> {
    type Relation;

    fn try_into_domain_model(
        self,
        relations: Self::Relation,
    ) -> Result<DomainModel, RepositoryError>;
}

pub trait TryIntoDomainModelSimple<DomainModel> {
    fn try_into_domain_model_simple(self) -> Result<DomainModel, RepositoryError>;
}

impl<T, DomainModel> TryIntoDomainModel<DomainModel> for T
where
    T: TryIntoDomainModelSimple<DomainModel>,
{
    type Relation = ();

    fn try_into_domain_model(
        self,
        _relations: Self::Relation,
    ) -> Result<DomainModel, RepositoryError> {
        self.try_into_domain_model_simple()
    }
}
