// Software Unit Tests
//
// This test suite contains pure software unit tests that validate
// the internal logic and behavior of individual components WITHOUT
// reference to MBSE specifications.
//
// These are traditional software tests focused on:
// - Code correctness
// - Edge cases
// - Error handling
// - Performance characteristics
// - Internal implementation details

use uav_swarm::drone::{Drone, Position, Velocity, DroneStatus};
use uav_swarm::formation::{FormationManager, FormationType};
use std::collections::HashMap;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// POSITION UNIT TESTS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_position_distance_calculation() {
    let p1 = Position::new(0.0, 0.0, 0.0);
    let p2 = Position::new(3.0, 4.0, 0.0);

    let distance = p1.distance_to(&p2);
    assert_eq!(distance, 5.0, "3-4-5 triangle should have distance 5.0");
}

#[test]
fn test_position_distance_3d() {
    let p1 = Position::new(0.0, 0.0, 0.0);
    let p2 = Position::new(1.0, 1.0, 1.0);

    let distance = p1.distance_to(&p2);
    let expected = (3.0_f64).sqrt();
    assert!((distance - expected).abs() < 0.0001,
        "3D distance calculation: expected {}, got {}", expected, distance);
}

#[test]
fn test_position_add() {
    let p1 = Position::new(1.0, 2.0, 3.0);
    let p2 = Position::new(4.0, 5.0, 6.0);

    let result = p1.add(&p2);
    assert_eq!(result.x, 5.0);
    assert_eq!(result.y, 7.0);
    assert_eq!(result.z, 9.0);
}

#[test]
fn test_position_subtract() {
    let p1 = Position::new(10.0, 8.0, 6.0);
    let p2 = Position::new(1.0, 2.0, 3.0);

    let result = p1.subtract(&p2);
    assert_eq!(result.x, 9.0);
    assert_eq!(result.y, 6.0);
    assert_eq!(result.z, 3.0);
}

#[test]
fn test_position_normalize() {
    let p = Position::new(3.0, 4.0, 0.0);
    let normalized = p.normalize();

    assert_eq!(normalized.x, 0.6);
    assert_eq!(normalized.y, 0.8);

    // Magnitude should be 1.0
    let magnitude = (normalized.x.powi(2) + normalized.y.powi(2) + normalized.z.powi(2)).sqrt();
    assert!((magnitude - 1.0).abs() < 0.0001, "Normalized vector should have magnitude 1.0");
}

#[test]
fn test_position_normalize_zero_vector() {
    let p = Position::new(0.0, 0.0, 0.0);
    let normalized = p.normalize();

    // Should return the same zero vector
    assert_eq!(normalized.x, 0.0);
    assert_eq!(normalized.y, 0.0);
    assert_eq!(normalized.z, 0.0);
}

#[test]
fn test_position_scale() {
    let p = Position::new(2.0, 3.0, 4.0);
    let scaled = p.scale(2.5);

    assert_eq!(scaled.x, 5.0);
    assert_eq!(scaled.y, 7.5);
    assert_eq!(scaled.z, 10.0);
}

#[test]
fn test_position_scale_negative() {
    let p = Position::new(1.0, 2.0, 3.0);
    let scaled = p.scale(-1.0);

    assert_eq!(scaled.x, -1.0);
    assert_eq!(scaled.y, -2.0);
    assert_eq!(scaled.z, -3.0);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// VELOCITY UNIT TESTS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_velocity_magnitude() {
    let v = Velocity::new(3.0, 4.0, 0.0);
    assert_eq!(v.magnitude(), 5.0);
}

#[test]
fn test_velocity_magnitude_3d() {
    let v = Velocity::new(1.0, 2.0, 2.0);
    let expected = 3.0; // sqrt(1 + 4 + 4) = sqrt(9) = 3
    assert_eq!(v.magnitude(), expected);
}

#[test]
fn test_velocity_zero() {
    let v = Velocity::zero();
    assert_eq!(v.vx, 0.0);
    assert_eq!(v.vy, 0.0);
    assert_eq!(v.vz, 0.0);
    assert_eq!(v.magnitude(), 0.0);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// DRONE UNIT TESTS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_drone_creation() {
    let pos = Position::new(1.0, 2.0, 3.0);
    let drone = Drone::new("test-drone".to_string(), pos);

    assert_eq!(drone.id, "test-drone");
    assert_eq!(drone.position.x, 1.0);
    assert_eq!(drone.position.y, 2.0);
    assert_eq!(drone.position.z, 3.0);
    assert_eq!(drone.velocity.magnitude(), 0.0);
    assert!(matches!(drone.status, DroneStatus::Idle));
    assert_eq!(drone.max_speed, 5.0);
}

#[test]
fn test_drone_move_to_sets_target() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    let target = Position::new(10.0, 10.0, 10.0);

    drone.move_to(target);

    assert!(drone.target_position.is_some());
    assert_eq!(drone.target_position.unwrap().x, 10.0);
    assert!(matches!(drone.status, DroneStatus::Navigating));
}

#[test]
fn test_drone_set_formation_offset() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    let offset = Position::new(5.0, 5.0, 0.0);

    drone.set_formation_offset(offset);

    assert!(drone.formation_offset.is_some());
    assert_eq!(drone.formation_offset.unwrap().x, 5.0);
    assert!(matches!(drone.status, DroneStatus::InFormation));
}

#[test]
fn test_drone_update_position_with_no_target() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    let initial_pos = drone.position;

    drone.update_position(0.1);

    // Position should not change without a target
    assert_eq!(drone.position.x, initial_pos.x);
    assert_eq!(drone.position.y, initial_pos.y);
    assert_eq!(drone.position.z, initial_pos.z);
}

#[test]
fn test_drone_reaches_target() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    let target = Position::new(0.05, 0.0, 0.0); // Very close target

    drone.move_to(target);
    drone.update_position(0.1);

    // Should reach target and transition to Idle
    assert!(matches!(drone.status, DroneStatus::Idle));
    assert!(drone.target_position.is_none());
    assert_eq!(drone.velocity.magnitude(), 0.0);
}

#[test]
fn test_drone_velocity_respects_max_speed() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    let target = Position::new(1000.0, 1000.0, 1000.0); // Very distant target

    drone.move_to(target);

    for _ in 0..10 {
        drone.update_position(0.1);
        let speed = drone.velocity.magnitude();
        assert!(speed <= drone.max_speed + 0.001,
            "Velocity {} exceeds max_speed {}", speed, drone.max_speed);
    }
}

#[test]
fn test_drone_get_status_info() {
    let drone = Drone::new("test-123".to_string(), Position::new(1.0, 2.0, 3.0));
    let status_info = drone.get_status_info();

    assert_eq!(status_info.id, "test-123");
    assert_eq!(status_info.position.x, 1.0);
    assert!(matches!(status_info.status, DroneStatus::Idle));
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// FORMATION MANAGER UNIT TESTS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_formation_manager_creation() {
    let _manager = FormationManager::new();
    // Just verify it can be created
    assert!(true);
}

#[test]
fn test_formation_manager_add_drone() {
    let mut manager = FormationManager::new();
    manager.add_drone("drone1".to_string());

    let pos = manager.get_target_position("drone1");
    assert!(pos.is_some(), "Added drone should have a target position");
}

#[test]
fn test_formation_manager_multiple_drones() {
    let mut manager = FormationManager::new();

    for i in 1..=5 {
        manager.add_drone(format!("drone{}", i));
    }

    // All drones should have positions
    for i in 1..=5 {
        let pos = manager.get_target_position(&format!("drone{}", i));
        assert!(pos.is_some(), "Drone {} should have position", i);
    }
}

#[test]
fn test_formation_manager_set_formation_type() {
    let mut manager = FormationManager::new();
    manager.add_drone("d1".to_string());
    manager.add_drone("d2".to_string());

    // Test all formation types
    for formation in [FormationType::Triangle, FormationType::Line, FormationType::VFormation] {
        manager.set_formation_type(formation);

        let pos1 = manager.get_target_position("d1");
        let pos2 = manager.get_target_position("d2");

        assert!(pos1.is_some());
        assert!(pos2.is_some());
    }
}

#[test]
fn test_formation_manager_separation_distance() {
    let mut manager = FormationManager::new();
    manager.add_drone("d1".to_string());
    manager.add_drone("d2".to_string());

    // Test different separation distances
    for separation in [5.0, 10.0, 20.0, 50.0] {
        manager.set_separation_distance(separation);
        manager.set_formation_type(FormationType::Line);

        let pos1 = manager.get_target_position("d1").unwrap();
        let pos2 = manager.get_target_position("d2").unwrap();

        let distance = pos1.distance_to(&pos2);
        assert!(distance >= separation * 0.9,
            "Distance {} should be close to separation {}", distance, separation);
    }
}

#[test]
fn test_formation_manager_leader_position() {
    let mut manager = FormationManager::new();
    manager.add_drone("d1".to_string());

    let leader_pos = Position::new(100.0, 200.0, 50.0);
    manager.set_leader_position(leader_pos);

    let drone_pos = manager.get_target_position("d1").unwrap();

    // First drone should be at or near leader position (depending on formation)
    let distance = drone_pos.distance_to(&leader_pos);
    assert!(distance < 50.0, "Drone should be within reasonable distance of leader");
}

#[test]
fn test_formation_manager_is_formation_stable_empty() {
    let manager = FormationManager::new();
    let drones = HashMap::new();

    assert!(manager.is_formation_stable(&drones),
        "Empty formation should be considered stable");
}

#[test]
fn test_formation_manager_is_formation_stable_single_drone() {
    let mut manager = FormationManager::new();
    manager.add_drone("d1".to_string());

    let mut drones = HashMap::new();
    let target = manager.get_target_position("d1").unwrap();
    drones.insert("d1".to_string(), Drone::new("d1".to_string(), target));

    assert!(manager.is_formation_stable(&drones),
        "Single drone at target should be stable");
}

#[test]
fn test_formation_type_from_str() {
    assert_eq!(FormationType::from_str("triangle"), Some(FormationType::Triangle));
    assert_eq!(FormationType::from_str("Triangle"), Some(FormationType::Triangle));
    assert_eq!(FormationType::from_str("TRIANGLE"), Some(FormationType::Triangle));

    assert_eq!(FormationType::from_str("line"), Some(FormationType::Line));
    assert_eq!(FormationType::from_str("v_formation"), Some(FormationType::VFormation));

    assert_eq!(FormationType::from_str("invalid"), None);
    assert_eq!(FormationType::from_str(""), None);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// EDGE CASES AND ERROR HANDLING
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_position_with_very_large_coordinates() {
    let p1 = Position::new(1e10, 1e10, 1e10);
    let p2 = Position::new(1e10 + 1.0, 1e10, 1e10);

    let distance = p1.distance_to(&p2);
    assert!((distance - 1.0).abs() < 0.001,
        "Should handle large coordinates accurately");
}

#[test]
fn test_position_with_negative_coordinates() {
    let p1 = Position::new(-10.0, -20.0, -5.0);
    let p2 = Position::new(-5.0, -10.0, 0.0);

    let result = p1.add(&p2);
    assert_eq!(result.x, -15.0);
    assert_eq!(result.y, -30.0);
    assert_eq!(result.z, -5.0);
}

#[test]
fn test_drone_with_zero_max_speed() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    drone.max_speed = 0.0;

    drone.move_to(Position::new(10.0, 10.0, 10.0));
    drone.update_position(0.1);

    // Drone should not move with zero max speed
    assert_eq!(drone.velocity.magnitude(), 0.0);
}

#[test]
fn test_formation_update_with_no_drones() {
    let mut manager = FormationManager::new();
    let mut drones = HashMap::new();

    // Should not panic with empty drone set
    manager.update_formation(&mut drones);
    assert!(true);
}

#[test]
fn test_drone_status_transitions() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));

    // Idle -> Navigating
    assert!(matches!(drone.status, DroneStatus::Idle));
    drone.move_to(Position::new(10.0, 10.0, 10.0));
    assert!(matches!(drone.status, DroneStatus::Navigating));

    // Navigating -> InFormation
    drone.set_formation_offset(Position::new(5.0, 5.0, 0.0));
    assert!(matches!(drone.status, DroneStatus::InFormation));
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// PERFORMANCE AND PRECISION TESTS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn test_position_calculation_precision() {
    let p = Position::new(0.1, 0.2, 0.3);

    // Multiple operations should maintain reasonable precision
    let result = p.scale(10.0).scale(0.1);

    assert!((result.x - 0.1).abs() < 0.0001);
    assert!((result.y - 0.2).abs() < 0.0001);
    assert!((result.z - 0.3).abs() < 0.0001);
}

#[test]
fn test_drone_update_consistency() {
    let mut drone = Drone::new("test".to_string(), Position::new(0.0, 0.0, 0.0));
    drone.move_to(Position::new(10.0, 0.0, 0.0));

    // Multiple small updates should be consistent
    for _ in 0..10 {
        drone.update_position(0.01);
    }

    let pos_after_small_steps = drone.position;

    let mut drone2 = Drone::new("test2".to_string(), Position::new(0.0, 0.0, 0.0));
    drone2.move_to(Position::new(10.0, 0.0, 0.0));
    drone2.update_position(0.1);

    // Results should be reasonably close
    let diff = (pos_after_small_steps.x - drone2.position.x).abs();
    assert!(diff < 1.0, "Multiple small steps should approximate single large step");
}
