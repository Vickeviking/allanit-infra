# customer_repo:

use diesel::{prelude::\*, insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::database::models::customer::{Customer, NewCustomer};
use crate::database::schema::customers;

pub struct CustomerRepository;

impl CustomerRepository {
pub async fn upsert(conn: &mut AsyncPgConnection, rec: NewCustomer) -> QueryResult<Customer> {
insert_into(customers::table)
.values(&rec)
.on_conflict(customers::external_id)
.do_update()
.set((
customers::name.eq(rec.name),
customers::email.eq(rec.email),
customers::phone.eq(rec.phone),
customers::org_no.eq(rec.org_no),
))
.get_result(conn)
.await
}

    pub async fn find_by_external_id(conn: &mut AsyncPgConnection, ext: &str) -> QueryResult<Option<Customer>> {
        customers::table
            .filter(customers::external_id.eq(ext))
            .first::<Customer>(conn)
            .await
            .optional()
    }

    pub async fn list(conn: &mut AsyncPgConnection, limit: i64, offset: i64) -> QueryResult<Vec<Customer>> {
        customers::table.limit(limit).offset(offset).load(conn).await
    }

}

# purchase_order_repo.rs

use diesel::{prelude::\*, insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::database::models::purchase_order::{PurchaseOrder, NewPurchaseOrder};
use crate::database::schema::purchase_orders;

pub struct PurchaseOrderRepository;

impl PurchaseOrderRepository {
pub async fn upsert(conn: &mut AsyncPgConnection, rec: NewPurchaseOrder) -> QueryResult<PurchaseOrder> {
insert_into(purchase_orders::table)
.values(&rec)
.on_conflict(purchase_orders::external_id)
.do_update()
.set((
purchase_orders::customer_id.eq(rec.customer_id),
purchase_orders::status.eq(rec.status),
purchase_orders::description.eq(rec.description),
purchase_orders::amount.eq(rec.amount),
))
.get_result(conn)
.await
}

    pub async fn list(conn: &mut AsyncPgConnection, limit: i64, offset: i64) -> QueryResult<Vec<PurchaseOrder>> {
        purchase_orders::table.limit(limit).offset(offset).load(conn).await
    }

}

# email_inbound_repo.rs

use diesel::{prelude::\*, insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::database::models::email_inbound::{EmailInbound, NewEmailInbound};
use crate::database::schema::emails_inbound;
use crate::database::schema::emails_inbound::dsl as ei;

pub struct EmailInboundRepository;

impl EmailInboundRepository {
pub async fn insert(conn: &mut AsyncPgConnection, rec: NewEmailInbound) -> QueryResult<EmailInbound> {
insert*into(emails_inbound::table)
.values(&rec)
.on_conflict(ei::message_id)
.do_nothing()
.get_result(conn)
.await
.or_else(|*| Self::find_by_message_id(conn, &rec.message_id).await?
.ok_or(diesel::result::Error::NotFound))
}

    pub async fn find_by_message_id(conn: &mut AsyncPgConnection, msg_id: &str) -> QueryResult<Option<EmailInbound>> {
        ei::emails_inbound
            .filter(ei::message_id.eq(msg_id))
            .first::<EmailInbound>(conn)
            .await
            .optional()
    }

    pub async fn list_recent(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<EmailInbound>> {
        ei::emails_inbound
            .order(ei::received_at.desc())
            .limit(limit)
            .load(conn)
            .await
    }

    pub async fn set_parsed_and_status(
        conn: &mut AsyncPgConnection,
        id: i64,
        parsed: Option<serde_json::Value>,
        status: crate::database::schema::sql_types::EmailStatus,
    ) -> QueryResult<EmailInbound> {
        diesel::update(ei::emails_inbound.find(id))
            .set((ei::parsed.eq(parsed), ei::status.eq(status)))
            .get_result(conn)
            .await
    }

}

# invoice_raw_repo.rs

use diesel::{prelude::\*, insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::database::models::invoice_raw::{InvoiceRaw, NewInvoiceRaw};
use crate::database::schema::invoices_raw;
use crate::database::schema::invoices_raw::dsl as ir;

pub struct InvoiceRawRepository;

impl InvoiceRawRepository {
pub async fn insert(conn: &mut AsyncPgConnection, rec: NewInvoiceRaw) -> QueryResult<InvoiceRaw> {
insert_into(invoices_raw::table)
.values(&rec)
.get_result(conn)
.await
}

    /// Idempotent upsert baserat på (source_system, external_id).
    /// Om external_id saknas: insert.
    pub async fn upsert_by_external(
        conn: &mut AsyncPgConnection,
        mut rec: NewInvoiceRaw,
    ) -> QueryResult<InvoiceRaw> {
        if let Some(ext) = rec.external_id.clone() {
            if let Some(existing) = Self::find_by_source_and_external(conn, &rec.source_system, &ext).await? {
                // update fält som kan ändras
                return diesel::update(ir::invoices_raw.find(existing.id))
                    .set((
                        ir::customer_external_id.eq(rec.customer_external_id.take()),
                        ir::email_id.eq(rec.email_id.take()),
                        ir::payload.eq(rec.payload),
                        ir::file_uri.eq(rec.file_uri.take()),
                        ir::ocr_text.eq(rec.ocr_text.take()),
                    ))
                    .get_result(conn)
                    .await;
            }
        }
        Self::insert(conn, rec).await
    }

    pub async fn find_by_source_and_external(
        conn: &mut AsyncPgConnection,
        source: &str,
        external_id: &str,
    ) -> QueryResult<Option<InvoiceRaw>> {
        ir::invoices_raw
            .filter(ir::source_system.eq(source))
            .filter(ir::external_id.eq(external_id))
            .first::<InvoiceRaw>(conn)
            .await
            .optional()
    }

    pub async fn list_recent(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<InvoiceRaw>> {
        ir::invoices_raw
            .order(ir::received_at.desc())
            .limit(limit)
            .load(conn)
            .await
    }

    pub async fn set_status(
        conn: &mut AsyncPgConnection,
        id: i64,
        status: crate::database::schema::sql_types::InvoiceStatus,
    ) -> QueryResult<InvoiceRaw> {
        diesel::update(ir::invoices_raw.find(id))
            .set(ir::status.eq(status))
            .get_result(conn)
            .await
    }

}
