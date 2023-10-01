use std::sync::Arc;

use axum::{debug_handler, Extension, Json};

use crate::{
    controller::errors::ErrorResponse,
    user::{
        dtos::{user_login_dto::UserLoginDto, user_login_response_dto::UserLoginResponseDto},
        service::UsersService,
    },
};

#[debug_handler]
pub async fn user_login_handler(
    Extension(user_service): Extension<Arc<UsersService>>,
    Json(body): Json<UserLoginDto>,
) -> Result<Json<UserLoginResponseDto>, ErrorResponse> {
    let response = user_service
        .login(body)
        .await
        .map_err(|error| ErrorResponse::from(error))?;

    Ok(Json(response))
}
