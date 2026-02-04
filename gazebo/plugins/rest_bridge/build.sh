#!/bin/bash
# Build script for RestBridgePlugin
# Compiles the C++ plugin for Gazebo-Rust communication

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=========================================="
echo "Building RestBridgePlugin"
echo "=========================================="
echo ""

# Check for required tools
if ! command -v cmake &> /dev/null; then
    echo "Error: cmake not found. Please install cmake."
    exit 1
fi

if ! command -v make &> /dev/null; then
    echo "Error: make not found. Please install build tools."
    exit 1
fi

# Check for Ignition Gazebo
if ! command -v ign &> /dev/null; then
    echo "Error: Ignition Gazebo not found."
    echo ""
    echo "Install instructions:"
    echo "  macOS:  brew install ignition-fortress"
    echo "  Ubuntu: sudo apt-get install ignition-fortress"
    echo ""
    exit 1
fi

echo "Build directory: $SCRIPT_DIR"
echo ""

# Create build directory
cd "$SCRIPT_DIR"
if [ ! -d "build" ]; then
    mkdir build
    echo "Created build/ directory"
fi

cd build

# Run CMake
echo "Running CMake..."
cmake .. || {
    echo ""
    echo "CMake failed. Make sure you have installed:"
    echo "  - ignition-gazebo7-dev (or ignition-fortress)"
    echo "  - ignition-transport12-dev"
    echo "  - ignition-math7-dev"
    echo ""
    echo "On Ubuntu/Debian:"
    echo "  sudo apt-get install libignition-gazebo7-dev libignition-transport12-dev libignition-math7-dev"
    echo ""
    echo "On macOS:"
    echo "  brew install ignition-fortress"
    echo ""
    exit 1
}

echo ""

# Compile
echo "Compiling plugin..."
make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 2) || {
    echo ""
    echo "Compilation failed. Check error messages above."
    exit 1
}

echo ""
echo "=========================================="
echo "Build completed successfully!"
echo "=========================================="
echo ""
echo "Plugin location:"
echo "  $SCRIPT_DIR/build/libRestBridgePlugin.so"
echo ""
echo "To use the plugin, set environment variable:"
echo "  export IGN_GAZEBO_SYSTEM_PLUGIN_PATH=\$IGN_GAZEBO_SYSTEM_PLUGIN_PATH:$SCRIPT_DIR/build"
echo ""
echo "Or use the launch script which sets it automatically:"
echo "  $SCRIPT_DIR/../../launch/start_simulation.sh"
echo ""
