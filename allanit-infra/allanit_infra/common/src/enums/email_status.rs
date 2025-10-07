use core::fmt;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;

#[derive(AsExpression, FromSqlRow, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[diesel(sql_type = diesel::sql_types::VarChar)]
pub enum EmailStatus {
    Received,
    Parsed,
    Failed,
}

impl EmailStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            EmailStatus::Received => "received",
            EmailStatus::Parsed => "parsed",
            EmailStatus::Failed => "failed",
        }
    }
}

impl fmt::Display for EmailStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for EmailStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "received" => Ok(EmailStatus::Received),
            "parsed" => Ok(EmailStatus::Parsed),
            "failed" => Ok(EmailStatus::Failed),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for EmailStatus {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"received" => Ok(EmailStatus::Received),
            b"parsed" => Ok(EmailStatus::Parsed),
            b"failed" => Ok(EmailStatus::Failed),
            _ => Err("Unexpected value for EmailStatus".into()),
        }
    }
}

impl ToSql<Text, Pg> for EmailStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
    }
}
