# UAV Swarm System - UML Documentation

This directory contains comprehensive UML diagrams and architecture documentation for the UAV Swarm Controller.

## Overview

The UAV Swarm System follows a **Ports & Adapters (Hexagonal) Architecture**. The core domain (drones, formations, missions) is fully decoupled from infrastructure via three ports:

| Port | Role |
|---|---|
| `CommandDispatcher` | Send movement commands to drones |
| `EventPublisher` | Push `DroneUpdate` events to WebSocket clients |
| `DroneStateSource` | Fetch current drone states from a simulation backend |

## Main Document

→ **[ARCHITECTURE.md](ARCHITECTURE.md)** — full architecture documentation including:

- Hexagonal architecture overview (diagram + explanation of layers)
- Ports and adapters description
- Module diagram (updated to include `ports/`, `api/`, `simulation/`)
- Class diagram (ports, adapters, domain)
- Behavioral diagrams (mission execution, formation change, drone state machine)
- Design patterns (Ports & Adapters, Strategy, Observer, Command, State)
- Code navigation guide

## Architecture in One Diagram

```
┌─────────────────────────────────────────────────────────┐
│              Primary Adapters (driving)                  │
│   HTTP Handlers (actix-web)  │  CLI (main.rs)            │
└──────────────────┬──────────────────────────────────────┘
                   │ uses Arc<dyn EventPublisher>
         ┌─────────▼──────────────────────┐
         │         PORTS                  │
         │  CommandDispatcher             │
         │  EventPublisher                │
         │  DroneStateSource              │
         └─────────┬──────────────────────┘
                   │
         ┌─────────▼──────────────────────┐
         │         DOMAIN                 │
         │  DroneSwarm / Drone            │
         │  FormationManager              │
         │  MissionExecutor               │
         └─────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────┐
│              Secondary Adapters (driven)                  │
│  GazeboCommandDispatcher  │  BroadcastEventPublisher     │
│  GazeboDroneStateSource   │  NullEventPublisher          │
│  InternalCommandDispatcher                               │
└─────────────────────────────────────────────────────────┘
```

## Key Design Decisions

1. **No domain→infra dependency**: `src/drone.rs`, `src/swarm.rs`, `src/formation.rs`, `src/mission.rs` never import from `api` or `simulation`
2. **`Arc<dyn Port>`**: adapters injected as trait objects — cloneable and thread-safe across actix handlers
3. **`NullEventPublisher`**: no-op adapter for CLI mode and unit tests (no Tokio broadcast needed)
4. **`new_with_state_source()`**: `GazeboSimulationEngine` accepts an injected `DroneStateSource` for integration testing with a mock

## Code Navigation

```
src/
├── ports/                     ← interfaces (traits)
│   ├── command_dispatcher.rs
│   ├── event_publisher.rs
│   └── drone_state_source.rs
├── api/websocket/publisher.rs ← EventPublisher adapters
├── simulation/gazebo_client.rs ← CommandDispatcher + DroneStateSource adapters
└── api/state.rs               ← wiring: AppState injects Arc<dyn EventPublisher>
```
