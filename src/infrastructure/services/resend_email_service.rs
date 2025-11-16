use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

use crate::domain::entities::email::Email;
use crate::domain::services::email_service::{EmailService, EmailTemplateService};
use crate::infrastructure::services::email_template_service::DefaultEmailTemplateService;
use crate::shared::error_types::{ApiError, ERROR_INTERNAL_SERVER_ERROR};

#[derive(Clone)]
pub struct ResendEmailService {
    client: Client,
    template_service: DefaultEmailTemplateService,
}

#[derive(Serialize)]
struct ResendEmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct ResendEmailResponse {
    id: String,
}

impl ResendEmailService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            template_service: DefaultEmailTemplateService::new(),
        }
    }

    fn get_api_key() -> Result<String, ApiError> {
        env::var("RESEND_API_KEY").map_err(|_| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Email configuration error",
                "RESEND_API_KEY environment variable not set",
            )
        })
    }

    fn get_from_email() -> Result<String, ApiError> {
        env::var("RESEND_FROM_EMAIL").map_err(|_| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Email configuration error",
                "RESEND_FROM_EMAIL environment variable not set",
            )
        })
    }

    async fn send_raw_email(
        &self,
        to_email: &str,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), ApiError> {
        let api_key = Self::get_api_key()?;
        let from_email = Self::get_from_email()?;

        println!("📧 Enviando email a: {}", to_email);
        println!("📧 Desde: {}", from_email);
        println!("📧 API Key: {}...", &api_key[..10]);

        let request_body = ResendEmailRequest {
            from: from_email,
            to: vec![to_email.to_string()],
            subject: subject.to_string(),
            html: html_content.to_string(),
            text: Some(text_content.to_string()),
        };

        // Para testing, si la API key es de prueba, simular envío exitoso
        if api_key.starts_with("re_test_") {
            println!("🧪 Modo testing - Simulando envío de email exitoso");
            println!("📧 Email simulado enviado a: {}", to_email);
            println!("📧 Subject: {}", subject);
            return Ok(());
        }

        let response = self
            .client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_INTERNAL_SERVER_ERROR,
                    "Email sending error",
                    &format!("Failed to send HTTP request: {}", e),
                )
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            println!(
                "❌ Resend API Error - Status: {}, Body: {}",
                status, error_text
            );
            return Err(ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Email sending error",
                &format!("Resend API error (Status: {}): {}", status, error_text),
            ));
        }

        let _email_response: ResendEmailResponse = response.json().await.map_err(|e| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Email sending error",
                &format!("Failed to parse response: {}", e),
            )
        })?;

        Ok(())
    }
}

#[async_trait]
impl EmailService for ResendEmailService {
    async fn send_email(&self, email: &Email) -> Result<(), ApiError> {
        self.send_raw_email(
            &email.to_email,
            &email.subject,
            &email.html_content,
            &email.text_content,
        )
        .await
    }

    async fn send_email_verification(
        &self,
        to_email: String,
        to_name: String,
        verification_token: String,
    ) -> Result<(), ApiError> {
        let (html_content, text_content) = self
            .template_service
            .generate_email_verification_template(&to_name, &verification_token);

        self.send_raw_email(
            &to_email,
            "Verifica tu Email - Marketplace Local",
            &html_content,
            &text_content,
        )
        .await
    }

    async fn send_password_reset(
        &self,
        to_email: String,
        to_name: String,
        reset_token: String,
    ) -> Result<(), ApiError> {
        let (html_content, text_content) = self
            .template_service
            .generate_password_reset_template(&to_name, &reset_token);

        self.send_raw_email(
            &to_email,
            "Restablecer Contraseña - Marketplace Local",
            &html_content,
            &text_content,
        )
        .await
    }

    async fn send_welcome_email(&self, to_email: String, to_name: String) -> Result<(), ApiError> {
        let (html_content, text_content) =
            self.template_service.generate_welcome_template(&to_name);

        self.send_raw_email(
            &to_email,
            "¡Bienvenido a Marketplace Local!",
            &html_content,
            &text_content,
        )
        .await
    }
}
