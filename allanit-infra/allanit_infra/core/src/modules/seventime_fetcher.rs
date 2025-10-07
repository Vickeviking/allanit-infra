//! SeventimeFetcher
//! -----------------
//! Purpose: Periodically fetch Customers and Purchase Orders from the Seventime mock API
//! and push canonical "new record" batches into the internal pipeline.
//!
//! Responsibilities:
//! - Subscribe to lifecycle (CoreEvent) and Pulse::Medium ticks.
//! - On each allowed tick (throttled by FETCH_MIN_INTERVAL_MS) and on-demand via CommandBus,
//!   fetch `/api/v1/customers` and `/api/v1/purchase-orders` from the Seventime mock.
//! - Convert remote JSON into internal domain structs (`NewCustomer`, `NewPurchaseOrder`).
//! - Emit `EventPayload::UpsertCustomers(Vec<NewCustomer>)` and
//!   `EventPayload::UpsertPurchaseOrders(Vec<NewPurchaseOrder>)` onto the
//!   `ChannelType::FetcherToIngestor` pipe.
//!
//! Non-responsibilities:
//! - No DB access or relation resolution (e.g. `customer_id`).
//! - No deduplication or policy validation. Those belong to IngestorRouter and DbWriter.
//!
//! Inputs/Outputs:
//! - Inputs:
//!   - Broadcast: `CoreEvent` (Startup/Restart/Shutdown)
//!   - Broadcast: `Pulse::Medium` (activity tick)
//!   - Pipe: `ChannelType::CommandBusToFetcher` (on-demand sync triggers)
//! - Outputs:
//!   - Pipe: `ChannelType::FetcherToIngestor` with `EventPayload` batches
//!
//! Configuration (env):
//! - `SEVENTIME_BASE_URL` (e.g. http://localhost:4000)
//! - `SEVENTIME_CLIENT_SECRET`
//! - `SEVENTIME_TIMEOUT_MS` (default 5000)
//! - `FETCH_MIN_INTERVAL_MS` (default 5000)
//!
//! Notes:
//! - Amount handling uses `f64` (DOUBLE PRECISION). Ensure your DB schema and Diesel models
//!   use `DOUBLE PRECISION`/`Float8` accordingly.
//! - This file keeps a tiny DTO layer locally to be robust against API variations and then
//!   maps to internal `New*` structs for emission.

use std::{
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context;
use reqwest::Client;
use tokio::sync::{broadcast, mpsc};
use tokio::{select, task::JoinHandle};

use crate::core::shared_resources::SharedResources;
use crate::modules::logger::Logger;
use crate::modules::module_channels::{ChannelType, EventPayload, NotificationEvent};

use common::enums::log::{LogActionEnum, LogLevelEnum};
use common::enums::system::{CoreEvent, Pulse, SystemModuleEnum};

use common::database::models::customer::NewCustomer;
use common::database::models::purchase_order::NewPurchaseOrder;

const DEBUG: bool = true;

/// Runtime config sourced from env
struct Cfg {
    base: String,
    secret: String,
    timeout_ms: u64,
    min_interval_ms: u64,
}

impl Cfg {
    fn from_env() -> Self {
        Self {
            base: std::env::var("SEVENTIME_BASE_URL").expect("SEVENTIME_BASE_URL missing"),
            secret: std::env::var("SEVENTIME_CLIENT_SECRET")
                .expect("SEVENTIME_CLIENT_SECRET missing"),
            timeout_ms: std::env::var("SEVENTIME_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5000),
            min_interval_ms: std::env::var("FETCH_MIN_INTERVAL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5000),
        }
    }
}

pub struct SeventimeFetcher;

impl SeventimeFetcher {
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

            // Config and HTTP client
            let cfg = Cfg::from_env();
            let http = Client::builder()
                .timeout(Duration::from_millis(cfg.timeout_ms))
                .build()
                .expect("failed to build reqwest client");

            // Startup log
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::SeventimeFetcher,
                    LogActionEnum::SystemStartup,
                    Some("active".to_string()),
                )
                .await;

            let mut last_run = Instant::now() - Duration::from_secs(60);

            // Main loop
            loop {
                select! {
                    // Lifecycle events
                    Ok(ev) = core_rx.recv() => {
                        match ev {
                            CoreEvent::Startup => {
                                logger.debug_mod(SystemModuleEnum::SeventimeFetcher, "CoreEvent::Startup").await;
                            }
                            CoreEvent::Restart => {
                                logger.debug_mod(SystemModuleEnum::SeventimeFetcher, "CoreEvent::Restart").await;
                                last_run = Instant::now() - Duration::from_millis(cfg.min_interval_ms);
                            }
                            CoreEvent::Shutdown => {
                                logger.log_now(
                                    LogLevelEnum::Info,
                                    SystemModuleEnum::SeventimeFetcher,
                                    LogActionEnum::SystemShutdown,
                                    Some("draining".to_string()),
                                ).await;

                                // Drain incoming commands
                                if let Some(rx) = my_rx.as_mut() {
                                    while let Ok(msg) = rx.try_recv() {
                                        logger.debug_mod(
                                            SystemModuleEnum::SeventimeFetcher,
                                            format!("drain on shutdown: {:?}", msg)
                                        ).await;
                                    }
                                }

                                println!("SeventimeFetcher | Shutdown complete.");
                                break;
                            }
                        }
                    }

                    // Medium Pulse tick
                    Ok(Pulse::Medium) = pulse_rx.recv() => {
                        logger
                            .log_now(
                                LogLevelEnum::Info,
                                SystemModuleEnum::SeventimeFetcher,
                                LogActionEnum::DebugPulse,
                                Some("tick".to_string()),
                            )
                            .await;

                        if last_run.elapsed() < Duration::from_millis(cfg.min_interval_ms) { continue; }
                        last_run = Instant::now();

                        if let Err(e) = fetch_and_emit(&http, &cfg, &logger, to_ingestor_tx.clone()).await {
                            logger.log_now(
                                LogLevelEnum::Error,
                                SystemModuleEnum::SeventimeFetcher,
                                LogActionEnum::ApiResponseError,
                                Some(format!("fetch error: {e:#}")),
                            ).await;
                        }

                        if DEBUG {
                            if let Some(tx) = to_ingestor_tx.clone() {
                                let _ = tx.send(EventPayload::NotificationEvent(
                                    NotificationEvent::new("Hej från SeventimeFetcher"))
                                );
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

                            if let Err(e) = fetch_and_emit(&http, &cfg, &logger, to_ingestor_tx.clone()).await {
                                logger.log_now(
                                    LogLevelEnum::Error,
                                    SystemModuleEnum::SeventimeFetcher,
                                    LogActionEnum::SyncSeventimeFailed,
                                    Some(format!("on-demand fetch error: {e:#}")),
                                ).await;
                            }
                        }
                    }
                }
            }

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

async fn fetch_and_emit(
    http: &Client,
    cfg: &Cfg,
    logger: &Logger,
    tx_opt: Option<mpsc::UnboundedSender<EventPayload>>,
) -> anyhow::Result<()> {
    let tx_customers = tx_opt.clone();
    let tx_orders = tx_opt;

    // Kör parallellt
    let (cust_res, order_res) = future::join(
        async {
            logger
                .debug_mod(
                    SystemModuleEnum::SeventimeFetcher,
                    "attempt fetch customers",
                )
                .await;
            let customers = fetch_customers(http, cfg)
                .await
                .context("fetch_customers")?;
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::SeventimeFetcher,
                    LogActionEnum::ApiResponseOk,
                    Some(format!("fetched customers {}", customers.len())),
                )
                .await;

            let batch: Vec<NewCustomer> = customers
                .into_iter()
                .map(|c| NewCustomer {
                    external_id: c.external_id,
                    name: c.name,
                    email: c.email,
                    phone: c.phone,
                    org_no: c.org_no,
                })
                .collect();

            logger
                .debug_mod(
                    SystemModuleEnum::SeventimeFetcher,
                    format!("emit customers {}", batch.len()),
                )
                .await;
            if let Some(tx) = tx_customers {
                let _ = tx.send(EventPayload::UpsertCustomers(batch));
                logger
                    .log_now(
                        LogLevelEnum::Info,
                        SystemModuleEnum::SeventimeFetcher,
                        LogActionEnum::MessageSent,
                        Some("customers batch sent".to_string()),
                    )
                    .await;
            }
            anyhow::Ok(())
        },
        async {
            logger
                .debug_mod(SystemModuleEnum::SeventimeFetcher, "attempt fetch orders")
                .await;
            let pos = fetch_pos(http, cfg).await.context("fetch_pos")?;
            logger
                .log_now(
                    LogLevelEnum::Info,
                    SystemModuleEnum::SeventimeFetcher,
                    LogActionEnum::ApiResponseOk,
                    Some(format!("fetched orders {}", pos.len())),
                )
                .await;

            let batch: Vec<NewPurchaseOrder> = pos
                .into_iter()
                .map(|o| NewPurchaseOrder {
                    external_id: o.external_id,
                    customer_id: None,
                    status: o.status,
                    description: o.description,
                    amount: parse_amount(&o.amount),
                })
                .collect();

            logger
                .debug_mod(
                    SystemModuleEnum::SeventimeFetcher,
                    format!("emit orders {}", batch.len()),
                )
                .await;
            if let Some(tx) = tx_orders {
                let _ = tx.send(EventPayload::UpsertPurchaseOrders(batch));
                logger
                    .log_now(
                        LogLevelEnum::Info,
                        SystemModuleEnum::SeventimeFetcher,
                        LogActionEnum::MessageSent,
                        Some("orders batch sent".to_string()),
                    )
                    .await;
            }
            anyhow::Ok(())
        },
    )
    .await;

    if let Err(e) = cust_res {
        logger
            .log_now(
                LogLevelEnum::Error,
                SystemModuleEnum::SeventimeFetcher,
                LogActionEnum::ApiResponseError,
                Some(format!("customers failed: {e:#}")),
            )
            .await;
    }
    if let Err(e) = order_res {
        logger
            .log_now(
                LogLevelEnum::Error,
                SystemModuleEnum::SeventimeFetcher,
                LogActionEnum::ApiResponseError,
                Some(format!("orders failed: {e:#}")),
            )
            .await;
    }

    Ok(())
}
#[derive(serde::Deserialize)]
struct CustomerDTO {
    external_id: String,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    org_no: Option<String>,
}

#[derive(serde::Deserialize)]
struct ListCustomers {
    results: Vec<CustomerDTO>,
}

async fn fetch_customers(http: &Client, cfg: &Cfg) -> anyhow::Result<Vec<CustomerDTO>> {
    let r = http
        .get(format!("{}/api/v1/customers", cfg.base))
        .header("Client-Secret", &cfg.secret)
        .send()
        .await?
        .error_for_status()?;
    Ok(r.json::<ListCustomers>().await?.results)
}

#[derive(serde::Deserialize)]
struct PoDTO {
    external_id: String,
    status: String,
    description: Option<String>,
    amount: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct ListPOs {
    results: Vec<PoDTO>,
}

async fn fetch_pos(http: &Client, cfg: &Cfg) -> anyhow::Result<Vec<PoDTO>> {
    let r = http
        .get(format!("{}/api/v1/purchase-orders", cfg.base))
        .header("Client-Secret", &cfg.secret)
        .send()
        .await?
        .error_for_status()?;
    Ok(r.json::<ListPOs>().await?.results)
}

fn parse_amount(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => f64::from_str(s).unwrap_or(0.0),
        _ => 0.0,
    }
}
