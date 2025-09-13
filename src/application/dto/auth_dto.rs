use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
    pub confirm_password: String,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub user: UserDto,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserDto,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct VerifyEmailResponse {
    pub message: String,
    pub user: UserDto,
}

#[derive(Debug, Serialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

// User DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub is_host: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserDto {
    pub fn from_domain(user: &crate::domain::entities::user::User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            phone: user.phone.clone(),
            avatar_url: user.avatar_url.clone(),
            is_host: user.is_host.unwrap_or(false),
            is_verified: user.is_verified.unwrap_or(false),
            created_at: user.created_at.unwrap_or_else(|| chrono::Utc::now()),
            updated_at: user.updated_at.unwrap_or_else(|| chrono::Utc::now()),
        }
    }
}
