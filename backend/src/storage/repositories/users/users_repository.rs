use chrono::Utc;
use uuid::Uuid;

use crate::{
    storage::{
        entities::{user::UserEntity, user_choice::UserChoice},
        postgres::PostgresStorage,
    },
    tmdb::dtos::recommendation_type::RecommendationType,
};

use super::queries::{
    GET_LAST_USER_SIMILAR_CHOICE, GET_USER_BY_EMAIL, GET_USER_BY_UUID, INSERT_USER,
    INSERT_USER_CHOICE,
};

#[async_trait::async_trait]
pub trait UsersRepository: Sync + Send {
    async fn get_user_by_uuid(&self, user_uuid: &Uuid) -> anyhow::Result<Option<UserEntity>>;
    async fn get_user_by_email(&self, user_email: &str) -> anyhow::Result<Option<UserEntity>>;
    async fn create_user(
        &self,
        user_uuid: &Uuid,
        user_email: &str,
        user_hashed_password: &str,
    ) -> anyhow::Result<UserEntity>;
    async fn insert_user_choice(
        &self,
        user_uuid: &Uuid,
        recommendation_type: &RecommendationType,
    ) -> anyhow::Result<()>;
    async fn get_last_user_similar_choice(
        &self,
        user_uuid: &Uuid,
        recommendation_type: &RecommendationType,
    ) -> anyhow::Result<Option<UserChoice>>;
}

#[async_trait::async_trait]
impl UsersRepository for PostgresStorage {
    async fn get_user_by_uuid(&self, user_uuid: &Uuid) -> anyhow::Result<Option<UserEntity>> {
        let connection = self.get_connection().await?;
        let stmt = connection.prepare_cached(GET_USER_BY_UUID).await?;
        let row = connection.query_opt(&stmt, &[&user_uuid]).await?;

        let user_entity = match row {
            Some(row) => Some(serde_json::from_value(row.try_get("user_entity")?)?),
            None => None,
        };

        Ok(user_entity)
    }

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
        user_hashed_password: &str,
    ) -> anyhow::Result<UserEntity> {
        let connection = self.get_connection().await?;
        let stmt = connection.prepare_cached(INSERT_USER).await?;
        let row = connection
            .query_one(&stmt, &[&user_uuid, &user_email, &user_hashed_password])
            .await?;

        let user_entity = serde_json::from_value(row.try_get("user_entity")?)?;

        Ok(user_entity)
    }

    async fn insert_user_choice(
        &self,
        user_uuid: &Uuid,
        recommendation_type: &RecommendationType,
    ) -> anyhow::Result<()> {
        let last_user_choice = self
            .get_last_user_similar_choice(user_uuid, recommendation_type)
            .await?;

        if let Some(last_user_choice) = last_user_choice {
            let now = Utc::now().naive_utc();
            if now - last_user_choice.inserted_at <= chrono::Duration::milliseconds(200) {
                return Ok(());
            }
        }

        let connection = self.get_connection().await?;
        let stmt = connection.prepare_cached(INSERT_USER_CHOICE).await?;
        connection
            .execute(&stmt, &[&user_uuid, &recommendation_type.to_string()])
            .await?;

        Ok(())
    }

    async fn get_last_user_similar_choice(
        &self,
        user_uuid: &Uuid,
        recommendation_type: &RecommendationType,
    ) -> anyhow::Result<Option<UserChoice>> {
        let connection = self.get_connection().await?;
        let stmt = connection
            .prepare_cached(GET_LAST_USER_SIMILAR_CHOICE)
            .await?;
        let row = connection
            .query_opt(&stmt, &[&user_uuid, &recommendation_type.to_string()])
            .await?;

        let recommendation_type = match row {
            Some(row) => Some(serde_json::from_value(row.try_get("user_choice")?)?),
            None => None,
        };

        Ok(recommendation_type)
    }
}
