use crate::database::models::purchase_order::{NewPurchaseOrder, PurchaseOrder};
use crate::database::schema::purchase_orders;
use diesel::{insert_into, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct PurchaseOrderRepository;

impl PurchaseOrderRepository {
    pub async fn upsert(
        conn: &mut AsyncPgConnection,
        rec: NewPurchaseOrder,
    ) -> QueryResult<PurchaseOrder> {
        insert_into(purchase_orders::table)
            .values(rec.clone())
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

    pub async fn list(
        conn: &mut AsyncPgConnection,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .limit(limit)
            .offset(offset)
            .load(conn)
            .await
    }
}
