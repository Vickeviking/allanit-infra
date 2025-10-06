use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload, NotificationEvent};

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

const DEBUG: bool = true;

pub struct IngestorRouter;

impl IngestorRouter {
    pub fn spawn(shared: Arc<SharedResources>) -> JoinHandle<()> {
        tokio::spawn(async move {
            // Logger och pulser
            let logger: Arc<Logger> = shared.get_logger();
            let pulses = shared.get_pulse_subscriptions();
            let mut pulse_rx = pulses.subscribe_medium();

            // Core events
            let mut core_rx: broadcast::Receiver<CoreEvent> = {
                let chans = shared.get_module_channels();
                chans.subscribe_to_core_event()
            };

            // Wiring IN: Fetcher -> Ingestor
            let mut fetcher_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let mut guard = wiring.lock().await;
                guard.take_rx(ChannelType::FetcherToIngestor)
            };

            // Wiring OUT: Ingestor -> DBWriter
            let to_dbwriter_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let guard = wiring.lock().await;
                guard.get_tx(ChannelType::IngestorToDbWriter)
            };

            // Wiring OUT: Ingestor -> DeadLetter
            let to_deadletter_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let guard = wiring.lock().await;
                guard.get_tx(ChannelType::IngestorToDeadLetter)
            };

            // Startlogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::IngestorRouter,
                    LogActionEnum::SystemStartup,
                    Some("active".to_string()),
                )
                .await;

            // Huvudloop
            loop {
                select! {
                    // Livscykel
                    Ok(ev) = core_rx.recv() => {
                        match ev {
                            CoreEvent::Startup => {
                                logger.debug_mod(SystemModuleEnum::IngestorRouter, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::IngestorRouter, "CoreEvent::Restart").await;
                                // Nollställ temporär state här om behövs
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::IngestorRouter,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                // Dränera inkommande kö
                                if let Some(rx) = fetcher_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::IngestorRouter,
                                            format!("drain from Fetcher on shutdown: {:?}", msg)
                                        ).await;

                                        // Valfritt: skicka allt kvar till DeadLetter
                                        if let Some(tx) = to_deadletter_tx.as_ref() {
                                            let _ = tx.send(msg);
                                        }
                                    }
                                }

                                println!("IngestorRouter | Shutdown complete.");

                                break;
                            }
                        }
                    }

                    // Puls
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::IngestorRouter,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;

                        if DEBUG {
                            if let Some(tx) = to_dbwriter_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från IngestorRouter till DBWriter")
                                ));
                            }
                            if let Some(tx) = to_deadletter_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från IngestorRouter till DeadLetter")
                                ));
                            }
                        }
                    }

                    // MPSC-ingång: Fetcher -> Ingestor
                    maybe_msg = async {
                        if let Some(rx) = fetcher_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            logger
                                .debug_mod(
                                    SystemModuleEnum::IngestorRouter,
                                    format!("received on wiring: {:?}", msg),
                                )
                                .await;

                            // TODO: matcha på msg och routa
                            // Exempel:
                            // match msg {
                            //     EventPayload::QueryOk(q) => if let Some(tx) = to_dbwriter_tx.as_ref() { let _ = tx.send(EventPayload::from(q)); }
                            //     EventPayload::QueryErr(e) => if let Some(tx) = to_deadletter_tx.as_ref() { let _ = tx.send(EventPayload::from(e)); }
                            //     _ => {}
                            // }

                            logger
                                .log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::IngestorRouter,
                                    LogActionEnum::MessageReceived,
                                    Some("fetcher -> ingestor".to_string()),
                                )
                                .await;
                        }
                    }
                }
            }

            // Avslutningslogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::IngestorRouter,
                    LogActionEnum::SystemShutdown,
                    Some("stopped".to_string()),
                )
                .await;
        })
    }
}
