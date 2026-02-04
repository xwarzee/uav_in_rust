use super::engine::SimulationEngine;
use super::mode::SimulationMode;
use crate::drone::{Drone, Position, Velocity};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Gazebo simulation engine using HTTP bridge
///
/// This engine communicates with a Gazebo C++ plugin via HTTP REST API
/// to synchronize drone states and send commands.
pub struct GazeboSimulationEngine {
    client: Client,
    bridge_url: String,
    connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DroneStateUpdate {
    position: Position,
    velocity: Velocity,
}

#[derive(Serialize, Deserialize, Debug)]
struct DroneCommand {
    target_position: Position,
}

#[derive(Deserialize, Debug)]
struct HealthResponse {
    status: String,
    drones: Option<Vec<String>>,
}

impl GazeboSimulationEngine {
    pub fn new(bridge_url: String, timeout_ms: u64) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_millis(timeout_ms))
                .build()
                .unwrap(),
            bridge_url,
            connected: false,
        }
    }

    async fn check_connection(&self) -> Result<(), String> {
        let url = format!("{}/health", self.bridge_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Bridge connection failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Bridge returned status: {}", response.status()));
        }

        let health: HealthResponse = response.json()
            .await
            .map_err(|e| format!("Failed to parse health response: {}", e))?;

        if health.status != "ok" {
            return Err(format!("Bridge health check failed: {:?}", health));
        }

        Ok(())
    }

    async fn start_sync(&self) -> Result<(), String> {
        let url = format!("{}/start", self.bridge_url);
        self.client.post(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to start bridge sync: {}", e))?;
        Ok(())
    }
}

#[async_trait]
impl SimulationEngine for GazeboSimulationEngine {
    async fn initialize(&mut self) -> Result<(), String> {
        tracing::info!("Connecting to Gazebo bridge at {}", self.bridge_url);

        // Check if bridge is available
        self.check_connection().await?;

        // Start synchronization
        self.start_sync().await?;

        self.connected = true;
        tracing::info!("Gazebo bridge connected successfully");
        Ok(())
    }

    async fn update_drones(&mut self, drones: &mut HashMap<String, Drone>, _dt: f64) -> Result<(), String> {
        if !self.connected {
            return Err("Bridge not connected".to_string());
        }

        // Fetch latest states from Gazebo via bridge
        let url = format!("{}/drones/states", self.bridge_url);

        let response: HashMap<String, DroneStateUpdate> = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch drone states: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse drone states: {}", e))?;

        // Update internal drone states with Gazebo data
        for (drone_id, state) in response {
            if let Some(drone) = drones.get_mut(&drone_id) {
                drone.position = state.position;
                drone.velocity = state.velocity;
                // Note: We don't update target_position - that's a command, not state
            }
        }

        Ok(())
    }

    async fn send_command(&self, drone_id: &str, target: Position) -> Result<(), String> {
        if !self.connected {
            return Err("Bridge not connected".to_string());
        }

        let url = format!("{}/drones/{}/command", self.bridge_url, drone_id);
        let command = DroneCommand {
            target_position: target,
        };

        self.client.post(&url)
            .json(&command)
            .send()
            .await
            .map_err(|e| format!("Failed to send command to {}: {}", drone_id, e))?;

        tracing::debug!("Sent command to {} via Gazebo bridge: {:?}", drone_id, target);
        Ok(())
    }

    fn mode(&self) -> SimulationMode {
        SimulationMode::Gazebo
    }

    async fn shutdown(&mut self) -> Result<(), String> {
        if self.connected {
            tracing::info!("Shutting down Gazebo bridge connection");
            let url = format!("{}/stop", self.bridge_url);
            let _ = self.client.post(&url).send().await;
            self.connected = false;
        }
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = GazeboSimulationEngine::new("http://localhost:8081".to_string(), 5000);
        assert_eq!(engine.mode(), SimulationMode::Gazebo);
        assert!(!engine.is_connected());
    }

    // Note: Integration tests with actual bridge are in tests/gazebo_integration_test.rs
}
