use std::sync::Arc;

use axum::{debug_handler, Extension, Json};

use crate::{
    controller::errors::ErrorResponse,
    storage::entities::user::UserEntity,
    user::{dtos::register_user_choice_dto::RegisterUserChoiceDto, service::UsersService},
};

#[debug_handler]
pub async fn register_user_choice_handler(
    Extension(user_service): Extension<Arc<UsersService>>,
    Extension(user): Extension<UserEntity>,
    Json(body): Json<RegisterUserChoiceDto>,
) -> Result<Json<()>, ErrorResponse> {
    user_service.register_user_choice(&user, body).await?;
    Ok(Json(()))
}
