use crate::database::schema::invoices_raw;
use crate::enums::invoice_status::InvoiceStatus;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = invoices_raw)]
pub struct InvoiceRaw {
    pub id: i64,
    pub source_system: String,
    pub external_id: Option<String>,
    pub customer_external_id: Option<String>,
    pub email_id: Option<i64>,
    pub payload: serde_json::Value,
    pub file_uri: Option<String>,
    pub ocr_text: Option<String>,
    pub status: InvoiceStatus,
    pub received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = invoices_raw)]
pub struct NewInvoiceRaw {
    pub source_system: String,
    pub external_id: Option<String>,
    pub customer_external_id: Option<String>,
    pub email_id: Option<i64>,
    pub payload: serde_json::Value,
    pub file_uri: Option<String>,
    pub ocr_text: Option<String>,
}
