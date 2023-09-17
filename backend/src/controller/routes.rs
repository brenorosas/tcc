use axum::{routing::get, Router};

pub fn build_healthcheck_route() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, tcc!" }))
        .route("/health", get(|| async { "OK" }))
}

pub fn build_routes() -> Router {
    let healthcheck = build_healthcheck_route();

    Router::new().merge(healthcheck)
}
