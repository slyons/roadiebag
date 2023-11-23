use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use leptos::ServerFnError;
use thiserror::Error;

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
    ValidationFailedForField(String),
    #[error("Item name can't be empty")]
    ItemNameNonEmpty,
    #[error("Item size must be set")]
    ItemSizeMustBeSet,
    #[error("Item quantity must be > 0")]
    ItemQntGtZero,
    #[error("Multiple errors")]
    MultipleErrors(HashMap<String, String>),
    #[error("Server error {0}")]
    ServerError(ServerFnError),
    #[error("Signups are not enabled")]
    SignupsNotEnabled,
}

pub type RoadieResult<T> = Result<T, RoadieAppError>;

impl RoadieAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            RoadieAppError::NotFound => StatusCode::NOT_FOUND,
            RoadieAppError::BadUserPassword | RoadieAppError::Unauthorized => {
                StatusCode::UNAUTHORIZED
            }
            RoadieAppError::PasswordsDoNotMatch => StatusCode::EXPECTATION_FAILED,
            RoadieAppError::InternalServerError | RoadieAppError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RoadieAppError::ValidationFailedError
            | RoadieAppError::ItemQntGtZero
            | RoadieAppError::ItemSizeMustBeSet
            | RoadieAppError::ItemNameNonEmpty => StatusCode::EXPECTATION_FAILED,
            RoadieAppError::ValidationFailedForField(_) => StatusCode::EXPECTATION_FAILED,
            RoadieAppError::MultipleErrors(_) => StatusCode::EXPECTATION_FAILED,
            RoadieAppError::SignupsNotEnabled => StatusCode::BAD_REQUEST
        }
    }
}

pub trait IntoRoadie<T> {
    fn into_rr(self) -> RoadieResult<T>;
}

impl<T> IntoRoadie<T> for Result<RoadieResult<T>, ServerFnError> {
    fn into_rr(self) -> RoadieResult<T> {
        match self {
            Ok(o) => o,
            Err(e) => Err(RoadieAppError::ServerError(e))
        }
    }
}

pub trait IntoRoadieOption<T> {
    fn into_rr(self) -> Option<RoadieResult<T>>;
}


impl<T> IntoRoadieOption<T> for Option<Result<RoadieResult<T>, ServerFnError>> {
    fn into_rr(self) -> Option<RoadieResult<T>> {
        self.map(|u| u.into_rr())
    }
}
