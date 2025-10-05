///! Spawns the core services, as of now pulse broadcaster
use std::sync::Arc;
use thiserror::Error;
use tokio::signal;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

use crate::core::shared_resources::SharedResources;
use crate::core::PulseBroadcaster;

#[derive(Error, Debug)]
pub enum InitError {
    #[error("pulse broadcaster already taken")]
    MissingBroadcaster,
    #[error("failed to listen for ctrl-c: {0}")]
    SignalError(#[from] std::io::Error),
}

/// Spawns the core servicec, as of now the pulse broadcaster and shutdown watcher
pub struct ServiceInitializer {
    pulse_broadcaster: Option<PulseBroadcaster>,
    /// a notifier for ctrl-c, that will emit up to main
    pub shutdown_notify: Arc<Notify>,
}

impl ServiceInitializer {
    pub async fn new(
        _shared_resources: Arc<SharedResources>,
        pulse_broadcaster: PulseBroadcaster,
    ) -> Self {
        ServiceInitializer {
            pulse_broadcaster: Some(pulse_broadcaster),
            shutdown_notify: Arc::new(Notify::new()),
        }
    }

    /// Spawns the service tasks.  
    /// Returns error if we’d already taken the broadcaster,
    /// or if listening for Ctrl-C failed.
    pub fn start(self) -> Result<(JoinHandle<()>, JoinHandle<()>), InitError> {
        // 1) Safely take the broadcaster
        let pb = self
            .pulse_broadcaster
            .ok_or(InitError::MissingBroadcaster)?;

        // 2) Spawn the pulse task
        let pulse_handle: JoinHandle<()> = tokio::spawn(async move {
            pb.start().await;
        });

        // 3) Spawn the Ctrl-C watcher
        let notify = self.shutdown_notify.clone();
        let signal_handle: JoinHandle<()> = tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(()) => {
                    println!("Ctrl-C detected; triggering shutdown");
                    notify.notify_one();
                }
                Err(e) => eprintln!("Warning: failed to listen for ctrl-c: {}", e),
            }
        });

        Ok((pulse_handle, signal_handle))
    }
}
