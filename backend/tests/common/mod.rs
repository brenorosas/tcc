use std::sync::Arc;

use backend::{
    jwt::service::JwtService,
    storage::postgres::{config::PostgresConfig, PostgresStorage},
    user::service::UsersService,
};

pub async fn get_postgres_storage() -> Arc<PostgresStorage> {
    let postgres_config = PostgresConfig::new();
    let postgres_storage = Arc::new(PostgresStorage::new(postgres_config).await.unwrap());

    postgres_storage
}

pub async fn delete_all_from_db() {
    let postgres_config = PostgresConfig::new();
    let postgres_storage = Arc::new(PostgresStorage::new(postgres_config).await.unwrap());
    let connection = postgres_storage.get_connection().await.unwrap();
    connection
        .execute("DELETE FROM user_choices", &[])
        .await
        .unwrap();
    connection.execute("DELETE FROM users", &[]).await.unwrap();
}

pub async fn create_users_service() -> UsersService {
    let postgres_config = PostgresConfig::new();
    let postgres_storage = Arc::new(PostgresStorage::new(postgres_config).await.unwrap());
    let jwt_service = Arc::new(JwtService::new());
    let user_service = UsersService::new(postgres_storage, jwt_service);

    user_service
}
