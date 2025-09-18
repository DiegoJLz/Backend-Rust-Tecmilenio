use serde::{Deserialize, Serialize};
use crate::shared::{error_types::ApiError, validation_utils::ValidationUtils};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(email: &str) -> Result<Self, ApiError> {
        ValidationUtils::validate_email(email)?;

        Ok(Self {
            value: email.to_lowercase().trim().to_string(),
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn domain(&self) -> &str {
        self.value.split('@').nth(1).unwrap_or("")
    }

    pub fn local_part(&self) -> &str {
        self.value.split('@').nth(0).unwrap_or("")
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.value
    }
}
