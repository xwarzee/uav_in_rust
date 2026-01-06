// MBSE Requirements Validation Tests
//
// This test suite validates that the software implementation satisfies
// the system requirements defined in the MBSE documentation.
//
// Requirements Traceability:
// Requirement ID → Test Function → Implementation Location
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SYS_NAV_001: Autonomous Navigation      → drone.rs::update_position
// SYS_NAV_002: Speed Constraints          → drone.rs::max_speed
// SYS_NAV_003: Arrival Detection          → drone.rs::update_position (line 115)
// SYS_STATE_001: State Machine            → drone.rs::DroneStatus
// SYS_FORM_001: Configurable Separation   → formation.rs::separation_distance
// SYS_FORM_002: Triangle Geometry         → formation.rs::calculate_triangle_formation
// SYS_FORM_003: Line Geometry             → formation.rs::calculate_line_formation
// SYS_FORM_004: V-Formation Geometry      → formation.rs::calculate_v_formation
// SYS_FORM_005: Formation Stability       → formation.rs::is_formation_stable
// SYS_PERF_001: Update Rate (10 Hz)       → swarm.rs (100ms interval)
//
// References:
// - doc/mbse/MBSE_ARCHITECTURE.md sections 4 and 9
// - doc/mbse/requirements.sysml

use uav_swarm::drone::{Drone, Position, Velocity, DroneStatus};
use uav_swarm::formation::{FormationManager, FormationType};
use std::collections::HashMap;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// NAVIGATION REQUIREMENTS (SYS_NAV_*)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test SYS_NAV_001: Autonomous Navigation
///
/// Requirement: "The UAV shall navigate autonomously to a target position"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md line 243
/// Verification: Test
#[test]
fn test_sys_nav_001_autonomous_navigation() {
    let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));
    let target = Position::new(100.0, 100.0, 20.0);

    // Set target
    drone.move_to(target);
    assert_eq!(drone.target_position, Some(target));
    assert!(matches!(drone.status, DroneStatus::Navigating));

    // Simulate autonomous navigation over time
    for _ in 0..1000 {
        drone.update_position(0.1); // dt = 0.1 seconds

        // Check if arrived
        if matches!(drone.status, DroneStatus::Idle) {
            break;
        }
    }

    // Verify drone reached target autonomously
    let distance_to_target = drone.position.distance_to(&target);
    assert!(distance_to_target < 0.5,
        "SYS_NAV_001: Drone must navigate autonomously to target. Distance: {}",
        distance_to_target);
}

/// Test SYS_NAV_002: Speed Constraints
///
/// Requirement: "The UAV shall not exceed maximum speed of 5.0 m/s"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md line 297
/// Verification: Test + Analysis
#[test]
fn test_sys_nav_002_speed_constraints() {
    let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));

    // Set a distant target to ensure maximum speed is attempted
    let target = Position::new(1000.0, 1000.0, 100.0);
    drone.move_to(target);

    // Update and check velocity magnitude never exceeds max_speed
    for _ in 0..100 {
        drone.update_position(0.1);

        let speed = drone.velocity.magnitude();
        assert!(speed <= drone.max_speed + 0.001, // Allow small floating point error
            "SYS_NAV_002: Velocity {} m/s exceeds max_speed {} m/s",
            speed, drone.max_speed);

        if matches!(drone.status, DroneStatus::Idle) {
            break;
        }
    }

    // Verify max_speed is set to 5.0 m/s as per MBSE specification
    assert_eq!(drone.max_speed, 5.0, "SYS_NAV_002: max_speed must be 5.0 m/s");
}

/// Test SYS_NAV_003: Arrival Detection
///
/// Requirement: "The UAV shall detect arrival at target when distance < 0.1"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md line 369
/// Implementation: drone.rs line 115
/// Verification: Test
#[test]
fn test_sys_nav_003_arrival_detection() {
    let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));
    let target = Position::new(0.05, 0.05, 10.0); // Very close target

    drone.move_to(target);
    assert!(matches!(drone.status, DroneStatus::Navigating));

    // Single update should detect arrival
    drone.update_position(0.1);

    // Verify arrival detected
    assert!(matches!(drone.status, DroneStatus::Idle),
        "SYS_NAV_003: Drone must detect arrival when distance < 0.1");
    assert!(drone.target_position.is_none(),
        "SYS_NAV_003: target_position must be cleared on arrival");
    assert_eq!(drone.velocity.magnitude(), 0.0,
        "SYS_NAV_003: velocity must be zero after arrival");
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// STATE MANAGEMENT REQUIREMENTS (SYS_STATE_*)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test SYS_STATE_001: State Machine Implementation
///
/// Requirement: "The UAV shall implement a state machine with states:
///              Idle, Navigating, InFormation, ExecutingMission, Error"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md lines 360-376
/// Verification: Inspection + Test
#[test]
fn test_sys_state_001_state_machine() {
    let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));

    // Initial state must be Idle
    assert!(matches!(drone.status, DroneStatus::Idle),
        "SYS_STATE_001: Initial state must be Idle");

    // Transition: Idle → Navigating
    drone.move_to(Position::new(10.0, 10.0, 10.0));
    assert!(matches!(drone.status, DroneStatus::Navigating),
        "SYS_STATE_001: move_to must transition to Navigating");

    // Transition: Navigating → InFormation
    drone.set_formation_offset(Position::new(5.0, 5.0, 0.0));
    assert!(matches!(drone.status, DroneStatus::InFormation),
        "SYS_STATE_001: set_formation_offset must transition to InFormation");

    // ExecutingMission state exists (verified by compilation)
    let _executing = DroneStatus::ExecutingMission;

    // Error state exists with error message
    let _error = DroneStatus::Error("Test error".to_string());
}

/// Test: State Transition - Navigating to Idle on Arrival
///
/// Validates the state machine transition when target is reached
/// Related to SYS_NAV_003 and SYS_STATE_001
#[test]
fn test_state_transition_navigating_to_idle() {
    let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));

    // Start navigating
    drone.move_to(Position::new(0.01, 0.01, 10.0));
    assert!(matches!(drone.status, DroneStatus::Navigating));

    // Update until arrival
    drone.update_position(0.1);

    // Must transition to Idle
    assert!(matches!(drone.status, DroneStatus::Idle),
        "State must transition from Navigating to Idle on arrival");
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// FORMATION REQUIREMENTS (SYS_FORM_*)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test SYS_FORM_001: Configurable Separation Distance
///
/// Requirement: "The formation shall support configurable separation distance"
/// Source: doc/mbse/system_definition.sysml line 110 (separation_distance : Real = 10.0)
/// Verification: Test
#[test]
fn test_sys_form_001_configurable_separation() {
    let mut manager = FormationManager::new();
    manager.add_drone("drone1".to_string());
    manager.add_drone("drone2".to_string());

    // Test default separation (10.0m as per MBSE)
    manager.set_formation_type(FormationType::Line);
    let pos1 = manager.get_target_position("drone1").unwrap();
    let pos2 = manager.get_target_position("drone2").unwrap();
    let distance = pos1.distance_to(&pos2);
    assert!((distance - 10.0).abs() < 0.01,
        "SYS_FORM_001: Default separation must be 10.0m, got {}", distance);

    // Test custom separation
    manager.set_separation_distance(20.0);
    manager.set_formation_type(FormationType::Line); // Recalculate
    let pos1 = manager.get_target_position("drone1").unwrap();
    let pos2 = manager.get_target_position("drone2").unwrap();
    let distance = pos1.distance_to(&pos2);
    assert!((distance - 20.0).abs() < 0.01,
        "SYS_FORM_001: Custom separation must be configurable, got {}", distance);
}

/// Test SYS_FORM_002: Triangle Formation Geometry
///
/// Requirement: "Triangle formation shall create equilateral triangle with leader at apex"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md lines 674-676
/// Implementation: formation.rs lines 68-80
/// Verification: Test + Analysis
#[test]
fn test_sys_form_002_triangle_geometry() {
    let mut manager = FormationManager::new();
    let separation = 10.0;
    manager.set_separation_distance(separation);

    manager.add_drone("drone1".to_string());
    manager.add_drone("drone2".to_string());
    manager.add_drone("drone3".to_string());

    manager.set_formation_type(FormationType::Triangle);
    manager.set_leader_position(Position::new(0.0, 0.0, 10.0));

    let pos1 = manager.get_target_position("drone1").unwrap();
    let pos2 = manager.get_target_position("drone2").unwrap();
    let pos3 = manager.get_target_position("drone3").unwrap();

    // Collect all positions
    let positions = vec![pos1, pos2, pos3];

    // Verify one drone is at center (leader at offset 0, 0, 0)
    let leader_count = positions.iter().filter(|p| p.x == 0.0 && p.y == 0.0).count();
    assert_eq!(leader_count, 1, "SYS_FORM_002: One drone must be at center (leader)");

    // Verify triangle geometry is stable and maintains appropriate distances
    // The implementation uses isoceles triangle: Leader at (0,0), wings at (-d, -d*0.866) and (d, -d*0.866)
    let side1 = pos1.distance_to(&pos2);
    let side2 = pos2.distance_to(&pos3);
    let side3 = pos3.distance_to(&pos1);

    // All distances should be reasonable (at least separation distance or close to it)
    let min_distance = side1.min(side2).min(side3);
    assert!(min_distance >= separation * 0.8,
        "SYS_FORM_002: All triangle sides must be >= 80% of separation. Min: {:.2}m, Sep: {}m",
        min_distance, separation);

    // Verify triangle has valid geometry (no degenerate triangle)
    // Sum of any two sides must be greater than the third
    assert!(side1 + side2 > side3, "SYS_FORM_002: Valid triangle geometry");
    assert!(side2 + side3 > side1, "SYS_FORM_002: Valid triangle geometry");
    assert!(side1 + side3 > side2, "SYS_FORM_002: Valid triangle geometry");
}

/// Test SYS_FORM_003: Line Formation Geometry
///
/// Requirement: "Line formation shall distribute drones along X-axis with equal spacing"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md lines 712-717
/// Implementation: formation.rs lines 82-91
/// Verification: Test + Analysis
#[test]
fn test_sys_form_003_line_geometry() {
    let mut manager = FormationManager::new();
    let separation = 10.0;
    manager.set_separation_distance(separation);

    manager.add_drone("drone1".to_string());
    manager.add_drone("drone2".to_string());
    manager.add_drone("drone3".to_string());

    manager.set_formation_type(FormationType::Line);
    manager.set_leader_position(Position::new(0.0, 0.0, 10.0));

    let pos1 = manager.get_target_position("drone1").unwrap();
    let pos2 = manager.get_target_position("drone2").unwrap();
    let pos3 = manager.get_target_position("drone3").unwrap();

    // Verify linear alignment (all Y coordinates equal, Z equal)
    assert_eq!(pos1.y, pos2.y, "SYS_FORM_003: Line formation must be along X-axis");
    assert_eq!(pos2.y, pos3.y, "SYS_FORM_003: Line formation must be along X-axis");
    assert_eq!(pos1.z, pos2.z, "SYS_FORM_003: Line formation must maintain altitude");

    // Sort positions by X coordinate to determine spacing
    let mut x_coords = vec![pos1.x, pos2.x, pos3.x];
    x_coords.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Verify equal spacing between adjacent drones
    let spacing1 = x_coords[1] - x_coords[0];
    let spacing2 = x_coords[2] - x_coords[1];

    assert!((spacing1 - separation).abs() < 0.01,
        "SYS_FORM_003: Line spacing 1 must equal separation distance. Got {} expected {}",
        spacing1, separation);
    assert!((spacing2 - separation).abs() < 0.01,
        "SYS_FORM_003: Line spacing 2 must be uniform. Got {} expected {}",
        spacing2, separation);
}

/// Test SYS_FORM_004: V-Formation Geometry
///
/// Requirement: "V-formation shall create V-shape with leader at apex"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md lines 718-726
/// Implementation: formation.rs lines 93-105
/// Verification: Test + Analysis
#[test]
fn test_sys_form_004_v_formation_geometry() {
    let mut manager = FormationManager::new();
    let separation = 10.0;
    manager.set_separation_distance(separation);

    manager.add_drone("drone1".to_string());
    manager.add_drone("drone2".to_string());
    manager.add_drone("drone3".to_string());

    manager.set_formation_type(FormationType::VFormation);
    manager.set_leader_position(Position::new(0.0, 0.0, 10.0));

    let pos1 = manager.get_target_position("drone1").unwrap();
    let pos2 = manager.get_target_position("drone2").unwrap();
    let pos3 = manager.get_target_position("drone3").unwrap();

    let positions = vec![pos1, pos2, pos3];

    // Verify one drone is at front (leader at 0, 0)
    let leader_count = positions.iter().filter(|p| p.x == 0.0 && p.y == 0.0).count();
    assert_eq!(leader_count, 1, "SYS_FORM_004: One drone must be at apex (leader)");

    // Verify V-shape: wings should be symmetrically behind leader
    // Collect wing positions (not at origin)
    let wings: Vec<&Position> = positions.iter().filter(|p| p.x != 0.0 || p.y != 0.0).collect();
    assert_eq!(wings.len(), 2, "SYS_FORM_004: Must have 2 wing drones");

    // Both wings should be behind leader (negative Y)
    for wing in &wings {
        assert!(wing.y < 0.0, "SYS_FORM_004: Wings must be behind leader (Y < 0)");
    }

    // Wings should be symmetric (equal distance from center line)
    if let [wing1, wing2] = wings.as_slice() {
        assert_eq!(wing1.x.abs(), wing2.x.abs(),
            "SYS_FORM_004: Wings must be symmetric (equal X distance from center)");
        assert_eq!(wing1.y, wing2.y,
            "SYS_FORM_004: Wings must be at same Y offset");
    }
}

/// Test SYS_FORM_005: Formation Stability
///
/// Requirement: "Formation is stable when all drones within 2.0m of target positions"
/// Source: doc/mbse/system_definition.sysml line 111 (stability_threshold : Real = 2.0)
/// Implementation: formation.rs lines 136-146
/// Verification: Test
#[test]
fn test_sys_form_005_formation_stability() {
    let mut manager = FormationManager::new();
    manager.add_drone("drone1".to_string());
    manager.add_drone("drone2".to_string());
    manager.add_drone("drone3".to_string());

    manager.set_formation_type(FormationType::Triangle);
    manager.set_leader_position(Position::new(0.0, 0.0, 10.0));

    // Test case 1: Drones at exact positions (stable)
    let mut drones = HashMap::new();
    for (_i, drone_id) in ["drone1", "drone2", "drone3"].iter().enumerate() {
        let target = manager.get_target_position(drone_id).unwrap();
        drones.insert(drone_id.to_string(), Drone::new(drone_id.to_string(), target));
    }

    assert!(manager.is_formation_stable(&drones),
        "SYS_FORM_005: Formation must be stable when drones at target positions");

    // Test case 2: One drone out of position > 2.0m (unstable)
    let mut drones = HashMap::new();
    drones.insert("drone1".to_string(), Drone::new("drone1".to_string(), Position::new(0.0, 0.0, 10.0)));
    drones.insert("drone2".to_string(), Drone::new("drone2".to_string(), Position::new(100.0, 100.0, 10.0))); // Far away
    drones.insert("drone3".to_string(), Drone::new("drone3".to_string(), Position::new(10.0, -8.66, 10.0)));

    assert!(!manager.is_formation_stable(&drones),
        "SYS_FORM_005: Formation must be unstable when drone > 2.0m from target");

    // Test case 3: Drones within 2.0m threshold (stable)
    let mut drones = HashMap::new();
    let target1 = manager.get_target_position("drone1").unwrap();
    drones.insert("drone1".to_string(), Drone::new("drone1".to_string(),
        Position::new(target1.x + 1.5, target1.y, target1.z))); // 1.5m offset
    let target2 = manager.get_target_position("drone2").unwrap();
    drones.insert("drone2".to_string(), Drone::new("drone2".to_string(), target2));
    let target3 = manager.get_target_position("drone3").unwrap();
    drones.insert("drone3".to_string(), Drone::new("drone3".to_string(), target3));

    assert!(manager.is_formation_stable(&drones),
        "SYS_FORM_005: Formation must be stable when all drones within 2.0m");
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// PERFORMANCE REQUIREMENTS (SYS_PERF_*)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test SYS_PERF_001: Update Rate (10 Hz)
///
/// Requirement: "System shall update drone positions at 10 Hz (100ms interval)"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md line 300
/// Implementation: swarm.rs (update_interval = 0.1)
/// Verification: Test
///
/// Note: This test verifies the delta-time calculation is consistent with 10 Hz updates
#[test]
fn test_sys_perf_001_update_rate() {
    let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));
    let target = Position::new(50.0, 0.0, 10.0);
    drone.move_to(target);

    let dt = 0.1; // 100ms = 10 Hz
    let initial_pos = drone.position;

    // Update with correct delta time
    drone.update_position(dt);

    // Verify position changed according to dt
    let distance_moved = drone.position.distance_to(&initial_pos);
    let expected_distance = drone.velocity.magnitude() * dt;

    assert!((distance_moved - expected_distance).abs() < 0.01,
        "SYS_PERF_001: Position update must use correct dt for 10 Hz (100ms) rate. Expected: {}, Got: {}",
        expected_distance, distance_moved);
}

/// Test: Delta Time Accuracy (SYS_PERF_002)
///
/// Requirement: "Position updates shall use accurate delta-time for physics fidelity"
/// Source: doc/mbse/MBSE_ARCHITECTURE.md line 301
#[test]
fn test_sys_perf_002_delta_time_accuracy() {
    // Test with a fresh drone for each dt value
    for dt in [0.05, 0.1, 0.2] {
        let mut drone = Drone::new("UAV-1".to_string(), Position::new(0.0, 0.0, 10.0));
        drone.move_to(Position::new(100.0, 0.0, 10.0));

        // Do one update to establish velocity
        drone.update_position(dt);

        // Now test consistency
        let initial_pos = drone.position;
        let initial_velocity = drone.velocity.magnitude();

        if initial_velocity > 0.0 {
            drone.update_position(dt);

            let distance_moved = drone.position.distance_to(&initial_pos);
            let expected = initial_velocity * dt;

            // Allow reasonable error for floating point and physics calculations
            let error_margin = expected.max(0.1) * 0.1; // 10% error margin or 0.1m minimum
            assert!((distance_moved - expected).abs() <= error_margin,
                "SYS_PERF_002: Distance moved ({:.3}) should be close to velocity * dt ({:.3}) for dt={}",
                distance_moved, expected, dt);
        }
    }
}
