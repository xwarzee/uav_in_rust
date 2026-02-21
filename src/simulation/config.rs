use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use super::mode::SimulationMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    #[serde(default)]
    pub simulation: SimulationSettings,
    #[serde(default)]
    pub gazebo: GazeboConfig,
    #[serde(default)]
    pub ros2: Ros2Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSettings {
    #[serde(default)]
    pub mode: SimulationMode,
    #[serde(default = "default_update_rate")]
    pub update_rate_hz: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GazeboConfig {
    #[serde(default = "default_bridge_url")]
    pub bridge_url: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ros2Config {
    #[serde(default = "default_ros2_bridge_url")]
    pub bridge_url: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

impl Default for Ros2Config {
    fn default() -> Self {
        Self {
            bridge_url: default_ros2_bridge_url(),
            enabled: false,
            timeout_ms: default_timeout(),
        }
    }
}

fn default_ros2_bridge_url() -> String {
    "http://localhost:8082".to_string()
}

fn default_update_rate() -> f64 {
    10.0
}

fn default_bridge_url() -> String {
    "http://localhost:8081".to_string()
}

fn default_timeout() -> u64 {
    5000
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            mode: SimulationMode::default(),
            update_rate_hz: default_update_rate(),
        }
    }
}

impl Default for GazeboConfig {
    fn default() -> Self {
        Self {
            bridge_url: default_bridge_url(),
            enabled: false,
            auto_start: false,
            timeout_ms: default_timeout(),
        }
    }
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            simulation: SimulationSettings::default(),
            gazebo: GazeboConfig::default(),
            ros2: Ros2Config::default(),
        }
    }
}

impl SimulationConfig {
    /// Load configuration from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Load configuration with environment variable overrides
    pub fn from_file_with_env<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut config = Self::from_file(path)?;
        config.apply_env_overrides();
        Ok(config)
    }

    /// Load default configuration with environment variable overrides
    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.apply_env_overrides();
        config
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) {
        if let Ok(mode_str) = std::env::var("UAV_SIMULATION_MODE") {
            if let Some(mode) = SimulationMode::from_str(&mode_str) {
                self.simulation.mode = mode;
            }
        }

        if let Ok(url) = std::env::var("UAV_GAZEBO_BRIDGE_URL") {
            self.gazebo.bridge_url = url;
        }

        if let Ok(enabled) = std::env::var("UAV_GAZEBO_ENABLED") {
            self.gazebo.enabled = enabled.to_lowercase() == "true";
        }

        if let Ok(auto_start) = std::env::var("UAV_GAZEBO_AUTO_START") {
            self.gazebo.auto_start = auto_start.to_lowercase() == "true";
        }

        if let Ok(timeout) = std::env::var("UAV_GAZEBO_TIMEOUT_MS") {
            if let Ok(timeout_val) = timeout.parse() {
                self.gazebo.timeout_ms = timeout_val;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SimulationConfig::default();
        assert_eq!(config.simulation.mode, SimulationMode::Internal);
        assert_eq!(config.simulation.update_rate_hz, 10.0);
        assert_eq!(config.gazebo.bridge_url, "http://localhost:8081");
        assert_eq!(config.gazebo.enabled, false);
        assert_eq!(config.gazebo.timeout_ms, 5000);
    }

    #[test]
    fn test_toml_serialization() {
        let config = SimulationConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: SimulationConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.simulation.mode, config.simulation.mode);
        assert_eq!(parsed.gazebo.bridge_url, config.gazebo.bridge_url);
    }
}
