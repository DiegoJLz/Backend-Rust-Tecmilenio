use async_trait::async_trait;
use crate::domain::entities::email::{Email, EmailType};
use crate::shared::error_types::ApiError;

#[async_trait]
pub trait EmailService: Send + Sync {
    /// Sends an email
    async fn send_email(&self, email: &Email) -> Result<(), ApiError>;

    /// Sends an email verification email
    async fn send_email_verification(
        &self,
        to_email: String,
        to_name: String,
        verification_token: String,
    ) -> Result<(), ApiError>;

    /// Sends a password reset email
    async fn send_password_reset(
        &self,
        to_email: String,
        to_name: String,
        reset_token: String,
    ) -> Result<(), ApiError>;

    /// Sends a welcome email
    async fn send_welcome_email(
        &self,
        to_email: String,
        to_name: String,
    ) -> Result<(), ApiError>;
}

/// Email template service for generating email content
pub trait EmailTemplateService: Send + Sync {
    /// Generates email verification template
    fn generate_email_verification_template(
        &self,
        user_name: &str,
        verification_token: &str,
    ) -> (String, String); // (html_content, text_content)

    /// Generates password reset template
    fn generate_password_reset_template(
        &self,
        user_name: &str,
        reset_token: &str,
    ) -> (String, String); // (html_content, text_content)

    /// Generates welcome email template
    fn generate_welcome_template(
        &self,
        user_name: &str,
    ) -> (String, String); // (html_content, text_content)
}
