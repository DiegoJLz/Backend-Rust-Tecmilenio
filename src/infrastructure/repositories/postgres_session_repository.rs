use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::entities::session::{NewSession, Session};
use crate::domain::repositories::session_repository::SessionRepository;
use crate::infrastructure::database::{schema::sessions, DbPool};
use crate::shared::error_types::{ApiError, ERROR_DATABASE_ERROR};

#[derive(Clone)]
pub struct PostgresSessionRepository {
    pool: DbPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn create(&self, session: &Session) -> Result<Session, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let new_session_data = NewSession::from_session(session);
        let created_session = diesel::insert_into(sessions::table)
            .values(&new_session_data)
            .get_result::<Session>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to create session",
                    &e.to_string(),
                )
            })?;

        Ok(created_session)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let session = sessions::table
            .filter(sessions::id.eq(id))
            .select(Session::as_select())
            .first::<Session>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find session by ID",
                    &e.to_string(),
                )
            })?;

        Ok(session)
    }

    async fn find_by_session_token(
        &self,
        session_token: &str,
    ) -> Result<Option<Session>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let session = sessions::table
            .filter(sessions::session_token.eq(session_token))
            .select(Session::as_select())
            .first::<Session>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find session by token",
                    &e.to_string(),
                )
            })?;

        Ok(session)
    }

    async fn find_by_access_token(&self, access_token: &str) -> Result<Option<Session>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let session = sessions::table
            .filter(sessions::access_token.eq(access_token))
            .select(Session::as_select())
            .first::<Session>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find session by access token",
                    &e.to_string(),
                )
            })?;

        Ok(session)
    }

    async fn find_active_sessions_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Session>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let sessions_list = sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::is_active.eq(true))
            .filter(sessions::expires_at.gt(Utc::now()))
            .select(Session::as_select())
            .load::<Session>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find active sessions",
                    &e.to_string(),
                )
            })?;

        Ok(sessions_list)
    }

    async fn update(&self, session: &Session) -> Result<Session, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let updated_session = diesel::update(sessions::table.filter(sessions::id.eq(session.id)))
            .set((
                sessions::is_active.eq(session.is_active),
                sessions::updated_at.eq(Some(Utc::now())),
            ))
            .returning(Session::as_returning())
            .get_result::<Session>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to update session",
                    &e.to_string(),
                )
            })?;

        Ok(updated_session)
    }

    async fn deactivate(&self, session_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        diesel::update(sessions::table.filter(sessions::id.eq(session_id)))
            .set((
                sessions::is_active.eq(false),
                sessions::updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to deactivate session",
                    &e.to_string(),
                )
            })?;

        Ok(())
    }

    async fn deactivate_all_user_sessions(&self, user_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        diesel::update(sessions::table.filter(sessions::user_id.eq(user_id)))
            .set((
                sessions::is_active.eq(false),
                sessions::updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to deactivate user sessions",
                    &e.to_string(),
                )
            })?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let deleted_count =
            diesel::delete(sessions::table.filter(sessions::expires_at.lt(Utc::now())))
                .execute(&mut conn)
                .map_err(|e| {
                    ApiError::with_details(
                        ERROR_DATABASE_ERROR,
                        "Failed to delete expired sessions",
                        &e.to_string(),
                    )
                })?;

        Ok(deleted_count as u64)
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        diesel::delete(sessions::table.filter(sessions::id.eq(id)))
            .execute(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to delete session",
                    &e.to_string(),
                )
            })?;

        Ok(())
    }
}
