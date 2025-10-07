use crate::database::schema::emails_inbound;
use crate::enums::email_status::EmailStatus;
use chrono::{DateTime, Utc};
use diesel::{prelude::*, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = emails_inbound)]
pub struct EmailInbound {
    pub id: i64,
    pub message_id: String,
    pub from_addr: String,
    pub to_addr: Vec<Option<String>>,
    pub subject: Option<String>,
    pub received_at: DateTime<Utc>,
    pub raw_source: Option<String>,
    pub parsed: Option<serde_json::Value>,
    pub status: EmailStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = emails_inbound)]
pub struct NewEmailInbound {
    pub message_id: String,
    pub from_addr: String,
    pub to_addr: Vec<Option<String>>,
    pub subject: Option<String>,
    pub raw_source: Option<String>,
    pub parsed: Option<serde_json::Value>,
}
