use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::user::User;
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a new user
    async fn create(&self, user: &User, password_hash: &str) -> Result<User, ApiError>;

    /// Finds a user by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ApiError>;

    /// Finds a user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ApiError>;

    /// Finds a user by username
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError>;

    /// Updates a user
    async fn update(&self, user: &User) -> Result<User, ApiError>;

    /// Deletes a user by ID
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;

    /// Checks if email exists
    async fn email_exists(&self, email: &str) -> Result<bool, ApiError>;

    /// Checks if username exists
    async fn username_exists(&self, username: &str) -> Result<bool, ApiError>;

    /// Gets user password hash
    async fn get_password_hash(&self, user_id: Uuid) -> Result<Option<String>, ApiError>;

    /// Updates user password
    async fn update_password(&self, user_id: Uuid, password_hash: &str) -> Result<(), ApiError>;

    /// Verifies user email
    async fn verify_user(&self, user_id: Uuid) -> Result<(), ApiError>;
}
