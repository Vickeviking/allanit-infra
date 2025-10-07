//! IngestorRouter
//! ---------------
//! Purpose: First-line validation and routing for domain payloads emitted by the
//! SeventimeFetcher. It keeps DB-specific policy out of the fetcher and funnels
//! only clean, deduplicated batches forward.
//!
//! Responsibilities
//! - Subscribe to lifecycle (CoreEvent) and Pulse::Medium.
//! - Receive batches from `ChannelType::FetcherToIngestor`.
//! - Validate minimal shape (e.g. non-empty `external_id`).
//! - Deduplicate within a batch by `external_id`.
//! - Route valid batches to `ChannelType::IngestorToDbWriter`.
//! - Route trivial problems (like empty batches) to DeadLetter as *notifications*
//!   for now (can be upgraded to a richer DeadLetter payload later).
//!
//! Non-responsibilities
//! - No DB writes or transactions (DbWriter does that).
//! - No cross-batch or historical dedup (MVP keeps this in-batch).
//! - No relation resolution (e.g., mapping to `customer_id`). That can be added
//!   here or done in DbWriter once the policy is clear.
//!
//! Inputs / Outputs
//! - In:  Pipe `FetcherToIngestor` (EventPayload::{UpsertCustomers, UpsertPurchaseOrders})
//! - Out: Pipe `IngestorToDbWriter` (forwarded payloads)
//! - Out: Pipe `IngestorToDeadLetter` (NotificationEvent for now)
//!
//! Configuration
//! - None. Pure in-memory routing.
//!
//!
//! ***OBS***
//! In the future we can deduct "bad" fetches here, block some and add additional logic, where we
//! got data but perhaps not what we wanted
//!
//!

use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload, NotificationEvent};

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

const DEBUG: bool = true;

pub struct IngestorRouter;

impl IngestorRouter {
    pub fn spawn(shared: Arc<SharedResources>) -> JoinHandle<()> {
        tokio::spawn(async move {
            // Logger & pulses
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

            // Startup log
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::IngestorRouter,
                    LogActionEnum::SystemStartup,
                    Some("active".to_string()),
                )
                .await;

            // Main loop
            loop {
                select! {
                    // Lifecycle
                    Ok(ev) = core_rx.recv() => {
                        match ev {
                            CoreEvent::Startup => {
                                logger.debug_mod(SystemModuleEnum::IngestorRouter, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::IngestorRouter, "CoreEvent::Restart").await;
                                // reset transient state here if needed later
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::IngestorRouter,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                // Drain incoming queue
                                if let Some(rx) = fetcher_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::IngestorRouter,
                                            format!("drain from Fetcher on shutdown: {:?}", msg)
                                        ).await;

                                        // For now, notify DeadLetter as info (no structured DL payload yet)
                                        if let Some(tx) = to_deadletter_tx.as_ref() {
                                            let _ = tx.send(EventPayload::NotificationEvent(
                                                NotificationEvent::new("IngestorRouter drained message on shutdown")
                                            ));
                                        }
                                    }
                                }

                                println!("IngestorRouter | Shutdown complete.");
                                break;
                            }
                        }
                    }

                    // Pulse
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

                    // MPSC: Fetcher -> Ingestor
                    maybe_msg = async {
                        if let Some(rx) = fetcher_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            match msg {
                                EventPayload::UpsertCustomers(mut xs) => {
                                    let before = xs.len();

                                    // minimal shape validation
                                    xs.retain(|c| !c.external_id.trim().is_empty());

                                    // in-batch dedup by external_id
                                    let mut seen: HashSet<String> = HashSet::with_capacity(xs.len());
                                    xs.retain(|c| seen.insert(c.external_id.clone()));

                                    logger.debug_mod(
                                        SystemModuleEnum::IngestorRouter,
                                        format!("customers before {} after {}", before, xs.len())
                                    ).await;

                                    if xs.is_empty() {
                                        if let Some(tx) = to_deadletter_tx.as_ref() {
                                            let _ = tx.send(EventPayload::NotificationEvent(
                                                NotificationEvent::new("IngestorRouter: empty customer batch after validation/dedup")
                                            ));
                                        }
                                        continue;
                                    }

                                    if let Some(tx) = to_dbwriter_tx.as_ref() {
                                        let _ = tx.send(EventPayload::UpsertCustomers(xs));
                                    }
                                }

                                EventPayload::UpsertPurchaseOrders(mut xs) => {
                                    let before = xs.len();

                                    // minimal shape validation
                                    xs.retain(|o| !o.external_id.trim().is_empty());

                                    // in-batch dedup by external_id
                                    let mut seen: HashSet<String> = HashSet::with_capacity(xs.len());
                                    xs.retain(|o| seen.insert(o.external_id.clone()));

                                    logger.debug_mod(
                                        SystemModuleEnum::IngestorRouter,
                                        format!("orders before {} after {}", before, xs.len())
                                    ).await;

                                    if xs.is_empty() {
                                        if let Some(tx) = to_deadletter_tx.as_ref() {
                                            let _ = tx.send(EventPayload::NotificationEvent(
                                                NotificationEvent::new("IngestorRouter: empty PO batch after validation/dedup")
                                            ));
                                        }
                                        continue;
                                    }

                                    if let Some(tx) = to_dbwriter_tx.as_ref() {
                                        let _ = tx.send(EventPayload::UpsertPurchaseOrders(xs));
                                    }
                                }

                                other => {
                                    // Forward diagnostics or ignore unknowns for now
                                    logger.debug_mod(
                                        SystemModuleEnum::IngestorRouter,
                                        format!("ignored payload: {:?}", other)
                                    ).await;
                                }
                            }
                        }
                    }
                }
            }

            // Final log
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
