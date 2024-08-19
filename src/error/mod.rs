use axum::response::IntoResponse;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error: {0}")]
    Error(String),

    #[error("MySqlError: {0}")]
    MySqlError(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::MySqlError(e) => {
                error!("MySqlError: {:?}", e);
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            _ => {
                error!("Error: {:?}", self);
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
