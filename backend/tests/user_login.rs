use backend::{
    jwt::service::JwtService,
    user::dtos::{create_user_dto::CreateUserDto, user_login_dto::UserLoginDto},
};
use uuid::Uuid;

use crate::common::{create_users_service, delete_all_from_db};

pub mod common;
#[tokio::test]
async fn user_login() {
    delete_all_from_db().await;
    let users_service = create_users_service().await;

    let user_uuid = users_service
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

    let jwt_service = JwtService::new();

    let decoded_token = jwt_service
        .decode_token::<Uuid>(&response.jwt_token)
        .expect("decode token");

    assert_eq!(decoded_token, user_uuid);
}
