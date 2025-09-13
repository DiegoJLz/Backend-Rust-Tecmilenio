use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl ApiError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
        }
    }

    pub fn with_details(code: &str, message: &str, details: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: Some(details.to_string()),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

// Error codes for authentication
pub const ERROR_USER_ALREADY_EXISTS: &str = "USER_ALREADY_EXISTS";
pub const ERROR_INVALID_EMAIL: &str = "INVALID_EMAIL";
pub const ERROR_INVALID_PASSWORD: &str = "INVALID_PASSWORD";
pub const ERROR_PASSWORDS_DO_NOT_MATCH: &str = "PASSWORDS_DO_NOT_MATCH";
pub const ERROR_INVALID_USERNAME: &str = "INVALID_USERNAME";
pub const ERROR_DATABASE_ERROR: &str = "DATABASE_ERROR";
pub const ERROR_INTERNAL_SERVER_ERROR: &str = "INTERNAL_SERVER_ERROR";
pub const ERROR_VALIDATION_ERROR: &str = "VALIDATION_ERROR";
