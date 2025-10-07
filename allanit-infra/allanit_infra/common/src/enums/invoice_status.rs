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
pub enum InvoiceStatus {
    Received,
    Validated,
    Exported,
    Failed,
}

impl InvoiceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceStatus::Received => "received",
            InvoiceStatus::Validated => "validated",
            InvoiceStatus::Exported => "exported",
            InvoiceStatus::Failed => "failed",
        }
    }
}

impl fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for InvoiceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "received" => Ok(InvoiceStatus::Received),
            "validated" => Ok(InvoiceStatus::Validated),
            "exported" => Ok(InvoiceStatus::Exported),
            "failed" => Ok(InvoiceStatus::Failed),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for InvoiceStatus {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"received" => Ok(InvoiceStatus::Received),
            b"validated" => Ok(InvoiceStatus::Validated),
            b"exported" => Ok(InvoiceStatus::Exported),
            b"failed" => Ok(InvoiceStatus::Failed),
            _ => Err("Unexpected value for InvoiceStatus".into()),
        }
    }
}

impl ToSql<Text, Pg> for InvoiceStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
    }
}
