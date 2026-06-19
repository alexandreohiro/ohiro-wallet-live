use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("nao autorizado")]
    Unauthorized,

    #[error("registro nao encontrado")]
    NotFound,

    #[error("entrada invalida: {0}")]
    InvalidInput(String),

    #[error("conflito: {0}")]
    Conflict(String),

    #[error("erro de memoria compartilhada")]
    MemoryPoisoned,

    #[error("erro interno")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::MemoryPoisoned | AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.to_string()
        }));

        (status, body).into_response()
    }
}
