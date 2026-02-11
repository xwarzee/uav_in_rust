# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-02-11

### 🎉 Initial Release

#### Added

- **Core System**
  - Internal simulation engine (Rust physics)
  - Gazebo simulation engine (external integration)
  - Management of 3-drone swarm
  - Formations: triangle, line, V-formation
  - Complete REST API (port 8080)
  - WebSocket for real-time updates

- **Gazebo Integration**
  - C++ RestBridge plugin (port 8092)
  - Bidirectional communication Rust ↔ Gazebo
  - Lazy drone search (resolves loading issues)
  - ApplyLinkWrench plugin support for future forces

- **Configuration**
  - TOML file for central configuration
  - Environment variables for overrides
  - Support for two deployment scenarios:
    1. Local Rust + Remote Gazebo
    2. Unified server (Rust + Gazebo)

- **REST API**
  - Simulation endpoints: mode, status, start, stop
  - Drone endpoints: list, get, update, command
  - Formation endpoints: get, set
  - Automatic Swagger UI documentation

- **3D Models**
  - x3_uav drone model (simplified quadcopter)
  - uav_swarm.sdf world with 3 drones
  - External model support via `<include>`

- **Documentation**
  - Complete user guide with UML diagrams
  - Technical deployment guide
  - Mermaid diagrams for architecture
  - Illustrated deployment scenarios

- **Scripts**
  - `start_simulation.sh` to launch Gazebo
  - `test_simulation_api.sh` for automated tests
  - Headless mode support for GUI-less servers

#### Fixed

- TOML configuration: added required quotes for URLs
- RestBridgePlugin: lazy drone search to avoid startup errors
- ApplyLinkWrench plugin: moved to world level instead of models
- Test script: macOS compatibility (`head -n-1` → `sed '$d'`)
- AppState: use real configuration instead of default values
- Plugin paths: corrected to point to `build/lib/` instead of `build/`

#### Security

- Documented firewall configuration
- VPN support (WireGuard) for secure communications
- SSL/TLS recommendations for production
- Configured CORS for REST API

---

## [Unreleased]

### Planned Features

#### v1.1.0

- [ ] Support for more than 3 drones (dynamic configuration)
- [ ] Advanced formations (circle, spiral, grid)
- [ ] Complex missions (patrol, search)
- [ ] Real-time web dashboard
- [ ] Mission configuration persistence

#### v1.2.0

- [ ] Collision management between drones
- [ ] Wind and weather conditions simulation
- [ ] Battery management and RTH (Return To Home)
- [ ] Path optimization
- [ ] Multi-world support (parallel simulations)

#### v2.0.0

- [ ] Architecture refactoring: Event-driven
- [ ] ROS2 integration support
- [ ] Machine Learning for autonomous behaviors
- [ ] Real hardware support (DJI SDK, PX4)
- [ ] Multi-tenancy (multiple users/missions)

---

## Known Issues

### v1.0.0

#### Critical

- None

#### Major

- **[GAZ-001]** Gazebo can take up to 5 seconds to load models
  - **Workaround**: Plugin does lazy search, wait a few seconds
  - **Status**: Normal Gazebo behavior, no fix planned

#### Minor

- **[API-001]** WebSocket not yet implemented
  - **Status**: Planned for v1.1.0

- **[DOC-001]** Mermaid diagrams not rendered in some IDEs
  - **Workaround**: Use GitHub or Mermaid Live Editor
  - **Status**: IDE limitation, no fix possible

---

## Migrations

### From dev to v1.0.0

**Breaking changes**:

- `api::run_server()` signature modified to accept `SimulationConfig`
- TOML file now requires quotes for URLs

**Required actions**:

1. Update code calling `run_server()`:

```rust
// Before
api::run_server(swarm, host, port).await?;

// After
api::run_server(swarm, config, host, port).await?;
```

2. Fix `config/simulation.toml`:

```toml
# Before
bridge_url = http://localhost:8092

# After
bridge_url = "http://localhost:8092"
```

3. Recompile Gazebo plugin:

```bash
cd gazebo/plugins/rest_bridge/build
cmake .. && make
```

4. Copy new world file to Gazebo server:

```bash
scp gazebo/worlds/uav_swarm.sdf ubuntu@server:/home/ubuntu/gazebo/worlds/
```

---

## Deprecations

### v1.0.0

- No deprecations in this first version

---

## Acknowledgments

- **Ignition Gazebo** for the simulation engine
- **Rust Community** for the actix-web ecosystem
- **Claude.AI** for development assistance

---

## Version Comparison

| Feature | v1.0.0 | v1.1.0 (planned) | v2.0.0 (planned) |
|---------|--------|------------------|------------------|
| Number of drones | 3 fixed | Configurable | Unlimited |
| Simulation modes | 2 | 2 | 3 (+ Hardware) |
| Formations | 3 | 8 | Customizable |
| API | REST | REST + WS | REST + WS + gRPC |
| Dashboard | CLI | Web | Advanced Web |
| ML/AI | ❌ | ❌ | ✅ |
| ROS2 | ❌ | ❌ | ✅ |
| Real hardware | ❌ | ❌ | ✅ |

---

**Legend**:

- 🎉 Major new feature
- 🐛 Bug fix
- 🔒 Security improvement
- 📝 Documentation
- ⚠️ Breaking change
- 🗑️ Deprecated

---

For more details on each version, see [GitHub releases](https://github.com/your-org/uav_in_rust/releases).
