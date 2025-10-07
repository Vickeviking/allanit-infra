## customer.rs

use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable, Identifiable, Selectable};
use crate::database::schema::customers;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)] #[diesel(table_name = customers)]
pub struct Customer {
pub id: i64,
pub external_id: String,
pub name: String,
pub email: Option<String>,
pub phone: Option<String>,
pub org_no: Option<String>,
pub created_at: DateTime<Utc>,
pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)] #[diesel(table_name = customers)]
pub struct NewCustomer {
pub external_id: String,
pub name: String,
pub email: Option<String>,
pub phone: Option<String>,
pub org_no: Option<String>,
}

## purchase_order.rs

use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable, Identifiable, Selectable};
use crate::database::schema::purchase_orders;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)] #[diesel(table_name = purchase_orders)]
pub struct PurchaseOrder {
pub id: i64,
pub external_id: String,
pub customer_id: Option<i64>,
pub status: String,
pub description: Option<String>,
pub amount: bigdecimal::BigDecimal,
pub created_at: DateTime<Utc>,
pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)] #[diesel(table_name = purchase_orders)]
pub struct NewPurchaseOrder {
pub external_id: String,
pub customer_id: Option<i64>,
pub status: String,
pub description: Option<String>,
pub amount: bigdecimal::BigDecimal,
}

## email_inbound.rs

use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable, Identifiable, Selectable};
use crate::database::schema::emails_inbound;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)] #[diesel(table_name = emails_inbound)]
pub struct EmailInbound {
pub id: i64,
pub message_id: String,
pub from_addr: String,
pub to_addr: Vec<Option<String>>,
pub subject: Option<String>,
pub received_at: DateTime<Utc>,
pub raw_source: Option<String>,
pub parsed: Option<serde_json::Value>,
pub status: crate::database::schema::sql_types::EmailStatus,
pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)] #[diesel(table_name = emails_inbound)]
pub struct NewEmailInbound {
pub message_id: String,
pub from_addr: String,
pub to_addr: Vec<Option<String>>,
pub subject: Option<String>,
pub raw_source: Option<String>,
pub parsed: Option<serde_json::Value>,
}

# invoice_raw.rs

use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable, Identifiable, Selectable};
use crate::database::schema::invoices_raw;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)] #[diesel(table_name = invoices_raw)]
pub struct InvoiceRaw {
pub id: i64,
pub source_system: String,
pub external_id: Option<String>,
pub customer_external_id: Option<String>,
pub email_id: Option<i64>,
pub payload: serde_json::Value,
pub file_uri: Option<String>,
pub ocr_text: Option<String>,
pub status: crate::database::schema::sql_types::InvoiceStatus,
pub received_at: DateTime<Utc>,
pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)] #[diesel(table_name = invoices_raw)]
pub struct NewInvoiceRaw {
pub source_system: String,
pub external_id: Option<String>,
pub customer_external_id: Option<String>,
pub email_id: Option<i64>,
pub payload: serde_json::Value,
pub file_uri: Option<String>,
pub ocr_text: Option<String>,
}
