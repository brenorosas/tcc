use uuid::Uuid;

use crate::storage::{entities::user::UserEntity, postgres::PostgresStorage};

use super::queries::{GET_USER_BY_EMAIL, INSERT_USER};

#[async_trait::async_trait]
pub trait UsersRepository: Sync + Send {
    async fn get_user_by_email(&self, user_email: &str) -> anyhow::Result<Option<UserEntity>>;
    async fn create_user(
        &self,
        user_uuid: &Uuid,
        user_email: &str,
        user_encrypted_password: &str,
    ) -> anyhow::Result<UserEntity>;
}

#[async_trait::async_trait]
impl UsersRepository for PostgresStorage {
    async fn get_user_by_email(&self, user_email: &str) -> anyhow::Result<Option<UserEntity>> {
        let connection = self.get_connection().await?;
        let stmt = connection.prepare_cached(GET_USER_BY_EMAIL).await?;
        let row = connection.query_opt(&stmt, &[&user_email]).await?;

        let user_entity = match row {
            Some(row) => Some(serde_json::from_value(row.try_get("user_entity")?)?),
            None => None,
        };

        Ok(user_entity)
    }

    async fn create_user(
        &self,
        user_uuid: &Uuid,
        user_email: &str,
        user_encrypted_password: &str,
    ) -> anyhow::Result<UserEntity> {
        let connection = self.get_connection().await?;
        let stmt = connection.prepare_cached(INSERT_USER).await?;
        let row = connection
            .query_one(&stmt, &[&user_uuid, &user_email, &user_encrypted_password])
            .await?;

        let user_entity = serde_json::from_value(row.try_get("user_entity")?)?;

        Ok(user_entity)
    }
}
