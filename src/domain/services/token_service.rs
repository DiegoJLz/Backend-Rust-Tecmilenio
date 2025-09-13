use async_trait::async_trait;
use uuid::Uuid;
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait TokenService: Send + Sync {
    /// Generates a random token
    async fn generate_token(&self, length: usize) -> Result<String, ApiError>;

    /// Generates a UUID-based token
    async fn generate_uuid_token(&self) -> Result<String, ApiError>;

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

    async fn generate_verification_code(&self, length: usize) -> Result<String, ApiError> {
        use rand::Rng;

        let code: String = (0..length)
            .map(|_| rand::thread_rng().gen_range(0..10).to_string())
            .collect();

        Ok(code)
    }
}
