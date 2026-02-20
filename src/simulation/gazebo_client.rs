use super::engine::SimulationEngine;
use super::mode::SimulationMode;
use crate::drone::{Drone, Position, Velocity};
use crate::ports::CommandDispatcher;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
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
        // Create a robust HTTP client with explicit configuration
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(2)
            .user_agent("uav-swarm-rust/1.0")
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .unwrap();

        tracing::info!(
            "Created Gazebo client with timeout={}ms, bridge_url={}",
            timeout_ms,
            bridge_url
        );

        Self {
            client,
            bridge_url,
            connected: false,
        }
    }

    async fn check_connection(&self) -> Result<(), String> {
        let url = format!("{}/health", self.bridge_url);

        tracing::debug!("Checking connection to Gazebo bridge at: {}", url);

        // Send request with explicit headers
        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .header("Connection", "close")  // Prevent connection reuse issues
            .send()
            .await
            .map_err(|e| {
                // Detailed error logging
                if e.is_timeout() {
                    tracing::error!("Connection timeout to Gazebo bridge: {}", e);
                    format!("Bridge connection timeout: {}", e)
                } else if e.is_connect() {
                    tracing::error!("Failed to connect to Gazebo bridge: {}", e);
                    format!("Bridge connection failed (cannot connect): {}", e)
                } else if e.is_request() {
                    tracing::error!("Request error to Gazebo bridge: {}", e);
                    format!("Bridge request error: {}", e)
                } else {
                    tracing::error!("Unknown error connecting to Gazebo bridge: {}", e);
                    format!("Bridge connection failed: {}", e)
                }
            })?;

        let status = response.status();
        tracing::debug!("Received response from Gazebo bridge: HTTP {}", status);

        if !status.is_success() {
            tracing::error!("Gazebo bridge returned error status: {}", status);
            return Err(format!("Bridge returned status: {}", status));
        }

        // Get response body as text first for better error messages
        let body = response.text()
            .await
            .map_err(|e| {
                tracing::error!("Failed to read response body: {}", e);
                format!("Failed to read health response: {}", e)
            })?;

        tracing::debug!("Gazebo bridge health response body: {}", body);

        // Parse JSON
        let health: HealthResponse = serde_json::from_str(&body)
            .map_err(|e| {
                tracing::error!("Failed to parse health response as JSON: {}", e);
                tracing::error!("Response body was: {}", body);
                format!("Failed to parse health response: {}", e)
            })?;

        if health.status != "ok" {
            tracing::error!("Gazebo bridge health check failed: {:?}", health);
            return Err(format!("Bridge health check failed: {:?}", health));
        }

        tracing::info!("Gazebo bridge health check passed: {:?}", health);
        
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

/// Adapter: dispatches movement commands to drones via the Gazebo HTTP bridge
pub struct GazeboCommandDispatcher {
    client: Client,
    bridge_url: String,
}

impl GazeboCommandDispatcher {
    pub fn new(bridge_url: String, timeout_ms: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .unwrap_or_default();
        Self { client, bridge_url }
    }
}

#[async_trait]
impl CommandDispatcher for GazeboCommandDispatcher {
    async fn send_command(&self, drone_id: &str, target: Position) -> Result<(), String> {
        let url = format!("{}/drones/{}/command", self.bridge_url, drone_id);
        let command = DroneCommand { target_position: target };

        self.client.post(&url)
            .json(&command)
            .send()
            .await
            .map_err(|e| format!("Failed to send command to {}: {}", drone_id, e))?;

        tracing::debug!("Sent command to {} via Gazebo bridge: {:?}", drone_id, target);
        Ok(())
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
