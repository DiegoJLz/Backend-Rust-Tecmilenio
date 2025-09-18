use crate::domain::repositories::{session_repository::SessionRepository, access_token_repository::AccessTokenRepository};
use crate::shared::error_types::{ApiError, ERROR_INVALID_TOKEN, ERROR_TOKEN_EXPIRED};
use uuid::Uuid;

#[derive(Clone)]
pub struct LogoutUseCase {
    session_repository: crate::infrastructure::repositories::postgres_session_repository::PostgresSessionRepository,
    access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
}

impl LogoutUseCase {
    pub fn new(
        session_repository: crate::infrastructure::repositories::postgres_session_repository::PostgresSessionRepository,
        access_token_repository: crate::infrastructure::repositories::postgres_access_token_repository::PostgresAccessTokenRepository,
    ) -> Self {
        Self {
            session_repository,
            access_token_repository,
        }
    }

    pub async fn execute(&self, session_token: String) -> Result<(), ApiError> {
        // 1. Buscar la sesión por session_token
        let session = self.session_repository
            .find_by_session_token(&session_token)
            .await?
            .ok_or_else(|| ApiError::with_details(
                ERROR_INVALID_TOKEN,
                "Invalid session",
                "The provided session token is invalid"
            ))?;

        // 2. Verificar que la sesión esté activa
        if !session.is_valid() {
            return Err(ApiError::with_details(
                ERROR_TOKEN_EXPIRED,
                "Session expired",
                "The session has expired or is no longer active"
            ));
        }

        // 3. Revocar el access token asociado
        if let Some(access_token) = self.access_token_repository
            .find_by_token(&session.access_token)
            .await? {
            self.access_token_repository
                .revoke(access_token.id)
                .await?;
        }

        // 4. Revocar el refresh token si existe
        if let Some(refresh_token) = &session.refresh_token {
            if let Some(refresh_token_entity) = self.access_token_repository
                .find_by_token(refresh_token)
                .await? {
                self.access_token_repository
                    .revoke(refresh_token_entity.id)
                    .await?;
            }
        }

        // 5. Desactivar la sesión
        self.session_repository
            .deactivate(session.id)
            .await?;

        Ok(())
    }

    pub async fn execute_by_user_id(&self, user_id: Uuid) -> Result<(), ApiError> {
        // Logout forzado: desactivar todas las sesiones del usuario
        self.session_repository
            .deactivate_all_user_sessions(user_id)
            .await?;

        // Revocar todos los tokens del usuario
        self.access_token_repository
            .revoke_all_tokens_by_user_id_and_type(user_id, "access")
            .await?;

        self.access_token_repository
            .revoke_all_tokens_by_user_id_and_type(user_id, "refresh")
            .await?;

        Ok(())
    }
}
