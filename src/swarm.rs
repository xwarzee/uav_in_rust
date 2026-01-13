use crate::drone::{Drone, Position, DroneStatus, DroneStatusInfo};
use crate::formation::{FormationManager, FormationType};
use crate::mission::{MissionExecutor, MissionType};
use std::collections::HashMap;
use tokio::time::{sleep, Duration, Instant};

pub struct DroneSwarm {
    pub drones: HashMap<String, Drone>,
    pub formation_manager: FormationManager,
    pub mission_executor: MissionExecutor,
    pub simulation_running: bool,
    last_update: Instant,
}

impl DroneSwarm {
    pub fn new() -> Self {
        Self {
            drones: HashMap::new(),
            formation_manager: FormationManager::new(),
            mission_executor: MissionExecutor::new(),
            simulation_running: false,
            last_update: Instant::now(),
        }
    }

    pub fn add_drone(&mut self, drone_id: &str, initial_position: Position) {
        let drone = Drone::new(drone_id.to_string(), initial_position);
        self.drones.insert(drone_id.to_string(), drone);
        self.formation_manager.add_drone(drone_id.to_string());
        
        println!("Added drone '{}' at position ({:.1}, {:.1}, {:.1})", 
                drone_id, initial_position.x, initial_position.y, initial_position.z);
    }

    pub fn set_formation(&mut self, formation_type: &str) {
        if let Some(formation) = FormationType::from_str(formation_type) {
            self.formation_manager.set_formation_type(formation);

            // Move all drones to formation positions
            self.formation_manager.update_formation(&mut self.drones);

            println!("Formation changed to: {}", formation_type);
        } else {
            println!("Unknown formation type: {}. Available: triangle, line, v_formation", formation_type);
        }
    }

    pub fn set_separation_distance(&mut self, distance: f64) {
        self.formation_manager.set_separation_distance(distance);
        self.formation_manager.update_formation(&mut self.drones);
    }

    pub async fn execute_mission_by_id(&mut self, mission_id: &str) -> Result<(), String> {
        self.mission_executor.execute_mission(mission_id, &mut self.drones).await
    }

    /// Execute a single tick of mission execution
    /// Returns Ok(true) if mission is still running, Ok(false) if completed
    pub fn tick_mission_by_id(&mut self, mission_id: &str) -> Result<bool, String> {
        self.mission_executor.tick_mission_execution(mission_id, &mut self.drones)
    }

    pub async fn execute_mission(&mut self, target: Position) {
        let drone_ids: Vec<String> = self.drones.keys().cloned().collect();
        
        if drone_ids.is_empty() {
            println!("No drones available for mission");
            return;
        }

        let mission_id = self.mission_executor.create_mission(
            MissionType::MoveTo(target),
            drone_ids.clone()
        );

        if let Err(e) = self.mission_executor.start_mission(&mission_id) {
            println!("Failed to start mission: {}", e);
            return;
        }

        println!("Executing mission to ({:.1}, {:.1}, {:.1}) with {} drones", 
                target.x, target.y, target.z, drone_ids.len());

        if let Err(e) = self.mission_executor.execute_mission(&mission_id, &mut self.drones).await {
            println!("Mission failed: {}", e);
        }
    }

    pub async fn execute_patrol_mission(&mut self, waypoints: Vec<Position>) {
        let drone_ids: Vec<String> = self.drones.keys().cloned().collect();
        
        if drone_ids.is_empty() {
            println!("No drones available for patrol mission");
            return;
        }

        let mission_id = self.mission_executor.create_mission(
            MissionType::Patrol(waypoints.clone()),
            drone_ids.clone()
        );

        if let Err(e) = self.mission_executor.start_mission(&mission_id) {
            println!("Failed to start patrol mission: {}", e);
            return;
        }

        println!("Executing patrol mission with {} waypoints and {} drones", 
                waypoints.len(), drone_ids.len());

        if let Err(e) = self.mission_executor.execute_mission(&mission_id, &mut self.drones).await {
            println!("Patrol mission failed: {}", e);
        }
    }

    pub async fn execute_search_mission(&mut self, center: Position, radius: f64) {
        let drone_ids: Vec<String> = self.drones.keys().cloned().collect();
        
        if drone_ids.is_empty() {
            println!("No drones available for search mission");
            return;
        }

        let mission_id = self.mission_executor.create_mission(
            MissionType::Search(center, radius),
            drone_ids.clone()
        );

        if let Err(e) = self.mission_executor.start_mission(&mission_id) {
            println!("Failed to start search mission: {}", e);
            return;
        }

        println!("Executing search mission at ({:.1}, {:.1}, {:.1}) with radius {:.1} using {} drones", 
                center.x, center.y, center.z, radius, drone_ids.len());

        if let Err(e) = self.mission_executor.execute_mission(&mission_id, &mut self.drones).await {
            println!("Search mission failed: {}", e);
        }
    }

    pub fn get_swarm_status(&self) -> Vec<DroneStatusInfo> {
        self.drones.values().map(|drone| drone.get_status_info()).collect()
    }

    pub fn update_swarm(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f64();
        
        // Update all drone positions
        for drone in self.drones.values_mut() {
            drone.update_position(dt);
        }

        // Update formation if needed
        if self.formation_manager.is_formation_stable(&self.drones) {
            // Maintain formation
            self.formation_manager.update_formation(&mut self.drones);
        }

        self.last_update = now;
    }

    pub async fn start_simulation(&mut self) {
        self.simulation_running = true;
        println!("Starting swarm simulation with {} drones", self.drones.len());
        
        self.print_status();
        
        // Run simulation loop
        let mut iteration = 0;
        while self.simulation_running && iteration < 100 { // Limit iterations for demo
            self.update_swarm();
            
            // Print status every 10 iterations
            if iteration % 10 == 0 {
                self.print_status();
            }

            sleep(Duration::from_millis(100)).await;
            iteration += 1;
        }
        
        println!("Simulation ended");
    }

    pub fn stop_simulation(&mut self) {
        self.simulation_running = false;
    }

    fn print_status(&self) {
        println!("\n=== Swarm Status ===");
        for (i, drone_info) in self.get_swarm_status().iter().enumerate() {
            println!("Drone {}: {} at ({:.1}, {:.1}, {:.1}) - Status: {:?}", 
                    i + 1,
                    drone_info.id,
                    drone_info.position.x,
                    drone_info.position.y,
                    drone_info.position.z,
                    drone_info.status);
        }
        
        let formation_stable = self.formation_manager.is_formation_stable(&self.drones);
        println!("Formation stable: {}", formation_stable);
        println!("Active missions: {}", self.mission_executor.list_active_missions().len());
        println!("==================\n");
    }

    pub fn demonstrate_capabilities(&mut self) {
        println!("\n=== Drone Swarm Capabilities ===");
        println!("✓ 3 Drone Management");
        println!("✓ Formation Control (Triangle, Line, V-Formation)");
        println!("✓ Collaborative Navigation");
        println!("✓ Mission Execution (MoveTo, Patrol, Search)");
        println!("✓ Real-time Status Monitoring");
        println!("✓ Collision Avoidance (Basic)");
        println!("================================\n");
    }
}