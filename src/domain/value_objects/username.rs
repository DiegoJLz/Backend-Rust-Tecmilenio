use crate::shared::{error_types::ApiError, validation_utils::ValidationUtils};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Username {
    value: String,
}

impl Username {
    pub fn new(username: &str) -> Result<Self, ApiError> {
        ValidationUtils::validate_username(username)?;

        Ok(Self {
            value: username.to_lowercase().trim().to_string(),
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_available(&self) -> bool {
        // This would typically check against a database
        // For now, we'll implement basic availability rules
        !self.value.is_empty() && self.value.len() >= 3
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.value
    }
}
