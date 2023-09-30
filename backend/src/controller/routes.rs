use std::sync::Arc;

use axum::{
    debug_handler,
    extract::Extension,
    routing::{get, post},
    Json, Router,
};

use crate::user::{
    dtos::{create_user_dto::CreateUserDto, create_user_response_dto::CreateUserResponseDto},
    service::UsersService,
};

use super::errors::ErrorResponse;

pub fn build_healthcheck_route() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, tcc!" }))
        .route("/health", get(|| async { "OK" }))
}

pub fn build_public_api_routes() -> Router {
    Router::new().route("/users/register", post(user_register_handler))
}

pub fn build_routes(user_service: Arc<UsersService>) -> Router {
    let healthcheck = build_healthcheck_route();
    let public_api = build_public_api_routes();

    Router::new().merge(healthcheck).nest(
        "/api/v1",
        Router::new()
            .merge(public_api)
            .layer(Extension(user_service)),
    )
}

#[debug_handler]
async fn user_register_handler(
    Extension(user_service): Extension<Arc<UsersService>>,
    Json(body): Json<CreateUserDto>,
) -> Result<Json<CreateUserResponseDto>, ErrorResponse> {
    let response = user_service
        .create_user(body)
        .await
        .map_err(|error| ErrorResponse::from(error))?;

    Ok(Json(response))
}
