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

pub struct IngestorRoutes;

impl IngestorRoutes {
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
            let mut fetcher_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let mut locked = wiring.lock().await;
                locked.take_rx(ChannelType::FetcherToIngestor);
            };

            // Wiring: utgående TX till fetcher
            let to_fetcher_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let locked = wiring.lock().await; // read scope
                locked.get_tx(ChannelType::IngestorToDbWriter).await
            };

            // Wiring: utgående TX till deadletter
            let to_dead_letter_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let locked = wiring.lock().await; // read scope
                locked.get_tx(ChannelType::IngestorToDeadLetter).await
            };

            logger
                .info_mod(
                    SystemModuleEnum::IngestorRoutes,
                    LogActionEnum::Custom,
                    Some("I am active".to_string()),
                )
                .await;

            loop {
                select! {
                    // 2s-pulsen
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger.info_mod(
                            SystemModuleEnum::IngestorRoutes,
                            LogActionEnum::SystemStartup,
                            Some("spawned".to_string()),
                        ).await;

                        if DEBUG {
                            if let Some(tx) = to_fetcher_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från IngestorRoutes")));
                            }
                            if let Some(tx) = to_dead_letter_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från IngestorRoutes")));
                            }
                        }
                    }

                    // valfri mpsc-ingång (kommando från IngestorRoutes, t.ex. "synka nu")
                    maybe_msg = async {
                        if let Some(rx) = my_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            logger.debug("IngestorRoutes", &format!("Received on wiring: {:?}", msg));
                        }
                    }
                }
            }
        })
    }
}
