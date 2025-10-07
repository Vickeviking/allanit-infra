//! core/bootstrap.rs
//! -----------------
//! Purpose: Wire up the runtime: channels, wiring, pulse broadcaster, logger,
//! db pool, SharedResources, services, and modules.

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::shared_resources::SharedResources;
use crate::modules::{Logger, ModuleChannels, ModuleInitializer, ModuleWiring};
use crate::services::{PulseBroadcaster, ServiceInitializer};

use common::enums::system::CoreEvent;

// Diesel async + bb8
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub struct Bootstrap {
    pub shared: Arc<SharedResources>,
    pub module_channels: Arc<ModuleChannels>,
    pub service_initializer: ServiceInitializer,
    pub module_initializer: ModuleInitializer,
}

async fn build_db_pool() -> anyhow::Result<Pool<AsyncPgConnection>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    // bb8 style builder
    let pool: Pool<AsyncPgConnection> = Pool::builder().max_size(10).build(manager).await?;
    Ok(pool)
}

pub async fn bootstrap() -> anyhow::Result<Bootstrap> {
    // Channels and wiring
    let module_channels = Arc::new(ModuleChannels::new());
    let module_wiring = Arc::new(Mutex::new(ModuleWiring::new()));
    let pulse_broadcaster = PulseBroadcaster::new(module_channels.subscribe_to_core_event());

    // Logger
    let logger: Arc<Logger> = Arc::new(
        Logger::from_env(
            module_channels.subscribe_to_core_event(),
            Arc::new(pulse_broadcaster.subscriptions()),
        )
        .await,
    );
    tokio::spawn({
        let logger = Arc::clone(&logger);
        async move { logger.init().await }
    });

    // DB pool
    let db_pool = build_db_pool().await?;

    // Shared resources
    let shared = Arc::new(SharedResources::new(
        Arc::clone(&logger),
        Arc::new(pulse_broadcaster.subscriptions()),
        Arc::clone(&module_channels),
        module_wiring,
        db_pool,
    ));

    // Services
    let service_initializer = ServiceInitializer::new(Arc::clone(&shared), pulse_broadcaster).await;

    // Modules
    let module_initializer = ModuleInitializer::new(Arc::clone(&shared)).await;

    Ok(Bootstrap {
        shared,
        module_channels,
        service_initializer,
        module_initializer,
    })
}

pub async fn start_system(boot: Bootstrap) -> anyhow::Result<()> {
    let shutdown_notify = boot.service_initializer.shutdown_notify.clone();
    boot.service_initializer.start()?;

    let module_handles = boot.module_initializer;

    boot.module_channels
        .send_event_to_all_modules(CoreEvent::Startup)
        .await;

    println!("System started. Awaiting commands...");

    shutdown_notify.notified().await;
    println!("Notify-based shutdown triggered.");

    boot.module_channels
        .send_event_to_all_modules(CoreEvent::Shutdown)
        .await;
    module_handles.join_tasks().await;

    println!("Shutdown complete.");
    Ok(())
}
