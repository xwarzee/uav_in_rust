use super::engine::SimulationEngine;
use super::mode::SimulationMode;
use crate::drone::{Drone, Position, Velocity};
use crate::ports::{CommandDispatcher, DroneState, DroneStateSource};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::time::Duration;

/// Fetches drone states from the ROS2 HTTP bridge
pub struct Ros2DroneStateSource {
    client: Client,
    bridge_url: String,
}

impl Ros2DroneStateSource {
    pub fn new(bridge_url: String, timeout_ms: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .unwrap_or_default();
        Self { client, bridge_url }
    }
}

#[async_trait]
impl DroneStateSource for Ros2DroneStateSource {
    async fn fetch_states(&self) -> Result<HashMap<String, DroneState>, String> {
        let url = format!("{}/drones/states", self.bridge_url);
        let raw: HashMap<String, DroneStateUpdate> = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch drone states: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse drone states: {}", e))?;
        Ok(raw.into_iter()
            .map(|(id, s)| (id, DroneState { position: s.position, velocity: s.velocity }))
            .collect())
    }
}

/// ROS2 simulation engine using HTTP bridge
///
/// This engine communicates with a ROS2 node via HTTP REST API
/// to synchronize drone states and send commands.
pub struct Ros2SimulationEngine {
    client: Client,
    bridge_url: String,
    connected: bool,
    state_source: Box<dyn DroneStateSource>,
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
}

impl Ros2SimulationEngine {
    pub fn new(bridge_url: String, timeout_ms: u64) -> Self {
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
            "Created ROS2 client with timeout={}ms, bridge_url={}",
            timeout_ms,
            bridge_url
        );

        let state_source = Box::new(Ros2DroneStateSource::new(bridge_url.clone(), timeout_ms));

        Self {
            client,
            bridge_url,
            connected: false,
            state_source,
        }
    }

    pub fn new_with_state_source(bridge_url: String, timeout_ms: u64, state_source: Box<dyn DroneStateSource>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(2)
            .user_agent("uav-swarm-rust/1.0")
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .unwrap();

        Self {
            client,
            bridge_url,
            connected: false,
            state_source,
        }
    }

    async fn check_connection(&self) -> Result<(), String> {
        let url = format!("{}/health", self.bridge_url);

        tracing::debug!("Checking connection to ROS2 bridge at: {}", url);

        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .header("Connection", "close")
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    tracing::error!("Connection timeout to ROS2 bridge: {}", e);
                    format!("Bridge connection timeout: {}", e)
                } else if e.is_connect() {
                    tracing::error!("Failed to connect to ROS2 bridge: {}", e);
                    format!("Bridge connection failed (cannot connect): {}", e)
                } else if e.is_request() {
                    tracing::error!("Request error to ROS2 bridge: {}", e);
                    format!("Bridge request error: {}", e)
                } else {
                    tracing::error!("Unknown error connecting to ROS2 bridge: {}", e);
                    format!("Bridge connection failed: {}", e)
                }
            })?;

        let status = response.status();
        tracing::debug!("Received response from ROS2 bridge: HTTP {}", status);

        if !status.is_success() {
            tracing::error!("ROS2 bridge returned error status: {}", status);
            return Err(format!("Bridge returned status: {}", status));
        }

        let body = response.text()
            .await
            .map_err(|e| {
                tracing::error!("Failed to read response body: {}", e);
                format!("Failed to read health response: {}", e)
            })?;

        tracing::debug!("ROS2 bridge health response body: {}", body);

        let health: HealthResponse = serde_json::from_str(&body)
            .map_err(|e| {
                tracing::error!("Failed to parse health response as JSON: {}", e);
                tracing::error!("Response body was: {}", body);
                format!("Failed to parse health response: {}", e)
            })?;

        if health.status != "ok" {
            tracing::error!("ROS2 bridge health check failed: {:?}", health);
            return Err(format!("Bridge health check failed: {:?}", health));
        }

        tracing::info!("ROS2 bridge health check passed");

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
impl SimulationEngine for Ros2SimulationEngine {
    async fn initialize(&mut self) -> Result<(), String> {
        tracing::info!("Connecting to ROS2 bridge at {}", self.bridge_url);

        self.check_connection().await?;
        self.start_sync().await?;

        self.connected = true;
        tracing::info!("ROS2 bridge connected successfully");
        Ok(())
    }

    async fn update_drones(&mut self, drones: &mut HashMap<String, Drone>, _dt: f64) -> Result<(), String> {
        if !self.connected {
            return Err("Bridge not connected".to_string());
        }

        let states = self.state_source.fetch_states().await?;

        for (drone_id, state) in states {
            if let Some(drone) = drones.get_mut(&drone_id) {
                drone.position = state.position;
                drone.velocity = state.velocity;
            }
        }

        Ok(())
    }

    fn mode(&self) -> SimulationMode {
        SimulationMode::Ros2
    }

    async fn shutdown(&mut self) -> Result<(), String> {
        if self.connected {
            tracing::info!("Shutting down ROS2 bridge connection");
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

/// Adapter: dispatches movement commands to drones via the ROS2 HTTP bridge
pub struct Ros2CommandDispatcher {
    client: Client,
    bridge_url: String,
}

impl Ros2CommandDispatcher {
    pub fn new(bridge_url: String, timeout_ms: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .unwrap_or_default();
        Self { client, bridge_url }
    }
}

#[async_trait]
impl CommandDispatcher for Ros2CommandDispatcher {
    async fn send_command(&self, drone_id: &str, target: Position) -> Result<(), String> {
        let url = format!("{}/drones/{}/command", self.bridge_url, drone_id);
        let command = DroneCommand { target_position: target };

        self.client.post(&url)
            .json(&command)
            .send()
            .await
            .map_err(|e| format!("Failed to send command to {}: {}", drone_id, e))?;

        tracing::debug!("Sent command to {} via ROS2 bridge: {:?}", drone_id, target);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = Ros2SimulationEngine::new("http://localhost:8082".to_string(), 5000);
        assert_eq!(engine.mode(), SimulationMode::Ros2);
        assert!(!engine.is_connected());
    }
}
