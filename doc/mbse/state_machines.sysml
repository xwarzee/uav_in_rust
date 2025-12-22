/**
 * UAV Swarm System - State Machine Models
 * SysML v2 Model
 *
 * This model defines the state machines for key components of the
 * UAV Swarm System, showing states, transitions, and behaviors.
 */

package UAVSwarmStateMachines {

    import UAVSwarmSystem::*;
    import UAVSwarmRequirements::*;

    // ============================================================================
    // Drone State Machine
    // ============================================================================

    state def DroneStateMachine {
        doc /*
         * Complete state machine for individual UAV behavior.
         * Defines all possible states, transitions, guards, and actions.
         */

        in ref drone : UAV;

        // Entry point
        entry; then idle;

        // ========================================================================
        // State: Idle
        // ========================================================================
        state idle {
            doc /*
             * Drone is stationary and waiting for commands.
             * Velocity is zero, no target position set.
             */

            entry action {
                doc /* On entry: ensure velocity is zero */
                // drone.velocity = Velocity::zero();
                // drone.target_position = None;
                // drone.status = DroneStatus::Idle;
            }

            // Transitions from Idle
            transition idleToNavigating
                accept move_to_command
                if hasValidTarget
                do setNavigatingState
                then navigating;

            transition idleToInFormation
                accept formation_offset_command
                if hasValidFormationOffset
                do setInFormationState
                then inFormation;

            transition idleToExecutingMission
                accept mission_assignment
                do setExecutingMissionState
                then executingMission;
        }

        // ========================================================================
        // State: Navigating
        // ========================================================================
        state navigating {
            doc /*
             * Drone is actively moving toward a target position.
             * Velocity is non-zero and directed toward target.
             */

            entry action {
                doc /* On entry: set status and calculate initial velocity */
                // drone.status = DroneStatus::Navigating;
                // Calculate direction to target
                // Set velocity based on direction and max_speed
            }

            do action {
                doc /*
                 * Continuous activity while in Navigating state:
                 * - Calculate delta time
                 * - Update position based on velocity and dt
                 * - Check distance to target
                 */
                // drone.update_position(dt);
            }

            // Transitions from Navigating
            transition navigatingToIdle
                accept target_reached
                if distanceToTarget < 0.1
                do stopDrone
                then idle;

            transition navigatingToInFormation
                accept formation_offset_command
                do setInFormationState
                then inFormation;

            transition navigatingToExecutingMission
                accept mission_assignment
                do setExecutingMissionState
                then executingMission;

            transition navigatingToError
                accept error_event
                do setErrorState
                then error;
        }

        // ========================================================================
        // State: InFormation
        // ========================================================================
        state inFormation {
            doc /*
             * Drone is maintaining a formation offset from the leader.
             * May be stationary or moving with the formation.
             */

            entry action {
                doc /* On entry: set formation status */
                // drone.status = DroneStatus::InFormation;
                // drone.formation_offset = received_offset;
            }

            do action {
                doc /*
                 * Continuous activity:
                 * - Monitor distance to formation position
                 * - If distance > 1.0, may trigger navigation
                 */
            }

            // Transitions from InFormation
            transition inFormationToNavigating
                accept position_correction_needed
                if distanceToFormationPosition > 1.0
                do setNavigatingState
                then navigating;

            transition inFormationToExecutingMission
                accept mission_assignment
                do setExecutingMissionState
                then executingMission;

            transition inFormationToError
                accept error_event
                do setErrorState
                then error;
        }

        // ========================================================================
        // State: ExecutingMission
        // ========================================================================
        state executingMission {
            doc /*
             * Drone is following mission waypoints in coordination with swarm.
             * High-level state that internally uses Navigating for movement.
             */

            entry action {
                doc /* On entry: prepare for mission execution */
                // drone.status = DroneStatus::ExecutingMission;
            }

            // Substates for mission execution
            state waitingForWaypoint {
                doc /* Waiting for mission executor to assign next waypoint */
            }

            state navigatingToWaypoint {
                doc /* Moving toward assigned waypoint */

                do action {
                    doc /* Update position toward waypoint */
                    // drone.update_position(dt);
                }
            }

            state arrivedAtWaypoint {
                doc /* Reached waypoint, waiting for swarm synchronization */
            }

            // Initial substate
            entry; then waitingForWaypoint;

            // Internal transitions between substates
            transition waitToNavigate
                first waitingForWaypoint
                accept waypoint_assigned
                then navigatingToWaypoint;

            transition navigateToArrived
                first navigatingToWaypoint
                accept waypoint_reached
                if distanceToWaypoint < 1.0
                then arrivedAtWaypoint;

            transition arrivedToWait
                first arrivedAtWaypoint
                accept advance_to_next_waypoint
                then waitingForWaypoint;

            // Transitions from ExecutingMission to other states
            transition missionCompleted
                accept mission_complete_event
                do returnToIdle
                then idle;

            transition missionFailed
                accept mission_failure_event
                do setErrorState
                then error;
        }

        // ========================================================================
        // State: Error
        // ========================================================================
        state error {
            doc /*
             * Drone has encountered an error condition.
             * Requires intervention to resolve.
             */

            entry action {
                doc /* On entry: log error and halt operations */
                // drone.status = DroneStatus::Error(error_message);
                // drone.velocity = Velocity::zero();
                // Log error details
            }

            // Transition from Error
            transition errorToIdle
                accept error_resolved_event
                do clearError
                then idle;
        }

        // ========================================================================
        // Guards (Conditions)
        // ========================================================================
        constraint hasValidTarget {
            doc /* Target position is set and valid */
            // drone.target_position.is_some()
        }

        constraint hasValidFormationOffset {
            doc /* Formation offset is set and valid */
            // drone.formation_offset.is_some()
        }

        constraint distanceToTarget : Real {
            doc /* Distance from current position to target */
            // drone.position.distance_to(drone.target_position.unwrap())
        }

        constraint distanceToWaypoint : Real {
            doc /* Distance to current mission waypoint */
        }

        constraint distanceToFormationPosition : Real {
            doc /* Distance to desired formation position */
        }

        // ========================================================================
        // Actions
        // ========================================================================
        action def setNavigatingState {
            doc /* Configure drone for navigation */
            // Set status to Navigating
            // Calculate initial velocity
        }

        action def setInFormationState {
            doc /* Configure drone for formation maintenance */
            // Set status to InFormation
            // Store formation offset
        }

        action def setExecutingMissionState {
            doc /* Configure drone for mission execution */
            // Set status to ExecutingMission
        }

        action def setErrorState {
            doc /* Enter error state with error message */
            // Set status to Error
            // Store error message
        }

        action def stopDrone {
            doc /* Stop drone movement */
            // Set velocity to zero
            // Clear target_position
        }

        action def returnToIdle {
            doc /* Return to idle state after mission */
            // Clear mission data
            // Set velocity to zero
        }

        action def clearError {
            doc /* Clear error condition */
            // Clear error message
            // Reset to safe state
        }

        // Requirements satisfaction
        assert satisfy SYS_STATE_001;
        assert satisfy SYS_STATE_002;
        assert satisfy SYS_STATE_003;
    }

    // ============================================================================
    // Mission State Machine
    // ============================================================================

    state def MissionStateMachine {
        doc /*
         * State machine for mission lifecycle management.
         */

        in ref mission : Mission;

        // Entry point
        entry; then notStarted;

        // ========================================================================
        // State: NotStarted
        // ========================================================================
        state notStarted {
            doc /*
             * Mission has been created but not yet started.
             * Drones may be assigned, waypoints configured.
             */

            entry action {
                doc /* Initialize mission */
                // mission.status = MissionStatus::NotStarted;
                // mission.current_waypoint = 0;
            }

            transition startMission
                accept start_command
                if hasDronesAssigned and hasWaypoints
                do beginExecution
                then inProgress;
        }

        // ========================================================================
        // State: InProgress
        // ========================================================================
        state inProgress {
            doc /*
             * Mission is actively being executed.
             * Drones are navigating waypoints.
             */

            entry action {
                doc /* Begin execution */
                // mission.status = MissionStatus::InProgress;
            }

            // Substates
            state navigatingToWaypoint {
                doc /* Drones navigating to current waypoint */

                do action {
                    doc /* Monitor drone progress */
                }
            }

            state waitingForSwarmSync {
                doc /* Waiting for all drones to reach waypoint */

                do action {
                    doc /* Check if all drones have arrived */
                }
            }

            state advancingWaypoint {
                doc /* Moving to next waypoint in sequence */

                entry action {
                    doc /* Increment current_waypoint */
                    // mission.current_waypoint += 1;
                }
            }

            // Initial substate
            entry; then navigatingToWaypoint;

            // Internal transitions
            transition navigateToWaiting
                first navigatingToWaypoint
                accept all_drones_arrived
                then waitingForSwarmSync;

            transition waitingToAdvancing
                first waitingForSwarmSync
                accept proceed_to_next
                if hasMoreWaypoints
                then advancingWaypoint;

            transition advancingToNavigating
                first advancingWaypoint
                then navigatingToWaypoint;

            // Transitions to terminal states
            transition missionComplete
                first waitingForSwarmSync
                accept proceed_to_next
                if not hasMoreWaypoints
                do finalizeMission
                then completed;

            transition missionFail
                accept failure_event
                do handleFailure
                then failed;
        }

        // ========================================================================
        // State: Completed
        // ========================================================================
        state completed {
            doc /*
             * Mission successfully completed all waypoints.
             * Terminal state.
             */

            entry action {
                doc /* Finalize successful mission */
                // mission.status = MissionStatus::Completed;
                // Return drones to Idle
            }
        }

        // ========================================================================
        // State: Failed
        // ========================================================================
        state failed {
            doc /*
             * Mission failed due to error.
             * Terminal state.
             */

            entry action {
                doc /* Handle mission failure */
                // mission.status = MissionStatus::Failed(reason);
                // Return drones to Idle
                // Log failure reason
            }
        }

        // ========================================================================
        // Guards
        // ========================================================================
        constraint hasDronesAssigned : Boolean {
            doc /* Mission has at least one drone assigned */
            // !mission.assigned_drones.is_empty()
        }

        constraint hasWaypoints : Boolean {
            doc /* Mission has at least one waypoint */
            // !mission.waypoints.is_empty()
        }

        constraint hasMoreWaypoints : Boolean {
            doc /* More waypoints remain after current */
            // mission.current_waypoint < mission.waypoints.len() - 1
        }

        // ========================================================================
        // Actions
        // ========================================================================
        action def beginExecution {
            doc /* Start mission execution */
        }

        action def finalizeMission {
            doc /* Complete mission successfully */
        }

        action def handleFailure {
            doc /* Handle mission failure */
        }

        // Requirements satisfaction
        assert satisfy SYS_MISS_003;
    }

    // ============================================================================
    // Formation Manager State Machine
    // ============================================================================

    state def FormationStateMachine {
        doc /*
         * State machine for formation management lifecycle.
         */

        in ref formationMgr : FormationManagementSubsystem;

        entry; then noFormation;

        // ========================================================================
        // State: NoFormation
        // ========================================================================
        state noFormation {
            doc /* No formation configured */

            transition setFormation
                accept formation_type_command
                do calculateNewFormation
                then formationConfigured;
        }

        // ========================================================================
        // State: FormationConfigured
        // ========================================================================
        state formationConfigured {
            doc /*
             * Formation type is set and offsets calculated.
             * Continuously maintained during operations.
             */

            // Parallel substates
            state concurrent maintaining {
                state monitoringStability {
                    doc /* Continuously check if formation is stable */

                    do action {
                        doc /* Check formation stability */
                        // formationMgr.is_formation_stable()
                    }
                }

                state updatingPositions {
                    doc /* Update drone positions to maintain formation */

                    do action {
                        doc /* Update formation positions */
                        // formationMgr.update_formation()
                    }
                }
            }

            transition changeFormation
                accept formation_type_command
                do recalculateFormation
                then formationConfigured;  // Self-transition
        }

        // ========================================================================
        // Actions
        // ========================================================================
        action def calculateNewFormation {
            doc /* Calculate offsets for new formation type */
        }

        action def recalculateFormation {
            doc /* Recalculate offsets when formation changes */
        }

        // Requirements satisfaction
        assert satisfy SYS_FORM_005;
    }

    // ============================================================================
    // Simulation State Machine
    // ============================================================================

    state def SimulationStateMachine {
        doc /*
         * Overall simulation lifecycle state machine.
         */

        in ref swarm : DroneSwarmController;

        entry; then initialized;

        // ========================================================================
        // State: Initialized
        // ========================================================================
        state initialized {
            doc /* Swarm created, drones added, ready to start */

            entry action {
                doc /* System initialization */
                // swarm.simulation_running = false;
            }

            transition startSimulation
                accept start_command
                do beginSimulation
                then running;
        }

        // ========================================================================
        // State: Running
        // ========================================================================
        state running {
            doc /* Simulation is actively running */

            entry action {
                doc /* Start simulation */
                // swarm.simulation_running = true;
            }

            do action {
                doc /*
                 * Simulation loop:
                 * - Update all drone positions
                 * - Maintain formations
                 * - Update missions
                 * - Print status periodically
                 * - Sleep 100ms
                 */
                // swarm.update_swarm();
            }

            transition stopSimulation
                accept stop_command
                do haltSimulation
                then stopped;

            transition autoStop
                accept iteration_limit_reached
                do haltSimulation
                then stopped;
        }

        // ========================================================================
        // State: Stopped
        // ========================================================================
        state stopped {
            doc /* Simulation has been stopped */

            entry action {
                doc /* Cleanup and final status */
                // swarm.simulation_running = false;
                // Print final status
            }
        }

        // ========================================================================
        // Actions
        // ========================================================================
        action def beginSimulation {
            doc /* Initialize simulation loop */
        }

        action def haltSimulation {
            doc /* Stop simulation gracefully */
        }

        // Requirements satisfaction
        assert satisfy SYS_PERF_001;
    }
}
