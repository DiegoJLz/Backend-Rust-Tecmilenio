use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::shared::error_types::{ApiError, ERROR_INTERNAL_SERVER_ERROR};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub username: String,
    pub iat: u64, // Issued at
    pub exp: u64, // Expiration time
    pub token_type: String,
}

impl Claims {
    pub fn new(user_id: Uuid, email: String, username: String, expiration_hours: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            sub: user_id.to_string(),
            email,
            username,
            iat: now,
            exp: now + (expiration_hours * 3600), // Convert hours to seconds
            token_type: "access".to_string(),
        }
    }
}

pub struct JwtUtils;

impl JwtUtils {
    /// Generates a JWT token
    pub fn generate_token(
        user_id: Uuid,
        email: String,
        username: String,
        secret: &str,
        expiration_hours: u64,
    ) -> Result<String, ApiError> {
        let claims = Claims::new(user_id, email, username, expiration_hours);

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Failed to generate JWT token",
                &e.to_string(),
            )
        })
    }

    /// Validates and decodes a JWT token
    pub fn validate_token(token: &str, secret: &str) -> Result<Claims, ApiError> {
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map(|token_data| token_data.claims)
        .map_err(|e| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Invalid or expired token",
                &e.to_string(),
            )
        })
    }

    /// Extracts user ID from token
    pub fn extract_user_id(token: &str, secret: &str) -> Result<Uuid, ApiError> {
        let claims = Self::validate_token(token, secret)?;
        Uuid::parse_str(&claims.sub)
            .map_err(|_| ApiError::new(ERROR_INTERNAL_SERVER_ERROR, "Invalid user ID in token"))
    }

    /// Checks if token is expired
    pub fn is_token_expired(token: &str, secret: &str) -> bool {
        match Self::validate_token(token, secret) {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}
