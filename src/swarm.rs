use crate::drone::{Drone, Position, DroneStatusInfo};
use crate::formation::{FormationManager, FormationType};
use crate::mission::{MissionExecutor, MissionType};
use crate::ports::CommandDispatcher;
use crate::simulation::{SimulationEngine, SimulationMode, InternalSimulationEngine, GazeboSimulationEngine, Ros2SimulationEngine, SimulationConfig};
use crate::simulation::{InternalCommandDispatcher, GazeboCommandDispatcher, Ros2CommandDispatcher};
use std::collections::HashMap;
use tokio::time::{sleep, Duration, Instant};

pub struct DroneSwarm {
    pub drones: HashMap<String, Drone>,
    pub formation_manager: FormationManager,
    pub mission_executor: MissionExecutor,
    pub simulation_running: bool,
    last_update: Instant,

    // Simulation engine (state sync) and command dispatcher (send commands) — two separate ports
    simulation_engine: Box<dyn SimulationEngine>,
    simulation_mode: SimulationMode,
    dispatcher: Box<dyn CommandDispatcher>,
}

impl DroneSwarm {
    pub fn new() -> Self {
        let engine = Box::new(InternalSimulationEngine::new());
        let dispatcher = Box::new(InternalCommandDispatcher);
        Self::new_with_engine_and_dispatcher(engine, dispatcher)
    }

    pub fn new_with_engine(engine: Box<dyn SimulationEngine>) -> Self {
        let dispatcher = Box::new(InternalCommandDispatcher);
        Self::new_with_engine_and_dispatcher(engine, dispatcher)
    }

    pub fn new_with_engine_and_dispatcher(
        engine: Box<dyn SimulationEngine>,
        dispatcher: Box<dyn CommandDispatcher>,
    ) -> Self {
        let mode = engine.mode();
        Self {
            drones: HashMap::new(),
            formation_manager: FormationManager::new(),
            mission_executor: MissionExecutor::new(),
            simulation_running: false,
            last_update: Instant::now(),
            simulation_engine: engine,
            simulation_mode: mode,
            dispatcher,
        }
    }

    pub fn add_drone(&mut self, drone_id: &str, initial_position: Position) {
        let drone = Drone::new(drone_id.to_string(), initial_position);
        self.drones.insert(drone_id.to_string(), drone);
        self.formation_manager.add_drone(drone_id.to_string());
        
        println!("Added drone '{}' at position ({:.1}, {:.1}, {:.1})", 
                drone_id, initial_position.x, initial_position.y, initial_position.z);
    }

    pub async fn set_formation(&mut self, formation_type: &str) {
        if let Some(formation) = FormationType::from_str(formation_type) {
            self.formation_manager.set_formation_type(formation);

            // Move all drones to formation positions
            self.formation_manager.update_formation(&mut self.drones);

            // Send commands to simulation engine
            for (drone_id, drone) in &self.drones {
                if let Some(target) = drone.target_position {
                    if let Err(e) = self.dispatcher.send_command(drone_id, target).await {
                        tracing::error!("Failed to send formation command to {}: {}", drone_id, e);
                    }
                }
            }

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

        // Execute mission with simulation engine integration
        let mut iteration = 0;
        let max_iterations = 1000; // Safety limit to prevent infinite loops

        loop {
            // Update drone positions from simulation engine (Gazebo or internal)
            self.update_swarm().await;

            // Send commands to simulation engine for drones with targets
            for (drone_id, drone) in &self.drones {
                if let Some(target) = drone.target_position {
                    if let Err(e) = self.dispatcher.send_command(drone_id, target).await {
                        tracing::error!("Failed to send command to {}: {}", drone_id, e);
                    }
                }
            }

            // Advance mission by one tick
            match self.tick_mission_by_id(&mission_id) {
                Ok(true) => {
                    // Mission still in progress
                    if iteration % 10 == 0 {
                        tracing::debug!("Mission in progress, iteration {}", iteration);
                    }
                }
                Ok(false) => {
                    // Mission completed successfully
                    println!("Mission completed successfully!");
                    break;
                }
                Err(e) => {
                    // Mission failed
                    println!("Mission failed: {}", e);
                    break;
                }
            }

            iteration += 1;
            if iteration >= max_iterations {
                println!("Mission timeout after {} iterations", max_iterations);
                break;
            }

            // Sleep to match simulation update rate
            sleep(Duration::from_millis(100)).await;
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

        // Execute mission with simulation engine integration
        let mut iteration = 0;
        let max_iterations = 2000; // Higher limit for patrol missions

        loop {
            // Update drone positions from simulation engine (Gazebo or internal)
            self.update_swarm().await;

            // Send commands to simulation engine for drones with targets
            for (drone_id, drone) in &self.drones {
                if let Some(target) = drone.target_position {
                    if let Err(e) = self.dispatcher.send_command(drone_id, target).await {
                        tracing::error!("Failed to send command to {}: {}", drone_id, e);
                    }
                }
            }

            // Advance mission by one tick
            match self.tick_mission_by_id(&mission_id) {
                Ok(true) => {
                    // Mission still in progress
                    if iteration % 10 == 0 {
                        tracing::debug!("Patrol mission in progress, iteration {}", iteration);
                    }
                }
                Ok(false) => {
                    // Mission completed successfully
                    println!("Patrol mission completed successfully!");
                    break;
                }
                Err(e) => {
                    // Mission failed
                    println!("Patrol mission failed: {}", e);
                    break;
                }
            }

            iteration += 1;
            if iteration >= max_iterations {
                println!("Patrol mission timeout after {} iterations", max_iterations);
                break;
            }

            // Sleep to match simulation update rate
            sleep(Duration::from_millis(100)).await;
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

        // Execute mission with simulation engine integration
        let mut iteration = 0;
        let max_iterations = 3000; // Higher limit for search missions

        loop {
            // Update drone positions from simulation engine (Gazebo or internal)
            self.update_swarm().await;

            // Send commands to simulation engine for drones with targets
            for (drone_id, drone) in &self.drones {
                if let Some(target) = drone.target_position {
                    if let Err(e) = self.dispatcher.send_command(drone_id, target).await {
                        tracing::error!("Failed to send command to {}: {}", drone_id, e);
                    }
                }
            }

            // Advance mission by one tick
            match self.tick_mission_by_id(&mission_id) {
                Ok(true) => {
                    // Mission still in progress
                    if iteration % 10 == 0 {
                        tracing::debug!("Search mission in progress, iteration {}", iteration);
                    }
                }
                Ok(false) => {
                    // Mission completed successfully
                    println!("Search mission completed successfully!");
                    break;
                }
                Err(e) => {
                    // Mission failed
                    println!("Search mission failed: {}", e);
                    break;
                }
            }

            iteration += 1;
            if iteration >= max_iterations {
                println!("Search mission timeout after {} iterations", max_iterations);
                break;
            }

            // Sleep to match simulation update rate
            sleep(Duration::from_millis(100)).await;
        }
    }

    pub fn get_swarm_status(&self) -> Vec<DroneStatusInfo> {
        self.drones.values().map(|drone| drone.get_status_info()).collect()
    }

    /// Get the current simulation mode
    pub fn get_simulation_mode(&self) -> SimulationMode {
        self.simulation_mode
    }

    /// Check if simulation engine is connected
    pub fn is_engine_connected(&self) -> bool {
        self.simulation_engine.is_connected()
    }

    /// Switch between simulation modes (internal vs Gazebo)
    pub async fn switch_mode(&mut self, new_mode: SimulationMode, config: &SimulationConfig) -> Result<(), String> {
        if self.simulation_mode == new_mode {
            tracing::info!("Already in {:?} mode", new_mode);
            return Ok(()); // Already in this mode
        }

        tracing::info!("Switching from {:?} to {:?} mode", self.simulation_mode, new_mode);

        // Shutdown current engine
        self.simulation_engine.shutdown().await?;

        // Create new engine and dispatcher based on mode
        let (mut new_engine, new_dispatcher): (Box<dyn SimulationEngine>, Box<dyn CommandDispatcher>) = match new_mode {
            SimulationMode::Internal => (
                Box::new(InternalSimulationEngine::new()),
                Box::new(InternalCommandDispatcher),
            ),
            SimulationMode::Gazebo => (
                Box::new(GazeboSimulationEngine::new(
                    config.gazebo.bridge_url.clone(),
                    config.gazebo.timeout_ms,
                )),
                Box::new(GazeboCommandDispatcher::new(
                    config.gazebo.bridge_url.clone(),
                    config.gazebo.timeout_ms,
                )),
            ),
            SimulationMode::Ros2 => (
                Box::new(Ros2SimulationEngine::new(
                    config.ros2.bridge_url.clone(),
                    config.ros2.timeout_ms,
                )),
                Box::new(Ros2CommandDispatcher::new(
                    config.ros2.bridge_url.clone(),
                    config.ros2.timeout_ms,
                )),
            ),
        };

        // Initialize new engine
        new_engine.initialize().await?;

        self.simulation_engine = new_engine;
        self.dispatcher = new_dispatcher;
        self.simulation_mode = new_mode;

        tracing::info!("Successfully switched to {:?} simulation mode", new_mode);
        Ok(())
    }

    pub async fn update_swarm(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f64();

        // Delegate position updates to simulation engine
        if let Err(e) = self.simulation_engine.update_drones(&mut self.drones, dt).await {
            tracing::error!("Simulation engine update error: {}", e);
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
            self.update_swarm().await;
            
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