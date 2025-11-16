use crate::domain::entities::{access_token::AccessToken, session::Session, user::User};
use crate::domain::repositories::{
    access_token_repository::AccessTokenRepository, session_repository::SessionRepository,
    user_repository::UserRepository,
};
use crate::domain::services::{password_service::PasswordService, token_service::TokenService};
use crate::shared::error_types::{ApiError, ERROR_INVALID_EMAIL, ERROR_INVALID_PASSWORD};
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct LoginUserUseCase {
    user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    session_repository: crate::infrastructure::repositories::postgres_session_repository::PostgresSessionRepository,
    access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
    password_service: crate::domain::services::password_service::BcryptPasswordService,
    token_service: crate::domain::services::token_service::DefaultTokenService,
}

impl LoginUserUseCase {
    pub fn new(
        user_repository: crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
        session_repository: crate::infrastructure::repositories::postgres_session_repository::PostgresSessionRepository,
        access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
        password_service: crate::domain::services::password_service::BcryptPasswordService,
        token_service: crate::domain::services::token_service::DefaultTokenService,
    ) -> Self {
        Self {
            user_repository,
            session_repository,
            access_token_repository,
            password_service,
            token_service,
        }
    }

    pub async fn execute(
        &self,
        email: String,
        password: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(User, String, String), ApiError> {
        // 1. Validar formato de email
        crate::shared::validation_utils::ValidationUtils::validate_email(&email)?;

        // 2. Buscar usuario por email
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| {
                ApiError::with_details(
                    ERROR_INVALID_EMAIL,
                    "Invalid email or password",
                    "The provided email or password is incorrect",
                )
            })?;

        // 3. Verificar que el usuario esté verificado
        if !user.is_verified.unwrap_or(false) {
            return Err(ApiError::with_details(
                ERROR_INVALID_EMAIL,
                "Email not verified",
                "Please verify your email before logging in",
            ));
        }

        // 4. Obtener hash de contraseña del usuario
        let stored_password_hash = self
            .user_repository
            .get_password_hash(user.id)
            .await?
            .ok_or_else(|| {
                ApiError::with_details(
                    ERROR_INVALID_PASSWORD,
                    "Invalid email or password",
                    "The provided email or password is incorrect",
                )
            })?;

        // 5. Verificar contraseña
        let is_password_valid = self
            .password_service
            .verify_password(&password, &stored_password_hash)
            .await?;

        if !is_password_valid {
            return Err(ApiError::with_details(
                ERROR_INVALID_PASSWORD,
                "Invalid email or password",
                "The provided email or password is incorrect",
            ));
        }

        // 6. Generar access token JWT
        let access_token = self
            .token_service
            .generate_access_token(user.id, user.email.clone(), user.username.clone())
            .await?;

        // 7. Generar refresh token JWT
        let refresh_token = self
            .token_service
            .generate_refresh_token(user.id, user.email.clone(), user.username.clone())
            .await?;

        // 8. Crear sesión
        let session_id = Uuid::new_v4();
        let session_token = self.token_service.generate_token(32).await?;
        let expires_at = Utc::now() + Duration::hours(24); // 24 horas

        let session = Session::new(
            session_id,
            user.id,
            session_token.clone(),
            access_token.clone(),
            Some(refresh_token.clone()),
            expires_at,
            ip_address,
            user_agent,
        );

        // 9. Guardar sesión en la base de datos
        self.session_repository.create(&session).await?;

        // 10. Guardar access token en access_tokens table
        let access_token_entity = AccessToken::new(
            Uuid::new_v4(),
            user.id,
            access_token.clone(),
            "access".to_string(),
            Utc::now() + Duration::hours(24),
            None,
        );
        self.access_token_repository
            .create(&access_token_entity)
            .await?;

        // 11. Guardar refresh token en access_tokens table
        let refresh_token_entity = AccessToken::new(
            Uuid::new_v4(),
            user.id,
            refresh_token.clone(),
            "refresh".to_string(),
            Utc::now() + Duration::days(30), // 30 días
            None,
        );
        self.access_token_repository
            .create(&refresh_token_entity)
            .await?;

        Ok((user, access_token, session_token))
    }
}
