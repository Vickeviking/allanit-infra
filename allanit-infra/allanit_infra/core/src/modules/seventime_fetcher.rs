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

pub struct SeventimeFetcher;

impl SeventimeFetcher {
    pub fn spawn(shared: Arc<SharedResources>) -> JoinHandle<()> {
        tokio::spawn(async move {
            // 1) Logger, pulses, broadcast
            let logger: Arc<Logger> = shared.get_logger();
            let pulses = shared.get_pulse_subscriptions();
            let mut pulse_rx = pulses.subscribe_medium(); // ~2s i din broadcaster

            let _core_rx = shared.get_service_channels().subscribe_to_core_event(); // om du vill lyssna på Startup/Shutdown

            // 2) Wiring: egen IN (oftast ingen för fetcher) och utgående TX till Ingestor
            let wiring = shared.get_service_wiring();
            let mut my_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let mut locked = wiring.lock().await;
                locked.take_rx(ChannelType::CommandBusToFetcher)
            };

            let to_ingestor_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let locked = wiring.lock().await; // read scope
                locked.get_tx(ChannelType::FetcherToIngestor).await
            };

            logger.info("SeventimeFetcher", "spawned");

            loop {
                select! {
                    // 2s-pulsen
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger.info("SeventimeFetcher", "I am active");

                        if DEBUG {
                            if let Some(tx) = to_ingestor_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från SeventimeFetcher")));
                            }
                        }
                    }

                    // valfri mpsc-ingång (kommando från CommandBus, t.ex. "synka nu")
                    maybe_msg = async {
                        if let Some(rx) = my_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            logger.debug("SeventimeFetcher", &format!("Received on wiring: {:?}", msg));
                            // här skulle du reagera på t.ex. ReprocessRequest eller on-demand sync
                        }
                    }
                }
            }
        })
    }
}
