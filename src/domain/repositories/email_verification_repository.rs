use crate::domain::entities::email_verification_token::EmailVerificationToken;
use crate::shared::error_types::ApiError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait EmailVerificationRepository: Send + Sync {
    /// Creates a new email verification token
    async fn create(
        &self,
        token: &EmailVerificationToken,
    ) -> Result<EmailVerificationToken, ApiError>;

    /// Finds a token by token string
    async fn find_by_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, ApiError>;

    /// Finds tokens by user ID
    async fn find_by_user_id(&self, user_id: Uuid)
        -> Result<Vec<EmailVerificationToken>, ApiError>;

    /// Marks a token as used
    async fn mark_as_used(&self, token_id: Uuid) -> Result<(), ApiError>;

    /// Deletes expired tokens
    async fn delete_expired(&self) -> Result<u64, ApiError>;

    /// Deletes all tokens for a user
    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), ApiError>;
}
