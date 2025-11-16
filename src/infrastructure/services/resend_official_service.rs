use async_trait::async_trait;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::Resend;
use std::env;

use crate::domain::services::email_service::EmailService;
use crate::shared::error_types::{ApiError, ERROR_INTERNAL_SERVER_ERROR};

#[derive(Clone)]
pub struct ResendOfficialService {
    client: Resend,
}

impl ResendOfficialService {
    pub fn new() -> Result<Self, ApiError> {
        let api_key = env::var("RESEND_API_KEY").map_err(|_| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Email configuration error",
                "RESEND_API_KEY environment variable not set",
            )
        })?;

        let client = Resend::new(&api_key);

        Ok(Self { client })
    }

    async fn send_raw_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), ApiError> {
        let from_email = env::var("RESEND_FROM_EMAIL").map_err(|_| {
            ApiError::with_details(
                ERROR_INTERNAL_SERVER_ERROR,
                "Email configuration error",
                "RESEND_FROM_EMAIL environment variable not set",
            )
        })?;

        println!("📧 Enviando email real via Resend a: {}", to_email);
        println!("📧 Desde: {}", from_email);
        println!("📧 Subject: {}", subject);

        let email = CreateEmailBaseOptions::new(&from_email, [to_email], subject)
            .with_html(html_content)
            .with_text(text_content);

        match self.client.emails.send(email).await {
            Ok(response) => {
                println!("✅ Email enviado exitosamente a: {}", to_email);
                println!("📧 Resend ID: {:?}", response);
                Ok(())
            }
            Err(e) => {
                println!("❌ Error enviando email: {}", e);
                Err(ApiError::with_details(
                    ERROR_INTERNAL_SERVER_ERROR,
                    "Email sending error",
                    &format!("Failed to send email via Resend: {}", e),
                ))
            }
        }
    }
}

#[async_trait]
impl EmailService for ResendOfficialService {
    async fn send_email(
        &self,
        _email: &crate::domain::entities::email::Email,
    ) -> Result<(), ApiError> {
        // Esta función no se usa en nuestro caso, pero es requerida por el trait
        Ok(())
    }

    async fn send_email_verification(
        &self,
        to_email: String,
        to_name: String,
        verification_token: String,
    ) -> Result<(), ApiError> {
        let verification_link = format!(
            "http://localhost:3000/verify-email?token={}",
            verification_token
        );

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto;">
                <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 30px; text-align: center; border-radius: 10px 10px 0 0;">
                    <h1 style="color: white; margin: 0; font-size: 28px;">🎉 ¡Bienvenido a Marketplace Local!</h1>
                </div>
                <div style="background: #f8f9fa; padding: 30px; border-radius: 0 0 10px 10px; border: 1px solid #e9ecef;">
                    <h2 style="color: #495057; margin-top: 0;">Hola, {}!</h2>
                    <p style="font-size: 16px; color: #6c757d;">Gracias por registrarte en nuestra plataforma. Para activar tu cuenta y comenzar a explorar experiencias únicas en tu ciudad, por favor verifica tu dirección de correo electrónico.</p>

                    <div style="text-align: center; margin: 30px 0;">
                        <a href="{}" style="display: inline-block; padding: 15px 30px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; text-decoration: none; border-radius: 25px; font-weight: bold; font-size: 16px; box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);">
                            ✨ Verificar Mi Email
                        </a>
                    </div>

                    <div style="background: #e3f2fd; padding: 20px; border-radius: 8px; margin: 20px 0; border-left: 4px solid #2196f3;">
                        <p style="margin: 0; color: #1976d2; font-size: 14px;">
                            <strong>💡 ¿No puedes hacer clic en el botón?</strong><br>
                            Copia y pega este enlace en tu navegador:<br>
                            <a href="{}" style="color: #1976d2; word-break: break-all;">{}</a>
                        </p>
                    </div>

                    <div style="background: #fff3cd; padding: 15px; border-radius: 8px; margin: 20px 0; border-left: 4px solid #ffc107;">
                        <p style="margin: 0; color: #856404; font-size: 14px;">
                            ⏰ <strong>Importante:</strong> Este enlace expirará en 24 horas por seguridad.
                        </p>
                    </div>

                    <p style="color: #6c757d; font-size: 14px; margin-bottom: 0;">
                        Si no te registraste en Marketplace Local, por favor ignora este correo electrónico.
                    </p>
                </div>
                <div style="text-align: center; padding: 20px; color: #6c757d; font-size: 12px;">
                    <p>© 2024 Marketplace Local. Todos los derechos reservados.</p>
                </div>
            </div>
            "#,
            to_name, verification_link, verification_link, verification_link
        );

        let text_content = format!(
            "¡Bienvenido a Marketplace Local, {}!\n\nGracias por registrarte en nuestra plataforma. Para activar tu cuenta, por favor verifica tu dirección de correo electrónico haciendo clic en el siguiente enlace:\n\n{}\n\nEste enlace expirará en 24 horas.\n\nSi no te registraste en Marketplace Local, por favor ignora este correo electrónico.\n\nSaludos cordiales,\nEl equipo de Marketplace Local",
            to_name, verification_link
        );

        self.send_raw_email(
            &to_email,
            &to_name,
            "🎉 Verifica tu Email - Marketplace Local",
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
        let reset_link = format!("http://localhost:3000/reset-password?token={}", reset_token);

        let html_content = format!(
            r#"
            <div style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto;">
                <div style="background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%); padding: 30px; text-align: center; border-radius: 10px 10px 0 0;">
                    <h1 style="color: white; margin: 0; font-size: 28px;">🔐 Restablecer Contraseña</h1>
                </div>
                <div style="background: #f8f9fa; padding: 30px; border-radius: 0 0 10px 10px; border: 1px solid #e9ecef;">
                    <h2 style="color: #495057; margin-top: 0;">Hola, {}!</h2>
                    <p style="font-size: 16px; color: #6c757d;">Hemos recibido una solicitud para restablecer la contraseña de tu cuenta en Marketplace Local.</p>

                    <div style="text-align: center; margin: 30px 0;">
                        <a href="{}" style="display: inline-block; padding: 15px 30px; background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%); color: white; text-decoration: none; border-radius: 25px; font-weight: bold; font-size: 16px; box-shadow: 0 4px 15px rgba(255, 107, 107, 0.4);">
                            🔑 Restablecer Contraseña
                        </a>
                    </div>

                    <div style="background: #fff3cd; padding: 15px; border-radius: 8px; margin: 20px 0; border-left: 4px solid #ffc107;">
                        <p style="margin: 0; color: #856404; font-size: 14px;">
                            ⏰ <strong>Importante:</strong> Este enlace expirará en 1 hora por seguridad.
                        </p>
                    </div>

                    <p style="color: #6c757d; font-size: 14px; margin-bottom: 0;">
                        Si no solicitaste un restablecimiento de contraseña, por favor ignora este correo electrónico.
                    </p>
                </div>
            </div>
            "#,
            to_name, reset_link
        );

        let text_content = format!(
            "Hola, {}!\n\nHemos recibido una solicitud para restablecer la contraseña de tu cuenta en Marketplace Local. Para continuar con el proceso, haz clic en el siguiente enlace:\n\n{}\n\nEste enlace expirará en 1 hora.\n\nSi no solicitaste un restablecimiento de contraseña, por favor ignora este correo electrónico.\n\nSaludos cordiales,\nEl equipo de Marketplace Local",
            to_name, reset_link
        );

        self.send_raw_email(
            &to_email,
            &to_name,
            "🔐 Restablece tu Contraseña - Marketplace Local",
            &html_content,
            &text_content,
        )
        .await
    }

    async fn send_welcome_email(&self, to_email: String, to_name: String) -> Result<(), ApiError> {
        let html_content = format!(
            "<div style=\"font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto;\">
                <div style=\"background: #28a745; padding: 30px; text-align: center; border-radius: 10px 10px 0 0;\">
                    <h1 style=\"color: white; margin: 0; font-size: 28px;\">🎉 ¡Bienvenido a Marketplace Local!</h1>
                </div>
                <div style=\"background: #f8f9fa; padding: 30px; border-radius: 0 0 10px 10px; border: 1px solid #e9ecef;\">
                    <h2 style=\"color: #495057; margin-top: 0;\">¡Hola, {}!</h2>
                    <p style=\"font-size: 16px; color: #6c757d;\">Estamos encantados de tenerte con nosotros. Prepárate para descubrir y ofrecer experiencias únicas en tu ciudad.</p>

                    <div style=\"text-align: center; margin: 30px 0;\">
                        <a href=\"#\" style=\"display: inline-block; padding: 15px 30px; background: #28a745; color: white; text-decoration: none; border-radius: 25px; font-weight: bold; font-size: 16px;\">
                            🚀 Explorar Marketplace
                        </a>
                    </div>

                    <p style=\"color: #6c757d; font-size: 14px; margin-bottom: 0;\">
                        Si tienes alguna pregunta, no dudes en contactarnos.
                    </p>
                </div>
            </div>",
            to_name
        );

        let text_content = format!(
            "¡Bienvenido a Marketplace Local, {}!\n\nEstamos encantados de tenerte con nosotros. Prepárate para descubrir y ofrecer experiencias únicas en tu ciudad.\n\nExplora nuestra plataforma y empieza tu aventura hoy mismo.\n\nSi tienes alguna pregunta, no dudes en contactarnos.\n\nSaludos cordiales,\nEl equipo de Marketplace Local",
            to_name
        );

        self.send_raw_email(
            &to_email,
            &to_name,
            "🎉 ¡Bienvenido a Marketplace Local!",
            &html_content,
            &text_content,
        )
        .await
    }
}
