#!/bin/bash
# Launch script for Gazebo with UAV swarm simulation
# Compatible with macOS and Linux
#
# Usage:
#   ./start_simulation.sh          # Launch with GUI (default)
#   ./start_simulation.sh --headless  # Launch without GUI (for servers)

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Parse arguments
HEADLESS=false
if [ "$1" = "--headless" ] || [ "$1" = "-s" ]; then
    HEADLESS=true
fi

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
echo "  Model path: $PROJECT_ROOT/gazebo/models"
echo "  Plugin path: $PROJECT_ROOT/gazebo/plugins/rest_bridge/build/lib"
echo ""

# Check if plugin is built
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

# Check if world file exists
WORLD_FILE="$PROJECT_ROOT/gazebo/worlds/uav_swarm.sdf"
if [ ! -f "$WORLD_FILE" ]; then
    echo "Error: World file not found at:"
    echo "  $WORLD_FILE"
    exit 1
fi

# Check if Rust API is running (optional warning)
if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "✓ Rust UAV API detected on http://localhost:8080"
else
    echo "⚠ Warning: Rust UAV API not detected on http://localhost:8080"
    echo "  The plugin will try to connect to the API when sync is enabled."
    echo "  Start the Rust server with: cargo run -- serve"
fi
echo ""

# Launch Gazebo
if [ "$HEADLESS" = true ]; then
    echo "Launching Gazebo in HEADLESS mode (no GUI)..."
    echo "  Mode: Server only (no visualization)"
else
    echo "Launching Gazebo with GUI..."
    echo "  Mode: Full visualization"
fi
echo "  World: $WORLD_FILE"
echo ""
echo "Plugin will listen on http://0.0.0.0:8092"
echo ""
echo "Available endpoints:"
echo "  GET  http://localhost:8092/health       - Health check"
echo "  POST http://localhost:8092/start        - Start sync to Rust"
echo "  POST http://localhost:8092/stop         - Stop sync"
echo "  GET  http://localhost:8092/drones/states - Get drone states"
echo "  POST http://localhost:8092/drones/{id}/command - Send command"
echo ""

if [ "$HEADLESS" = false ]; then
    echo "💡 Tip: For servers without display, use --headless flag"
    echo "   See GAZEBO_HEADLESS_SOLUTIONS.md for visualization options"
    echo ""
fi

echo "Press Ctrl+C to stop simulation"
echo "=========================================="
echo ""

# Launch with or without GUI
if [ "$HEADLESS" = true ]; then
    # Server mode: no GUI, lighter on resources
    ign gazebo -s "$WORLD_FILE" --verbose 2
else
    # Full mode: with GUI
    ign gazebo "$WORLD_FILE" --verbose 2
fi
