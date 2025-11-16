use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::entities::user::{NewUser, User};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::database::{schema::users, DbPool};
use crate::shared::error_types::{ApiError, ERROR_DATABASE_ERROR, ERROR_USER_ALREADY_EXISTS};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: DbPool,
}

impl PostgresUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User, password_hash: &str) -> Result<User, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        // Check if email already exists
        if self.email_exists(&user.email).await? {
            return Err(ApiError::new(
                ERROR_USER_ALREADY_EXISTS,
                "Email already exists",
            ));
        }

        // Check if username already exists
        if self.username_exists(&user.username).await? {
            return Err(ApiError::new(
                ERROR_USER_ALREADY_EXISTS,
                "Username already exists",
            ));
        }

        // Create user with password hash
        let mut user_with_password = user.clone();
        user_with_password.password_hash = password_hash.to_string();

        // Insert user
        let new_user_data = NewUser::from_user(&user_with_password);
        let new_user = diesel::insert_into(users::table)
            .values(&new_user_data)
            .get_result::<User>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to create user",
                    &e.to_string(),
                )
            })?;

        Ok(new_user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let user = users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find user by ID",
                    &e.to_string(),
                )
            })?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let user = users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find user by email",
                    &e.to_string(),
                )
            })?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let user = users::table
            .filter(users::username.eq(username))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to find user by username",
                    &e.to_string(),
                )
            })?;

        Ok(user)
    }

    async fn update(&self, user: &User) -> Result<User, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let updated_user = diesel::update(users::table.filter(users::id.eq(user.id)))
            .set((
                users::email.eq(&user.email),
                users::username.eq(&user.username),
                users::first_name.eq(&user.first_name),
                users::last_name.eq(&user.last_name),
                users::phone.eq(&user.phone),
                users::avatar_url.eq(&user.avatar_url),
                users::is_host.eq(user.is_host),
                users::is_verified.eq(user.is_verified),
                users::updated_at.eq(Some(Utc::now())),
            ))
            .returning(User::as_returning())
            .get_result::<User>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to update user",
                    &e.to_string(),
                )
            })?;

        Ok(updated_user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to delete user",
                    &e.to_string(),
                )
            })?;

        Ok(())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let count = users::table
            .filter(users::email.eq(email))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to check email existence",
                    &e.to_string(),
                )
            })?;

        Ok(count > 0)
    }

    async fn username_exists(&self, username: &str) -> Result<bool, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let count = users::table
            .filter(users::username.eq(username))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to check username existence",
                    &e.to_string(),
                )
            })?;

        Ok(count > 0)
    }

    async fn get_password_hash(&self, user_id: Uuid) -> Result<Option<String>, ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        let password_hash = users::table
            .filter(users::id.eq(user_id))
            .select(users::password_hash)
            .first::<String>(&mut conn)
            .optional()
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to get password hash",
                    &e.to_string(),
                )
            })?;

        Ok(password_hash)
    }

    async fn update_password(&self, user_id: Uuid, password_hash: &str) -> Result<(), ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::password_hash.eq(password_hash),
                users::updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to update password",
                    &e.to_string(),
                )
            })?;

        Ok(())
    }

    async fn verify_user(&self, user_id: Uuid) -> Result<(), ApiError> {
        let mut conn = self.pool.get().map_err(|e| {
            ApiError::with_details(
                ERROR_DATABASE_ERROR,
                "Failed to get database connection",
                &e.to_string(),
            )
        })?;

        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::is_verified.eq(true),
                users::updated_at.eq(Utc::now()),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                ApiError::with_details(
                    ERROR_DATABASE_ERROR,
                    "Failed to verify user",
                    &e.to_string(),
                )
            })?;

        Ok(())
    }
}
