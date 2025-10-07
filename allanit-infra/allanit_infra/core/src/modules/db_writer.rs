//! DbWriter
//! --------
//! Purpose: Single write gateway to PostgreSQL. Consumes validated, in-batch
//! deduplicated domain batches from `IngestorRouter` and performs idempotent
//! upserts inside transactions using repository helpers.
//!
//! Responsibilities
//! - Subscribe to lifecycle (CoreEvent) and Pulse::Medium (for simple liveness logs).
//! - Receive `EventPayload::{UpsertCustomers, UpsertPurchaseOrders}` from
//!   `ChannelType::IngestorToDbWriter`.
//! - Open a DB connection from the shared async pool and run a single transaction
//!   per incoming batch.
//! - Upsert each record via `CustomerRepository::upsert` and
//!   `PurchaseOrderRepository::upsert` (idempotent by external_id).
//! - Emit structured logs for submitted/completed/failed jobs.
//!
//! Non-responsibilities
//! - No HTTP calls, no ingestion policy, no cross-batch dedup.
//! - Relation lookups beyond what repositories already handle.
//!
//! Inputs / Outputs
//! - In:  Pipe `IngestorToDbWriter` (EventPayload batches)
//! - Out: None (future ACK channel optional)
//!
//! Notes
//! - The current approach upserts per-row inside a single transaction. This is
//!   simple and safe. If/when needed, you can optimize with true bulk upserts.

use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload};

use common::database::repositories::{
    customer_repo::CustomerRepository, purchase_order_repo::PurchaseOrderRepository,
};
use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection; // for .transaction(...) // for .scope_boxed()

pub struct DBWriter;

impl DBWriter {
    /// Spawn the DBWriter task. It listens to core lifecycle events, a pulse stream
    /// for liveness, and an mpsc input channel for domain batches that should be
    /// upserted into Postgres.
    pub fn spawn(shared: Arc<SharedResources>) -> JoinHandle<()> {
        tokio::spawn(async move {
            // Logger and pulses
            let logger: Arc<Logger> = shared.get_logger();
            let pulses = shared.get_pulse_subscriptions();
            let mut pulse_rx = pulses.subscribe_medium();

            // Core events
            let mut core_rx: broadcast::Receiver<CoreEvent> = {
                let chans = shared.get_module_channels();
                chans.subscribe_to_core_event()
            };

            // Wiring IN: Ingestor -> DBWriter
            let mut ingestor_rx: Option<mpsc::UnboundedReceiver<EventPayload>> = {
                let wiring = shared.get_module_wiring();
                let mut guard = wiring.lock().await;
                guard.take_rx(ChannelType::IngestorToDbWriter)
            };

            // Startup log
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::DbWriter,
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
                                logger.debug_mod(SystemModuleEnum::DbWriter, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::DbWriter, "CoreEvent::Restart").await;
                                // reset any transient state here if needed
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::DbWriter,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                // Drain incoming queue best effort
                                if let Some(rx) = ingestor_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::DbWriter,
                                            format!("drain on shutdown: {:?}", msg)
                                        ).await;
                                    }
                                }

                                println!("DBWriter | Shutdown complete.");

                                // Final flush of buffered logs
                                logger.store_all_logs().await;

                                break;
                            }
                        }
                    }

                    // Pulse (simple liveness signal)
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::DbWriter,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;
                    }

                    // MPSC: Ingestor -> DbWriter
                    maybe_msg = async {
                        if let Some(rx) = ingestor_rx.as_mut() { rx.recv().await } else { None }
                    } => {
                        if let Some(msg) = maybe_msg {
                            match msg {
                                EventPayload::UpsertCustomers(batch) => {
                                    // extra block ensures conn and pool drop before the branch ends
                                    let pool = shared.db_pool(); // cheap clone of bb8::Pool
                                    let mut count = batch.len();
                                    if count == 0 {
                                        continue;
                                    }
                                    // extract the connection first, do not make the whole match the branch value
                                    let mut conn = match pool.get().await {
                                        Ok(conn) => conn,
                                        Err(e) => {
                                            logger.log_now(
                                                LogLevelEnum::Error,
                                                SystemModuleEnum::DbWriter,
                                                LogActionEnum::DbConnectionFailed,
                                                Some(format!("db pool get failed: {}", e)),
                                            ).await;
                                            // early return from this inner block
                                            return;
                                        }
                                    };

                                    // one transaction per batch
                                    let res = conn.transaction(|tx| {
                                        let batch = batch.clone();
                                        async move {
                                            for c in batch {
                                                CustomerRepository::upsert(tx, c).await?;
                                            }
                                            Ok::<(), diesel::result::Error>(())
                                        }.scope_boxed()
                                    }).await;

                                    match res {
                                        Ok(_) => {
                                            logger.log_now(
                                                LogLevelEnum::Info,
                                                SystemModuleEnum::DbWriter,
                                                LogActionEnum::DbWriteOk,
                                                Some(format!("customers upserted {}", count)),
                                            ).await;
                                        }
                                        Err(e) => {
                                            logger.log_now(
                                                LogLevelEnum::Error,
                                                SystemModuleEnum::DbWriter,
                                                LogActionEnum::DbWriteFailed,
                                                Some(format!("customers upsert error: {}", e)),
                                            ).await;
                                        }
                                    }
                                }

                                EventPayload::UpsertPurchaseOrders(batch) => {
                                    let pool = shared.db_pool();
                                    let mut count = batch.len();
                                    if count == 0 {
                                        continue;
                                    }
                                    let mut conn = match pool.get().await {
                                        Ok(conn) => conn,
                                        Err(e) => {
                                            logger.log_now(
                                                LogLevelEnum::Error,
                                                SystemModuleEnum::DbWriter,
                                                LogActionEnum::DbConnectionFailed,
                                                Some(format!("db pool get failed: {}", e)),
                                            ).await;
                                            return;
                                        }
                                    };

                                    let res = conn.transaction(|tx| {
                                        let batch = batch.clone();
                                        async move {
                                            for o in batch {
                                                PurchaseOrderRepository::upsert(tx, o).await?;
                                            }
                                            Ok::<(), diesel::result::Error>(())
                                        }.scope_boxed()
                                    }).await;

                                    match res {
                                        Ok(_) => {
                                            logger.log_now(
                                                LogLevelEnum::Info,
                                                SystemModuleEnum::DbWriter,
                                                LogActionEnum::DbWriteOk,
                                                Some(format!("orders upserted {}", count)),
                                            ).await;
                                        }
                                        Err(e) => {
                                            logger.log_now(
                                                LogLevelEnum::Error,
                                                SystemModuleEnum::DbWriter,
                                                LogActionEnum::DbWriteFailed,
                                                Some(format!("orders upsert error: {}", e)),
                                            ).await;
                                        }
                                    }
                                }

                                other => {
                                    // Not a payload the DBWriter handles
                                    logger.debug_mod(
                                        SystemModuleEnum::DbWriter,
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
                    SystemModuleEnum::DbWriter,
                    LogActionEnum::SystemShutdown,
                    Some("stopped".to_string()),
                )
                .await;
        })
    }
}
