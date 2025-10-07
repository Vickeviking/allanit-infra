use crate::database::models::invoice_raw::{InvoiceRaw, NewInvoiceRaw};
use crate::database::schema::invoices_raw;
use crate::database::schema::invoices_raw::dsl as ir;
use crate::enums::invoice_status::InvoiceStatus;
use diesel::{insert_into, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct InvoiceRawRepository;

impl InvoiceRawRepository {
    pub async fn insert(
        conn: &mut AsyncPgConnection,
        rec: NewInvoiceRaw,
    ) -> QueryResult<InvoiceRaw> {
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
            if let Some(existing) =
                Self::find_by_source_and_external(conn, &rec.source_system, &ext).await?
            {
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

    pub async fn list_recent(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<InvoiceRaw>> {
        ir::invoices_raw
            .order(ir::received_at.desc())
            .limit(limit)
            .load(conn)
            .await
    }

    pub async fn set_status(
        conn: &mut AsyncPgConnection,
        id: i64,
        status: InvoiceStatus,
    ) -> QueryResult<InvoiceRaw> {
        diesel::update(ir::invoices_raw.find(id))
            .set(ir::status.eq(status))
            .get_result(conn)
            .await
    }
}
