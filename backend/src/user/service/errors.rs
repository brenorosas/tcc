use thiserror::Error;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{controller::errors::ErrorResponse, jwt::service::errors::JwtServiceError};

#[derive(Error, Debug)]
pub enum UsersServiceError {
    #[error("password confirmation does not match")]
    PasswordConfirmationDoesNotMatch,

    #[error("invalid email")]
    InvalidEmail,

    #[error("user already registered")]
    UserAlreadyRegistered,

    #[error("incorrect credentials")]
    IncorrectCredentials,

    #[error("user not found for uuid: {0}")]
    UserNotFound(Uuid),

    #[error(transparent)]
    JwtError(#[from] JwtServiceError),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<UsersServiceError> for ErrorResponse {
    fn from(error: UsersServiceError) -> ErrorResponse {
        match error {
            UsersServiceError::PasswordConfirmationDoesNotMatch => ErrorResponse {
                status_code: axum::http::StatusCode::BAD_REQUEST,
                message: "Password confirmation does not match".to_string(),
                pt_br_message: "A confirmação da senha não confere".to_string(),
            },
            UsersServiceError::InvalidEmail => ErrorResponse {
                status_code: axum::http::StatusCode::BAD_REQUEST,
                message: "Invalid email".to_string(),
                pt_br_message: "E-mail inválido".to_string(),
            },
            UsersServiceError::UserAlreadyRegistered => ErrorResponse {
                status_code: axum::http::StatusCode::CONFLICT,
                message: "User already registered".to_string(),
                pt_br_message: "Usuário já cadastrado".to_string(),
            },
            UsersServiceError::IncorrectCredentials => ErrorResponse {
                status_code: axum::http::StatusCode::UNAUTHORIZED,
                message: "Incorrect credentials".to_string(),
                pt_br_message: "Credenciais incorretas".to_string(),
            },
            UsersServiceError::UserNotFound(user_uuid) => ErrorResponse {
                status_code: axum::http::StatusCode::NOT_FOUND,
                message: format!("User not found for uuid: {}", user_uuid),
                pt_br_message: "Usuário não encontrado".to_string(),
            },
            UsersServiceError::JwtError(err) => ErrorResponse::from(err),
            UsersServiceError::Unknown(err) => {
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
