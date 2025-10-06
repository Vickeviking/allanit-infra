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

pub struct DeadLetterWorker;

impl DeadLetterWorker {
    pub fn spawn(shared: Arc<SharedResources>) -> JoinHandle<()> {
        tokio::spawn(async move {
            // Logger, pulses, broadcast
            let logger: Arc<Logger> = shared.get_logger();
            let pulses = shared.get_pulse_subscriptions();
            let mut pulse_rx = pulses.subscribe_medium(); // ~2s i din broadcaster

            let _core_rx = shared.get_service_channels().subscribe_to_core_event(); // om du vill lyssna på Startup/Shutdown

            // ========== Wiring ===========
            // Wiring: egen IN
            let wiring = shared.get_service_wiring();
            let mut ingestor_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let mut locked = wiring.lock().await;
                locked.take_rx(ChannelType::IngestorToDeadLetter);
            };
            let mut commandbus_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let mut locked = wiring.lock().await;
                locked.take_rx(ChannelType::CommandBusToDeadLetter);
            };

            logger
                .info_mod(
                    SystemModuleEnum::DeadLetterWorker,
                    LogActionEnum::Custom,
                    Some("I am active".to_string()),
                )
                .await;

            loop {
                select! {
                    // 2s-pulsen
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger.info_mod(
                            SystemModuleEnum::DeadLetterWorker,
                            LogActionEnum::SystemStartup,
                            Some("spawned".to_string()),
                        ).await;
                    }

                    // valfri mpsc-ingång (kommando från DeadLetterWorker, t.ex. "synka nu")
                    maybe_msg = async {
                        if let Some(rx) = ingestor_rx.as_mut() { rx.recv().await } else { None }
                        if let Some(rx) = commandbus_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            logger.debug("DeadLetterWorker", &format!("Received on wiring: {:?}", msg));
                        }
                    }
                }
            }
        })
    }
}
