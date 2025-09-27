use crate::drone::{Position, Drone, DroneStatus};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, PartialEq)]
pub enum MissionType {
    MoveTo(Position),
    Patrol(Vec<Position>),
    Search(Position, f64), // center position and radius
}

#[derive(Debug, Clone, PartialEq)]
pub enum MissionStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
}

pub struct Mission {
    pub id: String,
    pub mission_type: MissionType,
    pub status: MissionStatus,
    pub assigned_drones: Vec<String>,
    pub waypoints: Vec<Position>,
    pub current_waypoint: usize,
}

impl Mission {
    pub fn new(id: String, mission_type: MissionType) -> Self {
        let waypoints = match &mission_type {
            MissionType::MoveTo(pos) => vec![*pos],
            MissionType::Patrol(positions) => positions.clone(),
            MissionType::Search(center, radius) => {
                // Generate circular search pattern
                let mut search_points = Vec::new();
                let num_points = 8;
                for i in 0..num_points {
                    let angle = 2.0 * std::f64::consts::PI * (i as f64) / (num_points as f64);
                    let x = center.x + radius * angle.cos();
                    let y = center.y + radius * angle.sin();
                    search_points.push(Position::new(x, y, center.z));
                }
                search_points
            }
        };

        Self {
            id,
            mission_type,
            status: MissionStatus::NotStarted,
            assigned_drones: Vec::new(),
            waypoints,
            current_waypoint: 0,
        }
    }

    pub fn assign_drones(&mut self, drone_ids: Vec<String>) {
        self.assigned_drones = drone_ids;
    }

    pub fn start(&mut self) {
        if !self.assigned_drones.is_empty() && !self.waypoints.is_empty() {
            self.status = MissionStatus::InProgress;
            self.current_waypoint = 0;
        } else {
            self.status = MissionStatus::Failed("No drones assigned or no waypoints".to_string());
        }
    }

    pub fn get_current_target(&self) -> Option<Position> {
        if self.current_waypoint < self.waypoints.len() {
            Some(self.waypoints[self.current_waypoint])
        } else {
            None
        }
    }

    pub fn advance_waypoint(&mut self) -> bool {
        if self.current_waypoint < self.waypoints.len() - 1 {
            self.current_waypoint += 1;
            true
        } else {
            self.status = MissionStatus::Completed;
            false
        }
    }
}

pub struct MissionExecutor {
    active_missions: HashMap<String, Mission>,
    mission_counter: u32,
}

impl MissionExecutor {
    pub fn new() -> Self {
        Self {
            active_missions: HashMap::new(),
            mission_counter: 0,
        }
    }

    pub fn create_mission(&mut self, mission_type: MissionType, drone_ids: Vec<String>) -> String {
        self.mission_counter += 1;
        let mission_id = format!("mission_{}", self.mission_counter);
        
        let mut mission = Mission::new(mission_id.clone(), mission_type);
        mission.assign_drones(drone_ids);
        
        self.active_missions.insert(mission_id.clone(), mission);
        mission_id
    }

    pub fn start_mission(&mut self, mission_id: &str) -> Result<(), String> {
        if let Some(mission) = self.active_missions.get_mut(mission_id) {
            mission.start();
            Ok(())
        } else {
            Err("Mission not found".to_string())
        }
    }

    pub async fn execute_mission(
        &mut self,
        mission_id: &str,
        drones: &mut HashMap<String, Drone>,
    ) -> Result<(), String> {
        if !self.active_missions.contains_key(mission_id) {
            return Err("Mission not found".to_string());
        }

        loop {
            let mission = self.active_missions.get(mission_id).unwrap();
            
            match mission.status {
                MissionStatus::NotStarted => {
                    return Err("Mission not started".to_string());
                }
                MissionStatus::Completed => {
                    println!("Mission {} completed successfully", mission_id);
                    break;
                }
                MissionStatus::Failed(ref reason) => {
                    println!("Mission {} failed: {}", mission_id, reason);
                    return Err(reason.clone());
                }
                MissionStatus::InProgress => {
                    // Continue execution
                }
            }

            let current_target = mission.get_current_target();
            let assigned_drones = mission.assigned_drones.clone();

            if let Some(target) = current_target {
                // Move all assigned drones to current waypoint
                for drone_id in &assigned_drones {
                    if let Some(drone) = drones.get_mut(drone_id) {
                        drone.status = DroneStatus::ExecutingMission;
                        drone.move_to(target);
                    }
                }

                // Wait for drones to reach waypoint
                loop {
                    let mut all_arrived = true;
                    
                    for drone_id in &assigned_drones {
                        if let Some(drone) = drones.get_mut(drone_id) {
                            drone.update_position(0.1);
                            
                            let distance_to_target = drone.position.distance_to(&target);
                            if distance_to_target > 1.0 {
                                all_arrived = false;
                            }
                        }
                    }

                    if all_arrived {
                        println!("All drones reached waypoint: ({:.1}, {:.1}, {:.1})", 
                                target.x, target.y, target.z);
                        break;
                    }

                    sleep(Duration::from_millis(100)).await;
                }

                // Advance to next waypoint or complete mission
                if let Some(mission) = self.active_missions.get_mut(mission_id) {
                    if !mission.advance_waypoint() {
                        // Mission completed, set drones back to idle
                        for drone_id in &assigned_drones {
                            if let Some(drone) = drones.get_mut(drone_id) {
                                drone.status = DroneStatus::Idle;
                            }
                        }
                        break;
                    }
                }
            } else {
                // No more waypoints, complete mission
                if let Some(mission) = self.active_missions.get_mut(mission_id) {
                    mission.status = MissionStatus::Completed;
                }
            }

            sleep(Duration::from_millis(50)).await;
        }

        Ok(())
    }

    pub fn get_mission_status(&self, mission_id: &str) -> Option<&MissionStatus> {
        self.active_missions.get(mission_id).map(|m| &m.status)
    }

    pub fn list_active_missions(&self) -> Vec<String> {
        self.active_missions.keys().cloned().collect()
    }

    pub fn cancel_mission(&mut self, mission_id: &str) -> Result<(), String> {
        if let Some(mission) = self.active_missions.get_mut(mission_id) {
            mission.status = MissionStatus::Failed("Cancelled by user".to_string());
            Ok(())
        } else {
            Err("Mission not found".to_string())
        }
    }
}