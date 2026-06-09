use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(serde::Serialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<ApiError>,
}

#[derive(serde::Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

pub fn ok<T: serde::Serialize>(data: T) -> impl IntoResponse {
    (
        StatusCode::OK,
        serde_json::json!({
            "success": true,
            "data": data,
            "message": None::<String>,
            "error": None::<ApiError>,
        }),
    )
        .into_response()
}

pub fn created<T: serde::Serialize>(data: T) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        serde_json::json!({
            "success": true,
            "data": data,
            "message": None::<String>,
            "error": None::<ApiError>,
        }),
    )
        .into_response()
}

pub fn error_response(code: &str, message: &str) -> impl IntoResponse {
    (
        StatusCode::BAD_REQUEST,
        serde_json::json!({
            "success": false,
            "data": None::<()>,
            "message": None::<String>,
            "error": { "code": code, "message": message },
        }),
    )
        .into_response()
}
