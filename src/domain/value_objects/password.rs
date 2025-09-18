use serde::{Deserialize, Serialize};
use crate::shared::{error_types::ApiError, validation_utils::ValidationUtils};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    hashed_value: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Self, ApiError> {
        ValidationUtils::validate_password(password)?;

        let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| ApiError::with_details(
                "PASSWORD_HASH_ERROR",
                "Failed to hash password",
                &e.to_string(),
            ))?;

        Ok(Self {
            hashed_value: hashed,
        })
    }

    pub fn from_hash(hash: &str) -> Self {
        Self {
            hashed_value: hash.to_string(),
        }
    }

    pub fn verify(&self, password: &str) -> Result<bool, ApiError> {
        bcrypt::verify(password, &self.hashed_value)
            .map_err(|e| ApiError::with_details(
                "PASSWORD_VERIFY_ERROR",
                "Failed to verify password",
                &e.to_string(),
            ))
    }

    pub fn hash(&self) -> &str {
        &self.hashed_value
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[HIDDEN]")
    }
}
