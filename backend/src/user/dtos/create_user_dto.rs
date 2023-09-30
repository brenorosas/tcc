use crate::{user::service::errors::UsersServiceError, utils::email::is_valid_email};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl CreateUserDto {
    pub fn validate(&self) -> Result<(), UsersServiceError> {
        if !is_valid_email(&self.email) {
            return Err(UsersServiceError::InvalidEmail);
        }

        if self.password != self.password_confirmation {
            return Err(UsersServiceError::PasswordConfirmationDoesNotMatch);
        }

        Ok(())
    }
}
