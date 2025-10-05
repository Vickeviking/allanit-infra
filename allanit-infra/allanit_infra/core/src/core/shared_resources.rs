///! Shared resources between modules,
///! Logger - used by all modules, to log internal events, no mutex needed, have internal mutexes
///! Pulse subscriptions - used by all modules to subscribe to pulses, no mutex needed
///! Service channels - broadcast channels for one-to-many communication, subscribe to core event
///! Service wiring - one-to-one communication channels between modules,
///!                  each module take() its channel sides upon initialization
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::PulseSubscriptions;
use crate::services::{ServiceChannels, ServiceWiring};

/// All systemwide shared resources
pub struct SharedResources {
    pub logger: Arc<Logger>,
    pub pulse_subscriptions: Arc<PulseSubscriptions>,
    pub service_channels: Arc<ServiceChannels>, //only has ref ones so only Arc
    pub service_wiring: Arc<Mutex<ServiceWiring>>, // builds on take() so needs write = MutexLock
}

impl SharedResources {
    pub fn new(
        logger: Arc<Logger>,
        pulse_subscriptions: Arc<PulseSubscriptions>,
        service_channels: Arc<ServiceChannels>,
        service_wiring: Arc<Mutex<ServiceWiring>>,
    ) -> Self {
        SharedResources {
            logger,
            pulse_subscriptions,
            service_channels,
            service_wiring,
        }
    }

    // getters
    pub fn get_logger(&self) -> Arc<Logger> {
        Arc::clone(&self.logger)
    }

    pub fn get_pulse_subscriptions(&self) -> Arc<PulseSubscriptions> {
        Arc::clone(&self.pulse_subscriptions)
    }

    pub fn get_service_channels(&self) -> Arc<ServiceChannels> {
        Arc::clone(&self.service_channels)
    }

    pub fn get_service_wiring(&self) -> Arc<Mutex<ServiceWiring>> {
        //we dont lock, they lock we minimize cruitical section
        Arc::clone(&self.service_wiring)
    }
}
