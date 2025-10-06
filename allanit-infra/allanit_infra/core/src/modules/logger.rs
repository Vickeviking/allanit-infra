use chrono::{NaiveDateTime, Utc};
use std::{path::PathBuf, sync::Arc};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    select,
    sync::{broadcast::Receiver, Mutex, RwLock},
};

use crate::services::PulseSubscriptions;
use common::commands::load_db_connection;
use common::enums::{CoreEvent, LogActionEnum, LogLevelEnum, Pulse, SystemModuleEnum};

use common::database::{models::log::NewLogEntry, repositories::log::LogRepository};

/// Controls where logs are persisted
#[derive(Debug, Clone, Copy)]
pub enum LogSinkMode {
    /// Only in-memory + NDJSON file/stdout
    MemoryNdjson,
    /// Writes to Postgres (fallback to NDJSON if DB down)
    Postgres,
}

/// Simple in-memory log buffer entry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BufLog {
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub level: LogLevelEnum,
    pub module: SystemModuleEnum,
    pub action: LogActionEnum,
    pub custom_msg: Option<String>,
}

impl From<&BufLog> for NewLogEntry {
    fn from(b: &BufLog) -> Self {
        Self {
            created_at: b.created_at,
            expires_at: b.expires_at,
            level: b.level.clone(),
            module: b.module.clone(),
            action: b.action.clone(),
            custom_msg: b.custom_msg.clone(),
        }
    }
}

/// Central asynchronous logger
pub struct Logger {
    /// In-memory buffer flushed on pulse/shutdown
    buffer_logs: RwLock<Vec<BufLog>>,
    /// Lifecycle events (Startup/Restart/Shutdown)
    core_event_rx: Mutex<Receiver<CoreEvent>>,
    /// Slow pulse (~2s)
    pulse_rx: Mutex<Receiver<Pulse>>,
    /// Optional NDJSON file sink
    file_sink: Option<Mutex<tokio::fs::File>>,
    /// Determines sink type
    sink_mode: LogSinkMode,
}

impl Logger {
    /* ---------------- construction -------------------- */

    pub fn new(core_rx: Receiver<CoreEvent>, pulse_subs: Arc<PulseSubscriptions>) -> Self {
        Self {
            buffer_logs: RwLock::new(Vec::new()),
            core_event_rx: Mutex::new(core_rx),
            pulse_rx: Mutex::new(pulse_subs.subscribe_slow()),
            file_sink: None,
            sink_mode: LogSinkMode::MemoryNdjson,
        }
    }

    pub fn with_sink_mode(mut self, mode: LogSinkMode) -> Self {
        self.sink_mode = mode;
        self
    }

    pub async fn with_file_sink(mut self, path: Option<PathBuf>) -> Self {
        if let Some(p) = path {
            if let Ok(f) = OpenOptions::new().create(true).append(true).open(p).await {
                self.file_sink = Some(Mutex::new(f));
            }
        }
        self
    }

    /// Builds logger from env vars:
    /// LOG_FILE=/path/to/file.ndjson
    /// LOG_SINK=pg|memory
    pub async fn from_env(
        core_rx: Receiver<CoreEvent>,
        pulse_subs: Arc<PulseSubscriptions>,
    ) -> Self {
        let path = std::env::var_os("LOG_FILE").map(PathBuf::from);
        let mode = match std::env::var("LOG_SINK")
            .unwrap_or_else(|_| "memory".into())
            .as_str()
        {
            "pg" | "postgres" => LogSinkMode::Postgres,
            _ => LogSinkMode::MemoryNdjson,
        };
        Logger::new(core_rx, pulse_subs)
            .with_sink_mode(mode)
            .with_file_sink(path)
            .await
    }

    /* ---------------- background loop -------------------- */

    pub async fn init(self: Arc<Self>) {
        let mut core_rx = self.core_event_rx.lock().await;
        let mut pulse_rx = self.pulse_rx.lock().await;
        loop {
            select! {
                Ok(ev) = core_rx.recv() => match ev {
                    CoreEvent::Startup => {},
                    CoreEvent::Restart => { self.store_all_logs().await; },
                    CoreEvent::Shutdown => {
                        self.store_all_logs().await;
                        println!("Logger | Shutdown complete.");
                        break;
                    },
                },
                Ok(pulse) = pulse_rx.recv() => {
                    if matches!(pulse, Pulse::Slow) {
                        self.try_clean().await;
                        self.store_all_logs().await;
                    }
                }
            }
        }
    }

    /* ---------------- public API -------------------- */

    #[allow(clippy::too_many_arguments)]
    pub async fn log(
        logger: Arc<Self>,
        level: LogLevelEnum,
        module: SystemModuleEnum,
        action: LogActionEnum,
        custom: Option<String>,
    ) {
        let now = Utc::now().naive_utc();
        // ttl_for tar &LogLevelEnum, så låna bara:
        let expires_at = Self::ttl_for(&level, now);

        logger.buffer_logs.write().await.push(BufLog {
            created_at: now,
            expires_at,
            level, // flytta in värdet (ingen clone behövs)
            module,
            action,
            custom_msg: custom, // Option<String> flyttas in
        });
    }

    pub async fn log_now(
        &self,
        level: LogLevelEnum,
        module: SystemModuleEnum,
        action: LogActionEnum,
        custom: impl Into<Option<String>>,
    ) {
        let now = Utc::now().naive_utc();
        let expires_at = Self::ttl_for(&level, now);
        self.buffer_logs.write().await.push(BufLog {
            created_at: now,
            expires_at,
            level,
            module,
            action,
            custom_msg: custom.into(),
        });
    }

    pub async fn debug_mod(&self, module: SystemModuleEnum, msg: impl Into<String>) {
        let m = format!("[debug] {}", msg.into());
        self.log_now(
            LogLevelEnum::Info,
            module,
            LogActionEnum::DebugPipeMessage,
            Some(m),
        )
        .await;
    }

    /* ---------------- internal helpers -------------------- */

    fn ttl_for(level: &LogLevelEnum, now: NaiveDateTime) -> NaiveDateTime {
        match level {
            LogLevelEnum::Info => now + chrono::Duration::minutes(5),
            LogLevelEnum::Success => now + chrono::Duration::days(1),
            LogLevelEnum::Warning => now + chrono::Duration::days(3),
            LogLevelEnum::Error | LogLevelEnum::Fatal => now + chrono::Duration::days(7),
        }
    }

    /// Flush buffer → chosen sink
    pub async fn store_all_logs(&self) {
        let pending: Vec<BufLog> = {
            let mut guard = self.buffer_logs.write().await;
            if guard.is_empty() {
                return;
            }
            guard.drain(..).collect()
        };

        match self.sink_mode {
            LogSinkMode::Postgres => {
                if let Ok(mut conn) = load_db_connection().await {
                    let batch: Vec<NewLogEntry> = pending.iter().map(NewLogEntry::from).collect();
                    match LogRepository::insert_batch(&mut conn, &batch).await {
                        Ok(_) => return, // flushed to DB
                        Err(e) => {
                            eprintln!("Logger: DB flush failed: {e}. Falling back to NDJSON.")
                        }
                    }
                } else {
                    eprintln!("Logger: no DB connection; falling back to NDJSON.");
                }
                Self::write_ndjson_stdout(&pending).await;
                self.write_ndjson_file(&pending).await;
            }
            LogSinkMode::MemoryNdjson => {
                Self::write_ndjson_stdout(&pending).await;
                self.write_ndjson_file(&pending).await;
            }
        }
    }

    /// Cleanup expired rows (DB mode only)
    pub async fn try_clean(&self) {
        if !matches!(self.sink_mode, LogSinkMode::Postgres) {
            return;
        }
        if let Ok(mut conn) = load_db_connection().await {
            let _ = LogRepository::delete_expired(&mut conn).await;
        }
    }

    async fn write_ndjson_stdout(entries: &[BufLog]) {
        for e in entries {
            match serde_json::to_string(e) {
                Ok(line) => println!("LOG_NDJSON {}", line),
                Err(err) => eprintln!("Logger: NDJSON serialize error: {err}"),
            }
        }
    }

    async fn write_ndjson_file(&self, entries: &[BufLog]) {
        if let Some(file) = &self.file_sink {
            let mut f = file.lock().await;
            for e in entries {
                if let Ok(line) = serde_json::to_string(e) {
                    let _ = f.write_all(line.as_bytes()).await;
                    let _ = f.write_all(b"\n").await;
                }
            }
            let _ = f.flush().await;
        }
    }
}
