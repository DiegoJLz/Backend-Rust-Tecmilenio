use serde::{Deserialize, Serialize};
use crate::shared::error_types::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message.to_string()),
        }
    }

    pub fn error(error: ApiError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: None,
        }
    }

    pub fn error_with_message(error: ApiError, message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: Some(message.to_string()),
        }
    }
}

// Common response types
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessMessage {
    pub message: String,
}

impl SuccessMessage {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
