use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::access_token::AccessToken;
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait AccessTokenRepository: Send + Sync {
    /// Creates a new access token
    async fn create(&self, access_token: &AccessToken) -> Result<AccessToken, ApiError>;

    /// Finds an access token by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AccessToken>, ApiError>;

    /// Finds an access token by token string
    async fn find_by_token(&self, token: &str) -> Result<Option<AccessToken>, ApiError>;

    /// Finds tokens by user ID and type
    async fn find_by_user_id_and_type(&self, user_id: Uuid, token_type: &str) -> Result<Vec<AccessToken>, ApiError>;

    /// Finds a valid token by user ID and type
    async fn find_valid_token_by_user_id_and_type(&self, user_id: Uuid, token_type: &str) -> Result<Option<AccessToken>, ApiError>;

    /// Updates an access token
    async fn update(&self, access_token: &AccessToken) -> Result<AccessToken, ApiError>;

    /// Marks a token as used
    async fn mark_as_used(&self, token_id: Uuid) -> Result<(), ApiError>;

    /// Revokes a token
    async fn revoke(&self, token_id: Uuid) -> Result<(), ApiError>;

    /// Revokes all tokens of a specific type for a user
    async fn revoke_all_tokens_by_user_id_and_type(&self, user_id: Uuid, token_type: &str) -> Result<(), ApiError>;

    /// Deletes expired tokens
    async fn delete_expired(&self) -> Result<u64, ApiError>;

    /// Deletes a token
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}
