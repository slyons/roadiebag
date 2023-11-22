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
    ServerError(ServerFnError)
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
        }
    }
}

/*#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NestedResult<T, E> {
    Ok(T),
    Err(E)
}*/

pub struct NestedResult();

impl NestedResult {
    // TODO: Just make this a function, no need for the struct
    pub fn from<T>(res: Result<RoadieResult<T>, ServerFnError>) -> RoadieResult<T> {
        match res {
            Err(e) => Err(RoadieAppError::ServerError(e)),
            Ok(Err(e)) => Err(e),
            Ok(Ok(v)) => Ok(v)
        }
    }
}

/*impl<T> NestedResult<T, RoadieAppError> {
    pub fn from<T>(res: Result<RoadieResult<T>, ServerFnError>) -> NestedResult<T, RoadieAppError> {

    }

    pub fn is_ok(&self) -> bool {
        match self {
            NestedResult::Err(_) => false,
            NestedResult::Ok(_) => true
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}*/