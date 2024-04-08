use std::sync::Arc;

use backend::{
    storage::{
        entities::user_choice::UserChoice,
        postgres::{config::PostgresConfig, PostgresStorage},
    },
    tmdb::dtos::recommendation_type::RecommendationType,
    user::dtos::{
        create_user_dto::CreateUserDto, register_user_choice_dto::RegisterUserChoiceDto,
        user_login_dto::UserLoginDto,
    },
};

use crate::common::{create_users_service, delete_all_from_db};

pub mod common;
#[tokio::test]
async fn user_login() {
    delete_all_from_db().await;
    let users_service = create_users_service().await;

    users_service
        .create_user(CreateUserDto {
            email: "test@test.com".to_owned(),
            password: "test123".to_owned(),
            password_confirmation: "test123".to_owned(),
        })
        .await
        .expect("Should create user")
        .user_uuid;

    let response = users_service
        .login(UserLoginDto {
            email: "test@test.com".to_owned(),
            password: "test123".to_owned(),
        })
        .await
        .expect("login returning jwt token");

    let user_entity = users_service
        .validate_user(&response.jwt_token)
        .await
        .unwrap();

    users_service
        .register_user_choice(
            &user_entity,
            RegisterUserChoiceDto {
                recommendation_type: RecommendationType::SimilarGenres,
            },
        )
        .await
        .unwrap();

    // should not be inserted because we have deduplication
    users_service
        .register_user_choice(
            &user_entity,
            RegisterUserChoiceDto {
                recommendation_type: RecommendationType::SimilarGenres,
            },
        )
        .await
        .unwrap();

    let postgres_config = PostgresConfig::new();
    let postgres_storage = Arc::new(PostgresStorage::new(postgres_config).await.unwrap());
    let connection = postgres_storage.get_connection().await.unwrap();
    let rows = connection
        .query(
            "SELECT jsonb_build_object(
                'recommendation_type', recommendation_type,
                'inserted_at', inserted_at,
                'user_uuid', user_uuid
            ) AS user_choice
            FROM user_choices",
            &[],
        )
        .await
        .unwrap();

    assert_eq!(rows.len(), 1);

    let row = rows.get(0).unwrap();
    let user_choice: UserChoice =
        serde_json::from_value(row.try_get("user_choice").unwrap()).unwrap();

    assert_eq!(
        user_choice.recommendation_type,
        RecommendationType::SimilarGenres
    );
}
