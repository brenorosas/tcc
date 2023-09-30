use crate::{user::service::errors::UsersServiceError, utils::email::is_valid_email};

#[derive(Debug, serde::Deserialize, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate() {
        let mut create_user_dto = CreateUserDto {
            email: "".to_owned(),
            password: "test123".to_owned(),
            password_confirmation: "test12".to_owned(),
        };

        assert!(matches!(
            create_user_dto.validate().err().unwrap(),
            UsersServiceError::InvalidEmail
        ));

        create_user_dto.email = "test@test.com".to_owned();

        assert!(matches!(
            create_user_dto.validate().err().unwrap(),
            UsersServiceError::PasswordConfirmationDoesNotMatch
        ));

        create_user_dto.password_confirmation = "test123".to_owned();

        assert!(create_user_dto.validate().is_ok());
    }
}
