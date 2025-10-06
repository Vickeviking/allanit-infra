///! Spawns the core services, as of now pulse broadcaster
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

use crate::core::shared_resources::SharedResources;
use crate::services::PulseBroadcaster;

#[derive(Error, Debug)]
pub enum InitError {
    #[error("pulse broadcaster already taken")]
    MissingBroadcaster,
}

/// Spawns the core services and exposes a Notify used to coordinate shutdown.
pub struct ServiceInitializer {
    pulse_broadcaster: Option<PulseBroadcaster>,
    /// external notifier that main awaits and modules can react to
    pub shutdown_notify: Arc<Notify>,
}

impl ServiceInitializer {
    /// Create a new initializer.
    /// `shared_resources` is accepted for future extension, prefix with `_` to avoid warnings.
    pub async fn new(
        _shared_resources: Arc<SharedResources>,
        pulse_broadcaster: PulseBroadcaster,
    ) -> Self {
        Self {
            pulse_broadcaster: Some(pulse_broadcaster),
            shutdown_notify: Arc::new(Notify::new()),
        }
    }

    /// Spawn the pulse task and the signal watcher.
    /// Returns: (pulse_handle, signal_handle)
    pub fn start(self) -> Result<(JoinHandle<()>, JoinHandle<()>), InitError> {
        // 1) Take the broadcaster
        let pb = self
            .pulse_broadcaster
            .ok_or(InitError::MissingBroadcaster)?;

        // 2) Pulse task
        let pulse_handle: JoinHandle<()> = tokio::spawn(async move {
            pb.start().await;
        });

        // 3) Signal watcher (SIGTERM or SIGINT)
        let notify = Arc::clone(&self.shutdown_notify);
        let signal_handle: JoinHandle<()> = tokio::spawn(async move {
            wait_for_shutdown_signal().await;
            println!("Shutdown signal detected; triggering notify");
            notify.notify_one();
        });

        Ok((pulse_handle, signal_handle))
    }
}

/// Waits for SIGTERM or SIGINT on Unix, or Ctrl-C on other platforms.
#[cfg(unix)]
async fn wait_for_shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM handler");
    let mut sigint = signal(SignalKind::interrupt()).expect("install SIGINT handler");

    tokio::select! {
        _ = sigterm.recv() => eprintln!("Received SIGTERM"),
        _ = sigint.recv()  => eprintln!("Received SIGINT"),
    }
}

#[cfg(not(unix))]
async fn wait_for_shutdown_signal() {
    use tokio::signal;
    signal::ctrl_c()
        .await
        .expect("install Ctrl-C handler on non-unix");
    eprintln!("Received Ctrl-C");
}
