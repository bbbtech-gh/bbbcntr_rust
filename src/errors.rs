use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum CustomError {
    BadRequest,
    UserNotFound,
    UserExists,
    TokenNotFound,
    // TokenExists,
    ClientNotFound,
    ClientExists,
    InternalServerError,
    ClientApplicationExists
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Self::BadRequest=> (StatusCode::BAD_REQUEST, "Bad Request"),

            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
            Self::TokenNotFound => (StatusCode::NOT_FOUND, "Token Not Found"),
            Self::ClientNotFound => (StatusCode::NOT_FOUND, "Client Not Found"),

            // Self::TokenExists => (StatusCode::BAD_REQUEST, "Token already exists")
            Self::UserExists => (StatusCode::BAD_REQUEST, "User already exists"),
            Self::ClientExists => (StatusCode::BAD_REQUEST, "Client already exists"),
            Self::ClientApplicationExists => (StatusCode::BAD_REQUEST, "Client verification pending or expired.."),
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}
