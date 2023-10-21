use thiserror::Error;
use tracing::{event, Level};

use crate::controller::errors::ErrorResponse;

#[derive(Error, Debug)]
pub enum TmdbServiceError {
    #[error("unexpected response")]
    UnexpectedResponseDto,

    #[error("invalid api key")]
    InvalidApiKey,

    #[error("unexpected tmdb status code")]
    UnexpectedTmdbStatusCode(reqwest::StatusCode),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<TmdbServiceError> for ErrorResponse {
    fn from(error: TmdbServiceError) -> ErrorResponse {
        match error {
            TmdbServiceError::UnexpectedResponseDto => ErrorResponse {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                message: "Unexpected response".to_string(),
                pt_br_message: "Resposta inesperada".to_string(),
            },
            TmdbServiceError::InvalidApiKey => ErrorResponse {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                message: "Invalid api key".to_string(),
                pt_br_message: "Chave de API TMDB inválida".to_string(),
            },
            TmdbServiceError::UnexpectedTmdbStatusCode(status) => {
                event!(Level::ERROR, "Unexpected tmdb status code: {}", status);
                ErrorResponse {
                    status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Unexpected tmdb status code".to_string(),
                    pt_br_message: "Código de status tmdb inesperado".to_string(),
                }
            }
            TmdbServiceError::Unknown(err) => {
                event!(Level::ERROR, "Unknown error: {}", err);
                ErrorResponse {
                    status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Unknown error"),
                    pt_br_message: "Erro desconhecido".to_string(),
                }
            }
        }
    }
}
