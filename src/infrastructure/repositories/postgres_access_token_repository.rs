use async_trait::async_trait;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::entities::access_token::{AccessToken, NewAccessToken};
use crate::domain::repositories::access_token_repository::AccessTokenRepository;
use crate::infrastructure::database::{DbPool, schema::access_tokens};
use crate::shared::error_types::{ApiError, ERROR_DATABASE_ERROR};

#[derive(Clone)]
pub struct PostgresAccessTokenRepository {
    pool: DbPool,
}

impl PostgresAccessTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccessTokenRepository for PostgresAccessTokenRepository {
    async fn create(&self, access_token: &AccessToken) -> Result<AccessToken, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let new_token_data = NewAccessToken::from_access_token(access_token);
        let created_token = diesel::insert_into(access_tokens::table)
            .values(&new_token_data)
            .get_result::<AccessToken>(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to create access token",
                &e.to_string(),
            ))?;

        Ok(created_token)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AccessToken>, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let token = access_tokens::table
            .filter(access_tokens::id.eq(id))
            .select(AccessToken::as_select())
            .first::<AccessToken>(&mut conn)
            .optional()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to find access token by ID",
                &e.to_string(),
            ))?;

        Ok(token)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<AccessToken>, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let access_token = access_tokens::table
            .filter(access_tokens::token.eq(token))
            .select(AccessToken::as_select())
            .first::<AccessToken>(&mut conn)
            .optional()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to find access token by token",
                &e.to_string(),
            ))?;

        Ok(access_token)
    }

    async fn find_by_user_id_and_type(&self, user_id: Uuid, token_type: &str) -> Result<Vec<AccessToken>, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let tokens = access_tokens::table
            .filter(access_tokens::user_id.eq(user_id))
            .filter(access_tokens::token_type.eq(token_type))
            .select(AccessToken::as_select())
            .load::<AccessToken>(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to find tokens by user and type",
                &e.to_string(),
            ))?;

        Ok(tokens)
    }

    async fn find_valid_token_by_user_id_and_type(&self, user_id: Uuid, token_type: &str) -> Result<Option<AccessToken>, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let token = access_tokens::table
            .filter(access_tokens::user_id.eq(user_id))
            .filter(access_tokens::token_type.eq(token_type))
            .filter(access_tokens::is_used.eq(false))
            .filter(access_tokens::is_revoked.eq(false))
            .filter(access_tokens::expires_at.gt(Utc::now()))
            .select(AccessToken::as_select())
            .first::<AccessToken>(&mut conn)
            .optional()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to find valid token",
                &e.to_string(),
            ))?;

        Ok(token)
    }

    async fn update(&self, access_token: &AccessToken) -> Result<AccessToken, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let updated_token = diesel::update(access_tokens::table.filter(access_tokens::id.eq(access_token.id)))
            .set((
                access_tokens::is_used.eq(access_token.is_used),
                access_tokens::is_revoked.eq(access_token.is_revoked),
                access_tokens::updated_at.eq(Some(Utc::now())),
            ))
            .returning(AccessToken::as_returning())
            .get_result::<AccessToken>(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to update access token",
                &e.to_string(),
            ))?;

        Ok(updated_token)
    }

    async fn mark_as_used(&self, token_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        diesel::update(access_tokens::table.filter(access_tokens::id.eq(token_id)))
            .set((
                access_tokens::is_used.eq(true),
                access_tokens::updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to mark token as used",
                &e.to_string(),
            ))?;

        Ok(())
    }

    async fn revoke(&self, token_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        diesel::update(access_tokens::table.filter(access_tokens::id.eq(token_id)))
            .set((
                access_tokens::is_revoked.eq(true),
                access_tokens::updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to revoke token",
                &e.to_string(),
            ))?;

        Ok(())
    }

    async fn revoke_all_tokens_by_user_id_and_type(&self, user_id: Uuid, token_type: &str) -> Result<(), ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        diesel::update(access_tokens::table
            .filter(access_tokens::user_id.eq(user_id))
            .filter(access_tokens::token_type.eq(token_type)))
            .set((
                access_tokens::is_revoked.eq(true),
                access_tokens::updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to revoke tokens by user and type",
                &e.to_string(),
            ))?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let deleted_count = diesel::delete(access_tokens::table.filter(access_tokens::expires_at.lt(Utc::now())))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to delete expired tokens",
                &e.to_string(),
            ))?;

        Ok(deleted_count as u64)
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        diesel::delete(access_tokens::table.filter(access_tokens::id.eq(id)))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to delete access token",
                &e.to_string(),
            ))?;

        Ok(())
    }
}
