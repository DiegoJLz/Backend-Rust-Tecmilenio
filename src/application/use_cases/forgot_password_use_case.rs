use crate::domain::entities::access_token::AccessToken;
use crate::domain::repositories::{user_repository::UserRepository, access_token_repository::AccessTokenRepository};
use crate::domain::services::token_service::TokenService;
use crate::shared::error_types::{ApiError, ERROR_INVALID_EMAIL, ERROR_USER_NOT_FOUND};
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Clone)]
pub struct ForgotPasswordUseCase {
    user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
    token_service: crate::domain::services::token_service::DefaultTokenService,
    send_email_use_case: crate::application::use_cases::send_email_use_case::SendEmailUseCase,
}

impl ForgotPasswordUseCase {
    pub fn new(
        user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
        access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
        token_service: crate::domain::services::token_service::DefaultTokenService,
        send_email_use_case: crate::application::use_cases::send_email_use_case::SendEmailUseCase,
    ) -> Self {
        Self {
            user_repository,
            access_token_repository,
            token_service,
            send_email_use_case,
        }
    }

    pub async fn execute(&self, email: String) -> Result<String, ApiError> {
        // 1. Validar formato de email
        crate::shared::validation_utils::ValidationUtils::validate_email(&email)?;

        // 2. Buscar usuario por email
        let user = self.user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| ApiError::with_details(
                ERROR_USER_NOT_FOUND,
                "User not found",
                "No user found with the provided email address"
            ))?;

        // 3. Verificar que el usuario esté verificado
        if !user.is_verified.unwrap_or(false) {
            return Err(ApiError::with_details(
                ERROR_INVALID_EMAIL,
                "Email not verified",
                "Please verify your email before requesting password reset"
            ));
        }

        // 4. Revocar tokens de password reset existentes para este usuario
        self.access_token_repository
            .revoke_all_tokens_by_user_id_and_type(user.id, "password_reset")
            .await?;

        // 5. Generar nuevo token de password reset JWT
        let password_reset_token = self.token_service
            .generate_password_reset_token(user.id, user.email.clone(), user.username.clone())
            .await?;

        // 6. Guardar token en access_tokens table
        let access_token_entity = AccessToken::new(
            Uuid::new_v4(),
            user.id,
            password_reset_token.clone(),
            "password_reset".to_string(),
            Utc::now() + Duration::hours(1), // 1 hora de expiración
            Some(serde_json::json!({
                "email": user.email,
                "username": user.username,
                "purpose": "password_reset"
            })),
        );

        self.access_token_repository.create(&access_token_entity).await?;

        // 7. Enviar email con el token de reset
        if let Err(email_error) = self.send_email_use_case
            .send_password_reset(
                user.email.clone(),
                user.username.clone(),
                password_reset_token.clone(),
            )
            .await
        {
            // Log the error but don't fail the forgot password process
            println!("Failed to send password reset email to {}: {}", user.email, email_error);
        }

        Ok(password_reset_token)
    }
}
