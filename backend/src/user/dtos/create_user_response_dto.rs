use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponseDto {
    pub user_uuid: Uuid,
}
