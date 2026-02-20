use crate::api::websocket::messages::DroneUpdate;
use crate::ports::EventPublisher;
use tokio::sync::broadcast;

pub struct BroadcastEventPublisher {
    tx: broadcast::Sender<DroneUpdate>,
}

impl BroadcastEventPublisher {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }
}

impl EventPublisher for BroadcastEventPublisher {
    fn publish(&self, event: DroneUpdate) {
        let _ = self.tx.send(event);
    }

    fn subscribe(&self) -> broadcast::Receiver<DroneUpdate> {
        self.tx.subscribe()
    }
}

pub struct NullEventPublisher {
    tx: broadcast::Sender<DroneUpdate>,
}

impl NullEventPublisher {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        Self { tx }
    }
}

impl EventPublisher for NullEventPublisher {
    fn publish(&self, _event: DroneUpdate) {}

    fn subscribe(&self) -> broadcast::Receiver<DroneUpdate> {
        self.tx.subscribe()
    }
}
