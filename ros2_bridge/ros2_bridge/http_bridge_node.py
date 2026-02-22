#!/usr/bin/env python3
"""
ROS2 HTTP Bridge Node — UAV Swarm Controller.

Exposes the same REST contract as the C++ RestBridgePlugin so that the Rust
application can use either backend (mode = "gazebo" or "ros2") without change.

REST API contract:
  GET  /health               → {"status": "ok"}
  POST /start                → {"status": "ok"}   (enables control loop)
  POST /stop                 → {"status": "ok"}   (disables control loop)
  GET  /drones/states        → {drone_id: {position: {x,y,z}, velocity: {x,y,z}}}
  POST /drones/{id}/command  ← {"target_position": {x, y, z}}

ROS2 integration:
  - Reads drone positions from /model/{id}/pose (PosePublisher attached to each
    model → ros_gz_bridge → geometry_msgs/Pose). PosePublisher must be at model
    level in the SDF (world-level attachment is not supported in Fortress).
  - Moves drones by calling the Ignition Transport service
    /world/{world}/set_pose (UserCommands plugin), bridged to ROS2 as
    ros_gz_interfaces/srv/SetEntityPose.
"""

import json
import re
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from typing import Dict, Optional

import rclpy
from rclpy.callback_groups import ReentrantCallbackGroup
from rclpy.executors import MultiThreadedExecutor
from rclpy.node import Node
from geometry_msgs.msg import Pose
from ros_gz_interfaces.msg import Entity
from ros_gz_interfaces.srv import SetEntityPose


# ---------------------------------------------------------------------------
# Shared data structures
# ---------------------------------------------------------------------------

class _DroneState:
    """Thread-safe snapshot of one drone's state."""

    def __init__(self):
        # Position reported by Gazebo via PosePublisher → ros_gz_bridge (read-only).
        self.position: Dict[str, float] = {'x': 0.0, 'y': 0.0, 'z': 0.0}
        # Last position commanded to Gazebo via set_pose.
        # Used as the starting point for interpolation to avoid round-trip latency.
        self.commanded_position: Dict[str, float] = {'x': 0.0, 'y': 0.0, 'z': 0.0}
        self.velocity: Dict[str, float] = {'x': 0.0, 'y': 0.0, 'z': 0.0}
        self.pending_target: Optional[Dict[str, float]] = None


# ---------------------------------------------------------------------------
# ROS2 node
# ---------------------------------------------------------------------------

class HttpBridgeNode(Node):
    """
    ROS2 node that bridges HTTP REST ↔ Gazebo via ros_gz_bridge.

    Lifecycle:
      1. Subscribes to /world/{world}/pose/info → updates _states[drone].position
      2. HTTP POST /drones/{id}/command → sets _states[drone].pending_target
      3. _control_loop() timer (10 Hz) → calls set_pose service for pending targets
    """

    def __init__(self):
        super().__init__('ros2_http_bridge')

        # ---- parameters ----
        self.declare_parameter('http_port', 8082)
        self.declare_parameter('world_name', 'uav_swarm_world')
        self.declare_parameter('drone_ids', ['drone_1', 'drone_2', 'drone_3'])
        self.declare_parameter('max_speed', 5.0)   # m/s
        self.declare_parameter('arrival_threshold', 0.1)  # metres

        http_port: int = self.get_parameter('http_port').value
        self._world: str = self.get_parameter('world_name').value
        drone_ids: list = list(self.get_parameter('drone_ids').value)
        self._max_speed: float = self.get_parameter('max_speed').value
        self._threshold: float = self.get_parameter('arrival_threshold').value
        self._dt: float = 0.1  # must match timer period below

        # ---- shared state ----
        self._lock = threading.Lock()
        self._running = False
        self._states: Dict[str, _DroneState] = {
            did: _DroneState() for did in drone_ids
        }

        # ---- callback group (allows concurrent callbacks) ----
        cg = ReentrantCallbackGroup()

        # ---- ROS2 subscriptions: one per drone model ----
        # PosePublisher (model-level) publishes /model/{id}/pose as geometry_msgs/Pose.
        self._pose_subs = []
        for drone_id in drone_ids:
            sub = self.create_subscription(
                Pose,
                f'/model/{drone_id}/pose',
                lambda msg, did=drone_id: self._on_pose(did, msg),
                10,
                callback_group=cg,
            )
            self._pose_subs.append(sub)

        # ---- ROS2 service client: teleport a drone ----
        self._set_pose = self.create_client(
            SetEntityPose,
            f'/world/{self._world}/set_pose',
            callback_group=cg,
        )

        # ---- control loop timer: 10 Hz ----
        self.create_timer(0.1, self._control_loop, callback_group=cg)

        # ---- HTTP server in a daemon thread ----
        self._http_server = HTTPServer(('0.0.0.0', http_port), self._make_handler())
        threading.Thread(
            target=self._http_server.serve_forever, daemon=True
        ).start()

        self.get_logger().info(
            f'ROS2 HTTP bridge started — port={http_port}  '
            f'world="{self._world}"  drones={drone_ids}'
        )

    # ------------------------------------------------------------------
    # ROS2 callbacks
    # ------------------------------------------------------------------

    def _on_pose(self, drone_id: str, msg: Pose) -> None:
        """
        Update one drone's Gazebo-reported position (PosePublisher feedback).
        On first message, also seed commanded_position so the control loop
        starts from the correct initial position.
        """
        with self._lock:
            if drone_id not in self._states:
                return
            state = self._states[drone_id]
            p = msg.position
            new_pos = {'x': p.x, 'y': p.y, 'z': p.z}
            state.position = new_pos
            # Seed commanded_position on first feedback (all zeros → real pose)
            if (state.commanded_position['x'] == 0.0
                    and state.commanded_position['y'] == 0.0
                    and state.commanded_position['z'] == 0.0):
                state.commanded_position = dict(new_pos)

    def _control_loop(self) -> None:
        """
        Interpolated movement loop — 10 Hz.

        Each tick, each drone with a pending target advances by at most
        max_speed * dt metres toward it. When within arrival_threshold,
        the drone snaps to the target and the command is cleared.

        Uses commanded_position (last sent pose) as the starting point to
        avoid round-trip latency with the Gazebo → PosePublisher feedback.
        """
        if not self._running:
            return

        moves: Dict[str, Dict[str, float]] = {}

        with self._lock:
            for did, state in self._states.items():
                if state.pending_target is None:
                    continue

                cur = state.commanded_position
                tgt = state.pending_target

                dx = tgt['x'] - cur['x']
                dy = tgt['y'] - cur['y']
                dz = tgt['z'] - cur['z']
                dist = (dx ** 2 + dy ** 2 + dz ** 2) ** 0.5

                if dist < self._threshold:
                    # Arrived: snap and stop
                    new_pos = dict(tgt)
                    state.pending_target = None
                    state.velocity = {'x': 0.0, 'y': 0.0, 'z': 0.0}
                else:
                    # Advance one step toward target
                    step = min(self._max_speed * self._dt, dist)
                    scale = step / dist
                    new_pos = {
                        'x': cur['x'] + dx * scale,
                        'y': cur['y'] + dy * scale,
                        'z': cur['z'] + dz * scale,
                    }
                    state.velocity = {
                        'x': (dx / dist) * self._max_speed,
                        'y': (dy / dist) * self._max_speed,
                        'z': (dz / dist) * self._max_speed,
                    }

                state.commanded_position = new_pos
                moves[did] = new_pos

        for drone_id, new_pos in moves.items():
            self._teleport(drone_id, new_pos)

    def _teleport(self, drone_id: str, target: Dict[str, float]) -> None:
        """
        Call the /world/{world}/set_pose ROS2 service (fire-and-forget).
        Bridged from the Ignition Transport service by ros_gz_bridge.
        """
        if not self._set_pose.service_is_ready():
            self.get_logger().warn(
                f'set_pose service not ready — skipping {drone_id}'
            )
            return

        req = SetEntityPose.Request()
        req.entity.name = drone_id
        req.entity.type = Entity.MODEL   # 2 = MODEL in ros_gz_interfaces
        req.pose.position.x = float(target.get('x', 0.0))
        req.pose.position.y = float(target.get('y', 0.0))
        req.pose.position.z = float(target.get('z', 0.0))
        req.pose.orientation.w = 1.0    # identity rotation

        # Fire-and-forget: result is logged at debug level only
        future = self._set_pose.call_async(req)
        future.add_done_callback(
            lambda f: self.get_logger().debug(
                f'set_pose {drone_id} → {target}  ok={not f.exception()}'
            )
        )

    # ------------------------------------------------------------------
    # HTTP handler factory
    # ------------------------------------------------------------------

    def _make_handler(self):
        """Return a BaseHTTPRequestHandler class bound to this node."""
        node = self

        class _Handler(BaseHTTPRequestHandler):
            # Suppress the default "127.0.0.1 - - [date] GET /health" logs;
            # use the ROS2 logger instead.
            def log_message(self, fmt, *args):
                node.get_logger().debug(
                    f'HTTP {self.address_string()} {fmt % args}'
                )

            # ---- GET ----
            def do_GET(self):
                if self.path == '/health':
                    self._json({'status': 'ok'})

                elif self.path == '/drones/states':
                    with node._lock:
                        payload = {
                            did: {
                                'position': dict(s.position),
                                'velocity': dict(s.velocity),
                            }
                            for did, s in node._states.items()
                        }
                    self._json(payload)

                else:
                    self.send_response(404)
                    self.end_headers()

            # ---- POST ----
            def do_POST(self):
                if self.path == '/start':
                    with node._lock:
                        node._running = True
                    self._json({'status': 'ok'})

                elif self.path == '/stop':
                    with node._lock:
                        node._running = False
                    self._json({'status': 'ok'})

                else:
                    m = re.fullmatch(r'/drones/([^/]+)/command', self.path)
                    if m:
                        drone_id = m.group(1)
                        length = int(self.headers.get('Content-Length', 0))
                        body = json.loads(self.rfile.read(length))
                        target = body.get('target_position', {})
                        with node._lock:
                            if drone_id in node._states:
                                node._states[drone_id].pending_target = target
                                self._json({'status': 'ok'})
                            else:
                                self._json_error(404, f'Unknown drone: {drone_id}')
                    else:
                        self.send_response(404)
                        self.end_headers()

            # ---- helpers ----
            def _json(self, data: dict, code: int = 200) -> None:
                body = json.dumps(data).encode()
                self.send_response(code)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(body)))
                self.end_headers()
                self.wfile.write(body)

            def _json_error(self, code: int, message: str) -> None:
                self._json({'error': message}, code)

        return _Handler

    # ------------------------------------------------------------------
    # Cleanup
    # ------------------------------------------------------------------

    def destroy_node(self):
        self._http_server.shutdown()
        super().destroy_node()


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main():
    rclpy.init()
    node = HttpBridgeNode()
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
