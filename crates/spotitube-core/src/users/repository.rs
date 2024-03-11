use std::sync::Arc;

use axum::async_trait;
use spotitube_domain::users::UserDto;
use sqlx::prelude::FromRow;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use crate::errors::SpotitubeResult;

pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
pub trait UsersRepository {
    async fn create_user(
        &self,
        username: &str,
        hashed_password: &str,
    ) -> SpotitubeResult<UserEntity>;

    async fn get_user_by_username(&self, username: &str) -> SpotitubeResult<Option<UserEntity>>;

    async fn get_user_by_id(&self, user_id: &Uuid) -> SpotitubeResult<UserEntity>;
}

#[derive(FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            username: self.username,
            token: token,
        }
    }
}
