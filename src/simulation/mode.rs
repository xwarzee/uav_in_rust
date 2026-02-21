use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SimulationMode {
    Internal,
    Gazebo,
    Ros2,
}

impl SimulationMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "internal" => Some(SimulationMode::Internal),
            "gazebo" => Some(SimulationMode::Gazebo),
            "ros2" => Some(SimulationMode::Ros2),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SimulationMode::Internal => "internal",
            SimulationMode::Gazebo => "gazebo",
            SimulationMode::Ros2 => "ros2",
        }
    }
}

impl Default for SimulationMode {
    fn default() -> Self {
        SimulationMode::Internal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(SimulationMode::from_str("internal"), Some(SimulationMode::Internal));
        assert_eq!(SimulationMode::from_str("INTERNAL"), Some(SimulationMode::Internal));
        assert_eq!(SimulationMode::from_str("gazebo"), Some(SimulationMode::Gazebo));
        assert_eq!(SimulationMode::from_str("GAZEBO"), Some(SimulationMode::Gazebo));
        assert_eq!(SimulationMode::from_str("ros2"), Some(SimulationMode::Ros2));
        assert_eq!(SimulationMode::from_str("ROS2"), Some(SimulationMode::Ros2));
        assert_eq!(SimulationMode::from_str("invalid"), None);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(SimulationMode::Internal.as_str(), "internal");
        assert_eq!(SimulationMode::Gazebo.as_str(), "gazebo");
        assert_eq!(SimulationMode::Ros2.as_str(), "ros2");
    }

    #[test]
    fn test_default() {
        assert_eq!(SimulationMode::default(), SimulationMode::Internal);
    }
}
