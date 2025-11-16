use crate::domain::entities::email::{Email, EmailType};
use crate::domain::services::email_service::EmailService;
use crate::shared::error_types::ApiError;

#[derive(Clone)]
pub struct SendEmailUseCase {
    email_service: crate::infrastructure::services::resend_official_service::ResendOfficialService,
}

impl SendEmailUseCase {
    pub fn new(
        email_service: crate::infrastructure::services::resend_official_service::ResendOfficialService,
    ) -> Self {
        Self { email_service }
    }

    pub async fn send_email_verification(
        &self,
        to_email: String,
        to_name: String,
        verification_token: String,
    ) -> Result<(), ApiError> {
        self.email_service
            .send_email_verification(to_email, to_name, verification_token)
            .await
    }

    pub async fn send_password_reset(
        &self,
        to_email: String,
        to_name: String,
        reset_token: String,
    ) -> Result<(), ApiError> {
        self.email_service
            .send_password_reset(to_email, to_name, reset_token)
            .await
    }

    pub async fn send_welcome_email(
        &self,
        to_email: String,
        to_name: String,
    ) -> Result<(), ApiError> {
        self.email_service
            .send_welcome_email(to_email, to_name)
            .await
    }

    pub async fn send_custom_email(
        &self,
        to_email: String,
        to_name: Option<String>,
        subject: String,
        html_content: String,
        text_content: String,
        email_type: EmailType,
    ) -> Result<(), ApiError> {
        let email = Email::new(
            to_email,
            to_name,
            "noreply@marketplace.com".to_string(), // This will be overridden by SMTP config
            "Marketplace Local".to_string(),       // This will be overridden by SMTP config
            subject,
            html_content,
            text_content,
            email_type,
        );

        self.email_service.send_email(&email).await
    }
}
