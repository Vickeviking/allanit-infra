use crate::database::models::customer::{Customer, NewCustomer};
use crate::database::schema::customers;
use diesel::{insert_into, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct CustomerRepository;

impl CustomerRepository {
    pub async fn upsert(conn: &mut AsyncPgConnection, rec: NewCustomer) -> QueryResult<Customer> {
        insert_into(customers::table)
            .values(rec.clone())
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

    pub async fn find_by_external_id(
        conn: &mut AsyncPgConnection,
        ext: &str,
    ) -> QueryResult<Option<Customer>> {
        customers::table
            .filter(customers::external_id.eq(ext))
            .first::<Customer>(conn)
            .await
            .optional()
    }

    pub async fn list(
        conn: &mut AsyncPgConnection,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<Customer>> {
        customers::table
            .limit(limit)
            .offset(offset)
            .load(conn)
            .await
    }
}
