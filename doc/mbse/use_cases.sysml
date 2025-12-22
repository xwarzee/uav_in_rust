/**
 * UAV Swarm System - Use Case Model
 * SysML v2 Model
 *
 * This model defines the use cases for the UAV Swarm Management System,
 * describing how operators interact with the system to accomplish goals.
 */

package UAVSwarmUseCases {

    import UAVSwarmSystem::*;
    import UAVSwarmRequirements::*;

    // ============================================================================
    // Actor Definition
    // ============================================================================

    actor def SystemOperator {
        doc /*
         * Human operator who controls the UAV swarm through CLI commands.
         * Responsible for mission planning, formation configuration,
         * and monitoring swarm status.
         */
    }

    // ============================================================================
    // Primary Use Cases
    // ============================================================================

    use case StartSimulation {
        doc /*
         * Operator starts the simulation to initialize the swarm and
         * begin autonomous operations.
         */

        subject system : UAVSwarmManagementSystem;
        actor operator : SystemOperator;

        objective {
            doc /* Initialize and run the swarm simulation */
        }

        // Preconditions
        assume initialState {
            doc /* System is initialized with 3 drones at known positions */
        }

        // Main flow
        action operatorCommand {
            doc /* Operator issues 'start' command via CLI */
            actor operator;
        }

        then action systemInitialize {
            doc /* System sets simulation_running to true */
            action by system.droneSwarm;
        }

        then action displayInitialStatus {
            doc /* System displays initial swarm status */
            action by system.droneSwarm;
        }

        then action beginSimulationLoop {
            doc /*
             * System enters simulation loop:
             * - Update drone positions every 100ms
             * - Maintain formations
             * - Print status every 10 iterations
             * - Continue for 100 iterations or until stopped
             */
            action by system.droneSwarm;
        }

        // Postconditions
        ensure finalState {
            doc /* Simulation is running and updating drone states */
        }

        // Requirements satisfaction
        satisfy SR_005_1;
    }

    use case ChangeFormation {
        doc /*
         * Operator changes the swarm formation pattern while maintaining
         * swarm coordination.
         */

        subject system : UAVSwarmManagementSystem;
        actor operator : SystemOperator;

        objective {
            doc /* Reconfigure swarm to specified formation type */
        }

        // Preconditions
        assume swarmExists {
            doc /* Swarm contains 3 active drones */
        }

        // Main flow
        action operatorSpecifiesFormation {
            doc /*
             * Operator issues formation command with type:
             * - 'triangle'
             * - 'line'
             * - 'v_formation'
             */
            actor operator;
        }

        then action parseFormationType {
            doc /* System parses formation type from command */
            action by system.droneSwarm;
        }

        then action updateFormationType {
            doc /* FormationManager updates formation_type */
            action by system.formationManager;
        }

        then action calculateOffsets {
            doc /*
             * FormationManager calculates position offsets for each drone
             * based on formation geometry and separation distance
             */
            action by system.formationManager;
        }

        then action commandDronesToPositions {
            doc /*
             * For each drone, calculate target position and issue move command.
             * If distance > 1.0m: set to Navigating state
             * If distance <= 1.0m: set to InFormation state
             */
            action by system.formationManager;
        }

        then action confirmationFeedback {
            doc /* System displays confirmation of formation change */
            action by system.droneSwarm;
        }

        // Postconditions
        ensure formationEstablished {
            doc /*
             * All drones are moving toward or maintaining formation positions
             */
        }

        // Requirements satisfaction
        satisfy SR_002;
        satisfy SR_005_2;

        // Included use cases
        include ContinuousFormationMaintenance;
    }

    use case ExecuteMission {
        doc /*
         * Operator commands the swarm to execute a mission to a target location.
         */

        subject system : UAVSwarmManagementSystem;
        actor operator : SystemOperator;

        objective {
            doc /* Execute coordinated mission to target coordinates */
        }

        // Preconditions
        assume swarmReady {
            doc /* Swarm is initialized and drones are available */
        }

        // Main flow
        action operatorSpecifiesTarget {
            doc /*
             * Operator issues mission command with coordinates (x, y, z)
             */
            actor operator;
        }

        then action parseCoordinates {
            doc /* System parses target coordinates from command */
            action by system.droneSwarm;
        }

        then action createMission {
            doc /*
             * MissionExecutor creates MoveTo mission:
             * - Generates unique mission ID
             * - Assigns all drones to mission
             * - Sets target as waypoint
             */
            action by system.missionExecutor;
        }

        then action startMission {
            doc /*
             * Mission status changes to InProgress
             * Current waypoint set to 0
             */
            action by system.missionExecutor;
        }

        then action executeWaypoints {
            doc /*
             * For each waypoint:
             * 1. Set all drones to ExecutingMission status
             * 2. Command drones to navigate to waypoint
             * 3. Wait for all drones to arrive (distance < 1.0m)
             * 4. Display waypoint reached message
             * 5. Advance to next waypoint
             */
            action by system.missionExecutor;
        }

        then action completeMission {
            doc /*
             * When all waypoints reached:
             * - Mission status set to Completed
             * - All drones set to Idle status
             * - Display mission completed message
             */
            action by system.missionExecutor;
        }

        // Postconditions
        ensure missionCompleted {
            doc /* All drones have reached target and mission is completed */
        }

        // Requirements satisfaction
        satisfy SR_003_1;
        satisfy SR_005_3;
    }

    use case ExecutePatrolMission {
        doc /*
         * Operator commands swarm to patrol multiple waypoints in sequence.
         */

        subject system : UAVSwarmManagementSystem;
        actor operator : SystemOperator;

        objective {
            doc /* Execute patrol mission through multiple waypoints */
        }

        // Similar structure to ExecuteMission
        action operatorSpecifiesWaypoints {
            doc /* Operator provides multiple waypoint coordinates */
            actor operator;
        }

        then action createPatrolMission {
            doc /*
             * MissionExecutor creates Patrol mission with waypoint list
             */
            action by system.missionExecutor;
        }

        then action executePatrolLoop {
            doc /*
             * Execute waypoint navigation for each point in sequence
             */
            action by system.missionExecutor;
        }

        // Requirements satisfaction
        satisfy SR_003_2;
    }

    use case ExecuteSearchMission {
        doc /*
         * Operator commands swarm to search a circular area.
         */

        subject system : UAVSwarmManagementSystem;
        actor operator : SystemOperator;

        objective {
            doc /* Execute search pattern around target area */
        }

        action operatorSpecifiesSearchArea {
            doc /*
             * Operator provides center coordinates and search radius
             */
            actor operator;
        }

        then action generateSearchPattern {
            doc /*
             * MissionExecutor creates Search mission:
             * - Generates 8 waypoints in circular pattern
             * - Evenly distributed around center point
             * - At specified radius
             */
            action by system.missionExecutor;
        }

        then action executeSearchPattern {
            doc /* Execute navigation through generated search waypoints */
            action by system.missionExecutor;
        }

        // Requirements satisfaction
        satisfy SR_003_3;
        satisfy SYS_MISS_004;
    }

    use case MonitorSwarmStatus {
        doc /*
         * Operator monitors real-time status of all drones in the swarm.
         */

        subject system : UAVSwarmManagementSystem;
        actor operator : SystemOperator;

        objective {
            doc /* Observe current state, position, and velocity of all drones */
        }

        // Main flow
        action requestStatus {
            doc /* Operator requests swarm status (implicit in simulation loop) */
            actor operator;
        }

        then action gatherDroneStatus {
            doc /*
             * DroneSwarm queries each drone for status information:
             * - ID
             * - Position (x, y, z)
             * - Velocity (vx, vy, vz)
             * - Status (Idle, Navigating, InFormation, ExecutingMission, Error)
             */
            action by system.droneSwarm;
        }

        then action displayStatus {
            doc /*
             * System formats and displays status for all drones
             * in human-readable format
             */
            action by system.droneSwarm;
        }

        // Postconditions
        ensure statusDisplayed {
            doc /* Current status of all drones is visible to operator */
        }

        // Requirements satisfaction
        satisfy SR_004;
    }

    // ============================================================================
    // Secondary Use Cases
    // ============================================================================

    use case ContinuousFormationMaintenance {
        doc /*
         * System continuously maintains formation during operations.
         * This is an internal, automated use case.
         */

        subject system : UAVSwarmManagementSystem;

        objective {
            doc /* Keep drones in formation as they move */
        }

        action monitorFormation {
            doc /*
             * Simulation loop repeatedly calls update_formation()
             */
            action by system.droneSwarm;
        }

        then action checkStability {
            doc /*
             * FormationManager checks if all drones within
             * stability_threshold (2.0m) of target positions
             */
            action by system.formationManager;
        }

        then action correctPositions {
            doc /*
             * If drone distance > 1.0m from target:
             * - Issue move command
             * - Set status to Navigating
             * Otherwise:
             * - Set formation_offset
             * - Set status to InFormation
             */
            action by system.formationManager;
        }

        // Requirements satisfaction
        satisfy SYS_FORM_005;
    }

    use case NavigateToPosition {
        doc /*
         * Individual drone navigates autonomously to target position.
         * Internal use case performed by each drone.
         */

        subject drone : UAV;

        objective {
            doc /* Move drone from current position to target position */
        }

        // Preconditions
        assume hasTarget {
            doc /* Drone has valid target_position */
        }

        action calculateDirection {
            doc /*
             * Calculate direction vector:
             * direction = (target - position).normalize()
             */
            action by drone;
        }

        then action updateVelocity {
            doc /*
             * Set velocity = direction * max_speed
             * Constrained by max_speed (5.0 m/s)
             */
            action by drone;
        }

        then action updatePosition {
            doc /*
             * Calculate delta time since last update
             * Update position: position += velocity * dt
             */
            action by drone;
        }

        then action checkArrival {
            doc /*
             * If distance to target < 0.1:
             * - Set velocity to zero
             * - Set status to Idle
             * - Clear target_position
             */
            action by drone;
        }

        // Postconditions
        ensure arrived {
            doc /* Drone reaches target position or makes progress toward it */
        }

        // Requirements satisfaction
        satisfy SYS_NAV_001;
        satisfy SYS_NAV_002;
        satisfy SYS_NAV_003;
    }

    // ============================================================================
    // Use Case Relationships
    // ============================================================================

    // Extend relationships for error handling
    use case HandleNavigationError {
        doc /* Handle errors during navigation */

        extend NavigateToPosition when "navigation error occurs";

        action setErrorState {
            doc /*
             * Set drone status to Error with message
             * Halt navigation
             */
        }
    }

    use case HandleMissionFailure {
        doc /* Handle mission execution failures */

        extend ExecuteMission when "mission execution fails";
        extend ExecutePatrolMission when "patrol execution fails";
        extend ExecuteSearchMission when "search execution fails";

        action failMission {
            doc /*
             * Set mission status to Failed
             * Return drones to Idle state
             * Report error to operator
             */
        }
    }

    // ============================================================================
    // Use Case Scenarios
    // ============================================================================

    use case scenario TriangleFormationScenario : ChangeFormation {
        doc /*
         * Specific scenario: Changing to triangle formation with 3 drones
         */

        // Initial state
        given initial_positions {
            doc /*
             * Drone1: (0, 0, 10)
             * Drone2: (5, 5, 10)
             * Drone3: (10, 0, 10)
             */
        }

        // Operator action
        when operator_command {
            doc /* Operator: 'cargo run -- formation triangle' */
        }

        // System calculates offsets (separation_distance = 10.0)
        then calculated_offsets {
            doc /*
             * Drone1 (Leader): (0, 0, 0) offset
             * Drone2 (Left): (-10, -8.66, 0) offset
             * Drone3 (Right): (10, -8.66, 0) offset
             */
        }

        // Expected result
        then final_formation {
            doc /*
             * Drones form equilateral triangle
             * Leader at (0, 0, 10)
             * Left at (-10, -8.66, 10)
             * Right at (10, -8.66, 10)
             */
        }

        assert requirement {
            satisfy SYS_FORM_002;
        }
    }

    use case scenario CoordinatedMissionScenario : ExecuteMission {
        doc /*
         * Specific scenario: All drones execute mission to (100, 50, 20)
         */

        given initial_state {
            doc /*
             * 3 drones at various starting positions
             * All in Idle state
             */
        }

        when mission_command {
            doc /* Operator: 'cargo run -- mission 100.0 50.0 20.0' */
        }

        then mission_created {
            doc /*
             * MissionExecutor creates mission_1
             * Type: MoveTo
             * Waypoint: (100, 50, 20)
             * Assigned drones: [drone1, drone2, drone3]
             */
        }

        then execution_begins {
            doc /*
             * All drones:
             * - Status set to ExecutingMission
             * - Navigate toward (100, 50, 20)
             * - Update positions each frame
             */
        }

        then synchronization {
            doc /*
             * System waits for all drones to arrive
             * Continuously checks distances
             * Advances only when all distances < 1.0m
             */
        }

        then completion {
            doc /*
             * All drones arrived
             * Mission status: Completed
             * Drone status: Idle
             * Operator sees: "Mission completed successfully"
             */
        }

        assert requirement {
            satisfy SYS_MISS_002;
        }
    }
}
