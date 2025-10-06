use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload, NotificationEvent};

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

const DEBUG: bool = true;

pub struct CommandBus;

impl CommandBus {
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

            // Wiring
            let mut my_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let mut guard = wiring.lock().await;
                guard.take_rx(ChannelType::RocketToCommandBus)
            };

            let to_fetcher_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let guard = wiring.lock().await;
                guard.get_tx(ChannelType::CommandBusToFetcher)
            };

            let to_dead_letter_tx: Option<mpsc::UnboundedSender<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let guard = wiring.lock().await;
                guard.get_tx(ChannelType::CommandBusToDeadLetter)
            };

            // Startlogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::CommandBus,
                    LogActionEnum::SystemStartup,
                    Some("active".to_string()),
                )
                .await;

            // Huvudloop
            loop {
                select! {
                    // Lifecyle
                    Ok(ev) = core_rx.recv() => {
                        match ev {
                            CoreEvent::Startup => {
                                logger.debug_mod(SystemModuleEnum::CommandBus, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::CommandBus, "CoreEvent::Restart").await;
                                // Lägg ev. drain eller state reset här
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::CommandBus,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string())
                                ).await;

                                // Dränera kvarvarande MPSC-ingång icke-blockerande
                                if let Some(rx) = my_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger
                                            .debug_mod(
                                                SystemModuleEnum::CommandBus,
                                                format!("drain on shutdown: {:?}", msg),
                                            )
                                            .await;
                                    }
                                }


                                println!("CommandBuss | Shutdown complete.");

                                break;
                            }
                        }
                    }

                    // Pulsen
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::CommandBus,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;

                        if DEBUG {
                            if let Some(tx) = to_fetcher_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från CommandBus")
                                ));
                            }
                            if let Some(tx) = to_dead_letter_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från CommandBus")
                                ));
                            }
                        }
                    }

                    // MPSC-ingång: Rocket -> CommandBus
                    maybe_msg = async {
                        if let Some(rx) = my_rx.as_mut() {
                            rx.recv().await
                        } else {
                            None
                        }
                    } => {
                        if let Some(msg) = maybe_msg {
                            logger
                                .debug_mod(
                                    SystemModuleEnum::CommandBus,
                                    format!("received on wiring: {:?}", msg)
                                )
                                .await;

                            logger
                                .log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::CommandBus,
                                    LogActionEnum::DebugPipeMessage,
                                    Some("rocket -> commandbus".to_string()),
                                )
                                .await;

                            // Här placerar du faktiskt kommandohantering
                        }
                    }
                }
            }

            // Avslutningslogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::CommandBus,
                    LogActionEnum::SystemShutdown,
                    Some("stopped".to_string()),
                )
                .await;
        })
    }
}
