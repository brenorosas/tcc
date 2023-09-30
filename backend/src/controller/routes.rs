use std::sync::Arc;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use crate::user::service::UsersService;

use super::handlers::user_register_handler::user_register_handler;

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
