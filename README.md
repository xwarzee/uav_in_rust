# UAV Swarm Management System

A minimal Rust application for managing collaborative navigation, formation management, and mission execution for 3 drones.

## Features

- **Drone Management**: Control and monitor 3 autonomous drones
- **Formation Control**: Triangle, Line, and V-Formation patterns
- **Collaborative Navigation**: Coordinated movement and collision avoidance
- **Mission Execution**: MoveTo, Patrol, and Search missions
- **Real-time Monitoring**: Live status updates for all drones
- **REST API**: Complete HTTP API with OpenAPI/Swagger documentation
- **WebSocket Streaming**: Real-time drone telemetry updates
- **FitNesse Testing**: Comprehensive automated acceptance test suite

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

### Run REST API Server

```bash
cargo run -- serve --port 8080
```

Access the interactive API documentation at: http://localhost:8080/swagger-ui/

## REST API Testing

### Option 1: Swagger UI (Interactive)
1. Start server: `cargo run -- serve --port 8080`
2. Open http://localhost:8080/swagger-ui/
3. Try endpoints interactively

### Option 2: FitNesse Acceptance Tests (Recommended)

**Quick Start:**
```bash
# Terminal 1: Start API server
cargo run -- serve --port 8080

# Terminal 2: Build fixtures and run FitNesse
cd fitnesse/fixtures && mvn clean package
cd .. && ./run-fitnesse.sh
```

Then open http://localhost:8000/UavSwarmApi to run the test suite.

**Test Coverage:**
- **SwarmTests**: 5 tests for swarm management
- **DroneTests**: 7 tests for drone operations
- **FormationTests**: 10 tests for formation control
- **MissionTests**: 10 tests for mission execution

**Total: 35+ automated acceptance tests**

For detailed FitNesse documentation, see [fitnesse/README.md](fitnesse/README.md)

### Option 3: cURL (Command Line)

```bash
# Get swarm status
curl http://localhost:8080/api/swarm

# Change formation
curl -X POST http://localhost:8080/api/formations/current \
  -H "Content-Type: application/json" \
  -d '{"formation_type":"triangle"}'

# Create mission
curl -X POST http://localhost:8080/api/missions \
  -H "Content-Type: application/json" \
  -d '{"type":"MoveTo","params":{"target":{"x":100,"y":200,"z":50}}}'
```

## Architecture

### Core Modules
- `src/drone.rs` - Core drone physics and state management
- `src/formation.rs` - Formation patterns and positioning
- `src/mission.rs` - Mission types and execution logic
- `src/swarm.rs` - High-level swarm coordination
- `src/main.rs` - CLI interface and application entry point

### REST API
- `src/api/` - Complete REST API implementation
  - `handlers/` - HTTP endpoint handlers
  - `models/` - Request/response DTOs
  - `routes/` - Route configuration
  - `websocket/` - Real-time WebSocket support
  - `server.rs` - Actix-web server setup
  - `docs.rs` - OpenAPI specification

### Testing
- `fitnesse/` - FitNesse acceptance test suite
  - `FitNesseRoot/UavSwarmApi/` - Test wiki pages
  - `fixtures/` - Java test fixtures for REST API

### Documentation

#### User & Deployment Guides

Complete documentation with UML diagrams and deployment scenarios in `doc/man/`:

- **[USER_GUIDE.md](doc/man/USER_GUIDE.md)** - Complete user guide with:
  - System architecture and UML class diagrams
  - Two deployment scenarios (Rust local + Gazebo remote, or unified server)
  - Installation and configuration instructions
  - API REST reference
  - Troubleshooting guide

- **[DEPLOYMENT_GUIDE.md](doc/man/DEPLOYMENT_GUIDE.md)** - Technical deployment guide:
  - Infrastructure as Code (Terraform)
  - Docker/Docker Compose setup
  - Network and security configuration (firewall, VPN)
  - Monitoring (Prometheus, Grafana, ELK)
  - Maintenance scripts and backup procedures

- **[CHANGELOG.md](doc/man/CHANGELOG.md)** - Version history and migration guides

#### Software Architecture

UML diagrams and architecture documentation in `doc/software/`:

- **[ARCHITECTURE.md](doc/software/ARCHITECTURE.md)** - Complete architecture documentation
- **[VIEWING_DIAGRAMS.md](doc/software/VIEWING_DIAGRAMS.md)** - Guide for viewing diagrams
- **Individual Diagrams**:
  - [Class Diagram](doc/software/class_diagram.puml) - System structure
  - [Module Diagram](doc/software/module_diagram.puml) - Dependencies
  - [Mission Execution Sequence](doc/software/sequence_mission_execution.puml) - Mission flow
  - [Formation Change Sequence](doc/software/sequence_formation_change.puml) - Formation updates
  - [State Diagram](doc/software/state_diagram.puml) - Drone states
  - [Activity Diagram](doc/software/activity_diagram_simulation.puml) - Simulation flow

All diagrams are available as both PlantUML source files (`.puml`) and rendered images (PNG).

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
