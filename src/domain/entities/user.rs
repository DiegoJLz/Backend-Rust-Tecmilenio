use crate::infrastructure::database::schema::users;
use crate::shared::error_types::ApiError;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub password_hash: String,
    pub is_host: Option<bool>,
    pub is_verified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        username: String,
        first_name: String,
        last_name: String,
        phone: Option<String>,
        password_hash: String,
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            email,
            username,
            first_name,
            last_name,
            phone,
            avatar_url: None,
            password_hash,
            is_host: Some(false),
            is_verified: Some(false),
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    pub fn verify(&mut self) {
        self.is_verified = Some(true);
        self.updated_at = Some(Utc::now());
    }

    pub fn make_host(&mut self) {
        self.is_host = Some(true);
        self.updated_at = Some(Utc::now());
    }

    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<(), ApiError> {
        if let Some(name) = first_name {
            self.first_name = name;
        }

        if let Some(name) = last_name {
            self.last_name = name;
        }

        if let Some(phone) = phone {
            self.phone = Some(phone);
        }

        if let Some(avatar) = avatar_url {
            self.avatar_url = Some(avatar);
        }

        self.updated_at = Some(Utc::now());
        Ok(())
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub password_hash: String,
    pub is_host: Option<bool>,
    pub is_verified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl NewUser {
    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            phone: user.phone.clone(),
            avatar_url: user.avatar_url.clone(),
            password_hash: user.password_hash.clone(),
            is_host: user.is_host,
            is_verified: user.is_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
