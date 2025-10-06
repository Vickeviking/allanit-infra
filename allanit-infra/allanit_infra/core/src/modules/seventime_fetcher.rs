use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload, NotificationEvent};

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

const DEBUG: bool = true;

pub struct SeventimeFetcher;

impl SeventimeFetcher {
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

            // Wiring IN: CommandBus -> Fetcher
            let mut my_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let mut guard = wiring.lock().await;
                guard.take_rx(ChannelType::CommandBusToFetcher)
            };

            // Wiring OUT: Fetcher -> Ingestor
            let to_ingestor_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let guard = wiring.lock().await;
                guard.get_tx(ChannelType::FetcherToIngestor)
            };

            // Startlogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::SeventimeFetcher,
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
                                logger.debug_mod(SystemModuleEnum::SeventimeFetcher, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::SeventimeFetcher, "CoreEvent::Restart").await;
                                // Nollställ ev. intern state här
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::SeventimeFetcher,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                // Dränera inkommande kommandon
                                if let Some(rx) = my_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::SeventimeFetcher,
                                            format!("drain on shutdown: {:?}", msg)
                                        ).await;
                                    }
                                }

                                println!("Logger | Shutdown complete.");

                                break;
                            }
                        }
                    }

                    // Puls
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::SeventimeFetcher,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;

                        if DEBUG {
                            if let Some(tx) = to_ingestor_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från SeventimeFetcher")
                                ));
                            }
                        }
                    }

                    // MPSC: CommandBus -> Fetcher
                    maybe_msg = async {
                        if let Some(rx) = my_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            logger
                                .debug_mod(
                                    SystemModuleEnum::SeventimeFetcher,
                                    format!("received on wiring: {:?}", msg),
                                )
                                .await;

                            logger
                                .log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::SeventimeFetcher,
                                    LogActionEnum::MessageReceived,
                                    Some("commandbus -> fetcher".to_string()),
                                )
                                .await;

                            // TODO: trigga faktisk hämtning mot Seventime och skicka resultatet vidare
                            // if let Some(tx) = to_ingestor_tx.as_ref() { let _ = tx.send(transformed_payload); }
                        }
                    }
                }
            }

            // Avslutningslogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::SeventimeFetcher,
                    LogActionEnum::SystemShutdown,
                    Some("stopped".to_string()),
                )
                .await;
        })
    }
}
