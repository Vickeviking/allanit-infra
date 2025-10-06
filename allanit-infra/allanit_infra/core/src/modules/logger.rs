use std::{path::PathBuf, sync::Arc};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl}; // async execute/delete
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    select,
    sync::{broadcast::Receiver, Mutex, RwLock},
};

use crate::core::PulseSubscriptions;
use common::commands::load_db_connection;
use common::database::models::log::{
    ClientConnectedPayload, JobCompletedPayload, JobSubmittedPayload, LogEntry, NewDBLogEntry,
};
use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

/// Central logger accessed by all modules
pub struct Logger {
    /// in-memory log buffer, flushed on pulse/shutdown
    buffer_logs: RwLock<Vec<LogEntry>>,
    /// Life cycle events, controls Logger behavior
    core_event_rx: Mutex<Receiver<CoreEvent>>,
    /// Slow pulse every ~2s
    pulse_rx: Mutex<Receiver<Pulse>>,
    /// Optional NDJSON file sink (fallback when DB is unavailable)
    file_sink: Option<Mutex<tokio::fs::File>>,
}

impl Logger {
    /* ---------------- construction + background loop -------------------- */
    /// Init logger, only one instance ever running
    /// NOTE: prefer constructing via `Logger::from_env(...)` to auto-enable file sink
    pub fn new(core_rx: Receiver<CoreEvent>, pulse_subs: Arc<PulseSubscriptions>) -> Self {
        Self {
            buffer_logs: RwLock::new(Vec::new()),
            core_event_rx: Mutex::new(core_rx),
            pulse_rx: Mutex::new(pulse_subs.subscribe_slow()),
            file_sink: None,
        }
    }

    /// Builder that enables NDJSON file fallback if `path` is Some.
    pub async fn with_file_sink(mut self, path: Option<PathBuf>) -> Self {
        if let Some(p) = path {
            if let Ok(f) = OpenOptions::new().create(true).append(true).open(p).await {
                self.file_sink = Some(Mutex::new(f));
            }
        }
        self
    }

    /// Convenience: builds logger and reads `LOG_FILE` from env to enable file sink
    pub async fn from_env(
        core_rx: Receiver<CoreEvent>,
        pulse_subs: Arc<PulseSubscriptions>,
    ) -> Self {
        let path = std::env::var_os("LOG_FILE").map(PathBuf::from);
        Logger::new(core_rx, pulse_subs).with_file_sink(path).await
    }

    /// Spawn this on Tokio runtime: `tokio::spawn(logger.clone().init())`.
    pub async fn init(self: Arc<Self>) {
        let mut core_rx = self.core_event_rx.lock().await;
        let mut pulse_rx = self.pulse_rx.lock().await;
        loop {
            select! {
                Ok(ev) = core_rx.recv() => match ev {
                    CoreEvent::Startup => {},
                    CoreEvent::Restart => { self.store_all_logs().await; },
                    CoreEvent::Shutdown => { self.store_all_logs().await; break; },
                },
                Ok(pulse) = pulse_rx.recv() => {
                    // On pulse we do 2 things
                    // 1. flush buffer to DB (or NDJSON fallback)
                    // 2. delete expired rows (best-effort)
                    if matches!(pulse, Pulse::Slow) {
                        self.try_clean().await;
                        self.store_all_logs().await;
                    }
                }
            }
        }
    }

    /* ---------------- public API (ergonomic helpers) -------------------- */
    /// Core entry-point (buffer append). Prefer the convenience wrappers below.
    #[allow(clippy::too_many_arguments)]
    pub async fn log(
        logger: Arc<Self>,
        level: LogLevelEnum,
        module: SystemModuleEnum,
        action: LogActionEnum,
        custom: Option<String>,
    ) {
        let now = Utc::now().naive_utc();
        let expires_at = match level {
            LogLevelEnum::Info => now + chrono::Duration::minutes(5),
            LogLevelEnum::Success => now + chrono::Duration::days(1),
            LogLevelEnum::Warning => now + chrono::Duration::days(3),
            LogLevelEnum::Error | LogLevelEnum::Fatal => now + chrono::Duration::days(7),
        };
        logger.buffer_logs.write().await.push(LogEntry {
            id: 0,
            created_at: now,
            level,
            module,
            action,
            expires_at,
            custom_msg: custom,
        });
    }

    /// Small, ergonomic wrapper around `log` for direct `&self` usage.
    pub async fn log_now(
        &self,
        level: LogLevelEnum,
        module: SystemModuleEnum,
        action: LogActionEnum,
        custom: impl Into<Option<String>>,
    ) {
        Self::log(
            Arc::new(self.clone_self()),
            level,
            module,
            action,
            custom.into(),
        )
        .await;
    }

    /// Convenience: Info
    pub async fn info_mod(
        &self,
        module: SystemModuleEnum,
        action: LogActionEnum,
        msg: impl Into<Option<String>>,
    ) {
        self.log_now(LogLevelEnum::Info, module, action, msg).await;
    }

    /// Convenience: Success
    pub async fn success_mod(
        &self,
        module: SystemModuleEnum,
        action: LogActionEnum,
        msg: impl Into<Option<String>>,
    ) {
        self.log_now(LogLevelEnum::Success, module, action, msg)
            .await;
    }

    /// Convenience: Warning
    pub async fn warn_mod(
        &self,
        module: SystemModuleEnum,
        action: LogActionEnum,
        msg: impl Into<Option<String>>,
    ) {
        self.log_now(LogLevelEnum::Warning, module, action, msg)
            .await;
    }

    /// Convenience: Error
    pub async fn error_mod(
        &self,
        module: SystemModuleEnum,
        action: LogActionEnum,
        msg: impl Into<Option<String>>,
    ) {
        self.log_now(LogLevelEnum::Error, module, action, msg).await;
    }

    /// Convenience: Fatal
    pub async fn fatal_mod(
        &self,
        module: SystemModuleEnum,
        action: LogActionEnum,
        msg: impl Into<Option<String>>,
    ) {
        self.log_now(LogLevelEnum::Fatal, module, action, msg).await;
    }

    /// Debug helper. Maps to Info TTL but tags message.
    pub async fn debug_mod(&self, module: SystemModuleEnum, msg: impl Into<String>) {
        let m = format!("[debug] {}", msg.into());
        self.log_now(LogLevelEnum::Info, module, LogActionEnum::Custom, Some(m))
            .await;
    }

    /// Tiny utility so the `&self` helpers can call `log` without requiring callers to hold an Arc.
    fn clone_self(&self) -> Self {
        Self {
            buffer_logs: RwLock::new(Vec::new()), // not used; `log` does not read these fields from this clone
            core_event_rx: Mutex::new(self.core_event_rx.blocking_lock().clone()),
            pulse_rx: Mutex::new(self.pulse_rx.blocking_lock().clone()),
            file_sink: self.file_sink.as_ref().map(|_| None).flatten(), // file sink not needed for helpers
        }
    }

    /* ---------------- internal helpers --------------------------------- */

    /// Flush buffer → DB (best‑effort). Fallback: NDJSON to stdout and optional file.
    pub async fn store_all_logs(&self) {
        let mut pending: Vec<LogEntry> = {
            let mut guard = self.buffer_logs.write().await;
            guard.drain(..).collect()
        };
        if pending.is_empty() {
            return;
        }
        println!("Logger: flushing {} entries to DB", pending.len());

        match Self::insert_batch(&mut pending).await {
            Ok(_) => println!("Logger: flush OK"),
            Err(e) => {
                eprintln!("Logger: DB flush failed: {e}. Writing NDJSON fallback.");
                Self::write_ndjson_stdout(&pending).await;
                self.write_ndjson_file(&pending).await;
                // Do NOT push back into buffer: NDJSON is considered persisted for now.
            }
        }
    }

    /// Delete expired rows from DB on slow pulse (best effort). Ignores errors if DB is down.
    pub async fn try_clean(&self) {
        let mut conn = match load_db_connection().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Logger: cleanup DB connect error: {e}");
                return;
            }
        };
        let now: NaiveDateTime = Utc::now().naive_utc();
        use common::database::schema::logs::dsl as l;
        if let Err(e) = diesel::delete(l::logs.filter(l::expires_at.lt(now)))
            .execute(&mut conn)
            .await
        {
            eprintln!("Logger: cleanup delete error: {e}");
        }
    }

    /// Insert a batch of log entries to the DB.
    async fn insert_batch(entries: &[LogEntry]) -> anyhow::Result<()> {
        let mut conn: AsyncPgConnection = load_db_connection().await?;
        for e in entries {
            let new_row: NewDBLogEntry = e.into();
            let _ = LogEntryRepository::create(&mut conn, new_row).await; // ignore per-row errors
        }
        Ok(())
    }

    /// Write NDJSON lines to stdout so they show in `docker compose logs`.
    async fn write_ndjson_stdout(entries: &[LogEntry]) {
        for e in entries {
            match serde_json::to_string(e) {
                Ok(line) => println!("LOG_NDJSON {}", line),
                Err(err) => eprintln!("Logger: NDJSON serialize error: {err}"),
            }
        }
    }

    /// Append NDJSON lines to the optional file sink, if configured.
    async fn write_ndjson_file(&self, entries: &[LogEntry]) {
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
