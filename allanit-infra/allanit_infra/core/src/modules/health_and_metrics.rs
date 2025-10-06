use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::{select, task::JoinHandle};

const DEBUG: bool = true;

pub struct HealthAndMetrics;

impl HealthAndMetrics {
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

            // Startlogg
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::HealthAndMetrics,
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
                                logger.debug_mod(SystemModuleEnum::HealthAndMetrics, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::HealthAndMetrics, "CoreEvent::Restart").await;
                                // Nollställ ev. interna counters här
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::HealthAndMetrics,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                println!("HealthAndMetrics | Shutdown complete.");

                                break;
                            }
                        }
                    }

                    // Puls
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        // Heartbeat
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::HealthAndMetrics,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;

                        if DEBUG {
                            // Enkla mätpunkter kan samlas här
                            // Exempel: metrics snapshot ok
                            logger
                                .log_now(
                                    LogLevelEnum::Success,
                                    SystemModuleEnum::HealthAndMetrics,
                                    LogActionEnum::DebugPipeMessage,
                                    Some("metrics snapshot ok".to_string()),
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
                    SystemModuleEnum::HealthAndMetrics,
                    LogActionEnum::SystemShutdown,
                    Some("stopped".to_string()),
                )
                .await;
        })
    }
}
