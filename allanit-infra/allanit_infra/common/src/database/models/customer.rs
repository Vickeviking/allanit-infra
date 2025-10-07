use crate::database::schema::customers;
use chrono::{DateTime, Utc};
use diesel::{prelude::*, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = customers)]
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

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = customers)]
pub struct NewCustomer {
    pub external_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub org_no: Option<String>,
}
