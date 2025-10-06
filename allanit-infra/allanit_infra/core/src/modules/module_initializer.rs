use std::sync::Arc;
use tokio::{sync::mpsc, task};

use crate::core::shared_resources::SharedResources;
use crate::rocket_api;
use crate::services::wiring::{ChannelType, EventPayload}; // justera path

use crate::modules::{
    command_bus::CommandBus, db_writer::DbWriter, dead_letter::DeadLetterWorker,
    health_and_metrics::HealthAndMetrics, ingestor_router::IngestorRouter,
    seventime_fetcher::SeventimeFetcher,
};

/// Håller tokio-handles till alla moduler
pub struct ModuleInitializer {
    pub rocket_task: task::JoinHandle<()>,
    pub fetcher_task: task::JoinHandle<()>,
    pub ingestor_task: task::JoinHandle<()>,
    pub dbwriter_task: task::JoinHandle<()>,
    pub deadletter_task: task::JoinHandle<()>,
    pub commandbus_task: task::JoinHandle<()>,
    pub health_task: task::JoinHandle<()>,
}

impl ModuleInitializer {
    /// Skapar kanaler i ServiceWiring och spawnar alla moduler.
    /// Anropa som: `let handles = ModuleInitializer::new(Arc::clone(&shared)).await;`
    pub async fn new(shared: Arc<SharedResources>) -> Self {
        // 1) Lägg upp alla mpsc-kanaler i wiring
        {
            let wiring = shared.get_service_wiring();

            let (tx_f2i, rx_f2i) = mpsc::unbounded_channel::<EventPayload>();
            let (tx_i2db, rx_i2db) = mpsc::unbounded_channel::<EventPayload>();
            let (tx_i2dlq, rx_i2dlq) = mpsc::unbounded_channel::<EventPayload>();
            let (tx_r2cmd, rx_r2cmd) = mpsc::unbounded_channel::<EventPayload>();
            let (tx_cmd2f, rx_cmd2f) = mpsc::unbounded_channel::<EventPayload>();
            let (tx_cmd2dlq, rx_cmd2dlq) = mpsc::unbounded_channel::<EventPayload>();

            let mut guard = wiring.lock().await;
            guard
                .add_channel(ChannelType::FetcherToIngestor, tx_f2i, rx_f2i)
                .await;
            guard
                .add_channel(ChannelType::IngestorToDbWriter, tx_i2db, rx_i2db)
                .await;
            guard
                .add_channel(ChannelType::IngestorToDeadLetter, tx_i2dlq, rx_i2dlq)
                .await;
            guard
                .add_channel(ChannelType::RocketToCommandBus, tx_r2cmd, rx_r2cmd)
                .await;
            guard
                .add_channel(ChannelType::CommandBusToFetcher, tx_cmd2f, rx_cmd2f)
                .await;
            guard
                .add_channel(ChannelType::CommandBusToDeadLetter, tx_cmd2dlq, rx_cmd2dlq)
                .await;
            // lägg ev. fler här senare
        }

        // 2) Spawn Rocket och alla workers
        let rocket_task = task::spawn({
            let shared = Arc::clone(&shared);
            async move {
                rocket_api::rocket_server::launch_rocket(shared).await;
            }
        });

        let fetcher_task = SeventimeFetcher::spawn(Arc::clone(&shared));
        let ingestor_task = IngestorRouter::spawn(Arc::clone(&shared));
        let dbwriter_task = DbWriter::spawn(Arc::clone(&shared));
        let deadletter_task = DeadLetterWorker::spawn(Arc::clone(&shared));
        let commandbus_task = CommandBus::spawn(Arc::clone(&shared));
        let health_task = HealthAndMetrics::spawn(Arc::clone(&shared));

        Self {
            rocket_task,
            fetcher_task,
            ingestor_task,
            dbwriter_task,
            deadletter_task,
            commandbus_task,
            health_task,
        }
    }

    /// Vänta in alla tasks vid shutdown
    pub async fn join_tasks(self) {
        let _ = tokio::join!(
            self.rocket_task,
            self.fetcher_task,
            self.ingestor_task,
            self.dbwriter_task,
            self.deadletter_task,
            self.commandbus_task,
            self.health_task,
        );
    }
}
