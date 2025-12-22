/**
 * UAV Swarm System - Activity Models
 * SysML v2 Model
 *
 * This model defines the activity flows for key operations in the
 * UAV Swarm System, showing sequences, decisions, and parallelism.
 */

package UAVSwarmActivities {

    import UAVSwarmSystem::*;
    import UAVSwarmRequirements::*;

    // ============================================================================
    // Simulation Loop Activity
    // ============================================================================

    action def SimulationLoopActivity {
        doc /*
         * Main simulation loop that continuously updates the swarm state.
         * Runs at 10 Hz (100ms interval) for up to 100 iterations.
         */

        in swarm : DroneSwarmController;

        // Entry point
        action start {
            doc /* Simulation begins */
        }

        then action setRunningFlag {
            doc /* Set simulation_running = true */
        }

        then action initializeIterationCounter {
            doc /* iteration = 0 */
        }

        then action printInitialStatus {
            doc /* Display swarm status at start */
        }

        // Main loop
        then loop simulationLoop {
            doc /* Loop continues while running and iteration < 100 */

            // Calculate time
            action calculateDeltaTime {
                doc /*
                 * Calculate time elapsed since last update:
                 * dt = current_time - last_update
                 */
                out dt : Real;
            }

            // Parallel activities
            then parallel {
                // Fork 1: Update all drones
                action updateAllDrones {
                    doc /* Update position and state for each drone */

                    loop forEachDrone {
                        action selectDrone {
                            out drone : UAV;
                        }

                        then action updateDronePosition {
                            in drone : UAV;
                            in dt : Real;
                            doc /*
                             * Call drone.update_position(dt)
                             * Updates position based on velocity
                             */
                        }

                        then decide hasTarget {
                            if drone.target_position.hasValue {
                                action calculateDirection {
                                    doc /* direction = (target - position).normalize() */
                                }

                                then action updateVelocity {
                                    doc /* velocity = direction * max_speed */
                                }

                                then action moveTowardTarget {
                                    doc /* position += velocity * dt */
                                }

                                then decide arrivedAtTarget {
                                    if drone.position.distance_to(drone.target_position) < 0.1 {
                                        action reachTarget {
                                            doc /*
                                             * Set velocity = 0
                                             * Set status = Idle
                                             * Clear target_position
                                             */
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Fork 2: Check and maintain formation
                action checkFormation {
                    doc /* Verify and maintain formation stability */

                    action queryFormationStability {
                        doc /* formationMgr.is_formation_stable() */
                        out isStable : Boolean;
                    }

                    then decide formationStable {
                        if isStable {
                            action updateFormationPositions {
                                doc /*
                                 * Call formationMgr.update_formation()
                                 * Adjust drone positions as needed
                                 */
                            }
                        }
                    }
                }
            }

            // After parallel activities complete
            then action updateTimestamp {
                doc /* last_update = current_time */
            }

            then action incrementIteration {
                doc /* iteration += 1 */
            }

            then decide shouldPrintStatus {
                if iteration % 10 == 0 {
                    action printSwarmStatus {
                        doc /* Display status of all drones */
                    }
                }
            }

            then action sleepInterval {
                doc /* Sleep 100ms before next iteration */
            }

            then decide shouldContinue {
                if swarm.simulation_running and iteration < 100 {
                    continue simulationLoop;
                }
            }
        }

        then action finalizeSimulation {
            doc /* Print "Simulation ended" */
        }

        // Requirements satisfaction
        assert satisfy SYS_PERF_001;
        assert satisfy SYS_PERF_002;
    }

    // ============================================================================
    // Mission Execution Activity
    // ============================================================================

    action def ExecuteMissionActivity {
        doc /*
         * Activity for executing a complete mission from creation to completion.
         * Coordinates all drones through waypoints.
         */

        in target : Position;
        in swarm : DroneSwarmController;
        out result : MissionStatus;

        // Create mission
        action getAllDroneIds {
            doc /* Collect IDs of all drones in swarm */
            out droneIds : String[3];
        }

        then action createMission {
            in target : Position;
            in droneIds : String[3];
            doc /*
             * MissionExecutor.create_mission(
             *   MissionType::MoveTo(target),
             *   droneIds
             * )
             */
            out missionId : String;
        }

        then action startMission {
            in missionId : String;
            doc /*
             * MissionExecutor.start_mission(missionId)
             * Sets status = InProgress
             * Initializes current_waypoint = 0
             */
        }

        // Waypoint loop
        then loop waypointLoop {
            doc /* Loop through all waypoints */

            action getCurrentWaypoint {
                doc /* mission.get_current_target() */
                out waypoint : Position;
            }

            // Parallel: Assign target to all drones
            then parallel assignDrones {
                loop forEachDrone {
                    action selectDrone {
                        out drone : UAV;
                    }

                    then action setMissionStatus {
                        doc /* drone.status = ExecutingMission */
                    }

                    then action assignTarget {
                        doc /* drone.move_to(waypoint) */
                    }
                }
            }

            // Wait for all drones to arrive
            then loop waitForArrival {
                doc /* Loop until all drones arrive */

                parallel updateDrones {
                    loop forEachDrone {
                        action updatePosition {
                            in dt : Real;
                            doc /* drone.update_position(dt) */
                        }

                        then action checkDistance {
                            doc /* distance = drone.position.distance_to(waypoint) */
                            out distance : Real;
                        }
                    }
                }

                then decide allArrived {
                    if all distances < 1.0 {
                        merge;  // Exit waitForArrival loop
                    }
                    else {
                        action sleepShort {
                            doc /* Sleep 100ms */
                        }
                        then continue waitForArrival;
                    }
                }
            }

            then action announceWaypointReached {
                doc /* Print "All drones reached waypoint" */
            }

            then action advanceWaypoint {
                doc /*
                 * mission.advance_waypoint()
                 * Returns true if more waypoints, false if complete
                 */
                out hasMore : Boolean;
            }

            then decide moreWaypoints {
                if hasMore {
                    continue waypointLoop;
                }
                else {
                    action completeMission {
                        doc /*
                         * Set mission.status = Completed
                         * Set all drone.status = Idle
                         */
                    }
                    merge;  // Exit waypointLoop
                }
            }
        }

        then action reportCompletion {
            doc /* Print "Mission completed successfully" */
            result = MissionStatus::Completed;
        }

        // Requirements satisfaction
        assert satisfy SYS_MISS_001;
        assert satisfy SYS_MISS_002;
        assert satisfy SYS_MISS_003;
    }

    // ============================================================================
    // Formation Change Activity
    // ============================================================================

    action def ChangeFormationActivity {
        doc /*
         * Activity for changing the swarm formation.
         * Calculates new positions and commands drones to move.
         */

        in formationType : FormationManagementSubsystem::FormationType;
        in formationMgr : FormationManagementSubsystem;
        in drones : UAV[3];

        // Parse and validate formation type
        action validateFormationType {
            in formationType : FormationManagementSubsystem::FormationType;
            doc /* Ensure formation type is valid */
        }

        then action setFormationType {
            doc /* formationMgr.set_formation_type(formationType) */
        }

        // Calculate formation offsets
        then action calculateOffsets {
            doc /*
             * formationMgr.calculate_offsets()
             * Calls appropriate formation algorithm
             */

            decide whichFormation {
                if formationType == Triangle {
                    action calculateTriangleFormation {
                        doc /*
                         * Leader: (0, 0, 0)
                         * Left: (-d, -d*0.866, 0)
                         * Right: (d, -d*0.866, 0)
                         * where d = separation_distance
                         */
                        out offsets : Position[3];
                    }
                    merge;
                }
                else if formationType == Line {
                    action calculateLineFormation {
                        doc /*
                         * For drone i: ((i-1)*d, 0, 0)
                         * where d = separation_distance
                         */
                        out offsets : Position[3];
                    }
                    merge;
                }
                else if formationType == VFormation {
                    action calculateVFormation {
                        doc /*
                         * Leader: (0, 0, 0)
                         * Left: (-d, -d, 0)
                         * Right: (d, -d, 0)
                         * where d = separation_distance
                         */
                        out offsets : Position[3];
                    }
                    merge;
                }
            }
        }

        then action storeOffsets {
            doc /* Store calculated offsets in formationMgr */
        }

        // Update drone positions
        then action updateFormation {
            doc /* formationMgr.update_formation(drones) */

            action findLeader {
                doc /* Select first drone as leader */
                out leader : UAV;
            }

            then action setLeaderPosition {
                doc /* formationMgr.leader_position = leader.position */
            }

            // Command each drone
            then parallel commandDrones {
                loop forEachDrone {
                    action selectDrone {
                        out drone : UAV;
                        out offset : Position;
                    }

                    then action calculateTargetPosition {
                        doc /* target = leader_position + offset */
                        out targetPos : Position;
                    }

                    then action checkDistance {
                        doc /* distance = drone.position.distance_to(targetPos) */
                        out distance : Real;
                    }

                    then decide needsMovement {
                        if distance > 1.0 {
                            action commandMove {
                                doc /*
                                 * drone.move_to(targetPos)
                                 * Sets status = Navigating
                                 */
                            }
                        }
                        else {
                            action setInFormation {
                                doc /*
                                 * drone.set_formation_offset(offset)
                                 * Sets status = InFormation
                                 */
                            }
                        }
                    }
                }
            }
        }

        then action confirmFormationChange {
            doc /* Print "Formation changed to {formationType}" */
        }

        // Requirements satisfaction
        assert satisfy SYS_FORM_001;
        assert satisfy SYS_FORM_002;
        assert satisfy SYS_FORM_003;
        assert satisfy SYS_FORM_004;
    }

    // ============================================================================
    // Search Mission Generation Activity
    // ============================================================================

    action def GenerateSearchPatternActivity {
        doc /*
         * Generate circular search pattern with 8 waypoints.
         */

        in center : Position;
        in radius : Real;
        out waypoints : Position[8];

        action validateInputs {
            doc /* Ensure radius > 0 */
            assert radius > 0.0;
        }

        then loop generateWaypoints {
            doc /* Create 8 waypoints around circle */

            in counter : Integer = 0;

            action calculateAngle {
                doc /*
                 * angle = (2 * PI * counter) / 8
                 * Evenly distributed around circle
                 */
                out angle : Real;
            }

            then action calculatePosition {
                doc /*
                 * x = center.x + radius * cos(angle)
                 * y = center.y + radius * sin(angle)
                 * z = center.z
                 */
                out waypoint : Position;
            }

            then action storeWaypoint {
                doc /* waypoints[counter] = waypoint */
            }

            then decide continueLoop {
                if counter < 7 {
                    action incrementCounter {
                        counter = counter + 1;
                    }
                    then continue generateWaypoints;
                }
            }
        }

        // Requirements satisfaction
        assert satisfy SYS_MISS_004;
    }

    // ============================================================================
    // Drone Position Update Activity
    // ============================================================================

    action def UpdateDronePositionActivity {
        doc /*
         * Core activity for updating individual drone position.
         * Called every simulation frame.
         */

        in drone : UAV;
        in dt : Real;

        action checkHasTarget {
            doc /* Check if drone.target_position is set */
            out hasTarget : Boolean;
        }

        then decide hasTargetDecision {
            if hasTarget {
                action calculateDirection {
                    doc /*
                     * direction = target_position - position
                     * direction = direction.normalize()
                     */
                    out direction : Position;
                }

                then action calculateDesiredVelocity {
                    doc /*
                     * desired_velocity = direction * max_speed
                     */
                    out desiredVelocity : Velocity;
                }

                then action applySpeedConstraint {
                    doc /*
                     * Ensure velocity.magnitude() <= max_speed
                     */
                    in desiredVelocity : Velocity;
                    out constrainedVelocity : Velocity;
                }

                then action updateVelocity {
                    doc /* drone.velocity = constrainedVelocity */
                }

                then action updatePosition {
                    doc /*
                     * drone.position.x += drone.velocity.vx * dt
                     * drone.position.y += drone.velocity.vy * dt
                     * drone.position.z += drone.velocity.vz * dt
                     */
                }

                then action checkArrival {
                    doc /* distance = drone.position.distance_to(target) */
                    out distance : Real;
                }

                then decide arrivedDecision {
                    if distance < 0.1 {
                        action handleArrival {
                            doc /*
                             * drone.velocity = Velocity::zero()
                             * drone.status = Idle
                             * drone.target_position = None
                             */
                        }
                    }
                }
            }
        }

        then action updateTimestamp {
            doc /* drone.last_update = current_time */
        }

        // Requirements satisfaction
        assert satisfy SYS_NAV_001;
        assert satisfy SYS_NAV_002;
        assert satisfy SYS_NAV_003;
    }

    // ============================================================================
    // Command Processing Activity
    // ============================================================================

    action def ProcessCommandActivity {
        doc /*
         * Activity for processing CLI commands from operator.
         * Routes to appropriate subsystem based on command type.
         */

        in command : Command;
        in system : UAVSwarmManagementSystem;

        action parseCommand {
            doc /* Parse command_type and parameters */
            out commandType : String;
            out parameters : String[*];
        }

        then decide commandRouter {
            if commandType == "start" {
                action executeStartCommand {
                    doc /*
                     * Call system.droneSwarm.start_simulation()
                     * Begins simulation loop
                     */
                }
            }
            else if commandType == "formation" {
                action parseFormationType {
                    doc /* Extract formation type from parameters */
                    out formationType : String;
                }

                then action executeFormationCommand {
                    doc /*
                     * Call system.droneSwarm.set_formation(formationType)
                     * Triggers ChangeFormationActivity
                     */
                }
            }
            else if commandType == "mission" {
                action parseCoordinates {
                    doc /* Extract x, y, z from parameters */
                    out x : Real;
                    out y : Real;
                    out z : Real;
                }

                then action createTargetPosition {
                    doc /* target = Position::new(x, y, z) */
                    out target : Position;
                }

                then action executeMissionCommand {
                    doc /*
                     * Call system.droneSwarm.execute_mission(target)
                     * Triggers ExecuteMissionActivity
                     */
                }
            }
            else {
                action displayHelp {
                    doc /* Show usage information */
                }
            }
        }

        // Requirements satisfaction
        assert satisfy SYS_IF_001;
    }

    // ============================================================================
    // Status Reporting Activity
    // ============================================================================

    action def ReportSwarmStatusActivity {
        doc /*
         * Gather and display status of all drones in the swarm.
         */

        in swarm : DroneSwarmController;

        action gatherDroneStatuses {
            doc /* Call swarm.get_swarm_status() */
            out statuses : DroneStatusInfo[3];
        }

        then parallel formatStatuses {
            loop forEachStatus {
                action selectStatus {
                    out status : DroneStatusInfo;
                }

                then action formatStatus {
                    doc /*
                     * Format status as string:
                     * "Drone {id}: Position({x}, {y}, {z}),
                     *  Velocity({vx}, {vy}, {vz}), Status: {status}"
                     */
                    out formattedString : String;
                }
            }
        }

        then action displayStatuses {
            doc /* Print all formatted status strings to console */
        }

        // Requirements satisfaction
        assert satisfy SYS_IF_002;
    }
}
