use std::sync::Arc;

use axum::{debug_handler, Extension, Json};

use crate::{
    controller::errors::ErrorResponse,
    user::{
        dtos::{create_user_dto::CreateUserDto, create_user_response_dto::CreateUserResponseDto},
        service::UsersService,
    },
};

#[debug_handler]
pub async fn user_register_handler(
    Extension(user_service): Extension<Arc<UsersService>>,
    Json(body): Json<CreateUserDto>,
) -> Result<Json<CreateUserResponseDto>, ErrorResponse> {
    let response = user_service
        .create_user(body)
        .await
        .map_err(|error| ErrorResponse::from(error))?;

    Ok(Json(response))
}
