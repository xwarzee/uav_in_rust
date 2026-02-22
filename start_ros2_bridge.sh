#!/bin/bash
# ============================================================
# ROS2 Bridge launcher — UAV Swarm Controller
# ============================================================
#
# Starts the full ROS2 stack for the "ros2" simulation mode:
#   1. Ignition Gazebo  (uav_swarm_ros2.sdf)
#   2. ros_gz_bridge    (pose topics + set_pose service)
#   3. http_bridge_node (HTTP REST :8082 consumed by Rust app)
#
# Usage:
#   ./start_ros2_bridge.sh                   # GUI, default port 8082
#   ./start_ros2_bridge.sh --headless        # no GUI (server mode)
#   ./start_ros2_bridge.sh --bridge-only     # skip Gazebo (already running)
#   ./start_ros2_bridge.sh --port 8083       # custom HTTP port
#   ./start_ros2_bridge.sh --headless --port 8083
#
# After this script is running, start the Rust app with:
#   cargo run -- --mode ros2 serve
# ============================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ---- Default options ----
HEADLESS=false
BRIDGE_ONLY=false
HTTP_PORT=8082
ROS2_DISTRO="${ROS_DISTRO:-humble}"

# ---- Parse arguments ----
while [[ $# -gt 0 ]]; do
    case "$1" in
        --headless)    HEADLESS=true       ;;
        --bridge-only) BRIDGE_ONLY=true    ;;
        --port)        HTTP_PORT="$2"; shift ;;
        --port=*)      HTTP_PORT="${1#*=}" ;;
        -h|--help)
            sed -n '2,20p' "$0" | sed 's/^# \?//'
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Run '$0 --help' for usage."
            exit 1
            ;;
    esac
    shift
done

echo "=================================================="
echo " ROS2 Bridge — UAV Swarm Controller"
echo "=================================================="
echo ""

# ---- Check ROS2 ----
ROS2_SETUP="/opt/ros/${ROS2_DISTRO}/setup.bash"
if [ ! -f "$ROS2_SETUP" ]; then
    echo "Error: ROS2 ${ROS2_DISTRO} not found at ${ROS2_SETUP}"
    echo ""
    echo "Install ROS2 Humble:"
    echo "  sudo apt install ros-humble-ros-base"
    echo "  sudo apt install ros-humble-ros-gz-bridge ros-humble-ros-gz-sim \\"
    echo "                   ros-humble-ros-gz-interfaces ros-humble-tf2-msgs"
    exit 1
fi

# shellcheck source=/dev/null
source "$ROS2_SETUP"
echo "✓ ROS2 ${ROS2_DISTRO} sourced"

# ---- Source colcon workspace ----
WORKSPACE_SETUP="${SCRIPT_DIR}/install/setup.bash"
if [ ! -f "$WORKSPACE_SETUP" ]; then
    echo ""
    echo "ros2_bridge package not built yet. Building now..."
    echo ""
    cd "$SCRIPT_DIR"
    if ! command -v colcon &>/dev/null; then
        echo "Error: colcon not found."
        echo "  sudo apt install python3-colcon-common-extensions"
        exit 1
    fi
    colcon build --packages-select ros2_bridge
    echo ""
    echo "✓ ros2_bridge built"
fi

# shellcheck source=/dev/null
source "$WORKSPACE_SETUP"
echo "✓ workspace sourced (install/setup.bash)"

# ---- Verify ros2_bridge package is available ----
if ! ros2 pkg list 2>/dev/null | grep -q "^ros2_bridge$"; then
    echo ""
    echo "Error: ros2_bridge package not found after sourcing workspace."
    echo "Try rebuilding:"
    echo "  colcon build --packages-select ros2_bridge"
    echo "  source install/setup.bash"
    exit 1
fi
echo "✓ ros2_bridge package found"

# ---- Verify ros_gz_bridge is installed ----
if ! ros2 pkg list 2>/dev/null | grep -q "^ros_gz_bridge$"; then
    echo ""
    echo "Error: ros_gz_bridge not installed."
    echo "  sudo apt install ros-${ROS2_DISTRO}-ros-gz-bridge \\"
    echo "                   ros-${ROS2_DISTRO}-ros-gz-sim \\"
    echo "                   ros-${ROS2_DISTRO}-ros-gz-interfaces \\"
    echo "                   ros-${ROS2_DISTRO}-tf2-msgs"
    exit 1
fi
echo "✓ ros_gz_bridge found"
echo ""

# ---- Summary ----
echo "Configuration:"
echo "  HTTP port  : ${HTTP_PORT}"
echo "  Headless   : ${HEADLESS}"
echo "  Bridge only: ${BRIDGE_ONLY}"
echo ""

if [ "$BRIDGE_ONLY" = false ]; then
    echo "Starting: Gazebo + ros_gz_bridge + http_bridge_node"
else
    echo "Starting: ros_gz_bridge + http_bridge_node (Gazebo assumed running)"
fi
echo ""
echo "HTTP REST endpoints (consumed by Rust app):"
echo "  GET  http://localhost:${HTTP_PORT}/health"
echo "  GET  http://localhost:${HTTP_PORT}/drones/states"
echo "  POST http://localhost:${HTTP_PORT}/drones/{id}/command"
echo ""
echo "Press Ctrl+C to stop"
echo "=================================================="
echo ""

# ---- Launch ----
if [ "$BRIDGE_ONLY" = true ]; then
    # Start only ros_gz_bridge + http_bridge_node (no Gazebo)
    # Useful when Gazebo is already running via start_simulation.sh --ros2
    echo "[bridge-only mode] Starting ros_gz_bridge..."
    ros2 run ros_gz_bridge parameter_bridge \
        '/model/drone_1/pose@geometry_msgs/msg/Pose[ignition.msgs.Pose' \
        '/model/drone_2/pose@geometry_msgs/msg/Pose[ignition.msgs.Pose' \
        '/model/drone_3/pose@geometry_msgs/msg/Pose[ignition.msgs.Pose' \
        '/world/uav_swarm_world/set_pose@ros_gz_interfaces/srv/SetEntityPose' &
    ROS_GZ_PID=$!

    echo "[bridge-only mode] Starting http_bridge_node on port ${HTTP_PORT}..."
    ros2 run ros2_bridge http_bridge_node \
        --ros-args -p http_port:="${HTTP_PORT}" &
    HTTP_PID=$!

    # Wait for both and forward Ctrl+C
    trap "kill $ROS_GZ_PID $HTTP_PID 2>/dev/null; exit 0" INT TERM
    wait $ROS_GZ_PID $HTTP_PID
else
    # Full stack: Gazebo + ros_gz_bridge + http_bridge_node via launch file
    ros2 launch ros2_bridge ros2_gazebo_bridge.launch.py \
        headless:="${HEADLESS}" \
        http_port:="${HTTP_PORT}"
fi
