use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{
    ChannelType, EventPayload, ModuleChannels, ModuleWiring, NotificationEvent,
};
use common::enums::system::{CoreEvent, Pulse};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::{select, task::JoinHandle}; // antagande: finns

const DEBUG: bool = true; // sätt per modul

/// Commandbus
///
///
/// listens to rocket
/// Talks to, fetcher and deadletter module

pub struct HealthAndMetrics;

impl HealthAndMetrics {
    pub fn spawn(shared: Arc<SharedResources>) -> JoinHandle<()> {
        tokio::spawn(async move {
            // Logger, pulses, broadcast
            let logger: Arc<Logger> = shared.get_logger();
            let pulses = shared.get_pulse_subscriptions();
            let mut pulse_rx = pulses.subscribe_medium(); // ~2s i din broadcaster

            let _core_rx = shared.get_service_channels().subscribe_to_core_event(); // om du vill lyssna på Startup/Shutdown

            logger
                .info_mod(
                    SystemModuleEnum::HealthAndMetrics,
                    LogActionEnum::Custom,
                    Some("I am active".to_string()),
                )
                .await;

            loop {
                select! {
                    // 2s-pulsen
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger.info_mod(
                            SystemModuleEnum::HealthAndMetrics,
                            LogActionEnum::SystemStartup,
                            Some("active".to_string()),
                        ).await;
                    }

                }
            }
        })
    }
}
