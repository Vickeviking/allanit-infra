use crate::core::shared_resources::SharedResources;
use crate::modules::{Logger, ModuleChannels, ModuleInitializer, ModuleWiring};
use crate::services::{PulseBroadcaster, ServiceInitializer};

use common::enums::system::CoreEvent;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
pub mod core;
pub mod modules;
pub mod services;
pub mod utils;

fn main() {
    let runtime = Runtime::new().unwrap();
    let res = runtime.block_on(async { tokio_async_runtime().await });
    // Pretty print anyhow errors
    if let Err(err) = res {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

async fn tokio_async_runtime() -> anyhow::Result<(), anyhow::Error> {
    // === Init underlying railway ===
    let module_channels = Arc::new(ModuleChannels::new());
    let module_wiring = Arc::new(Mutex::new(ModuleWiring::new()));
    let pulse_broadcaster = PulseBroadcaster::new(module_channels.subscribe_to_core_event());

    // === Logger and shared_resources ===
    let logger: Arc<Logger> = Arc::new(
        Logger::from_env(
            module_channels.subscribe_to_core_event(),
            Arc::new(pulse_broadcaster.subscriptions()),
        )
        .await,
    );
    tokio::spawn({
        let logger: Arc<Logger> = Arc::clone(&logger);
        async move { logger.init().await }
    });

    let shared_resources = Arc::new(SharedResources::new(
        Arc::clone(&logger),
        Arc::new(pulse_broadcaster.subscriptions()),
        Arc::clone(&module_channels),
        module_wiring,
    ));

    // ===== Create and start the services ====
    let initializer =
        ServiceInitializer::new(Arc::clone(&shared_resources), pulse_broadcaster).await;
    let shutdown_notify = initializer.shutdown_notify.clone();
    let _ = initializer.start()?;
    // ==== create and start modules ====
    let module_handles = ModuleInitializer::new(Arc::clone(&shared_resources)).await;

    // Send startup signal to all services
    module_channels
        .send_event_to_all_modules(CoreEvent::Startup)
        .await;
    println!("System started. Awaiting commands...");

    // Loop to handle shutdown events
    shutdown_notify.notified().await;
    println!("Notify‐based shutdown triggered.");
    module_channels
        .send_event_to_all_modules(CoreEvent::Shutdown)
        .await;
    module_handles.join_tasks().await;
    println!("Shutdown complete.");

    Ok(())
}
