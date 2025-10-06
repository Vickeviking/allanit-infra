use chrono::NaiveDateTime;
use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::schema::logs;
use crate::enums::{LogActionEnum, LogLevelEnum, SystemModuleEnum};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = logs)]
pub struct LogEntry {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub level: LogLevelEnum,
    pub module: SystemModuleEnum,
    pub action: LogActionEnum,
    pub custom_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = logs)]
pub struct NewLogEntry {
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub level: LogLevelEnum,
    pub module: SystemModuleEnum,
    pub action: LogActionEnum,
    pub custom_msg: Option<String>,
}
