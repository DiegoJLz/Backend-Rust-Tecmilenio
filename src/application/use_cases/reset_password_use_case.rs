use crate::domain::entities::user::User;
use crate::domain::repositories::{user_repository::UserRepository, access_token_repository::AccessTokenRepository};
use crate::domain::services::{password_service::PasswordService, token_service::TokenService};
use crate::shared::error_types::{ApiError, ERROR_INVALID_TOKEN, ERROR_TOKEN_EXPIRED, ERROR_TOKEN_ALREADY_USED, ERROR_PASSWORDS_DO_NOT_MATCH};

#[derive(Clone)]
pub struct ResetPasswordUseCase {
    user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
    password_service: crate::domain::services::password_service::BcryptPasswordService,
    token_service: crate::domain::services::token_service::DefaultTokenService,
}

impl ResetPasswordUseCase {
    pub fn new(
        user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
        access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
        password_service: crate::domain::services::password_service::BcryptPasswordService,
        token_service: crate::domain::services::token_service::DefaultTokenService,
    ) -> Self {
        Self {
            user_repository,
            access_token_repository,
            password_service,
            token_service,
        }
    }

    pub async fn execute(&self, token: String, new_password: String, confirm_password: String) -> Result<User, ApiError> {
        // 1. Validar que las contraseñas coincidan
        if new_password != confirm_password {
            return Err(ApiError::with_details(
                ERROR_PASSWORDS_DO_NOT_MATCH,
                "Passwords do not match",
                "The new password and confirmation password must be identical"
            ));
        }

        // 2. Validar formato de la nueva contraseña
        crate::shared::validation_utils::ValidationUtils::validate_password(&new_password)?;

        // 3. Validar el JWT token de password reset
        let (user_id, _email, _username) = self.token_service
            .validate_password_reset_token(&token)
            .await?;

        // 4. Buscar el token en la base de datos para verificar que existe y no está usado
        let access_token = self.access_token_repository
            .find_by_token(&token)
            .await?
            .ok_or_else(|| ApiError::with_details(
                ERROR_INVALID_TOKEN,
                "Invalid token",
                "The provided password reset token is invalid"
            ))?;

        // 5. Verificar que el token es de tipo password_reset
        if !access_token.is_password_reset_token() {
            return Err(ApiError::with_details(
                ERROR_INVALID_TOKEN,
                "Invalid token type",
                "The provided token is not a password reset token"
            ));
        }

        // 6. Verificar que el token no esté usado
        if access_token.is_used() {
            return Err(ApiError::with_details(
                ERROR_TOKEN_ALREADY_USED,
                "Token already used",
                "This password reset token has already been used"
            ));
        }

        // 7. Verificar que el token no esté revocado
        if access_token.is_revoked() {
            return Err(ApiError::with_details(
                ERROR_INVALID_TOKEN,
                "Token revoked",
                "This password reset token has been revoked"
            ));
        }

        // 8. Verificar que el token no esté expirado
        if access_token.is_expired() {
            return Err(ApiError::with_details(
                ERROR_TOKEN_EXPIRED,
                "Token expired",
                "This password reset token has expired"
            ));
        }

        // 9. Buscar el usuario
        let user = self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ApiError::with_details(
                ERROR_INVALID_TOKEN,
                "User not found",
                "The user associated with this token was not found"
            ))?;

        // 10. Hashear la nueva contraseña
        let new_password_hash = self.password_service
            .hash_password(&new_password)
            .await?;

        // 11. Actualizar la contraseña del usuario
        self.user_repository
            .update_password(user.id, &new_password_hash)
            .await?;

        // 12. Marcar el token como usado
        self.access_token_repository
            .mark_as_used(access_token.id)
            .await?;

        // 13. Revocar todos los tokens de sesión del usuario (logout forzado)
        self.access_token_repository
            .revoke_all_tokens_by_user_id_and_type(user.id, "access")
            .await?;

        self.access_token_repository
            .revoke_all_tokens_by_user_id_and_type(user.id, "refresh")
            .await?;

        Ok(user)
    }
}
