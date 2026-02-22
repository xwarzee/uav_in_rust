from setuptools import find_packages, setup
import os
from glob import glob

package_name = 'ros2_bridge'

setup(
    name=package_name,
    version='0.1.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        (os.path.join('share', package_name, 'launch'),
            glob('launch/*.launch.py')),
        (os.path.join('share', package_name, 'config'),
            glob('config/*.yaml')),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='UAV Swarm Team',
    maintainer_email='uav@example.com',
    description='HTTP REST bridge between UAV Swarm Rust app and Gazebo via ROS2',
    license='MIT',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'http_bridge_node = ros2_bridge.http_bridge_node:main',
        ],
    },
)
