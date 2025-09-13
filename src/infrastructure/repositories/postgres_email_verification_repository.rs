use async_trait::async_trait;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::entities::email_verification_token::{EmailVerificationToken, NewEmailVerificationToken};
use crate::domain::repositories::email_verification_repository::EmailVerificationRepository;
use crate::infrastructure::database::{DbPool, schema::email_verification_tokens};
use crate::shared::error_types::{ApiError, ERROR_DATABASE_ERROR};

#[derive(Clone)]
pub struct PostgresEmailVerificationRepository {
    pool: DbPool,
}

impl PostgresEmailVerificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmailVerificationRepository for PostgresEmailVerificationRepository {
    async fn create(&self, token: &EmailVerificationToken) -> Result<EmailVerificationToken, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let new_token_data = NewEmailVerificationToken::from_token(token);
        let new_token = diesel::insert_into(email_verification_tokens::table)
            .values(&new_token_data)
            .get_result::<EmailVerificationToken>(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to create email verification token",
                &e.to_string(),
            ))?;

        Ok(new_token)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<EmailVerificationToken>, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let verification_token = email_verification_tokens::table
            .filter(email_verification_tokens::token.eq(token))
            .select(EmailVerificationToken::as_select())
            .first::<EmailVerificationToken>(&mut conn)
            .optional()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to find email verification token",
                &e.to_string(),
            ))?;

        Ok(verification_token)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<EmailVerificationToken>, ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        let tokens = email_verification_tokens::table
            .filter(email_verification_tokens::user_id.eq(user_id))
            .select(EmailVerificationToken::as_select())
            .load::<EmailVerificationToken>(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to find email verification tokens by user ID",
                &e.to_string(),
            ))?;

        Ok(tokens)
    }

    async fn mark_as_used(&self, token_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        diesel::update(email_verification_tokens::table.filter(email_verification_tokens::id.eq(token_id)))
            .set(email_verification_tokens::is_used.eq(true))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to mark email verification token as used",
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

        let deleted_count = diesel::delete(
            email_verification_tokens::table
                .filter(email_verification_tokens::expires_at.lt(chrono::Utc::now()))
        )
        .execute(&mut conn)
        .map_err(|e| ApiError::with_details(
            ERROR_DATABASE_ERROR,
            "Failed to delete expired email verification tokens",
            &e.to_string(),
        ))?;

        Ok(deleted_count as u64)
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get()
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            ))?;

        diesel::delete(email_verification_tokens::table.filter(email_verification_tokens::user_id.eq(user_id)))
            .execute(&mut conn)
            .map_err(|e| ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to delete email verification tokens by user ID",
                &e.to_string(),
            ))?;

        Ok(())
    }
}
