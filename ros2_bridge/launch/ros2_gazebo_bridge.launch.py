"""
Launch file: ROS2 + Gazebo + HTTP bridge for UAV Swarm (ros2 mode).

Starts three components:
  1. gz_sim        — Ignition Gazebo with the ROS2-compatible world
  2. ros_gz_bridge — bridges:
       /world/uav_swarm_world/pose/info  (GZ → ROS2, TFMessage)
       /world/uav_swarm_world/set_pose   (ROS2 → GZ service)
  3. http_bridge_node — HTTP REST server consumed by the Rust application

Usage:
  ros2 launch ros2_bridge ros2_gazebo_bridge.launch.py
  ros2 launch ros2_bridge ros2_gazebo_bridge.launch.py headless:=true
  ros2 launch ros2_bridge ros2_gazebo_bridge.launch.py world:=/path/to/custom.sdf
"""

import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.conditions import IfCondition, UnlessCondition
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare


def generate_launch_description():
    pkg_ros2_bridge = get_package_share_directory('ros2_bridge')

    # Default world path: packaged alongside this launch file
    default_world = os.path.join(
        # Navigate from share/ros2_bridge up to the project root, then into gazebo/worlds
        pkg_ros2_bridge, '..', '..', '..', '..',
        'gazebo', 'worlds', 'uav_swarm_ros2.sdf'
    )

    # ------------------------------------------------------------------ args
    world_arg = DeclareLaunchArgument(
        'world',
        default_value=default_world,
        description='Absolute path to the Gazebo SDF world file.',
    )

    headless_arg = DeclareLaunchArgument(
        'headless',
        default_value='false',
        description='Run Gazebo without GUI (server-only mode).',
    )

    http_port_arg = DeclareLaunchArgument(
        'http_port',
        default_value='8082',
        description='Port for the HTTP REST bridge (must match ros2.bridge_url).',
    )

    # ----------------------------------------------------------- gz_sim node
    # GUI mode
    gz_sim_gui = IncludeLaunchDescription(
        PythonLaunchDescriptionSource([
            FindPackageShare('ros_gz_sim'), '/launch/gz_sim.launch.py'
        ]),
        launch_arguments={
            'gz_args': ['-r ', LaunchConfiguration('world')],
        }.items(),
        condition=UnlessCondition(LaunchConfiguration('headless')),
    )

    # Headless mode (no rendering — faster for CI / remote servers)
    gz_sim_headless = IncludeLaunchDescription(
        PythonLaunchDescriptionSource([
            FindPackageShare('ros_gz_sim'), '/launch/gz_sim.launch.py'
        ]),
        launch_arguments={
            'gz_args': ['-r -s ', LaunchConfiguration('world')],
        }.items(),
        condition=IfCondition(LaunchConfiguration('headless')),
    )

    # ------------------------------------------------------- ros_gz_bridge
    #
    # Bridge syntax:
    #   topic@ros_type[gz_type   — GZ → ROS2 (unidirectional)
    #   topic@ros_type]gz_type   — ROS2 → GZ (unidirectional)
    #   topic@ros_type@gz_type   — bidirectional
    #   service@ros_srv_type     — ROS2 service ↔ Ignition Transport service
    #
    ros_gz_bridge = Node(
        package='ros_gz_bridge',
        executable='parameter_bridge',
        name='ros_gz_bridge',
        arguments=[
            # Per-model pose topics published by PosePublisher (model-level plugin).
            # PosePublisher must be attached to each model, not at world level.
            '/model/drone_1/pose@geometry_msgs/msg/Pose[ignition.msgs.Pose',
            '/model/drone_2/pose@geometry_msgs/msg/Pose[ignition.msgs.Pose',
            '/model/drone_3/pose@geometry_msgs/msg/Pose[ignition.msgs.Pose',

            # set_pose service: ROS2 client → Ignition Transport service
            # Provided by the UserCommands system plugin in the world.
            '/world/uav_swarm_world/set_pose'
            '@ros_gz_interfaces/srv/SetEntityPose',
        ],
        output='screen',
        remappings=[],
    )

    # --------------------------------------------------- HTTP bridge node
    http_bridge = Node(
        package='ros2_bridge',
        executable='http_bridge_node',
        name='ros2_http_bridge',
        parameters=[
            os.path.join(pkg_ros2_bridge, 'config', 'bridge_params.yaml'),
            # Override http_port from launch argument if provided
            {'http_port': LaunchConfiguration('http_port')},
        ],
        output='screen',
    )

    return LaunchDescription([
        world_arg,
        headless_arg,
        http_port_arg,
        gz_sim_gui,
        gz_sim_headless,
        ros_gz_bridge,
        http_bridge,
    ])
