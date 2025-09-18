use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::{Queryable, Insertable, Selectable, Identifiable};
use crate::infrastructure::database::schema::sessions;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub is_active: Option<bool>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Session {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        session_token: String,
        access_token: String,
        refresh_token: Option<String>,
        expires_at: DateTime<Utc>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            user_id,
            session_token,
            access_token,
            refresh_token,
            expires_at,
            is_active: Some(true),
            ip_address,
            user_agent,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn deactivate(&mut self) {
        self.is_active = Some(false);
        self.updated_at = Some(Utc::now());
    }

    pub fn is_valid(&self) -> bool {
        self.is_active.unwrap_or(false) && !self.is_expired()
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub is_active: Option<bool>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl NewSession {
    pub fn from_session(session: &Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            session_token: session.session_token.clone(),
            access_token: session.access_token.clone(),
            refresh_token: session.refresh_token.clone(),
            expires_at: session.expires_at,
            is_active: session.is_active,
            ip_address: session.ip_address.clone(),
            user_agent: session.user_agent.clone(),
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}
