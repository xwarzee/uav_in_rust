# Gazebo Simulation Assets

This directory contains all Gazebo-related files for the UAV swarm simulation, including the C++ REST bridge plugin, world definitions, and drone models.

## Directory Structure

```
gazebo/
├── plugins/
│   └── rest_bridge/          # C++ plugin for Gazebo-Rust communication
│       ├── CMakeLists.txt    # Build configuration
│       ├── RestBridgePlugin.hh
│       ├── RestBridgePlugin.cc
│       ├── HttpServer.hh
│       └── HttpServer.cc
│
├── worlds/
│   └── uav_swarm.sdf         # Main simulation world with 3 drones
│
├── models/
│   └── x3_uav/               # Quadrotor drone model
│       ├── model.config      # Model metadata
│       └── model.sdf         # Model definition
│
└── launch/
    └── start_simulation.sh   # Launch script (builds plugin + starts Gazebo)
```

## Quick Start (Local Testing - macOS)

### 1. Install Ignition Gazebo Fortress

```bash
brew tap osrf/simulation
brew install ignition-fortress
```

### 2. Build and Launch

```bash
cd gazebo/launch
./start_simulation.sh
```

The script will:
- Check for Ignition Gazebo installation
- Build the C++ plugin (if not already built)
- Set environment variables
- Launch Gazebo with the UAV swarm world

### 3. Test the Plugin

In a separate terminal:

```bash
# Health check
curl http://localhost:8092/health

# Start sync (Gazebo → Rust)
curl -X POST http://localhost:8092/start

# Send command to drone
curl -X POST http://localhost:8092/drones/drone_1/command \
  -H "Content-Type: application/json" \
  -d '{"target_position": {"x": 10, "y": 5, "z": 3}}'
```

## Deployment to Remote Linux Server

For production deployment on a remote Linux server, see the comprehensive guide: **[REMOTE_GAZEBO_SETUP.md](../REMOTE_GAZEBO_SETUP.md)**

### Quick Deployment Summary

1. **Copy files to server:**
```bash
scp -r gazebo/ user@server:/path/to/uav_in_rust/
```

2. **On server, install dependencies:**
```bash
# See REMOTE_GAZEBO_SETUP.md for full instructions
sudo apt-get install ignition-fortress
sudo apt-get install cmake g++ libignition-gazebo7-dev
```

3. **Build and run:**
```bash
cd gazebo/launch
./start_simulation.sh
```

4. **Configure firewall (if remote access needed):**
```bash
sudo ufw allow 8092/tcp
```

## Plugin Architecture

### REST Bridge Plugin

The `RestBridgePlugin` enables bidirectional communication between Gazebo and the Rust application:

**Gazebo → Rust (State Updates):**
- Plugin reads drone positions/velocities each physics tick
- Sends HTTP PUT to `http://localhost:8080/api/drones/{id}/state`
- Controlled by sync enabled/disabled state

**Rust → Gazebo (Commands):**
- Rust API sends HTTP POST to `http://localhost:8092/drones/{id}/command`
- Plugin receives target position and applies forces to drone model
- Uses simple P-controller for position control

### HTTP Endpoints (Port 8092)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check, returns drone list and sync status |
| POST | `/start` | Enable sync (start sending states to Rust) |
| POST | `/stop` | Disable sync |
| GET | `/drones/states` | Get current states of all drones |
| POST | `/drones/{id}/command` | Send target position command to drone |

## Plugin Configuration

The plugin is configured in the world SDF file (`worlds/uav_swarm.sdf`):

```xml
<plugin filename="libRestBridgePlugin.so" name="gazebo_plugins::RestBridgePlugin">
  <rust_api_url>http://localhost:8080</rust_api_url>
  <http_port>8092</http_port>
  <drone>drone_1</drone>
  <drone>drone_2</drone>
  <drone>drone_3</drone>
</plugin>
```

**Parameters:**
- `rust_api_url`: URL of the Rust UAV API (change for remote setup)
- `http_port`: Port for plugin HTTP server (default: 8092)
- `drone`: Name of each drone to track (can have multiple)

## Drone Model (x3_uav)

Simple quadrotor model with:
- **Mass:** 1.5 kg
- **Geometry:** Box body (0.3×0.3×0.15 m) + 4 cylindrical rotors
- **Visual:** Blue body with red/green propeller indicators
- **Physics:** Realistic inertia for quadrotor dynamics

The model uses Ignition's `ApplyLinkWrench` plugin to accept force/velocity commands.

## Manual Build (Advanced)

If you need to build the plugin manually:

```bash
cd gazebo/plugins/rest_bridge
mkdir -p build && cd build
cmake ..
make
```

### Build Requirements

- **CMake:** 3.10+
- **C++ Compiler:** GCC 9+ or Clang
- **Ignition Gazebo:** 7.x (Fortress) or 8.x (Garden)
- **Dependencies:**
  - `libignition-gazebo7-dev`
  - `libignition-transport12-dev`
  - `libignition-math7-dev`
  - `cpp-httplib` (auto-downloaded by CMake)

## Troubleshooting

### Plugin not loading

**Symptom:** Gazebo starts but plugin not active

**Solutions:**
1. Check plugin was built: `ls gazebo/plugins/rest_bridge/build/libRestBridgePlugin.so`
2. Verify environment variable: `echo $IGN_GAZEBO_SYSTEM_PLUGIN_PATH`
3. Check Gazebo console output for error messages
4. Run with verbose: `ign gazebo worlds/uav_swarm.sdf --verbose 4`

### Drones not found

**Symptom:** Plugin logs "Warning: Drone 'drone_X' not found in world!"

**Solutions:**
1. Check model path: `echo $IGN_GAZEBO_RESOURCE_PATH`
2. Verify model exists: `ls gazebo/models/x3_uav/model.sdf`
3. Ensure model.config is present
4. Check SDF syntax in world file

### HTTP server port in use

**Symptom:** "Failed to start HTTP server on port 8092"

**Solutions:**
1. Check if port is already used: `netstat -an | grep 8092` or `lsof -i :8092`
2. Kill existing process or change port in world SDF
3. Ensure firewall allows port 8092

### Connection to Rust API fails

**Symptom:** "Failed to send state update to Rust API"

**Solutions:**
1. Verify Rust server is running: `curl http://localhost:8080/health`
2. Check `rust_api_url` in world SDF matches Rust server address
3. For remote deployment, update URL to server IP
4. Check firewall allows outbound HTTP from Gazebo server

## Performance Notes

- **Physics tick rate:** 1000 Hz (1ms steps)
- **Sync frequency:** Controlled by Rust API update rate (default: 10 Hz)
- **HTTP latency:** ~1-5ms local, ~10-50ms LAN, ~50-300ms internet
- **Recommended setup:** Same datacenter or LAN for <10ms latency

## Next Steps

1. **Local testing:** Run `./launch/start_simulation.sh` to test locally
2. **Remote deployment:** Follow [REMOTE_GAZEBO_SETUP.md](../REMOTE_GAZEBO_SETUP.md)
3. **Integration:** Update `config/simulation.toml` with server URL
4. **Testing:** Use `test_simulation_api.sh` for end-to-end tests

## Resources

- **Ignition Gazebo Docs:** https://gazebosim.org/docs/fortress
- **SDF Format:** http://sdformat.org/
- **Plugin Tutorial:** https://gazebosim.org/api/gazebo/7/createplugins.html
- **cpp-httplib:** https://github.com/yhirose/cpp-httplib
