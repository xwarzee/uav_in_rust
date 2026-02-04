cmake_minimum_required(VERSION 3.10)
project(gazebo_rest_bridge)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(ignition-gazebo6 REQUIRED)
find_package(ignition-plugin1 REQUIRED)
find_package(ignition-common4 REQUIRED)
find_package(Threads REQUIRED)

# cpp-httplib (header-only HTTP server)
include(FetchContent)
FetchContent_Declare(
    httplib
    URL https://github.com/yhirose/cpp-httplib/archive/v0.14.0.tar.gz
)
FetchContent_MakeAvailable(httplib)

include_directories(
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${ignition-gazebo6_INCLUDE_DIRS}
)

add_library(RestBridgePlugin SHARED
  RestBridgePlugin.cc
  HttpServer.cc
)

target_link_libraries(RestBridgePlugin
  ignition-gazebo6::ignition-gazebo6
  ignition-plugin1::ignition-plugin1
  ignition-common4::ignition-common4
  httplib::httplib
  Threads::Threads
)

set_target_properties(RestBridgePlugin PROPERTIES
  LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)

install(TARGETS RestBridgePlugin
  LIBRARY DESTINATION lib
)
