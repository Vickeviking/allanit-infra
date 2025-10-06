use core::fmt;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::ToSql;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;

/// Enum for log levels, decides how long a log is stored
#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[diesel(sql_type = diesel::sql_types::VarChar)]
pub enum LogLevelEnum {
    Info,    //Expire in 5 minutes
    Success, //Expire in 1 day
    Warning, //Expire in 3 days
    Error,   // Expire in 7 days
    Fatal,
}

// used for logging alternatives, for choosing log level in dialoguer
impl LogLevelEnum {
    pub fn variants() -> &'static [&'static str] {
        &["Info", "Success", "Warning", "Error", "Fatal"]
    }
}

// used for selelection menue inside dialoguer
impl From<usize> for LogLevelEnum {
    fn from(idx: usize) -> Self {
        match idx {
            0 => LogLevelEnum::Info,
            1 => LogLevelEnum::Success,
            2 => LogLevelEnum::Warning,
            3 => LogLevelEnum::Error,
            4 => LogLevelEnum::Fatal,
            _ => LogLevelEnum::Info,
        }
    }
}

// serialization and logging
impl fmt::Display for LogLevelEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogLevelEnum::Info => "Info",
            LogLevelEnum::Success => "Success",
            LogLevelEnum::Warning => "Warning",
            LogLevelEnum::Error => "Error",
            LogLevelEnum::Fatal => "Fatal",
        };
        write!(f, "{}", s)
    }
}

// deserialize from database
impl FromSql<Text, Pg> for LogLevelEnum {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"Info" => Ok(LogLevelEnum::Info),
            b"Success" => Ok(LogLevelEnum::Success),
            b"Warning" => Ok(LogLevelEnum::Warning),
            b"Error" => Ok(LogLevelEnum::Error),
            b"Fatal" => Ok(LogLevelEnum::Fatal),
            _ => Err("Unexpected value".into()),
        }
    }
}

// serialize to database
impl ToSql<Text, Pg> for LogLevelEnum {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            LogLevelEnum::Info => out.write_all(b"Info")?,
            LogLevelEnum::Success => out.write_all(b"Success")?,
            LogLevelEnum::Warning => out.write_all(b"Warning")?,
            LogLevelEnum::Error => out.write_all(b"Error")?,
            LogLevelEnum::Fatal => out.write_all(b"Fatal")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

/// Enum for log actions
#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[diesel(sql_type = diesel::sql_types::VarChar)]
pub enum LogActionEnum {
    // System
    SystemStartup,
    SystemShutdown,
    SystemRestart,

    // Auth
    AuthLogin,
    AuthLogout,
    AuthTokenIssued,
    AuthTokenRejected,

    // Sync (Seventime mock och senare live)
    SyncSeventimeStarted,
    SyncSeventimeCompleted,
    SyncSeventimeFailed,

    // DB
    DbWriteOk,
    DbWriteFailed,
    DbMigrationMissing,
    DbCleanupRun,

    // Mail och kampanjer
    CampaignCreated,
    CampaignQueued,
    CampaignDispatched,
    CampaignCompleted,
    MailAcceptedByProvider,
    MailDelivered,
    MailOpened,
    MailClicked,
    MailBounced,
    MailFailed,

    // API
    ApiRequest,
    ApiResponseError,
}

// used for logging alternatives, for choosing log level in dialoguer
impl LogActionEnum {
    pub fn variants() -> &'static [&'static str] {
        &[
            "SystemStartup",
            "SystemShutdown",
            "SystemRestart",
            "AuthLogin",
            "AuthLogout",
            "AuthTokenIssued",
            "AuthTokenRejected",
            "SyncSeventimeStarted",
            "SyncSeventimeCompleted",
            "SyncSeventimeFailed",
            "DbWriteOk",
            "DbWriteFailed",
            "DbMigrationMissing",
            "DbCleanupRun",
            "CampaignCreated",
            "CampaignQueued",
            "CampaignDispatched",
            "CampaignCompleted",
            "MailAcceptedByProvider",
            "MailDelivered",
            "MailOpened",
            "MailClicked",
            "MailBounced",
            "MailFailed",
            "ApiRequest",
            "ApiResponseError",
        ]
    }
}

// used for selelection menue inside dialoguer
impl From<usize> for LogActionEnum {
    fn from(idx: usize) -> Self {
        match idx {
            0 => LogActionEnum::SystemStartup,
            1 => LogActionEnum::SystemShutdown,
            2 => LogActionEnum::SystemRestart,
            3 => LogActionEnum::AuthLogin,
            4 => LogActionEnum::AuthLogout,
            5 => LogActionEnum::AuthTokenIssued,
            6 => LogActionEnum::AuthTokenRejected,
            7 => LogActionEnum::SyncSeventimeStarted,
            8 => LogActionEnum::SyncSeventimeCompleted,
            9 => LogActionEnum::SyncSeventimeFailed,
            10 => LogActionEnum::DbWriteOk,
            11 => LogActionEnum::DbWriteFailed,
            12 => LogActionEnum::DbMigrationMissing,
            13 => LogActionEnum::DbCleanupRun,
            14 => LogActionEnum::CampaignCreated,
            15 => LogActionEnum::CampaignQueued,
            16 => LogActionEnum::CampaignDispatched,
            17 => LogActionEnum::CampaignCompleted,
            18 => LogActionEnum::MailAcceptedByProvider,
            19 => LogActionEnum::MailDelivered,
            20 => LogActionEnum::MailOpened,
            21 => LogActionEnum::MailClicked,
            22 => LogActionEnum::MailBounced,
            23 => LogActionEnum::MailFailed,
            24 => LogActionEnum::ApiRequest,
            25 => LogActionEnum::ApiResponseError,
            _ => LogActionEnum::SystemStartup,
        }
    }
}

// serialization and logging
impl fmt::Display for LogActionEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogActionEnum::SystemStartup => "SystemStartup",
            LogActionEnum::SystemShutdown => "SystemShutdown",
            LogActionEnum::SystemRestart => "SystemRestart",
            LogActionEnum::AuthLogin => "AuthLogin",
            LogActionEnum::AuthLogout => "AuthLogout",
            LogActionEnum::AuthTokenIssued => "AuthTokenIssued",
            LogActionEnum::AuthTokenRejected => "AuthTokenRejected",
            LogActionEnum::SyncSeventimeStarted => "SyncSeventimeStarted",
            LogActionEnum::SyncSeventimeCompleted => "SyncSeventimeCompleted",
            LogActionEnum::SyncSeventimeFailed => "SyncSeventimeFailed",
            LogActionEnum::DbWriteOk => "DbWriteOk",
            LogActionEnum::DbWriteFailed => "DbWriteFailed",
            LogActionEnum::DbMigrationMissing => "DbMigrationMissing",
            LogActionEnum::DbCleanupRun => "DbCleanupRun",
            LogActionEnum::CampaignCreated => "CampaignCreated",
            LogActionEnum::CampaignQueued => "CampaignQueued",
            LogActionEnum::CampaignDispatched => "CampaignDispatched",
            LogActionEnum::CampaignCompleted => "CampaignCompleted",
            LogActionEnum::MailAcceptedByProvider => "MailAcceptedByProvider",
            LogActionEnum::MailDelivered => "MailDelivered",
            LogActionEnum::MailOpened => "MailOpened",
            LogActionEnum::MailClicked => "MailClicked",
            LogActionEnum::MailBounced => "MailBounced",
            LogActionEnum::MailFailed => "MailFailed",
            LogActionEnum::ApiRequest => "ApiRequest",
            LogActionEnum::ApiResponseError => "ApiResponseError",
            LogActionEnum::ClientConnected => "ClientConnected",
            LogActionEnum::JobSubmitted => "JobSubmitted",
            LogActionEnum::JobCompleted => "JobCompleted",
            LogActionEnum::Custom => "Custom",
        };
        write!(f, "{}", s)
    }
}

// deserialize from json
impl FromStr for LogActionEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SystemStartup" => Ok(LogActionEnum::SystemStartup),
            "SystemShutdown" => Ok(LogActionEnum::SystemShutdown),
            "SystemRestart" => Ok(LogActionEnum::SystemRestart),
            "AuthLogin" => Ok(LogActionEnum::AuthLogin),
            "AuthLogout" => Ok(LogActionEnum::AuthLogout),
            "AuthTokenIssued" => Ok(LogActionEnum::AuthTokenIssued),
            "AuthTokenRejected" => Ok(LogActionEnum::AuthTokenRejected),
            "SyncSeventimeStarted" => Ok(LogActionEnum::SyncSeventimeStarted),
            "SyncSeventimeCompleted" => Ok(LogActionEnum::SyncSeventimeCompleted),
            "SyncSeventimeFailed" => Ok(LogActionEnum::SyncSeventimeFailed),
            "DbWriteOk" => Ok(LogActionEnum::DbWriteOk),
            "DbWriteFailed" => Ok(LogActionEnum::DbWriteFailed),
            "DbMigrationMissing" => Ok(LogActionEnum::DbMigrationMissing),
            "DbCleanupRun" => Ok(LogActionEnum::DbCleanupRun),
            "CampaignCreated" => Ok(LogActionEnum::CampaignCreated),
            "CampaignQueued" => Ok(LogActionEnum::CampaignQueued),
            "CampaignDispatched" => Ok(LogActionEnum::CampaignDispatched),
            "CampaignCompleted" => Ok(LogActionEnum::CampaignCompleted),
            "MailAcceptedByProvider" => Ok(LogActionEnum::MailAcceptedByProvider),
            "MailDelivered" => Ok(LogActionEnum::MailDelivered),
            "MailOpened" => Ok(LogActionEnum::MailOpened),
            "MailClicked" => Ok(LogActionEnum::MailClicked),
            "MailBounced" => Ok(LogActionEnum::MailBounced),
            "MailFailed" => Ok(LogActionEnum::MailFailed),
            "ApiRequest" => Ok(LogActionEnum::ApiRequest),
            "ApiResponseError" => Ok(LogActionEnum::ApiResponseError),
            "ClientConnected" => Ok(LogActionEnum::ClientConnected),
            "JobSubmitted" => Ok(LogActionEnum::JobSubmitted),
            "JobCompleted" => Ok(LogActionEnum::JobCompleted),
            "Custom" => Ok(LogActionEnum::Custom),
            _ => Err(()),
        }
    }
}

// deserialize from database
impl FromSql<Text, Pg> for LogActionEnum {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"SystemStartup" => Ok(LogActionEnum::SystemStartup),
            b"SystemShutdown" => Ok(LogActionEnum::SystemShutdown),
            b"SystemRestart" => Ok(LogActionEnum::SystemRestart),
            b"AuthLogin" => Ok(LogActionEnum::AuthLogin),
            b"AuthLogout" => Ok(LogActionEnum::AuthLogout),
            b"AuthTokenIssued" => Ok(LogActionEnum::AuthTokenIssued),
            b"AuthTokenRejected" => Ok(LogActionEnum::AuthTokenRejected),
            b"SyncSeventimeStarted" => Ok(LogActionEnum::SyncSeventimeStarted),
            b"SyncSeventimeCompleted" => Ok(LogActionEnum::SyncSeventimeCompleted),
            b"SyncSeventimeFailed" => Ok(LogActionEnum::SyncSeventimeFailed),
            b"DbWriteOk" => Ok(LogActionEnum::DbWriteOk),
            b"DbWriteFailed" => Ok(LogActionEnum::DbWriteFailed),
            b"DbMigrationMissing" => Ok(LogActionEnum::DbMigrationMissing),
            b"DbCleanupRun" => Ok(LogActionEnum::DbCleanupRun),
            b"CampaignCreated" => Ok(LogActionEnum::CampaignCreated),
            b"CampaignQueued" => Ok(LogActionEnum::CampaignQueued),
            b"CampaignDispatched" => Ok(LogActionEnum::CampaignDispatched),
            b"CampaignCompleted" => Ok(LogActionEnum::CampaignCompleted),
            b"MailAcceptedByProvider" => Ok(LogActionEnum::MailAcceptedByProvider),
            b"MailDelivered" => Ok(LogActionEnum::MailDelivered),
            b"MailOpened" => Ok(LogActionEnum::MailOpened),
            b"MailClicked" => Ok(LogActionEnum::MailClicked),
            b"MailBounced" => Ok(LogActionEnum::MailBounced),
            b"MailFailed" => Ok(LogActionEnum::MailFailed),
            b"ApiRequest" => Ok(LogActionEnum::ApiRequest),
            b"ApiResponseError" => Ok(LogActionEnum::ApiResponseError),
            b"ClientConnected" => Ok(LogActionEnum::ClientConnected),
            b"JobSubmitted" => Ok(LogActionEnum::JobSubmitted),
            b"JobCompleted" => Ok(LogActionEnum::JobCompleted),
            b"Custom" => Ok(LogActionEnum::Custom),
            _ => Err("Unexpected value".into()),
        }
    }
}

// serialize to database
impl ToSql<Text, Pg> for LogActionEnum {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            LogActionEnum::SystemStartup => out.write_all(b"SystemStartup")?,
            LogActionEnum::SystemShutdown => out.write_all(b"SystemShutdown")?,
            LogActionEnum::SystemRestart => out.write_all(b"SystemRestart")?,
            LogActionEnum::AuthLogin => out.write_all(b"AuthLogin")?,
            LogActionEnum::AuthLogout => out.write_all(b"AuthLogout")?,
            LogActionEnum::AuthTokenIssued => out.write_all(b"AuthTokenIssued")?,
            LogActionEnum::AuthTokenRejected => out.write_all(b"AuthTokenRejected")?,
            LogActionEnum::SyncSeventimeStarted => out.write_all(b"SyncSeventimeStarted")?,
            LogActionEnum::SyncSeventimeCompleted => out.write_all(b"SyncSeventimeCompleted")?,
            LogActionEnum::SyncSeventimeFailed => out.write_all(b"SyncSeventimeFailed")?,
            LogActionEnum::DbWriteOk => out.write_all(b"DbWriteOk")?,
            LogActionEnum::DbWriteFailed => out.write_all(b"DbWriteFailed")?,
            LogActionEnum::DbMigrationMissing => out.write_all(b"DbMigrationMissing")?,
            LogActionEnum::DbCleanupRun => out.write_all(b"DbCleanupRun")?,
            LogActionEnum::CampaignCreated => out.write_all(b"CampaignCreated")?,
            LogActionEnum::CampaignQueued => out.write_all(b"CampaignQueued")?,
            LogActionEnum::CampaignDispatched => out.write_all(b"CampaignDispatched")?,
            LogActionEnum::CampaignCompleted => out.write_all(b"CampaignCompleted")?,
            LogActionEnum::MailAcceptedByProvider => out.write_all(b"MailAcceptedByProvider")?,
            LogActionEnum::MailDelivered => out.write_all(b"MailDelivered")?,
            LogActionEnum::MailOpened => out.write_all(b"MailOpened")?,
            LogActionEnum::MailClicked => out.write_all(b"MailClicked")?,
            LogActionEnum::MailBounced => out.write_all(b"MailBounced")?,
            LogActionEnum::MailFailed => out.write_all(b"MailFailed")?,
            LogActionEnum::ApiRequest => out.write_all(b"ApiRequest")?,
            LogActionEnum::ApiResponseError => out.write_all(b"ApiResponseError")?,
            LogActionEnum::ClientConnected => out.write_all(b"ClientConnected")?,
            LogActionEnum::JobSubmitted => out.write_all(b"JobSubmitted")?,
            LogActionEnum::JobCompleted => out.write_all(b"JobCompleted")?,
            LogActionEnum::Custom => out.write_all(b"Custom")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}
