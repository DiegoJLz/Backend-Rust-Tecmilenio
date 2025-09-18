use async_trait::async_trait;
use uuid::Uuid;
use crate::shared::error_types::ApiError;
use crate::shared::jwt_utils::JwtUtils;

#[async_trait]
pub trait TokenService: Send + Sync {
    /// Generates a random token
    async fn generate_token(&self, length: usize) -> Result<String, ApiError>;

    /// Generates a UUID-based token
    async fn generate_uuid_token(&self) -> Result<String, ApiError>;

    /// Generates a JWT token for email verification
    async fn generate_email_verification_jwt(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError>;

    /// Validates a JWT token and extracts user information
    async fn validate_email_verification_jwt(&self, token: &str) -> Result<(Uuid, String, String), ApiError>;

    /// Validates a password reset JWT token and extracts user information
    async fn validate_password_reset_token(&self, token: &str) -> Result<(Uuid, String, String), ApiError>;

    /// Generates an access token JWT for authentication
    async fn generate_access_token(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError>;

    /// Generates a refresh token JWT for token renewal
    async fn generate_refresh_token(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError>;

    /// Generates a password reset token JWT
    async fn generate_password_reset_token(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError>;

    /// Generates a numeric verification code
    async fn generate_verification_code(&self, length: usize) -> Result<String, ApiError>;
}

#[derive(Clone)]
pub struct DefaultTokenService;

#[async_trait]
impl TokenService for DefaultTokenService {
    async fn generate_token(&self, length: usize) -> Result<String, ApiError> {
        use rand::Rng;
        use rand::distributions::Alphanumeric;

        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        Ok(token)
    }

    async fn generate_uuid_token(&self) -> Result<String, ApiError> {
        Ok(Uuid::new_v4().to_string())
    }

    async fn generate_email_verification_jwt(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError> {
        // Email verification tokens expire in 24 hours
        let expiration_hours = 24;

        JwtUtils::generate_token(
            user_id,
            email,
            username,
            expiration_hours,
            "email_verification"
        )
    }

    async fn validate_email_verification_jwt(&self, token: &str) -> Result<(Uuid, String, String), ApiError> {
        let claims = JwtUtils::validate_token(token, "email_verification")?;
        let user_id = claims.user_id()?;
        Ok((user_id, claims.email, claims.username))
    }

    async fn validate_password_reset_token(&self, token: &str) -> Result<(Uuid, String, String), ApiError> {
        let claims = JwtUtils::validate_token(token, "password_reset")?;
        let user_id = claims.user_id()?;
        Ok((user_id, claims.email, claims.username))
    }

    async fn generate_access_token(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError> {
        // Access tokens expire in 24 hours
        let expiration_hours = 24;

        JwtUtils::generate_token(
            user_id,
            email,
            username,
            expiration_hours,
            "access"
        )
    }

    async fn generate_refresh_token(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError> {
        // Refresh tokens expire in 30 days
        let expiration_hours = 24 * 30; // 30 days

        JwtUtils::generate_token(
            user_id,
            email,
            username,
            expiration_hours,
            "refresh"
        )
    }

    async fn generate_password_reset_token(&self, user_id: Uuid, email: String, username: String) -> Result<String, ApiError> {
        // Password reset tokens expire in 1 hour
        let expiration_hours = 1;

        JwtUtils::generate_token(
            user_id,
            email,
            username,
            expiration_hours,
            "password_reset"
        )
    }

    async fn generate_verification_code(&self, length: usize) -> Result<String, ApiError> {
        use rand::Rng;

        let code: String = (0..length)
            .map(|_| rand::thread_rng().gen_range(0..10).to_string())
            .collect();

        Ok(code)
    }
}
