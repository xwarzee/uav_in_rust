use crate::drone::Position;
use async_trait::async_trait;

/// Port defining how movement commands are dispatched to drones
///
/// This trait is the boundary between the domain (DroneSwarm, missions, formations)
/// and the infrastructure responsible for actually moving drones.
///
/// Implementations:
/// - `InternalCommandDispatcher`: no-op (target_position is already set in the domain)
/// - `GazeboCommandDispatcher`: sends HTTP commands to the Gazebo bridge
#[async_trait]
pub trait CommandDispatcher: Send + Sync {
    /// Send a movement command to a specific drone
    ///
    /// # Arguments
    /// * `drone_id` - ID of the drone to command
    /// * `target` - Target position for the drone
    async fn send_command(&self, drone_id: &str, target: Position) -> Result<(), String>;
}
