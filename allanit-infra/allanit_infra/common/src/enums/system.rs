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

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[diesel(sql_type = diesel::sql_types::VarChar)]
pub enum SystemModuleEnum {
    // Services
    PulseBroadcaster,
    ShutdownWatcher,
    Logger,

    // Modules
    SeventimeFetcher,
    IngestorRouter,
    DbWriter,
    DeadLetterWorker,
    CommandBus,
    RocketAPI,
    HealthAndMetrics,

    // Bootstrap and misc
    ServiceInitializer,
    ModuleInitializer,
    Core,
}

impl SystemModuleEnum {
    pub fn variants() -> &'static [&'static str] {
        &[
            "PulseBroadcaster",
            "ShutdownWatcher",
            "Logger",
            "SeventimeFetcher",
            "IngestorRouter",
            "DbWriter",
            "DeadLetterWorker",
            "CommandBus",
            "RocketAPI",
            "HealthAndMetrics",
            "ServiceInitializer",
            "ModuleInitializer",
            "Core",
        ]
    }
}

impl fmt::Display for SystemModuleEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SystemModuleEnum::*;
        let s = match self {
            PulseBroadcaster => "PulseBroadcaster",
            ShutdownWatcher => "ShutdownWatcher",
            Logger => "Logger",
            SeventimeFetcher => "SeventimeFetcher",
            IngestorRouter => "IngestorRouter",
            DbWriter => "DbWriter",
            DeadLetterWorker => "DeadLetterWorker",
            CommandBus => "CommandBus",
            RocketAPI => "RocketAPI",
            HealthAndMetrics => "HealthAndMetrics",
            ServiceInitializer => "ServiceInitializer",
            ModuleInitializer => "ModuleInitializer",
            Core => "Core",
        };
        write!(f, "{s}")
    }
}

impl FromSql<Text, Pg> for SystemModuleEnum {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"PulseBroadcaster" => Ok(Self::PulseBroadcaster),
            b"ShutdownWatcher" => Ok(Self::ShutdownWatcher),
            b"Logger" => Ok(Self::Logger),
            b"SeventimeFetcher" => Ok(Self::SeventimeFetcher),
            b"IngestorRouter" => Ok(Self::IngestorRouter),
            b"DbWriter" => Ok(Self::DbWriter),
            b"DeadLetterWorker" => Ok(Self::DeadLetterWorker),
            b"CommandBus" => Ok(Self::CommandBus),
            b"RocketAPI" => Ok(Self::RocketAPI),
            b"HealthAndMetrics" => Ok(Self::HealthAndMetrics),
            b"ServiceInitializer" => Ok(Self::ServiceInitializer),
            b"ModuleInitializer" => Ok(Self::ModuleInitializer),
            b"Core" => Ok(Self::Core),
            _ => Err("Unexpected value for SystemModuleEnum".into()),
        }
    }
}

impl ToSql<Text, Pg> for SystemModuleEnum {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
    }
}
