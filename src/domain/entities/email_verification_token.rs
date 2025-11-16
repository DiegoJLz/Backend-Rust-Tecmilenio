use crate::infrastructure::database::schema::email_verification_tokens;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = email_verification_tokens)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

impl EmailVerificationToken {
    pub fn new(user_id: Uuid, token: String, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(expiration_hours);

        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            expires_at,
            is_used: Some(false),
            created_at: Some(now),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        !self.is_used.unwrap_or(false) && !self.is_expired()
    }

    pub fn mark_as_used(&mut self) {
        self.is_used = Some(true);
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct NewEmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

impl NewEmailVerificationToken {
    pub fn from_token(token: &EmailVerificationToken) -> Self {
        Self {
            id: token.id,
            user_id: token.user_id,
            token: token.token.clone(),
            expires_at: token.expires_at,
            is_used: token.is_used,
            created_at: token.created_at,
        }
    }
}
