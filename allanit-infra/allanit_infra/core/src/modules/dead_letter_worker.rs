use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload};

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

const DEBUG: bool = true;

pub struct DeadLetterWorker;

impl DeadLetterWorker {
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

            // Wiring IN
            let mut ingestor_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let mut guard = wiring.lock().await;
                guard.take_rx(ChannelType::IngestorToDeadLetter)
            };
            let mut commandbus_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let mut guard = wiring.lock().await;
                guard.take_rx(ChannelType::CommandBusToDeadLetter)
            };

            // Startlogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::DeadLetterWorker,
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
                                logger.debug_mod(SystemModuleEnum::DeadLetterWorker, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::DeadLetterWorker, "CoreEvent::Restart").await;
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::DeadLetterWorker,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                // Dränera båda MPSC-ingångarna
                                if let Some(rx) = ingestor_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::DeadLetterWorker,
                                            format!("drain from Ingestor on shutdown: {:?}", msg)
                                        ).await;
                                    }
                                }
                                if let Some(rx) = commandbus_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::DeadLetterWorker,
                                            format!("drain from CommandBus on shutdown: {:?}", msg)
                                        ).await;
                                    }
                                }

                                println!("DeadLetterWorker | Shutdown complete.");

                                break;
                            }
                        }
                    }

                    // Puls
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::DeadLetterWorker,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;

                        if DEBUG {
                            // Lättvikts-självtest: verifiera att wiring finns
                            let _ = ingestor_rx.as_ref();
                            let _ = commandbus_rx.as_ref();
                        }
                    }

                    // MPSC: Ingestor -> DeadLetter
                    maybe_msg_ingestor = async {
                        if let Some(rx) = ingestor_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg_ingestor {
                            logger
                                .debug_mod(
                                    SystemModuleEnum::DeadLetterWorker,
                                    format!("received from Ingestor: {:?}", msg),
                                )
                                .await;

                            logger
                                .log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::DeadLetterWorker,
                                    LogActionEnum::DebugEventReceived,
                                    Some("ingestor -> deadletter".to_string()),
                                )
                                .await;

                            // TODO: persistera dead-letter, räkna retries, osv.
                        }
                    }

                    // MPSC: CommandBus -> DeadLetter
                    maybe_msg_cmd = async {
                        if let Some(rx) = commandbus_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg_cmd {
                            logger
                                .debug_mod(
                                    SystemModuleEnum::DeadLetterWorker,
                                    format!("received from CommandBus: {:?}", msg),
                                )
                                .await;

                            logger
                                .log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::DeadLetterWorker,
                                    LogActionEnum::DebugEventReceived,
                                    Some("commandbus -> deadletter".to_string()),
                                )
                                .await;

                            // TODO: persistera dead-letter, räkna retries, osv.
                        }
                    }
                }
            }

            // Avslutningslogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::DeadLetterWorker,
                    LogActionEnum::SystemShutdown,
                    Some("stopped".to_string()),
                )
                .await;
        })
    }
}
