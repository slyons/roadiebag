use http::status::StatusCode;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Error, Serialize, Deserialize, PartialEq)]
pub enum RoadieAppError {
    #[error("Username does not exist or password doesn't match")]
    BadUserPassword,
    #[error("Passwords do not match")]
    PasswordsDoNotMatch,
    #[error("User is unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Validation failed")]
    ValidationFailedError,
    #[error("Validation failed for field {0}")]
    ValidationFailedForField(String)
}

pub type RoadieResult<T> = Result<T, RoadieAppError>;

impl RoadieAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            RoadieAppError::NotFound => StatusCode::NOT_FOUND,
            RoadieAppError::BadUserPassword | RoadieAppError::Unauthorized => StatusCode::UNAUTHORIZED,
            RoadieAppError::PasswordsDoNotMatch => StatusCode::EXPECTATION_FAILED,
            RoadieAppError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            RoadieAppError::ValidationFailedError => StatusCode::EXPECTATION_FAILED,
            RoadieAppError::ValidationFailedForField(_) => StatusCode::EXPECTATION_FAILED
        }
    }
}