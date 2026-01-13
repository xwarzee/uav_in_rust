use crate::drone::{Position, Drone};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FormationType {
    Triangle,
    Line,
    VFormation,
}

impl FormationType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "triangle" => Some(FormationType::Triangle),
            "line" => Some(FormationType::Line),
            "v_formation" => Some(FormationType::VFormation),
            _ => None,
        }
    }
}

pub struct FormationManager {
    pub formation_type: FormationType,
    leader_position: Position,
    formation_offsets: HashMap<String, Position>,
    pub separation_distance: f64,
}

impl FormationManager {
    pub fn new() -> Self {
        Self {
            formation_type: FormationType::Triangle,
            leader_position: Position::new(0.0, 0.0, 0.0),
            formation_offsets: HashMap::new(),
            separation_distance: 10.0,
        }
    }

    pub fn set_formation_type(&mut self, formation_type: FormationType) {
        self.formation_type = formation_type;
        self.calculate_offsets();
    }

    pub fn set_leader_position(&mut self, position: Position) {
        self.leader_position = position;
    }

    pub fn set_separation_distance(&mut self, distance: f64) {
        self.separation_distance = distance;
        self.calculate_offsets();
    }

    pub fn add_drone(&mut self, drone_id: String) {
        self.formation_offsets.insert(drone_id, Position::new(0.0, 0.0, 0.0));
        self.calculate_offsets();
    }

    fn calculate_offsets(&mut self) {
        let drone_ids: Vec<String> = self.formation_offsets.keys().cloned().collect();
        
        match self.formation_type {
            FormationType::Triangle => self.calculate_triangle_formation(&drone_ids),
            FormationType::Line => self.calculate_line_formation(&drone_ids),
            FormationType::VFormation => self.calculate_v_formation(&drone_ids),
        }
    }

    fn calculate_triangle_formation(&mut self, drone_ids: &[String]) {
        let positions = vec![
            Position::new(0.0, 0.0, 0.0),  // Leader at center
            Position::new(-self.separation_distance, -self.separation_distance * 0.866, 0.0), // Left rear
            Position::new(self.separation_distance, -self.separation_distance * 0.866, 0.0),  // Right rear
        ];

        for (i, drone_id) in drone_ids.iter().enumerate() {
            if i < positions.len() {
                self.formation_offsets.insert(drone_id.clone(), positions[i]);
            }
        }
    }

    fn calculate_line_formation(&mut self, drone_ids: &[String]) {
        for (i, drone_id) in drone_ids.iter().enumerate() {
            let offset = Position::new(
                (i as f64 - 1.0) * self.separation_distance,
                0.0,
                0.0,
            );
            self.formation_offsets.insert(drone_id.clone(), offset);
        }
    }

    fn calculate_v_formation(&mut self, drone_ids: &[String]) {
        let positions = vec![
            Position::new(0.0, 0.0, 0.0),  // Leader at front
            Position::new(-self.separation_distance, -self.separation_distance, 0.0), // Left wing
            Position::new(self.separation_distance, -self.separation_distance, 0.0),  // Right wing
        ];

        for (i, drone_id) in drone_ids.iter().enumerate() {
            if i < positions.len() {
                self.formation_offsets.insert(drone_id.clone(), positions[i]);
            }
        }
    }

    pub fn get_target_position(&self, drone_id: &str) -> Option<Position> {
        self.formation_offsets.get(drone_id).map(|offset| {
            self.leader_position.add(offset)
        })
    }

    pub fn update_formation(&mut self, drones: &mut HashMap<String, Drone>) {
        // Find the leader (first drone)
        if let Some(leader_id) = drones.keys().next().cloned() {
            if let Some(leader_drone) = drones.get(&leader_id) {
                self.leader_position = leader_drone.position;
            }
        }

        // Update all drones to maintain formation
        for (drone_id, drone) in drones.iter_mut() {
            if let Some(target_pos) = self.get_target_position(drone_id) {
                let distance_to_target = drone.position.distance_to(&target_pos);
                
                // Only move if drone is significantly out of position
                if distance_to_target > 1.0 {
                    drone.move_to(target_pos);
                } else {
                    drone.set_formation_offset(self.formation_offsets[drone_id]);
                }
            }
        }
    }

    pub fn is_formation_stable(&self, drones: &HashMap<String, Drone>) -> bool {
        for (drone_id, drone) in drones {
            if let Some(target_pos) = self.get_target_position(drone_id) {
                let distance = drone.position.distance_to(&target_pos);
                if distance > 2.0 {
                    return false;
                }
            }
        }
        true
    }
}