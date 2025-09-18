use async_trait::async_trait;
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait PasswordService: Send + Sync {
    /// Hashes a password
    async fn hash_password(&self, password: &str) -> Result<String, ApiError>;

    /// Verifies a password against a hash
    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, ApiError>;

    /// Generates a random password
    async fn generate_random_password(&self, length: usize) -> Result<String, ApiError>;
}

#[derive(Clone)]
pub struct BcryptPasswordService;

#[async_trait]
impl PasswordService for BcryptPasswordService {
    async fn hash_password(&self, password: &str) -> Result<String, ApiError> {
        // Using cost 12 for better security (higher than default 10)
        // Cost 12 takes ~250ms to hash, making brute force attacks much harder
        bcrypt::hash(password, 12)
            .map_err(|e| ApiError::with_details(
                "PASSWORD_HASH_ERROR",
                "Failed to hash password",
                &e.to_string(),
            ))
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, ApiError> {
        bcrypt::verify(password, hash)
            .map_err(|e| ApiError::with_details(
                "PASSWORD_VERIFY_ERROR",
                "Failed to verify password",
                &e.to_string(),
            ))
    }

    async fn generate_random_password(&self, length: usize) -> Result<String, ApiError> {
        use rand::Rng;
        use rand::distributions::Alphanumeric;

        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        Ok(password)
    }
}
