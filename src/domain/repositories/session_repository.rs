use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::session::Session;
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Creates a new session
    async fn create(&self, session: &Session) -> Result<Session, ApiError>;

    /// Finds a session by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, ApiError>;

    /// Finds a session by session token
    async fn find_by_session_token(&self, session_token: &str) -> Result<Option<Session>, ApiError>;

    /// Finds a session by access token
    async fn find_by_access_token(&self, access_token: &str) -> Result<Option<Session>, ApiError>;

    /// Finds all active sessions for a user
    async fn find_active_sessions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>, ApiError>;

    /// Updates a session
    async fn update(&self, session: &Session) -> Result<Session, ApiError>;

    /// Deactivates a session
    async fn deactivate(&self, session_id: Uuid) -> Result<(), ApiError>;

    /// Deactivates all sessions for a user
    async fn deactivate_all_user_sessions(&self, user_id: Uuid) -> Result<(), ApiError>;

    /// Deletes expired sessions
    async fn delete_expired(&self) -> Result<u64, ApiError>;

    /// Deletes a session
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}
