#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}
