use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: Uuid,
    pub to_email: String,
    pub to_name: Option<String>,
    pub from_email: String,
    pub from_name: String,
    pub subject: String,
    pub html_content: String,
    pub text_content: String,
    pub email_type: EmailType,
    pub status: EmailStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailType {
    EmailVerification,
    PasswordReset,
    Welcome,
    Notification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailStatus {
    Pending,
    Sent,
    Failed,
    Bounced,
}

impl Email {
    pub fn new(
        to_email: String,
        to_name: Option<String>,
        from_email: String,
        from_name: String,
        subject: String,
        html_content: String,
        text_content: String,
        email_type: EmailType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            to_email,
            to_name,
            from_email,
            from_name,
            subject,
            html_content,
            text_content,
            email_type,
            status: EmailStatus::Pending,
            sent_at: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_as_sent(&mut self) {
        self.status = EmailStatus::Sent;
        self.sent_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn mark_as_failed(&mut self, error_message: String) {
        self.status = EmailStatus::Failed;
        self.error_message = Some(error_message);
        self.updated_at = Utc::now();
    }

    pub fn is_sent(&self) -> bool {
        matches!(self.status, EmailStatus::Sent)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, EmailStatus::Failed)
    }
}
