use super::engine::SimulationEngine;
use super::mode::SimulationMode;
use crate::drone::{Drone, Position};
use async_trait::async_trait;
use std::collections::HashMap;

/// Internal simulation engine using simple Rust physics
///
/// This engine uses basic kinematics for drone movement:
/// - Position updates based on velocity and time delta
/// - Velocity clamped to max_speed
/// - Simple point-to-point navigation
pub struct InternalSimulationEngine {
    initialized: bool,
}

impl InternalSimulationEngine {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
}

impl Default for InternalSimulationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SimulationEngine for InternalSimulationEngine {
    async fn initialize(&mut self) -> Result<(), String> {
        tracing::info!("Initialized internal simulation engine");
        self.initialized = true;
        Ok(())
    }

    async fn update_drones(&mut self, drones: &mut HashMap<String, Drone>, dt: f64) -> Result<(), String> {
        // Use existing simple physics from Drone::update_position
        for drone in drones.values_mut() {
            drone.update_position(dt);
        }
        Ok(())
    }

    async fn send_command(&self, _drone_id: &str, _target: Position) -> Result<(), String> {
        // In internal mode, commands are handled directly by updating
        // the drone's target_position field via the API handlers.
        // No need to send commands to an external system.
        Ok(())
    }

    fn mode(&self) -> SimulationMode {
        SimulationMode::Internal
    }

    async fn shutdown(&mut self) -> Result<(), String> {
        tracing::info!("Shutdown internal simulation engine");
        self.initialized = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drone::Position;

    #[tokio::test]
    async fn test_engine_initialization() {
        let mut engine = InternalSimulationEngine::new();
        assert_eq!(engine.mode(), SimulationMode::Internal);
        assert!(!engine.is_connected());

        assert!(engine.initialize().await.is_ok());
        assert!(engine.is_connected());
    }

    #[tokio::test]
    async fn test_update_drones() {
        let mut engine = InternalSimulationEngine::new();
        engine.initialize().await.unwrap();

        let mut drones = HashMap::new();
        let mut drone = Drone::new("test_drone".to_string(), Position::new(0.0, 0.0, 10.0));
        drone.move_to(Position::new(10.0, 10.0, 10.0));
        drones.insert("test_drone".to_string(), drone);

        // Update with 0.1s time delta
        assert!(engine.update_drones(&mut drones, 0.1).await.is_ok());

        // Drone should have moved
        let drone = drones.get("test_drone").unwrap();
        assert!(drone.position.x > 0.0 || drone.position.y > 0.0);
    }
}
