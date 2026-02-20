use crate::drone::Drone;
use async_trait::async_trait;
use std::collections::HashMap;
use super::mode::SimulationMode;

/// Trait defining the interface for simulation engines
///
/// This abstraction allows switching between different simulation backends
/// (internal Rust physics vs external Gazebo simulation)
#[async_trait]
pub trait SimulationEngine: Send + Sync {
    /// Initialize the simulation engine
    ///
    /// This is called once when the engine is created or switched to.
    /// Returns Ok(()) on success, Err with message on failure.
    async fn initialize(&mut self) -> Result<(), String>;

    /// Update all drone states for the current simulation tick
    ///
    /// # Arguments
    /// * `drones` - Mutable reference to the drone collection
    /// * `dt` - Time delta since last update (in seconds)
    async fn update_drones(&mut self, drones: &mut HashMap<String, Drone>, dt: f64) -> Result<(), String>;

    /// Get the simulation mode of this engine
    fn mode(&self) -> SimulationMode;

    /// Shutdown the simulation engine
    ///
    /// This is called when switching to a different engine or shutting down.
    async fn shutdown(&mut self) -> Result<(), String>;

    /// Check if the engine is currently connected/active
    fn is_connected(&self) -> bool {
        true // Default implementation
    }
}
