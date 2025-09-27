# UAV Swarm Management System

A minimal Rust application for managing collaborative navigation, formation management, and mission execution for 3 drones.

## Features

- **Drone Management**: Control and monitor 3 autonomous drones
- **Formation Control**: Triangle, Line, and V-Formation patterns
- **Collaborative Navigation**: Coordinated movement and collision avoidance
- **Mission Execution**: MoveTo, Patrol, and Search missions
- **Real-time Monitoring**: Live status updates for all drones

## Usage

### Build the project

```bash
cargo build --release
```

### Run the application

```bash
# Start simulation
cargo run -- start

# Set formation type
cargo run -- formation triangle
cargo run -- formation line  
cargo run -- formation v_formation

# Execute mission to coordinates
cargo run -- mission 100.0 50.0 20.0
```

## Architecture

- `drone.rs` - Core drone physics and state management
- `formation.rs` - Formation patterns and positioning
- `mission.rs` - Mission types and execution logic  
- `swarm.rs` - High-level swarm coordination
- `main.rs` - CLI interface and application entry point

## Example Session

```bash
# Initialize and demonstrate capabilities
cargo run -- start

# Change to V-formation
cargo run -- formation v_formation

# Execute coordinate mission
cargo run -- mission 200.0 100.0 30.0
```

The system automatically manages drone coordination, collision avoidance, and formation maintenance during mission execution.
