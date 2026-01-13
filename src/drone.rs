use serde::{Deserialize, Serialize};
use std::time::Instant;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    pub fn add(&self, other: &Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Position) -> Position {
        Position::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn normalize(&self) -> Position {
        let magnitude = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        if magnitude > 0.0 {
            Position::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
        } else {
            *self
        }
    }

    pub fn scale(&self, factor: f64) -> Position {
        Position::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Velocity {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

impl Velocity {
    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self { vx, vy, vz }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.vx.powi(2) + self.vy.powi(2) + self.vz.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum DroneStatus {
    Idle,
    Navigating,
    InFormation,
    ExecutingMission,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Drone {
    pub id: String,
    pub position: Position,
    pub velocity: Velocity,
    pub status: DroneStatus,
    pub target_position: Option<Position>,
    pub formation_offset: Option<Position>,
    pub max_speed: f64,
    pub last_update: Instant,
}

impl Drone {
    pub fn new(id: String, initial_position: Position) -> Self {
        Self {
            id,
            position: initial_position,
            velocity: Velocity::zero(),
            status: DroneStatus::Idle,
            target_position: None,
            formation_offset: None,
            max_speed: 5.0,
            last_update: Instant::now(),
        }
    }

    pub fn move_to(&mut self, target: Position) {
        self.target_position = Some(target);
        self.status = DroneStatus::Navigating;
    }

    pub fn set_formation_offset(&mut self, offset: Position) {
        self.formation_offset = Some(offset);
        self.status = DroneStatus::InFormation;
    }

    pub fn update_position(&mut self, dt: f64) {
        if let Some(target) = self.target_position {
            let direction = target.subtract(&self.position);
            let distance = direction.x.powi(2) + direction.y.powi(2) + direction.z.powi(2);
            let distance = distance.sqrt();

            if distance < 0.1 {
                self.position = target;
                self.velocity = Velocity::zero();
                self.status = DroneStatus::Idle;
                self.target_position = None;
            } else {
                let speed = self.max_speed.min(distance / dt);
                let normalized_direction = direction.normalize();
                
                self.velocity = Velocity::new(
                    normalized_direction.x * speed,
                    normalized_direction.y * speed,
                    normalized_direction.z * speed,
                );

                self.position.x += self.velocity.vx * dt;
                self.position.y += self.velocity.vy * dt;
                self.position.z += self.velocity.vz * dt;
            }
        }
        
        self.last_update = Instant::now();
    }

    pub fn get_status_info(&self) -> DroneStatusInfo {
        DroneStatusInfo {
            id: self.id.clone(),
            position: self.position,
            velocity: self.velocity,
            status: self.status.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DroneStatusInfo {
    pub id: String,
    pub position: Position,
    pub velocity: Velocity,
    pub status: DroneStatus,
}