/**
 * UAV Swarm System - System Definition
 * SysML v2 Model
 *
 * This model defines the top-level system structure, components,
 * interfaces, and their relationships for the UAV Swarm Management System.
 */

package UAVSwarmSystem {

    import ScalarValues::*;
    import SI::*;

    // ============================================================================
    // System Context
    // ============================================================================

    part def ExternalEnvironment {
        doc /*
         * The external environment in which the UAV swarm operates.
         * Includes 3D physical space and atmospheric conditions.
         */

        attribute airspace : ThreeDimensionalSpace;
        attribute altitude_range : Real[2];  // [min, max] in meters
    }

    part def Operator {
        doc /* Human operator controlling the swarm via CLI */

        port commandInterface : CommandPort;
        port statusInterface : StatusPort;
    }

    // ============================================================================
    // System Definition
    // ============================================================================

    part def UAVSwarmManagementSystem {
        doc /*
         * Top-level system definition for the UAV Swarm Management System.
         * Manages collaborative navigation, formation control, and mission
         * execution for 3 autonomous drones.
         */

        // System attributes
        attribute version : String = "0.1.0";
        attribute implementation_language : String = "Rust";
        attribute max_drones : Integer = 3;

        // External interfaces
        port cli_input : CommandPort;
        port cli_output : StatusPort;

        // System components
        part droneSwarm : DroneSwarmController;
        part formationManager : FormationManagementSubsystem;
        part missionExecutor : MissionExecutionSubsystem;
        part drones : UAV[3];  // Exactly 3 drones

        // Internal connections
        connect droneSwarm.formationControl to formationManager.controlInterface;
        connect droneSwarm.missionControl to missionExecutor.controlInterface;
        connect formationManager.droneInterface to drones.formationInterface;
        connect missionExecutor.droneInterface to drones.navigationInterface;
    }

    // ============================================================================
    // Subsystem Definitions
    // ============================================================================

    part def DroneSwarmController {
        doc /*
         * Central orchestrator for the swarm.
         * Coordinates formations, missions, and provides status updates.
         */

        // Interfaces
        port formationControl : FormationControlPort;
        port missionControl : MissionControlPort;
        port cliInterface : CommandPort;

        // Internal state
        attribute simulation_running : Boolean;
        attribute last_update : TimeInstant;
        attribute update_interval : Real = 0.1;  // seconds

        // Capabilities
        action add_drone(id: String, initial_position: Position);
        action set_formation(formation_type: String);
        action execute_mission(target: Position);
        action get_swarm_status() : DroneStatusInfo[3];
        action update_swarm();
        action start_simulation();
        action stop_simulation();
    }

    part def FormationManagementSubsystem {
        doc /*
         * Manages geometric formations and maintains formation stability.
         */

        // Interfaces
        port controlInterface : FormationControlPort;
        port droneInterface : FormationPort;

        // Configuration
        attribute formation_type : FormationType;
        attribute leader_position : Position;
        attribute separation_distance : Real = 10.0;  // meters
        attribute stability_threshold : Real = 2.0;   // meters

        // Formation types enumeration
        enum def FormationType {
            Triangle;
            Line;
            VFormation;
        }

        // Capabilities
        action set_formation_type(type: FormationType);
        action calculate_offsets() : Position[3];
        action update_formation();
        action is_formation_stable() : Boolean;
    }

    part def MissionExecutionSubsystem {
        doc /*
         * Orchestrates mission execution and waypoint navigation.
         */

        // Interfaces
        port controlInterface : MissionControlPort;
        port droneInterface : NavigationPort;

        // Internal state
        attribute active_missions : Mission[*];
        attribute mission_counter : Integer = 0;

        // Mission types enumeration
        enum def MissionType {
            MoveTo;
            Patrol;
            Search;
        }

        // Mission status enumeration
        enum def MissionStatus {
            NotStarted;
            InProgress;
            Completed;
            Failed;
        }

        // Capabilities
        action create_mission(type: MissionType, drones: String[*]) : String;
        action start_mission(mission_id: String) : Boolean;
        action execute_mission(mission_id: String);
        action cancel_mission(mission_id: String) : Boolean;
        action get_mission_status(mission_id: String) : MissionStatus;
    }

    // ============================================================================
    // UAV Component Definition
    // ============================================================================

    part def UAV {
        doc /*
         * Individual Unmanned Aerial Vehicle with autonomous capabilities.
         * Represents a single drone in the swarm.
         */

        // Identity
        attribute id : String;

        // Physical state
        attribute position : Position;
        attribute velocity : Velocity;
        attribute max_speed : Real = 5.0;  // m/s

        // Navigation state
        attribute target_position : Position[0..1];
        attribute formation_offset : Position[0..1];

        // Operational state
        attribute status : DroneStatus;
        attribute last_update : TimeInstant;

        // Drone status enumeration
        enum def DroneStatus {
            Idle;
            Navigating;
            InFormation;
            ExecutingMission;
            Error;
        }

        // Interfaces
        port formationInterface : FormationPort;
        port navigationInterface : NavigationPort;
        port telemetryInterface : TelemetryPort;

        // Capabilities
        action move_to(target: Position);
        action set_formation_offset(offset: Position);
        action update_position(dt: Real);
        action get_status_info() : DroneStatusInfo;

        // Constraints
        assert constraint max_speed_constraint {
            velocity.magnitude() <= max_speed
        }

        assert constraint altitude_constraint {
            position.z >= 0.0 and position.z <= 100.0  // meters
        }
    }

    // ============================================================================
    // Mission Definition
    // ============================================================================

    part def Mission {
        doc /*
         * Represents a mission with waypoints and assigned drones.
         */

        attribute id : String;
        attribute mission_type : MissionType;
        attribute status : MissionStatus;
        attribute assigned_drones : String[*];
        attribute waypoints : Position[*];
        attribute current_waypoint : Integer = 0;

        // Use mission types from MissionExecutionSubsystem
        import MissionExecutionSubsystem::MissionType;
        import MissionExecutionSubsystem::MissionStatus;

        action start();
        action get_current_target() : Position[0..1];
        action advance_waypoint() : Boolean;
    }

    // ============================================================================
    // Data Type Definitions
    // ============================================================================

    attribute def Position {
        doc /* 3D position in Cartesian coordinates */
        attribute x : Real;  // meters
        attribute y : Real;  // meters
        attribute z : Real;  // meters (altitude)
    }

    attribute def Velocity {
        doc /* 3D velocity vector */
        attribute vx : Real;  // m/s
        attribute vy : Real;  // m/s
        attribute vz : Real;  // m/s

        calc def magnitude : Real = (vx*vx + vy*vy + vz*vz)**0.5;
    }

    attribute def DroneStatusInfo {
        doc /* Status information snapshot for a drone */
        attribute id : String;
        attribute position : Position;
        attribute velocity : Velocity;
        attribute status : UAV::DroneStatus;
    }

    attribute def ThreeDimensionalSpace {
        doc /* Physical 3D space definition */
        attribute x_range : Real[2];  // [min, max]
        attribute y_range : Real[2];
        attribute z_range : Real[2];
    }

    // ============================================================================
    // Interface Definitions
    // ============================================================================

    port def CommandPort {
        doc /* CLI command interface */
        in item commandInput : Command;
        out item commandResponse : Response;
    }

    port def StatusPort {
        doc /* Status monitoring interface */
        out item statusUpdate : DroneStatusInfo[*];
    }

    port def FormationControlPort {
        doc /* Formation control interface */
        in item formationCommand : FormationCommand;
        out item formationStatus : FormationStatus;
    }

    port def MissionControlPort {
        doc /* Mission control interface */
        in item missionCommand : MissionCommand;
        out item missionStatus : MissionStatus;
    }

    port def FormationPort {
        doc /* Individual drone formation interface */
        in item formationOffset : Position;
        out item currentPosition : Position;
    }

    port def NavigationPort {
        doc /* Individual drone navigation interface */
        in item targetPosition : Position;
        out item currentStatus : DroneStatus;
    }

    port def TelemetryPort {
        doc /* Drone telemetry interface */
        out item telemetryData : DroneStatusInfo;
    }

    // ============================================================================
    // Message Definitions
    // ============================================================================

    attribute def Command {
        attribute command_type : String;
        attribute parameters : String[*];
    }

    attribute def Response {
        attribute success : Boolean;
        attribute message : String;
    }

    attribute def FormationCommand {
        attribute formation_type : FormationManagementSubsystem::FormationType;
    }

    attribute def FormationStatus {
        attribute is_stable : Boolean;
        attribute current_formation : FormationManagementSubsystem::FormationType;
    }

    attribute def MissionCommand {
        attribute mission_type : MissionExecutionSubsystem::MissionType;
        attribute target : Position;
        attribute parameters : Real[*];
    }

    // ============================================================================
    // System Allocations
    // ============================================================================

    allocation def SystemAllocation {
        doc /*
         * Allocates system functions to components.
         */

        // Drone management allocated to DroneSwarm
        allocate DroneSwarmController::add_drone to UAVSwarmManagementSystem::droneSwarm;
        allocate DroneSwarmController::update_swarm to UAVSwarmManagementSystem::droneSwarm;

        // Formation control allocated to FormationManager
        allocate FormationManagementSubsystem::calculate_offsets
            to UAVSwarmManagementSystem::formationManager;
        allocate FormationManagementSubsystem::update_formation
            to UAVSwarmManagementSystem::formationManager;

        // Mission execution allocated to MissionExecutor
        allocate MissionExecutionSubsystem::execute_mission
            to UAVSwarmManagementSystem::missionExecutor;

        // Navigation allocated to individual UAVs
        allocate UAV::update_position to UAVSwarmManagementSystem::drones;
    }
}
