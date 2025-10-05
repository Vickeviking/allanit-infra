use core::fmt;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;

/// not stored in db, used in IPC and logging
#[derive(Debug, Clone)]
pub enum CoreEvent {
    Startup,
    Shutdown,
    Restart,
}

impl fmt::Display for CoreEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CoreEvent::Startup => "Startup",
            CoreEvent::Shutdown => "Shutdown",
            CoreEvent::Restart => "Restart",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for CoreEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Startup" => Ok(CoreEvent::Startup),
            "Shutdown" => Ok(CoreEvent::Shutdown),
            "Restart" => Ok(CoreEvent::Restart),
            _ => Err(()),
        }
    }
}

/// not stored in db, used in IPC and logging
#[derive(Debug, Clone)]
pub enum Pulse {
    Slow,   // 1 minute
    Medium, // 1 second
    Fast,   // 50ms
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Pulse::Slow => "Slow",
            Pulse::Medium => "Medium",
            Pulse::Fast => "Fast",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Pulse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Slow" => Ok(Pulse::Slow),
            "Medium" => Ok(Pulse::Medium),
            "Fast" => Ok(Pulse::Fast),
            _ => Err(()),
        }
    }
}
