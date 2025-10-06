use common::enums::system::CoreEvent;
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc, RwLock};

/// Enum for different one-to-one channel types.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ChannelType {
    FetcherToIngestor,
    IngestorToDbWriter,
    IngestorToDeadLetter,
    RocketToCommandBus,
    CommandBusToFetcher,
    CommandBusToDeadLetter,
}

// ====== Channel PAYLOAD ======
#[derive(Clone, Debug)]
pub enum EventPayload {
    CoreEvent(CoreEvent),
    NotificationEvent(NotificationEvent),

    // domän:
    IngestAck,
    DbAck,
    ReprocessRequest { count: usize },
    DeadLettered { reason: String },
}

//Example on how to add payload
#[derive(Debug, Clone)]
pub struct NotificationEvent {
    pub notification: String,
}

impl NotificationEvent {
    pub fn new(notification: &str) -> Self {
        NotificationEvent {
            notification: notification.to_string(),
        }
    }
}

/// Global broadcast for system-wide events.
pub struct ModuleChannels {
    pub core_event_tx: broadcast::Sender<CoreEvent>,
}

impl ModuleChannels {
    pub fn new() -> Self {
        let (core_event_tx, _) = broadcast::channel::<CoreEvent>(16);
        ModuleChannels { core_event_tx }
    }

    /// Broadcasts an event to all subscribers.
    pub async fn send_event_to_all_modules(&self, event: CoreEvent) {
        let _ = self.core_event_tx.send(event);
    }

    /// Returns a new subscription to the global core event channel.
    pub fn subscribe_to_core_event(&self) -> broadcast::Receiver<CoreEvent> {
        self.core_event_tx.subscribe()
    }
}

impl Default for ModuleChannels {
    fn default() -> Self {
        Self::new()
    }
}

type ChannelPair = (
    Option<mpsc::UnboundedSender<EventPayload>>,
    Option<mpsc::UnboundedReceiver<EventPayload>>,
);

/// ModuleWiring encapsulates the one-to-one communication channels between modules.
/// This structure manages each channel identified by `ChannelType`.
pub struct ModuleWiring {
    map: HashMap<
        ChannelType,
        (
            Option<mpsc::UnboundedSender<EventPayload>>,
            Option<mpsc::UnboundedReceiver<EventPayload>>,
        ),
    >,
}

impl ModuleWiring {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Lägg till en 1-1-kanal
    pub fn add_channel(
        &mut self,
        channel: ChannelType,
        tx: mpsc::UnboundedSender<EventPayload>,
        rx: mpsc::UnboundedReceiver<EventPayload>,
    ) {
        self.map.insert(channel, (Some(tx), Some(rx)));
    }

    /// Ta ägarskap av TX
    pub fn take_tx(&mut self, channel: ChannelType) -> Option<mpsc::UnboundedSender<EventPayload>> {
        self.map.get_mut(&channel)?.0.take()
    }

    /// Ta ägarskap av RX
    pub fn take_rx(
        &mut self,
        channel: ChannelType,
    ) -> Option<mpsc::UnboundedReceiver<EventPayload>> {
        self.map.get_mut(&channel)?.1.take()
    }

    /// Hämta en klon av TX (utan att ta ägarskap)
    pub fn get_tx(&self, channel: ChannelType) -> Option<mpsc::UnboundedSender<EventPayload>> {
        self.map.get(&channel).and_then(|(tx, _)| tx.clone())
    }

    /// Rensa bort kanalen om både TX och RX är borttagna
    pub fn remove_if_empty(&mut self, channel: ChannelType) {
        if let Some((tx, rx)) = self.map.get(&channel) {
            if tx.is_none() && rx.is_none() {
                self.map.remove(&channel);
            }
        }
    }

    pub fn tx_exists(&self, channel: ChannelType) -> bool {
        self.map.get(&channel).is_some_and(|(tx, _)| tx.is_some())
    }

    pub fn rx_exists(&self, channel: ChannelType) -> bool {
        self.map.get(&channel).is_some_and(|(_, rx)| rx.is_some())
    }
}

impl Default for ModuleWiring {
    fn default() -> Self {
        Self::new()
    }
}

//Perfect example of how to use mpsc with module_channels
#[tokio::test]
async fn test_module_wiring() {
    let wiring = ModuleWiring::new();
    let (tx, rx) = mpsc::unbounded_channel::<EventPayload>();
    let chan = ChannelType::FetcherToIngestor;

    // Add channel
    wiring.add_channel(chan.clone(), tx.clone(), rx).await;

    // Check tx and rx existence
    assert!(wiring.tx_exists(chan.clone()).await);
    assert!(wiring.rx_exists(chan.clone()).await);

    // Take tx only
    let taken_tx = wiring.take_tx(chan.clone()).await;
    assert!(taken_tx.is_some());
    assert!(!wiring.tx_exists(chan.clone()).await);
    assert!(wiring.rx_exists(chan.clone()).await);

    // Take rx next
    let taken_rx = wiring.take_rx(chan.clone()).await;
    assert!(taken_rx.is_some());
    assert!(!wiring.rx_exists(chan.clone()).await);

    // Now channel should be empty and removable
    wiring.remove_if_empty(chan.clone()).await;
    assert!(!wiring.tx_exists(chan.clone()).await);
    assert!(!wiring.rx_exists(chan.clone()).await);

    // Add a new channel and get cloned tx
    let (tx2, rx2) = mpsc::unbounded_channel::<EventPayload>();
    wiring.add_channel(chan.clone(), tx2.clone(), rx2).await;

    let cloned = wiring.get_tx(chan.clone()).await;
    assert!(cloned.is_some());
    let _ = cloned
        .unwrap()
        .send(EventPayload::NotificationEvent(NotificationEvent::new(
            "test",
        )));

    // Test overwriting channel
    let (tx3, rx3) = mpsc::unbounded_channel::<EventPayload>();
    wiring.add_channel(chan.clone(), tx3.clone(), rx3).await;

    let newer_tx = wiring.take_tx(chan.clone()).await;
    assert_eq!(newer_tx.unwrap().same_channel(&tx3), true);
}
