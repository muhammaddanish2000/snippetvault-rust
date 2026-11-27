use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::Database(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(message) => {
                (StatusCode::NOT_FOUND, message)
            }

            AppError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, message)
            }

            AppError::Database(error) => {
                tracing::error!(
                    error = %error,
                    "Database error"
                );

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred.".to_string(),
                )
            }
        };

        (
            status,
            Json(json!({
                "success": false,
                "error": message
            })),
        )
            .into_response()
    }
}
