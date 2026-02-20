# UAV Swarm System - Architecture Documentation

**Version:** 0.4.0
**Language:** Rust
**Purpose:** Collaborative UAV (Unmanned Aerial Vehicle) Swarm Controller

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Architecture Principles](#architecture-principles)
3. [Hexagonal Architecture](#hexagonal-architecture)
   - [Overview](#overview)
   - [Business Domain (Core)](#business-domain-core)
   - [Ports (Interfaces)](#ports-interfaces)
   - [Adapters (Implementations)](#adapters-implementations)
4. [Module Architecture](#module-architecture)
5. [Class Diagram](#class-diagram)
6. [Behavioral Diagrams](#behavioral-diagrams)
   - [Mission Execution Flow](#mission-execution-flow)
   - [Formation Change Flow](#formation-change-flow)
   - [Drone State Machine](#drone-state-machine)
7. [Design Patterns](#design-patterns)
8. [Key Components](#key-components)
9. [Code Navigation](#code-navigation)

---

## System Overview

The UAV Swarm System is a drone coordination platform exposing a REST API and real-time WebSocket feed. It supports two simulation backends (internal physics engine and Gazebo).

### Key Capabilities

- **REST API** (actix-web): full CRUD for drones, formations, missions, swarm
- **Real-time WebSocket**: live drone position/status pushed to clients
- **Formation Control**: Triangle, Line, V-Formation with dynamic reconfiguration
- **Mission Execution**: MoveTo, Patrol, Search with async tick loop
- **Dual Simulation Backend**: internal physics engine or Gazebo via HTTP bridge
- **Hexagonal Architecture**: domain fully decoupled from infrastructure via ports

---

## Architecture Principles

### 1. Ports & Adapters (Hexagonal Architecture)

The architecture strictly separates three layers:

- **Domain** (inner hexagon): business logic, entities, rules — no framework dependency
- **Ports** (`src/ports/`): traits that define what the domain *needs* from the outside world
- **Adapters** (`src/api/`, `src/simulation/`): concrete implementations of ports, wired at startup

This allows swapping any adapter (e.g. replace Gazebo with ROS, replace WebSocket with MQTT) without touching the domain.

### 2. Dependency Rule

```
Adapters → Ports ← Domain
```

Dependencies always point inward. The domain never imports from `api` or `simulation`.

### 3. Async/Await Model

- Tokio runtime throughout
- Non-blocking mission ticks (lock acquired, tick executed, lock released, sleep)
- WebSocket sessions subscribed to a broadcast channel via `EventPublisher`

### 4. Ownership and Borrowing

- `Arc<Mutex<DroneSwarm>>` for shared mutable swarm state across HTTP handlers
- `Arc<dyn Port>` for injected adapters — cloneable, thread-safe
- Compile-time thread-safety guarantees

---

## Hexagonal Architecture

### Overview

```plantuml
@startuml Hexagonal Architecture
!theme plain
skinparam linetype ortho
skinparam nodesep 60
skinparam ranksep 80

' ─────────────────── DOMAIN (inner hexagon) ───────────────────
package "Domain (src/drone, swarm, formation, mission)" #LightYellow {
  [DroneSwarm] as Swarm
  [FormationManager] as FM
  [MissionExecutor] as ME
  [Drone / Position / Velocity] as D
}

' ─────────────────── PORTS ────────────────────────────────────
package "Ports (src/ports/)" #LightBlue {
  interface CommandDispatcher
  interface EventPublisher
  interface DroneStateSource
}

' ─────────────────── PRIMARY ADAPTERS (driving) ───────────────
package "Primary Adapters — API (src/api/)" #LightGreen {
  [HTTP Handlers\n(drones, formations,\nmissions, swarm)] as Handlers
  [AppState\n(Arc<dyn EventPublisher>)] as State
  [WebSocket Session\n(subscribe)] as WS
}

' ─────────────────── SECONDARY ADAPTERS (driven) ──────────────
package "Secondary Adapters — Simulation (src/simulation/)" #LightSalmon {
  [GazeboCommandDispatcher] as GCD
  [GazeboDroneStateSource] as GDSS
  [InternalCommandDispatcher] as ICD
  [GazeboSimulationEngine /\nInternalSimulationEngine] as Engine
}

package "Secondary Adapters — WebSocket (src/api/websocket/)" #LightSalmon {
  [BroadcastEventPublisher] as BEP
  [NullEventPublisher] as NEP
}

' ─────────────────── WIRING ───────────────────────────────────
Handlers --> State
Handlers --> Swarm : locks Arc<Mutex<DroneSwarm>>
State --> EventPublisher : Arc<dyn EventPublisher>
WS --> EventPublisher : subscribe()

Engine --> DroneStateSource : fetch_states()
Engine --> CommandDispatcher : send_command()

GCD ..|> CommandDispatcher
GDSS ..|> DroneStateSource
ICD ..|> CommandDispatcher
BEP ..|> EventPublisher
NEP ..|> EventPublisher

Swarm *-- FM
Swarm *-- ME
Swarm *-- D

@enduml
```

### Business Domain (Core)

The core of the system lives in `src/drone.rs`, `src/swarm.rs`, `src/formation.rs`, `src/mission.rs`. It contains **no** reference to actix-web, reqwest, tokio broadcast, or Gazebo. It is pure business logic:

- `DroneSwarm` orchestrates the drone collection, formations and missions
- `FormationManager` computes geometric offsets and checks formation stability
- `MissionExecutor` manages the mission lifecycle and waypoints

### Ports (Interfaces)

Defined in `src/ports/`, ports are **Rust traits** that infrastructure must implement:

| Port | File | Role |
|---|---|---|
| `CommandDispatcher` | `src/ports/command_dispatcher.rs` | Send a movement command to a drone |
| `EventPublisher` | `src/ports/event_publisher.rs` | Publish a `DroneUpdate` to WebSocket clients |
| `DroneStateSource` | `src/ports/drone_state_source.rs` | Fetch current drone states from a backend |

```rust
// Example: the EventPublisher port
pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: DroneUpdate);
    fn subscribe(&self) -> broadcast::Receiver<DroneUpdate>;
}
```

The domain only knows ports — never the concrete adapters.

### Adapters (Implementations)

#### Primary Adapters (driving — initiate the interaction)

| Adapter | Location | Port used |
|---|---|---|
| HTTP Handlers (drones, formations, missions, swarm) | `src/api/handlers/` | `EventPublisher` (via `AppState`) |
| CLI (`main.rs`) | `src/main.rs` | none — creates the swarm directly |

#### Secondary Adapters (driven — called by the domain/engine)

| Adapter | Location | Port implemented |
|---|---|---|
| `GazeboCommandDispatcher` | `src/simulation/gazebo_client.rs` | `CommandDispatcher` |
| `InternalCommandDispatcher` | `src/simulation/internal_engine.rs` | `CommandDispatcher` |
| `GazeboDroneStateSource` | `src/simulation/gazebo_client.rs` | `DroneStateSource` |
| `BroadcastEventPublisher` | `src/api/websocket/publisher.rs` | `EventPublisher` |
| `NullEventPublisher` | `src/api/websocket/publisher.rs` | `EventPublisher` (tests / CLI) |

#### Adapter Injection

The wiring happens at startup in `src/api/state.rs` and `src/api/server.rs` — the domain never knows which implementation is injected.

```rust
// AppState — EventPublisher wiring
pub struct AppState {
    pub swarm: Arc<Mutex<DroneSwarm>>,
    pub event_publisher: Arc<dyn EventPublisher>,  // ← port
    pub simulation_config: Arc<SimulationConfig>,
}
```

---

## Module Architecture

```plantuml
@startuml UAV Swarm Module Diagram
!theme plain

package "UAV Swarm System" {

  [main] as Main

  package "Domain" #LightYellow {
    [drone] as Drone
    [swarm] as Swarm
    [formation] as Formation
    [mission] as Mission
  }

  package "Ports" #LightBlue {
    [ports/command_dispatcher] as PCD
    [ports/event_publisher] as PEP
    [ports/drone_state_source] as PDSS
  }

  package "API (Primary Adapters)" #LightGreen {
    [api/server] as Server
    [api/state] as State
    [api/handlers] as Handlers
    [api/websocket] as WS
  }

  package "Simulation (Secondary Adapters)" #LightSalmon {
    [simulation/internal_engine] as Internal
    [simulation/gazebo_client] as Gazebo
  }

  package "External" {
    [actix-web] as Actix
    [tokio] as Tokio
    [reqwest] as Reqwest
    [serde] as Serde
  }
}

Main --> Server : run_server()
Main --> Swarm : DroneSwarm

Server --> State : AppState::new_with_config
State --> PEP : Arc<dyn EventPublisher>
State --> Swarm : Arc<Mutex<DroneSwarm>>

Handlers --> State
Handlers --> PEP : publish()
WS --> PEP : subscribe()

Internal ..|> PCD : InternalCommandDispatcher
Gazebo ..|> PCD : GazeboCommandDispatcher
Gazebo ..|> PDSS : GazeboDroneStateSource

Swarm --> Formation
Swarm --> Mission
Swarm --> Drone

Handlers --> Actix
WS --> Tokio
Gazebo --> Reqwest
Drone --> Serde

@enduml
```

### Module Responsibilities

#### `src/main.rs`
- CLI argument parsing (clap)
- Swarm initialization and drone registration
- Routing to `serve`, `status`, `simulate` commands

#### `src/ports/` — pure interfaces
- `command_dispatcher.rs` — `CommandDispatcher` trait
- `event_publisher.rs` — `EventPublisher` trait
- `drone_state_source.rs` — `DroneStateSource` trait + `DroneState` struct

#### `src/api/` — primary HTTP/WebSocket adapter
- `server.rs` — configures and starts the actix-web server
- `state.rs` — `AppState` shared across handlers (swarm + event_publisher)
- `handlers/` — one file per REST resource (drones, formations, missions, swarm)
- `websocket/` — WS handler, session, messages, `BroadcastEventPublisher`

#### `src/simulation/` — secondary adapters
- `engine.rs` — `SimulationEngine` trait
- `internal_engine.rs` — internal physics engine + `InternalCommandDispatcher`
- `gazebo_client.rs` — `GazeboSimulationEngine` + `GazeboCommandDispatcher` + `GazeboDroneStateSource`

#### `src/swarm.rs` — domain orchestrator
- Manages the drone collection
- Delegates to `FormationManager` and `MissionExecutor`
- Runs the simulation loop

#### `src/drone.rs`, `src/formation.rs`, `src/mission.rs` — pure domain
- Entities, business rules, geometric calculations
- No dependency on external layers

---

## Class Diagram

```plantuml
@startuml UAV Swarm Class Diagram
!theme plain
skinparam classAttributeIconSize 0
skinparam linetype ortho

' ─── Ports ───
interface CommandDispatcher {
  +send_command(drone_id, target): async Result
}

interface EventPublisher {
  +publish(DroneUpdate)
  +subscribe(): Receiver<DroneUpdate>
}

interface DroneStateSource {
  +fetch_states(): async Result<HashMap<String, DroneState>>
}

interface SimulationEngine {
  +initialize(): async Result
  +update_drones(...): async Result
  +shutdown(): async Result
  +mode(): SimulationMode
  +is_connected(): bool
}

' ─── Adapters ───
class BroadcastEventPublisher {
  -tx: Sender<DroneUpdate>
  +new(capacity): Self
}

class NullEventPublisher {
  +new(): Self
}

class GazeboCommandDispatcher {
  -client: Client
  -bridge_url: String
  +new(bridge_url, timeout_ms): Self
}

class InternalCommandDispatcher {}

class GazeboDroneStateSource {
  -client: Client
  -bridge_url: String
  +new(bridge_url, timeout_ms): Self
}

class GazeboSimulationEngine {
  -client: Client
  -bridge_url: String
  -connected: bool
  -state_source: Box<dyn DroneStateSource>
  +new(bridge_url, timeout_ms): Self
  +new_with_state_source(...): Self
}

class InternalSimulationEngine {
  -drones_last_update: HashMap<String, Instant>
}

' ─── AppState ───
class AppState {
  +swarm: Arc<Mutex<DroneSwarm>>
  +event_publisher: Arc<dyn EventPublisher>
  +simulation_config: Arc<SimulationConfig>
  +new(swarm): Self
  +new_with_config(swarm, config): Self
}

' ─── Domain ───
class DroneSwarm {
  +drones: HashMap<String, Drone>
  +formation_manager: FormationManager
  +mission_executor: MissionExecutor
  +simulation_running: bool
  --
  +add_drone(id, position)
  +set_formation(type): async
  +update_swarm(): async
  +tick_mission_by_id(id): Result<bool>
  +get_swarm_status(): Vec<DroneStatusInfo>
}

class Drone {
  +id: String
  +position: Position
  +velocity: Velocity
  +status: DroneStatus
  +target_position: Option<Position>
  +formation_offset: Option<Position>
  +max_speed: f64
  --
  +move_to(target)
  +update_position(dt)
  +set_formation_offset(offset)
}

class FormationManager {
  +formation_type: FormationType
  +separation_distance: f64
  -formation_offsets: HashMap<String, Position>
  --
  +set_formation_type(FormationType)
  +update_formation(drones)
  +is_formation_stable(drones): bool
}

class MissionExecutor {
  +active_missions: HashMap<String, Mission>
  --
  +create_mission(type, drones): String
  +start_mission(id): Result
  +cancel_mission(id): Result
}

enum DroneUpdate {
  PositionUpdate
  StatusChange
  FormationUpdate
  MissionProgress
}

' ─── Relationships ───
BroadcastEventPublisher ..|> EventPublisher
NullEventPublisher ..|> EventPublisher
GazeboCommandDispatcher ..|> CommandDispatcher
InternalCommandDispatcher ..|> CommandDispatcher
GazeboDroneStateSource ..|> DroneStateSource
GazeboSimulationEngine ..|> SimulationEngine
InternalSimulationEngine ..|> SimulationEngine

GazeboSimulationEngine o-- DroneStateSource : state_source
GazeboSimulationEngine ..> GazeboDroneStateSource : creates

AppState o-- EventPublisher
AppState o-- DroneSwarm

DroneSwarm *-- FormationManager
DroneSwarm *-- MissionExecutor
DroneSwarm o-- Drone

EventPublisher ..> DroneUpdate : publishes

@enduml
```

---

## Behavioral Diagrams

### Mission Execution Flow

```plantuml
@startuml Mission Execution Sequence
!theme plain
autonumber

actor User
participant "HTTP Handler\n(missions.rs)" as Handler
participant "AppState" as State
participant "DroneSwarm" as Swarm
participant "MissionExecutor" as ME
participant "EventPublisher" as EP

User -> Handler: POST /api/missions\n{ type: "move_to", target }
activate Handler

Handler -> State: swarm.lock()
activate State
State --> Handler: MutexGuard<DroneSwarm>

Handler -> ME: create_mission(MissionType, drone_ids)
ME --> Handler: mission_id

Handler -> ME: start_mission(mission_id)
Handler -> State: drop lock
deactivate State

Handler -> Handler: tokio::spawn(tick loop)
Handler --> User: 200 { id, status: "in_progress" }
deactivate Handler

loop tick every 100ms
  Handler -> State: swarm.lock()
  activate State
  State --> Handler: guard
  Handler -> Swarm: tick_mission_by_id(id)
  Swarm -> ME: advance waypoint
  ME --> Swarm: running=true / completed=false
  Handler -> ME: get current_waypoint
  Handler -> State: drop lock
  deactivate State

  Handler -> EP: publish(MissionProgress { mission_id, waypoint })
  EP --> User: WebSocket push

  alt completed
    Handler -> Handler: break
  end
end

@enduml
```

### Formation Change Flow

```plantuml
@startuml Formation Change Sequence
!theme plain
autonumber

actor User
participant "HTTP Handler\n(formations.rs)" as Handler
participant "DroneSwarm" as Swarm
participant "FormationManager" as FM
participant "EventPublisher" as EP

User -> Handler: POST /api/formations/current\n{ formation_type: "triangle" }
activate Handler

Handler -> Swarm: swarm.lock()
activate Swarm

Handler -> FM: FormationType::from_str("triangle")
FM --> Handler: Some(Triangle)

Handler -> Swarm: set_formation("triangle")
Swarm -> FM: set_formation_type(Triangle)
FM -> FM: calculate_offsets()
FM -> FM: update_formation(drones)

Handler -> FM: is_formation_stable(drones) → bool
Handler -> Swarm: drop lock
deactivate Swarm

Handler -> EP: publish(FormationUpdate { formation_stable })
EP --> User: WebSocket push

Handler --> User: 200 { message: "Formation changed to triangle" }
deactivate Handler

@enduml
```

### Drone State Machine

```plantuml
@startuml Drone State Diagram
!theme plain

[*] --> Idle : Drone created

state Idle {
  Idle : velocity = 0
  Idle : no target
}

state Navigating {
  Navigating : moving to target_position
  Navigating : velocity > 0
}

state InFormation {
  InFormation : maintaining offset
  InFormation : following leader
}

state ExecutingMission {
  ExecutingMission : following waypoints
  ExecutingMission : coordinated by MissionExecutor
}

state Error {
  Error : message stored
  Error : drone halted
}

Idle --> Navigating : move_to()
Idle --> InFormation : set_formation_offset()
Idle --> ExecutingMission : mission assigned

Navigating --> Idle : distance < 0.1
Navigating --> InFormation : set_formation_offset()
Navigating --> Error : error

InFormation --> Navigating : distance > 1.0
InFormation --> ExecutingMission : mission started
InFormation --> Error : error

ExecutingMission --> Navigating : waypoint movement
ExecutingMission --> Idle : mission completed
ExecutingMission --> Error : mission failed

Error --> Idle : resolved

@enduml
```

---

## Design Patterns

### 1. Ports & Adapters (Hexagonal Architecture)

**Ports**: `CommandDispatcher`, `EventPublisher`, `DroneStateSource`

The domain defines *what it needs* through traits. Infrastructure provides the implementations. No coupling to actix-web, reqwest, or tokio broadcast inside the domain.

**Benefits**:
- Testability: mock `DroneStateSource` to test the Gazebo engine in isolation
- Extensibility: new adapter (MQTT, ROS, gRPC) without touching the domain
- Isolation: replacing the HTTP server does not affect simulation physics

### 2. Strategy Pattern

**Implementation**: `SimulationEngine` (trait), `FormationType` (enum)

- `InternalSimulationEngine` vs `GazeboSimulationEngine`: interchangeable backends
- `Triangle / Line / VFormation`: formation algorithms selectable at runtime

### 3. Command Pattern

**Implementation**: `MissionType` enum

```rust
enum MissionType {
    MoveTo(Position),
    Patrol(Vec<Position>),
    Search(Position, f64),
}
```

Missions are first-class objects — loggable and cancellable.

### 4. Observer Pattern

**Implementation**: `EventPublisher` + `BroadcastEventPublisher`

Handlers publish `DroneUpdate` events; WebSocket sessions subscribe via `subscribe()`. The Tokio broadcast publisher ensures 1-to-N delivery.

### 5. State Pattern

**Implementation**: `DroneStatus` enum

Explicit transitions: Idle → Navigating → InFormation → ExecutingMission → Error.

### 6. Orchestrator Pattern

**Implementation**: `DroneSwarm`

Single entry point for the drone collection. Delegates to `FormationManager` and `MissionExecutor`.

---

## Key Components

### `AppState` (`src/api/state.rs`)

Adapter injection point for the HTTP layer:

```rust
pub struct AppState {
    pub swarm: Arc<Mutex<DroneSwarm>>,            // shared domain
    pub event_publisher: Arc<dyn EventPublisher>, // WebSocket port
    pub simulation_config: Arc<SimulationConfig>,
}
```

All HTTP handlers receive `web::Data<AppState>`.

### `BroadcastEventPublisher` (`src/api/websocket/publisher.rs`)

Adapter for the `EventPublisher` port, backed by `tokio::sync::broadcast`:

- `publish()`: sends to all subscribers, ignores errors (no subscriber = OK)
- `subscribe()`: returns a `Receiver` used by WebSocket sessions
- `NullEventPublisher`: no-op for tests and CLI mode

### `GazeboDroneStateSource` (`src/simulation/gazebo_client.rs`)

Adapter for the `DroneStateSource` port — fetches states via the Gazebo HTTP bridge:

```rust
async fn fetch_states(&self) -> Result<HashMap<String, DroneState>, String> {
    // GET {bridge_url}/drones/states → HashMap<id, DroneStateUpdate>
    // → maps to DroneState (port format)
}
```

`GazeboSimulationEngine` injects it via `new_with_state_source()` for testing.

### `CommandDispatcher` (`src/ports/command_dispatcher.rs`)

Port for sending movement commands:

- `GazeboCommandDispatcher`: POST to the Gazebo HTTP bridge
- `InternalCommandDispatcher`: directly updates `drone.target_position`

---

## Code Navigation

### File Structure

```
src/
├── main.rs                    # CLI entry point + initial wiring
├── drone.rs                   # Drone entity + Position + Velocity
├── formation.rs               # FormationManager
├── mission.rs                 # Mission + MissionExecutor
├── swarm.rs                   # DroneSwarm (domain orchestrator)
│
├── ports/
│   ├── mod.rs
│   ├── command_dispatcher.rs  # CommandDispatcher port
│   ├── event_publisher.rs     # EventPublisher port
│   └── drone_state_source.rs  # DroneStateSource port + DroneState
│
├── api/
│   ├── mod.rs
│   ├── server.rs              # actix-web server startup
│   ├── state.rs               # AppState (adapter injection)
│   ├── routes.rs              # Route configuration
│   ├── error.rs               # ApiError
│   ├── models.rs              # Request/response DTOs
│   ├── handlers/
│   │   ├── drones.rs          # GET/PUT /api/drones
│   │   ├── formations.rs      # GET/POST /api/formations
│   │   ├── missions.rs        # GET/POST /api/missions
│   │   └── swarm.rs           # GET/POST /api/swarm
│   └── websocket/
│       ├── messages.rs        # DroneUpdate enum
│       ├── session.rs         # WS session handler (subscribe)
│       ├── server.rs          # websocket_handler (actix-ws)
│       └── publisher.rs       # BroadcastEventPublisher + NullEventPublisher
│
└── simulation/
    ├── mod.rs
    ├── engine.rs              # SimulationEngine trait
    ├── mode.rs                # SimulationMode enum
    ├── config.rs              # SimulationConfig
    ├── internal_engine.rs     # Internal physics engine + InternalCommandDispatcher
    └── gazebo_client.rs       # GazeboSimulationEngine + GazeboCommandDispatcher
                               #   + GazeboDroneStateSource
```

### Key Locations

**Ports (interfaces)**:
- `src/ports/command_dispatcher.rs:1` — `CommandDispatcher` trait
- `src/ports/event_publisher.rs:1` — `EventPublisher` trait
- `src/ports/drone_state_source.rs:1` — `DroneStateSource` trait + `DroneState`

**Injection / wiring**:
- `src/api/state.rs:20` — `AppState::new_with_config` creates `BroadcastEventPublisher`
- `src/simulation/gazebo_client.rs` — `GazeboSimulationEngine::new` creates `GazeboDroneStateSource`
- `src/simulation/gazebo_client.rs` — `new_with_state_source()` for testing

**Event publishing**:
- `src/api/handlers/drones.rs:165` — `event_publisher.publish(PositionUpdate)`
- `src/api/handlers/formations.rs:68` — `event_publisher.publish(FormationUpdate)`
- `src/api/handlers/missions.rs:97` — `event_publisher.publish(MissionProgress)`
- `src/api/handlers/swarm.rs:77` — `event_publisher.publish(PositionUpdate)` (simulation loop)

**WebSocket subscribe**:
- `src/api/websocket/session.rs:11` — `state.event_publisher.subscribe()`

**Domain physics**:
- `src/drone.rs:109` — `update_position(dt)` with delta time
- `src/formation.rs:58` — formation algorithm dispatch
- `src/mission.rs:123` — waypoint execution loop

---

## Conclusion

The hexagonal architecture ensures that the business domain (drone physics, formations, missions) remains **independent** of any infrastructure. The three extracted ports — `CommandDispatcher`, `EventPublisher`, `DroneStateSource` — are the only interfaces between the core and the outside world.

**Strengths**:
- The domain is testable without actix-web, Gazebo, or WebSocket
- Each adapter can be replaced independently (mock in tests, Gazebo in production)
- `NullEventPublisher` enables CLI mode without an active Tokio broadcast
- `new_with_state_source()` on the Gazebo engine allows mock injection in integration tests
- The dependency rule is strictly enforced: no import of `api` or `simulation` in the domain

---

**Document Version**: 2.0
**Last Updated**: 2026-02-20
**Architecture**: Ports & Adapters (Hexagonal)
