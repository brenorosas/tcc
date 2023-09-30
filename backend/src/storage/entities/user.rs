use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserEntity {
    pub uuid: Uuid,
    pub email: String,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
