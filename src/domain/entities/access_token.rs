use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::{Queryable, Insertable, Selectable, Identifiable};
use crate::infrastructure::database::schema::access_tokens;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = access_tokens)]
pub struct AccessToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: Option<bool>,
    pub is_revoked: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AccessToken {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        token: String,
        token_type: String,
        expires_at: DateTime<Utc>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            user_id,
            token,
            token_type,
            expires_at,
            is_used: Some(false),
            is_revoked: Some(false),
            metadata,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_used(&self) -> bool {
        self.is_used.unwrap_or(false)
    }

    pub fn is_revoked(&self) -> bool {
        self.is_revoked.unwrap_or(false)
    }

    pub fn mark_as_used(&mut self) {
        self.is_used = Some(true);
        self.updated_at = Some(Utc::now());
    }

    pub fn revoke(&mut self) {
        self.is_revoked = Some(true);
        self.updated_at = Some(Utc::now());
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used() && !self.is_revoked()
    }

    pub fn is_password_reset_token(&self) -> bool {
        self.token_type == "password_reset"
    }

    pub fn is_email_verification_token(&self) -> bool {
        self.token_type == "email_verification"
    }

    pub fn is_access_token(&self) -> bool {
        self.token_type == "access"
    }

    pub fn is_refresh_token(&self) -> bool {
        self.token_type == "refresh"
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = access_tokens)]
pub struct NewAccessToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: Option<bool>,
    pub is_revoked: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl NewAccessToken {
    pub fn from_access_token(access_token: &AccessToken) -> Self {
        Self {
            id: access_token.id,
            user_id: access_token.user_id,
            token: access_token.token.clone(),
            token_type: access_token.token_type.clone(),
            expires_at: access_token.expires_at,
            is_used: access_token.is_used,
            is_revoked: access_token.is_revoked,
            metadata: access_token.metadata.clone(),
            created_at: access_token.created_at,
            updated_at: access_token.updated_at,
        }
    }
}
