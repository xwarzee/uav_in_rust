use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Drone not found: {0}")]
    DroneNotFound(String),

    #[error("Mission not found: {0}")]
    MissionNotFound(String),

    #[error("Invalid formation type: {0}")]
    InvalidFormation(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DroneNotFound(_) | ApiError::MissionNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InvalidFormation(_) | ApiError::ValidationError(_) | ApiError::BadRequest(_) => {
                StatusCode::BAD_REQUEST
            }
            ApiError::Internal(_) | ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.to_string(),
            status: self.status_code().as_u16(),
        })
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    status: u16,
}
