pub mod common;
use backend::user::{dtos::create_user_dto::CreateUserDto, service::errors::UsersServiceError};
use common::{create_users_service, delete_all_from_db};
#[tokio::test]
async fn create_user() {
    delete_all_from_db().await;
    let users_service = create_users_service().await;

    let mut create_user_dto = CreateUserDto {
        email: "test@test.".to_owned(),
        password: "test123".to_owned(),
        password_confirmation: "test12".to_owned(),
    };

    let response = users_service
        .create_user(create_user_dto.clone())
        .await
        .expect_err("Should return invalid email error");

    assert!(matches!(response, UsersServiceError::InvalidEmail));

    create_user_dto.email = "test@test.com".to_owned();

    let response = users_service
        .create_user(create_user_dto.clone())
        .await
        .expect_err("Should return password confirmation does not match error");

    assert!(matches!(
        response,
        UsersServiceError::PasswordConfirmationDoesNotMatch
    ));

    create_user_dto.password_confirmation = "test123".to_owned();

    users_service
        .create_user(create_user_dto.clone())
        .await
        .expect("Should create user");

    let response = users_service
        .create_user(create_user_dto.clone())
        .await
        .expect_err("Should return user already registered error");

    assert!(matches!(response, UsersServiceError::UserAlreadyRegistered));
}
