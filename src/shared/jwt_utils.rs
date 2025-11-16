use crate::shared::error_types::{ApiError, ERROR_INTERNAL_SERVER_ERROR};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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
    pub fn new(
        user_id: Uuid,
        email: String,
        username: String,
        expiration_hours: u64,
        token_type: &str,
    ) -> Self {
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
            token_type: token_type.to_string(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, ApiError> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| ApiError::new("INVALID_USER_ID", "Invalid user ID in token"))
    }
}

pub struct JwtUtils;

impl JwtUtils {
    /// Generates a JWT token
    pub fn generate_token(
        user_id: Uuid,
        email: String,
        username: String,
        expiration_hours: u64,
        token_type: &str,
    ) -> Result<String, ApiError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            ApiError::new(
                "JWT_SECRET_NOT_SET",
                "JWT_SECRET environment variable not set",
            )
        })?;

        let claims = Claims::new(user_id, email, username, expiration_hours, token_type);

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
    pub fn validate_token(token: &str, token_type: &str) -> Result<Claims, ApiError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            ApiError::new(
                "JWT_SECRET_NOT_SET",
                "JWT_SECRET environment variable not set",
            )
        })?;

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_nbf = false;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            ApiError::with_details("INVALID_TOKEN", "Invalid or expired token", &e.to_string())
        })?;

        let claims = token_data.claims;

        // Validate token type
        if claims.token_type != token_type {
            return Err(ApiError::new("INVALID_TOKEN_TYPE", "Token type mismatch"));
        }

        Ok(claims)
    }

    /// Extracts user ID from token
    pub fn extract_user_id(token: &str, token_type: &str) -> Result<Uuid, ApiError> {
        let claims = Self::validate_token(token, token_type)?;
        claims.user_id()
    }

    /// Checks if token is expired
    pub fn is_token_expired(token: &str, token_type: &str) -> bool {
        match Self::validate_token(token, token_type) {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}
