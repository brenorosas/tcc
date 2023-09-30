use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
    pub pt_br_message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}
