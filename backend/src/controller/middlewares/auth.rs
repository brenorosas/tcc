use std::sync::Arc;

use axum::{
    http::{self, Request},
    middleware::Next,
    response::Response,
};
use reqwest::StatusCode;

use crate::{controller::errors::ErrorResponse, user::service::UsersService};

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, ErrorResponse> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(ErrorResponse {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Missing authorization header".to_string(),
            pt_br_message: "Cabeçalho de autorização ausente".to_string(),
        });
    };
    if !auth_header.starts_with("Bearer ") {
        return Err(ErrorResponse {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Invalid authorization header".to_string(),
            pt_br_message: "Cabeçalho de autorização inválido".to_string(),
        });
    }

    let bearer = &auth_header[7..];
    let users_service: &Arc<UsersService> = req.extensions().get().unwrap();
    let user = users_service
        .validate_user(bearer)
        .await
        .map_err(|err| ErrorResponse::from(err))?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
