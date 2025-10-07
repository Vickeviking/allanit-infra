use crate::database::schema::purchase_orders;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::Deserialize;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = purchase_orders)]
pub struct PurchaseOrder {
    pub id: i64,
    pub external_id: String,
    pub customer_id: Option<i64>,
    pub status: String,
    pub description: Option<String>,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = purchase_orders)]
pub struct NewPurchaseOrder {
    pub external_id: String,
    pub customer_id: Option<i64>,
    pub status: String,
    pub description: Option<String>,
    #[serde(deserialize_with = "de_amount")]
    pub amount: f64,
}

fn de_amount<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    use serde_json::Value;

    let v = Value::deserialize(d)?;
    match v {
        Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0)),
        Value::String(s) => s.parse::<f64>().map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a number or numeric string")
        }),
        _ => Ok(0.0),
    }
}
