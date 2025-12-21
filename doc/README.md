# UAV Swarm System - UML Documentation

This directory contains comprehensive UML diagrams for the UAV (Unmanned Aerial Vehicle) Swarm Controller application written in Rust.

## Overview

The UAV Swarm System is a collaborative drone navigation and coordination system that manages multiple drones, formations, and mission execution.

## Diagrams

### 1. Class Diagram (`class_diagram.puml`)

**Purpose**: Shows all structs, enums, and their relationships in the system.

**Key Components**:
- **Drone Module**: Position, Velocity, Drone, DroneStatus, DroneStatusInfo
- **Formation Module**: FormationType, FormationManager
- **Mission Module**: MissionType, MissionStatus, Mission, MissionExecutor
- **Swarm Module**: DroneSwarm (main orchestrator)

**Relationships**:
- DroneSwarm aggregates drones and owns FormationManager and MissionExecutor
- FormationManager and MissionExecutor coordinate drone movements
- All components use Position for spatial calculations

### 2. Module Diagram (`module_diagram.puml`)

**Purpose**: Illustrates the module structure and dependencies.

**Key Insights**:
- `main` module serves as entry point with CLI interface (clap)
- `swarm` is the central orchestrator
- `drone`, `formation`, and `mission` are specialized modules
- External dependencies: tokio (async), clap (CLI), serde (serialization)

### 3. Mission Execution Sequence (`sequence_mission_execution.puml`)

**Purpose**: Details the flow when executing a mission from start to completion.

**Process Flow**:
1. User issues mission command with target coordinates
2. DroneSwarm creates mission via MissionExecutor
3. Mission is started and drones are assigned
4. For each waypoint:
   - Drones move to waypoint
   - System waits for all drones to arrive
   - Advances to next waypoint
5. Mission completes, drones return to Idle state

**Key Interactions**:
- MissionExecutor coordinates multiple drones
- Continuous position updates until waypoint reached
- Async execution with sleep intervals

### 4. Formation Change Sequence (`sequence_formation_change.puml`)

**Purpose**: Shows how formations are changed and maintained.

**Process Flow**:
1. User requests formation change (triangle, line, v_formation)
2. FormationManager calculates offsets for formation type
3. Target positions computed for each drone
4. Drones move to formation positions
5. Formation is maintained through continuous updates

**Formation Types**:
- **Triangle**: Leader center, two drones behind in triangle pattern
- **Line**: Drones spread along X-axis
- **V-Formation**: Leader front, wings spread back and outward

### 5. Drone State Diagram (`state_diagram.puml`)

**Purpose**: Visualizes all possible drone states and transitions.

**States**:
- **Idle**: Waiting for commands, velocity = 0
- **Navigating**: Moving to target position
- **InFormation**: Maintaining formation offset
- **ExecutingMission**: Following mission waypoints
- **Error**: Error condition requiring intervention

**Transitions**:
- Idle → Navigating: move_to() called
- Navigating → Idle: target reached (distance < 0.1)
- InFormation ↔ Navigating: maintains formation by moving
- ExecutingMission internally uses Navigating for waypoint movement

### 6. Simulation Activity Diagram (`activity_diagram_simulation.puml`)

**Purpose**: Shows the overall application flow and activities.

**Main Activities**:
- **Initialization**: Parse CLI, create swarm, add drones
- **Simulation Loop**: Continuous update of positions and formations
- **Formation Command**: Change and update formation patterns
- **Mission Command**: Execute coordinated mission with waypoints

**Key Features**:
- Parallel activities (position updates and formation maintenance)
- Iterative loops with termination conditions
- Async execution with sleep intervals (100ms)

## How to Use These Diagrams

### Viewing PlantUML Diagrams

These diagrams are in PlantUML format (`.puml` files). To view them:

1. **Online**:
   - Visit [PlantUML Web Server](http://www.plantuml.com/plantuml/uml/)
   - Copy/paste the content of any `.puml` file

2. **VS Code**:
   - Install "PlantUML" extension
   - Open `.puml` file and press Alt+D to preview

3. **Command Line**:
   ```bash
   # Install PlantUML
   brew install plantuml  # macOS

   # Generate PNG
   plantuml class_diagram.puml

   # Generate SVG
   plantuml -tsvg class_diagram.puml
   ```

4. **IntelliJ/PyCharm**:
   - Built-in PlantUML support
   - Right-click → Show PlantUML Diagram

## Architecture Insights

### Design Patterns

1. **Orchestrator Pattern**: DroneSwarm coordinates all subsystems
2. **Strategy Pattern**: Different formation types (Triangle, Line, V-Formation)
3. **Command Pattern**: Mission types (MoveTo, Patrol, Search)
4. **State Pattern**: Drone status transitions

### Key Design Decisions

1. **Async/Await**: Mission execution uses tokio for async operations
2. **Position-Based Navigation**: All movement calculated from positions and velocities
3. **Formation Maintenance**: Continuous recalculation in update loop
4. **Waypoint Navigation**: Missions broken down into waypoints for flexibility

### System Capabilities

- Manages multiple drones simultaneously
- Three formation types with dynamic reconfiguration
- Three mission types (MoveTo, Patrol, Search)
- Real-time position and status monitoring
- Basic collision avoidance through formation spacing
- Coordinated navigation with waypoint progression

## Code Navigation

To explore the implementation:

- `src/drone.rs:6-41`: Position struct and vector operations
- `src/drone.rs:74-147`: Drone struct with navigation logic
- `src/formation.rs:22-147`: Formation management system
- `src/mission.rs:20-88`: Mission definition and waypoint handling
- `src/mission.rs:90-229`: Mission execution engine
- `src/swarm.rs:7-203`: Main orchestrator coordinating all systems
- `src/main.rs:11-69`: CLI interface and application entry point

## Future Enhancements

Based on the architecture, potential areas for expansion:

1. **Collision Avoidance**: Enhanced algorithms beyond formation spacing
2. **Communication**: Inter-drone messaging for distributed coordination
3. **Path Planning**: Obstacle avoidance and optimal route calculation
4. **Sensor Integration**: Add sensor data for environment awareness
5. **Multi-Swarm**: Coordinate multiple swarms for larger operations
6. **Dynamic Formations**: More formation types and smooth transitions
