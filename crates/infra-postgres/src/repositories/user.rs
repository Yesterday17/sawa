use crate::{
    entities::user::{Column, Entity},
    error::DatabaseError,
};
use sawa_core::{
    errors::RepositoryError,
    models::user::{Email, User, UserId, Username},
    repositories::UserRepository,
};
use sea_orm::{QueryFilter, prelude::*};

pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError> {
        let entity = Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity.map(|e| e.try_into()).transpose()
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError> {
        let entity = Entity::find()
            .filter(Column::Email.eq(&email.0))
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity.map(|e| e.try_into()).transpose()
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, RepositoryError> {
        let entity = Entity::find()
            .filter(Column::Username.eq(&username.0))
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity.map(|e| e.try_into()).transpose()
    }

    async fn save(&self, user: User) -> Result<User, RepositoryError> {
        let active_model: crate::entities::user::ActiveModel = user.into();

        let user = Entity::insert(active_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(Column::Id)
                    .update_columns([
                        Column::Username,
                        Column::Email,
                        Column::PasswordHash,
                        Column::AvatarId,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(user.try_into()?)
    }

    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError> {
        Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}
