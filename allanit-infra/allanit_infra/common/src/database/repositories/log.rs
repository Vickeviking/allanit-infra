use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::database::models::log::{LogEntry, NewLogEntry};
use crate::database::schema::logs::dsl as l;

pub struct LogRepository;

impl LogRepository {
    pub async fn insert_batch(
        conn: &mut AsyncPgConnection,
        entries: &[NewLogEntry],
    ) -> QueryResult<usize> {
        diesel::insert_into(l::logs)
            .values(entries)
            .execute(conn)
            .await
    }

    pub async fn delete_expired(conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        use chrono::Utc;
        diesel::delete(l::logs.filter(l::expires_at.lt(Utc::now().naive_utc())))
            .execute(conn)
            .await
    }
}
