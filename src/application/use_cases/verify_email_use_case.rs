use crate::domain::entities::user::User;
use crate::domain::repositories::{
    email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
};
use crate::domain::services::token_service::TokenService;
use crate::shared::error_types::{
    ApiError, ERROR_INVALID_TOKEN, ERROR_TOKEN_ALREADY_USED, ERROR_USER_NOT_FOUND,
};

#[derive(Clone)]
pub struct VerifyEmailUseCase {
    user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    email_verification_repository: crate::infrastructure::repositories::postgres_email_verification_repository::PostgresEmailVerificationRepository,
    token_service: crate::domain::services::token_service::DefaultTokenService,
}

impl VerifyEmailUseCase {
    pub fn new(
        user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
        email_verification_repository: crate::infrastructure::repositories::postgres_email_verification_repository::PostgresEmailVerificationRepository,
        token_service: crate::domain::services::token_service::DefaultTokenService,
    ) -> Self {
        Self {
            user_repository,
            email_verification_repository,
            token_service,
        }
    }

    pub async fn execute(&self, token: String) -> Result<User, ApiError> {
        // 1. Validar el JWT token y extraer información del usuario
        let (user_id, _email, _username) = self
            .token_service
            .validate_email_verification_jwt(&token)
            .await
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_INVALID_TOKEN,
                    "Invalid verification token",
                    &e.to_string(),
                )
            })?;

        // 2. Buscar el token en la base de datos para verificar que existe y no está usado
        let verification_token = self
            .email_verification_repository
            .find_by_token(&token)
            .await?
            .ok_or_else(|| {
                ApiError::with_details(
                    ERROR_INVALID_TOKEN,
                    "Invalid verification token",
                    "The provided token does not exist in our records",
                )
            })?;

        // 3. Verificar que el token no esté ya usado
        if !verification_token.is_valid() {
            return Err(ApiError::with_details(
                ERROR_TOKEN_ALREADY_USED,
                "Verification token has already been used",
                "This token has already been used to verify the email",
            ));
        }

        // 4. Buscar el usuario asociado al token
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| {
                ApiError::with_details(
                    ERROR_USER_NOT_FOUND,
                    "User not found",
                    "The user associated with this token does not exist",
                )
            })?;

        // 5. Verificar que el usuario no esté ya verificado
        if user.is_verified.unwrap_or(false) {
            return Err(ApiError::with_details(
                ERROR_TOKEN_ALREADY_USED,
                "Email already verified",
                "This email address has already been verified",
            ));
        }

        // 6. Marcar el usuario como verificado
        user.verify();
        let verified_user = self.user_repository.update(&user).await?;

        // 7. Marcar el token como usado
        let mut updated_token = verification_token.clone();
        updated_token.mark_as_used();
        self.email_verification_repository
            .mark_as_used(updated_token.id)
            .await?;

        Ok(verified_user)
    }
}
