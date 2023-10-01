use anyhow::anyhow;
use axum::http::StatusCode;
use jsonwebtoken::errors::ErrorKind;
use thiserror::Error;
use tracing::{event, Level};

use crate::controller::errors::ErrorResponse;

#[derive(Error, Debug)]
pub enum JwtServiceError {
    #[error("invalid token")]
    InvalidToken,
    #[error("expired token")]
    ExpiredToken,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<JwtServiceError> for ErrorResponse {
    fn from(error: JwtServiceError) -> ErrorResponse {
        match error {
            JwtServiceError::InvalidToken => ErrorResponse {
                status_code: StatusCode::UNAUTHORIZED,
                message: "Invalid token".to_string(),
                pt_br_message: "Token inválido".to_string(),
            },
            JwtServiceError::ExpiredToken => ErrorResponse {
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                message: "Expired token".to_string(),
                pt_br_message: "Token expirado".to_string(),
            },
            JwtServiceError::Unknown(err) => {
                event!(Level::ERROR, "Unknown jwt service error: {}", err);
                ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Unknown error".to_string(),
                    pt_br_message: "Erro desconhecido".to_string(),
                }
            }
        }
    }
}

impl From<&ErrorKind> for JwtServiceError {
    fn from(error: &ErrorKind) -> JwtServiceError {
        match error {
            ErrorKind::InvalidToken => JwtServiceError::InvalidToken,
            ErrorKind::ExpiredSignature => JwtServiceError::ExpiredToken,
            _ => JwtServiceError::Unknown(anyhow!("{:?}", error)),
        }
    }
}
