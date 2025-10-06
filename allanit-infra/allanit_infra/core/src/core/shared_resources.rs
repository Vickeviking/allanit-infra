///! Shared resources between modules,
///! Logger - used by all modules, to log internal events, no mutex needed, have internal mutexes
///! Pulse subscriptions - used by all modules to subscribe to pulses, no mutex needed
///! Module channels - broadcast channels for one-to-many communication, subscribe to core event
///! Module wiring - one-to-one communication channels between modules,
///!                  each module take() its channel sides upon initialization
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::modules::{Logger, ModuleChannels, ModuleWiring};
use crate::services::PulseSubscriptions;

/// All systemwide shared resources
pub struct SharedResources {
    pub logger: Arc<Logger>,
    pub pulse_subscriptions: Arc<PulseSubscriptions>,
    pub module_channels: Arc<ModuleChannels>, //only has ref ones so only Arc
    pub module_wiring: Arc<Mutex<ModuleWiring>>, // builds on take() so needs write = MutexLock
}

impl SharedResources {
    pub fn new(
        logger: Arc<Logger>,
        pulse_subscriptions: Arc<PulseSubscriptions>,
        module_channels: Arc<ModuleChannels>,
        module_wiring: Arc<Mutex<ModuleWiring>>,
    ) -> Self {
        SharedResources {
            logger,
            pulse_subscriptions,
            module_channels,
            module_wiring,
        }
    }

    // getters
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
        //we dont lock, they lock we minimize cruitical section
        Arc::clone(&self.module_wiring)
    }
}
