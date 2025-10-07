use crate::database::models::email_inbound::{EmailInbound, NewEmailInbound};
use crate::database::schema::emails_inbound;
use crate::database::schema::emails_inbound::dsl as ei;
use crate::enums::email_status::EmailStatus;
use diesel::{insert_into, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct EmailInboundRepository;

impl EmailInboundRepository {
    pub async fn insert(
        conn: &mut AsyncPgConnection,
        rec: NewEmailInbound,
    ) -> QueryResult<EmailInbound> {
        match insert_into(emails_inbound::table)
            .values(&rec)
            .on_conflict(ei::message_id)
            .do_nothing()
            .get_result::<EmailInbound>(conn)
            .await
        {
            Ok(record) => Ok(record),
            Err(_) => {
                // fallback if already exists
                if let Some(existing) = Self::find_by_message_id(conn, &rec.message_id).await? {
                    Ok(existing)
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }
        }
    }

    pub async fn find_by_message_id(
        conn: &mut AsyncPgConnection,
        msg_id: &str,
    ) -> QueryResult<Option<EmailInbound>> {
        ei::emails_inbound
            .filter(ei::message_id.eq(msg_id))
            .first::<EmailInbound>(conn)
            .await
            .optional()
    }

    pub async fn list_recent(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<EmailInbound>> {
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
        status: EmailStatus,
    ) -> QueryResult<EmailInbound> {
        diesel::update(ei::emails_inbound.find(id))
            .set((ei::parsed.eq(parsed), ei::status.eq(status)))
            .get_result(conn)
            .await
    }
}
