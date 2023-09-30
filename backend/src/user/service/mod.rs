pub mod errors;

use std::sync::Arc;

use uuid::Uuid;

use crate::storage::repositories::users::users_repository::UsersRepository;

use self::errors::UsersServiceError;

use super::dtos::{
    create_user_dto::CreateUserDto, create_user_response_dto::CreateUserResponseDto,
};

use bcrypt::{hash, DEFAULT_COST};

pub struct UsersService {
    users_repository: Arc<dyn UsersRepository>,
}

impl UsersService {
    pub fn new(users_repository: Arc<dyn UsersRepository>) -> Self {
        Self { users_repository }
    }

    pub async fn create_user(
        &self,
        create_user_dto: CreateUserDto,
    ) -> Result<CreateUserResponseDto, UsersServiceError> {
        create_user_dto.validate()?;
        let user_email = create_user_dto.email;
        let user_already_registered = self
            .users_repository
            .get_user_by_email(&user_email)
            .await?
            .is_some();

        if user_already_registered {
            return Err(UsersServiceError::UserAlreadyRegistered);
        }

        let user_uuid = Uuid::new_v4();
        let user_hashed_password = hash(create_user_dto.password, DEFAULT_COST).map_err(|err| {
            UsersServiceError::Unknown(anyhow::anyhow!("Failed to hash user password err: {}", err))
        })?;

        let user_entity = self
            .users_repository
            .create_user(&user_uuid, &user_email, &user_hashed_password)
            .await?;

        Ok(CreateUserResponseDto {
            user_uuid: user_entity.uuid,
        })
    }
}
