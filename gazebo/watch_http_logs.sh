#!/bin/bash
#
# Watch HTTP logs from Gazebo RestBridge Plugin
#
# This script filters and displays only HTTP-related messages
# from the Gazebo simulation output
#

echo "=== Watching Gazebo HTTP Logs ==="
echo "Filtering for [HTTP] messages..."
echo "Press Ctrl+C to stop"
echo ""

# Check if Gazebo is running
if ! pgrep -f "ign gazebo" > /dev/null; then
    echo "⚠ Warning: Gazebo doesn't appear to be running"
    echo ""
    echo "Start Gazebo with:"
    echo "  ./gazebo/launch/start_simulation.sh --headless"
    echo ""
    exit 1
fi

# Watch system log for Gazebo messages (works on most systems)
if command -v journalctl &> /dev/null; then
    # SystemD systems
    echo "Using journalctl (SystemD)..."
    journalctl -f | grep --line-buffered "\[HTTP\]"
elif [ -f /var/log/syslog ]; then
    # Debian/Ubuntu systems without systemd
    echo "Using /var/log/syslog..."
    tail -f /var/log/syslog | grep --line-buffered "\[HTTP\]"
else
    echo "Could not find system log."
    echo ""
    echo "Alternative: Redirect output when starting Gazebo:"
    echo "  ./gazebo/launch/start_simulation.sh --headless 2>&1 | tee gazebo.log"
    echo ""
    echo "Then in another terminal:"
    echo "  tail -f gazebo.log | grep '\[HTTP\]'"
fi
