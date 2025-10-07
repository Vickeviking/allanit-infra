//! SharedResources
//! ---------------
//! Purpose: Central handle bundle passed to every module.
//!
//! Contains
//! - Logger
//! - PulseSubscriptions
//! - ModuleChannels (broadcast)
//! - ModuleWiring (mpsc pipes)
//! - DB pool (bb8 + diesel_async)
//!
//! Notes
//! - Use `db_pool().get().await` for runtime modules (preferred).
//! - `db_conn().await` wraps your existing one-off constructor for ad hoc use.

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::modules::{Logger, ModuleChannels, ModuleWiring};
use crate::services::PulseSubscriptions;

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;

pub struct SharedResources {
    logger: Arc<Logger>,
    pulse_subscriptions: Arc<PulseSubscriptions>,
    module_channels: Arc<ModuleChannels>,
    module_wiring: Arc<Mutex<ModuleWiring>>,
    db_pool: Pool<AsyncPgConnection>,
}

impl SharedResources {
    pub fn new(
        logger: Arc<Logger>,
        pulse_subscriptions: Arc<PulseSubscriptions>,
        module_channels: Arc<ModuleChannels>,
        module_wiring: Arc<Mutex<ModuleWiring>>,
        db_pool: Pool<AsyncPgConnection>,
    ) -> Self {
        Self {
            logger,
            pulse_subscriptions,
            module_channels,
            module_wiring,
            db_pool,
        }
    }

    pub fn get_logger(&self) -> Arc<Logger> {
        Arc::clone(&self.logger)
    }
    pub fn get_pulse_subscriptions(&self) -> Arc<PulseSubscriptions> {
        Arc::clone(&self.pulse_subscriptions)
    }
    pub fn get_module_channels(&self) -> Arc<ModuleChannels> {
        Arc::clone(&self.module_channels)
    }
    pub fn get_module_wiring(&self) -> Arc<Mutex<ModuleWiring>> {
        Arc::clone(&self.module_wiring)
    }

    pub fn db_pool(&self) -> Pool<AsyncPgConnection> {
        self.db_pool.clone()
    }

    // Keep this if you also support an ad hoc non pooled connection path
    pub async fn db_conn(&self) -> anyhow::Result<AsyncPgConnection> {
        common::commands::load_db_connection().await
    }
}
