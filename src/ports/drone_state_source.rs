use crate::drone::{Position, Velocity};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DroneState {
    pub position: Position,
    pub velocity: Velocity,
}

#[async_trait]
pub trait DroneStateSource: Send + Sync {
    async fn fetch_states(&self) -> Result<HashMap<String, DroneState>, String>;
}
