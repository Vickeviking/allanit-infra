use std::sync::Arc;
use tokio::{sync::mpsc, task};

use crate::core::shared_resources::SharedResources;
use crate::modules::{ChannelType, EventPayload};

use crate::modules::{
    command_bus::CommandBus, db_writer::DBWriter, dead_letter_worker::DeadLetterWorker,
    health_and_metrics::HealthAndMetrics, ingestor_router::IngestorRouter, rocket_api,
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
        let (tx_f2i, rx_f2i) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();
        let (tx_i2db, rx_i2db) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();
        let (tx_i2dlq, rx_i2dlq) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();
        let (tx_r2cmd, rx_r2cmd) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();
        let (tx_cmd2f, rx_cmd2f) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();
        let (tx_cmd2dlq, rx_cmd2dlq) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();

        // Hämta Arc<Mutex<ModuleWiring>> via getter på Arc<SharedResources>
        let wiring = shared.get_module_wiring();

        // Lås en gång, lägg in alla kanaler, släpp låset
        {
            let mut guard = wiring.lock().await;
            // Anta att add_channel är synkron: fn add_channel(&mut self, ty: ChannelType, tx: Sender<_>, rx: Receiver<_>)
            guard.add_channel(ChannelType::FetcherToIngestor, tx_f2i, rx_f2i);
            guard.add_channel(ChannelType::IngestorToDbWriter, tx_i2db, rx_i2db);
            guard.add_channel(ChannelType::IngestorToDeadLetter, tx_i2dlq, rx_i2dlq);
            guard.add_channel(ChannelType::RocketToCommandBus, tx_r2cmd, rx_r2cmd);
            guard.add_channel(ChannelType::CommandBusToFetcher, tx_cmd2f, rx_cmd2f);
            guard.add_channel(ChannelType::CommandBusToDeadLetter, tx_cmd2dlq, rx_cmd2dlq);
        }

        // Spawn Rocket och alla workers
        let rocket_task = task::spawn({
            let shared = Arc::clone(&shared);
            async move {
                rocket_api::rocket_server::launch_rocket(shared).await;
            }
        });

        let fetcher_task = SeventimeFetcher::spawn(Arc::clone(&shared));
        let ingestor_task = IngestorRouter::spawn(Arc::clone(&shared));
        let dbwriter_task = DBWriter::spawn(Arc::clone(&shared));
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
