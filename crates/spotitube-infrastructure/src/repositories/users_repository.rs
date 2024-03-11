use async_trait::async_trait;
use spotitube_core::{
    errors::SpotitubeResult,
    users::repository::{UserEntity, UsersRepository},
};
use uuid::Uuid;

use crate::connection_pool::SpotitubeConnectionPool;

#[derive(Clone)]
pub struct PostgresUsersRepository {
    pool: SpotitubeConnectionPool,
}

impl PostgresUsersRepository {
    pub fn new(pool: SpotitubeConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for PostgresUsersRepository {
    async fn create_user(
        &self,
        username: &str,
        hashed_password: &str,
    ) -> SpotitubeResult<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"INSERT INTO users (username, password) values ($1::varchar, $2::varchar) returning *"#,
            username,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_username(&self, username: &str) -> SpotitubeResult<Option<UserEntity>> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"SELECT * FROM users WHERE username = $1::varchar"#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> SpotitubeResult<UserEntity> {
        let user = sqlx::query_as!(UserEntity, r#"SELECT * FROM users WHERE id = $1"#, user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}
