#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginResponseDto {
    pub jwt_token: String,
}
