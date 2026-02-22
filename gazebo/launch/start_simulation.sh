#!/bin/bash
# Launch script for Gazebo with UAV swarm simulation
# Compatible with macOS and Linux
#
# Usage:
#   ./start_simulation.sh              # Gazebo mode — GUI, with C++ RestBridgePlugin
#   ./start_simulation.sh --headless   # Gazebo mode — headless (no GUI)
#   ./start_simulation.sh --ros2       # ROS2 mode   — GUI, no RestBridgePlugin
#   ./start_simulation.sh --ros2 --headless  # ROS2 mode — headless
#
# Two simulation modes:
#
#   gazebo (default)
#     Rust app ←HTTP:8092→ RestBridgePlugin (C++) ←ECM→ Gazebo
#     Started by: this script
#     Rust CLI:   cargo run -- --mode gazebo serve
#
#   ros2
#     Rust app ←HTTP:8082→ http_bridge_node.py ←ROS2→ ros_gz_bridge ←IGN→ Gazebo
#     For the full ROS2 stack (Gazebo + bridge + HTTP node), prefer:
#       ros2 launch ros2_bridge ros2_gazebo_bridge.launch.py [headless:=true]
#     This script only starts Gazebo with the ROS2-compatible world;
#     ros_gz_bridge and the Python HTTP node must be started separately.
#     Rust CLI:   cargo run -- --mode ros2 serve

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Parse arguments
HEADLESS=false
ROS2_MODE=false
for arg in "$@"; do
    case "$arg" in
        --headless|-s) HEADLESS=true ;;
        --ros2)        ROS2_MODE=true ;;
    esac
done

echo "=========================================="
echo "UAV Swarm Gazebo Simulation Launcher"
echo "=========================================="
echo ""

# Check if Ignition Gazebo is installed
if ! command -v ign &> /dev/null; then
    echo "Error: Ignition Gazebo (ign command) not found!"
    echo ""
    echo "Please install Ignition Gazebo Fortress:"
    echo "  macOS:  brew install ignition-fortress"
    echo "  Ubuntu: See REMOTE_GAZEBO_SETUP.md"
    echo ""
    exit 1
fi

# Display version
IGN_VERSION=$(ign gazebo --version 2>&1 | head -n1 || echo "Unknown")
echo "Ignition Gazebo version: $IGN_VERSION"
echo ""

# Set environment variables for model and plugin paths
export IGN_GAZEBO_RESOURCE_PATH="$IGN_GAZEBO_RESOURCE_PATH:$PROJECT_ROOT/gazebo/models"
export IGN_GAZEBO_SYSTEM_PLUGIN_PATH="$IGN_GAZEBO_SYSTEM_PLUGIN_PATH:$PROJECT_ROOT/gazebo/plugins/rest_bridge/build/lib"

echo "Environment:"
echo "  Model path:  $PROJECT_ROOT/gazebo/models"
if [ "$ROS2_MODE" = false ]; then
    echo "  Plugin path: $PROJECT_ROOT/gazebo/plugins/rest_bridge/build/lib"
fi
echo ""

# ---- Gazebo mode: check / build the C++ RestBridgePlugin ----
if [ "$ROS2_MODE" = false ]; then
    PLUGIN_PATH="$PROJECT_ROOT/gazebo/plugins/rest_bridge/build/lib/libRestBridgePlugin.so"
    if [ ! -f "$PLUGIN_PATH" ]; then
        echo "Warning: RestBridgePlugin not found at:"
        echo "  $PLUGIN_PATH"
        echo ""
        echo "Building plugin..."

        cd "$PROJECT_ROOT/gazebo/plugins/rest_bridge"

        if [ ! -d "build" ]; then
            mkdir build
        fi

        cd build

        echo "Running CMake..."
        cmake .. || {
            echo "Error: CMake failed. Make sure dependencies are installed:"
            echo "  - ignition-gazebo7-dev (or ignition-fortress)"
            echo "  - ignition-transport12-dev"
            echo "  - ignition-math7-dev"
            echo "  - cmake, g++/clang++"
            exit 1
        }

        echo "Compiling plugin..."
        make || {
            echo "Error: Compilation failed."
            exit 1
        }

        echo "Plugin built successfully!"
        echo ""
    fi
fi

# ---- Select world file ----
if [ "$ROS2_MODE" = true ]; then
    WORLD_FILE="$PROJECT_ROOT/gazebo/worlds/uav_swarm_ros2.sdf"
else
    WORLD_FILE="$PROJECT_ROOT/gazebo/worlds/uav_swarm_inline.sdf"
fi

if [ ! -f "$WORLD_FILE" ]; then
    echo "Error: World file not found at:"
    echo "  $WORLD_FILE"
    exit 1
fi

# ---- Status messages ----
if [ "$ROS2_MODE" = true ]; then
    echo "Mode: ROS2 (PosePublisher + set_pose service, no C++ plugin)"
    echo ""
    echo "NOTE: This script only starts Gazebo."
    echo "You still need to start ros_gz_bridge and the HTTP bridge node:"
    echo ""
    echo "  Option A — all-in-one via ROS2 launch:"
    echo "    ros2 launch ros2_bridge ros2_gazebo_bridge.launch.py headless:=true"
    echo ""
    echo "  Option B — separate terminals:"
    echo "    ros2 run ros_gz_bridge parameter_bridge \\"
    echo "      '/world/uav_swarm_world/pose/info@tf2_msgs/msg/TFMessage[ignition.msgs.Pose_V' \\"
    echo "      '/world/uav_swarm_world/set_pose@ros_gz_interfaces/srv/SetEntityPose'"
    echo ""
    echo "    ros2 run ros2_bridge http_bridge_node"
    echo ""
    echo "Then start the Rust app:"
    echo "    cargo run -- --mode ros2 serve"
else
    # Check if Rust API is running (optional warning)
    if curl -s http://localhost:8080/health > /dev/null 2>&1; then
        echo "✓ Rust UAV API detected on http://localhost:8080"
    else
        echo "⚠ Warning: Rust UAV API not detected on http://localhost:8080"
        echo "  The plugin will try to connect to the API when sync is enabled."
        echo "  Start the Rust server with: cargo run -- --mode gazebo serve"
    fi
    echo ""
    echo "Mode: Gazebo (C++ RestBridgePlugin — port 8092)"
    echo ""
    echo "Available endpoints:"
    echo "  GET  http://localhost:8092/health              - Health check"
    echo "  POST http://localhost:8092/start               - Start sync to Rust"
    echo "  POST http://localhost:8092/stop                - Stop sync"
    echo "  GET  http://localhost:8092/drones/states       - Get drone states"
    echo "  POST http://localhost:8092/drones/{id}/command - Send command"
fi
echo ""

if [ "$HEADLESS" = true ]; then
    echo "Launching Gazebo in HEADLESS mode (no GUI)..."
else
    echo "Launching Gazebo with GUI..."
    echo "Tip: use --headless for servers without display"
fi
echo "  World: $WORLD_FILE"
echo ""
echo "Press Ctrl+C to stop simulation"
echo "=========================================="
echo ""

# Launch Gazebo
if [ "$HEADLESS" = true ]; then
    ign gazebo -s "$WORLD_FILE" --verbose 4
else
    ign gazebo "$WORLD_FILE" --verbose 4
fi
