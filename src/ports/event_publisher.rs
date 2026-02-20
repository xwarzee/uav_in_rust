use crate::api::websocket::messages::DroneUpdate;
use tokio::sync::broadcast;

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: DroneUpdate);
    fn subscribe(&self) -> broadcast::Receiver<DroneUpdate>;
}
