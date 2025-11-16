use crate::domain::entities::email_verification_token::EmailVerificationToken;
use crate::domain::repositories::{
    email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
};
use crate::domain::services::token_service::TokenService;
use crate::shared::error_types::{ApiError, ERROR_USER_ALREADY_VERIFIED, ERROR_USER_NOT_FOUND};
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct ResendEmailVerificationUseCase {
    user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    email_verification_repository: crate::infrastructure::repositories::postgres_email_verification_repository::PostgresEmailVerificationRepository,
    token_service: crate::domain::services::token_service::DefaultTokenService,
    send_email_use_case: crate::application::use_cases::send_email_use_case::SendEmailUseCase,
}

impl ResendEmailVerificationUseCase {
    pub fn new(
        user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
        email_verification_repository: crate::infrastructure::repositories::postgres_email_verification_repository::PostgresEmailVerificationRepository,
        token_service: crate::domain::services::token_service::DefaultTokenService,
        send_email_use_case: crate::application::use_cases::send_email_use_case::SendEmailUseCase,
    ) -> Self {
        Self {
            user_repository,
            email_verification_repository,
            token_service,
            send_email_use_case,
        }
    }

    pub async fn execute(&self, email: String) -> Result<String, ApiError> {
        // 1. Buscar usuario por email
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| {
                ApiError::with_details(
                    ERROR_USER_NOT_FOUND,
                    "User not found",
                    "No user found with the provided email address",
                )
            })?;

        // 2. Verificar que el usuario no esté ya verificado
        if user.is_verified.unwrap_or(false) {
            return Err(ApiError::with_details(
                ERROR_USER_ALREADY_VERIFIED,
                "User already verified",
                "This email address has already been verified",
            ));
        }

        // 3. Buscar tokens de verificación existentes para este usuario
        let existing_tokens = self
            .email_verification_repository
            .find_by_user_id(user.id)
            .await?;

        // 4. Revocar tokens de verificación existentes que no estén usados
        for token in existing_tokens {
            if !token.is_used.unwrap_or(false) {
                // Marcar como usado para invalidarlo
                self.email_verification_repository
                    .mark_as_used(token.id)
                    .await?;
            }
        }

        // 5. Generar nuevo JWT token de verificación
        let verification_token = self
            .token_service
            .generate_email_verification_jwt(user.id, user.email.clone(), user.username.clone())
            .await?;

        // 6. Crear nuevo token de verificación en la base de datos
        let email_verification_token = EmailVerificationToken::new(
            user.id,
            verification_token.clone(),
            24, // 24 horas de expiración
        );

        self.email_verification_repository
            .create(&email_verification_token)
            .await?;

        // Enviar email de verificación
        if let Err(email_error) = self
            .send_email_use_case
            .send_email_verification(
                user.email.clone(),
                user.username.clone(),
                verification_token.clone(),
            )
            .await
        {
            // Log the error but don't fail the resend process
            println!(
                "Failed to send verification email to {}: {}",
                user.email, email_error
            );
        }

        Ok(verification_token)
    }
}
