use std::sync::Arc;

use axum::{
    extract::Extension,
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{tmdb::service::TmdbService, user::service::UsersService};

use super::{
    handlers::{
        tmdb_movie_discover::tmdb_movie_discover_handler, user_login::user_login_handler,
        user_register::user_register_handler,
    },
    middlewares::auth::auth_middleware,
};

pub fn build_healthcheck_route() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, tcc!" }))
        .route("/health", get(|| async { "OK" }))
}

pub fn build_public_routes() -> Router {
    Router::new()
        .route("/users/register", post(user_register_handler))
        .route("/users/login", post(user_login_handler))
}

pub fn build_authed_routes() -> Router {
    Router::new()
        .route(
            "/movies/tmdb/movie/discover",
            get(tmdb_movie_discover_handler),
        )
        .layer(middleware::from_fn(auth_middleware))
}

pub fn build_routes(user_service: Arc<UsersService>, tmdb_service: Arc<TmdbService>) -> Router {
    let healthcheck = build_healthcheck_route();
    let public_routes = build_public_routes();
    let authed_routes = build_authed_routes();
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new().merge(healthcheck).nest(
        "/api/v1",
        Router::new()
            .merge(public_routes)
            .merge(authed_routes)
            .layer(Extension(user_service))
            .layer(Extension(tmdb_service))
            .layer(cors),
    )
}
