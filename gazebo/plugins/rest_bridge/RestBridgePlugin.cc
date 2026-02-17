#include "RestBridgePlugin.hh"
#include "HttpServer.hh"

#include <ignition/gazebo/components/Pose.hh>
#include <ignition/gazebo/components/LinearVelocity.hh>
#include <ignition/gazebo/components/Link.hh>
#include <ignition/gazebo/components/Model.hh>
#include <ignition/gazebo/components/Name.hh>
#include <ignition/gazebo/Model.hh>
#include <ignition/plugin/Register.hh>
#include <ignition/common/Console.hh>

#include <httplib.h>
#include <sstream>
#include <iomanip>

using namespace gazebo_plugins;
using namespace ignition::gazebo;

RestBridgePlugin::RestBridgePlugin()
    : syncEnabled(false), httpPort(8092) {
}

RestBridgePlugin::~RestBridgePlugin() {
    if (httpServer) {
        httpServer->Stop();
    }
}

void RestBridgePlugin::Configure(const Entity &entity,
                                  const std::shared_ptr<const sdf::Element> &sdf,
                                  EntityComponentManager &ecm,
                                  EventManager &/*eventMgr*/) {
    // Read configuration from SDF
    if (sdf->HasElement("rust_api_url")) {
        rustApiUrl = sdf->Get<std::string>("rust_api_url");
    } else {
        rustApiUrl = "http://localhost:8080";
    }

    if (sdf->HasElement("http_port")) {
        httpPort = sdf->Get<int>("http_port");
    }

    // Parse drone names from SDF
    auto droneElem = sdf->FindElement("drone");
    while (droneElem) {
        std::string droneName = droneElem->Get<std::string>();
        droneNames.push_back(droneName);
        droneElem = droneElem->GetNextElement("drone");
    }

    ignmsg << "RestBridgePlugin configuration:" << std::endl;
    ignmsg << "  - Rust API URL: " << rustApiUrl << std::endl;
    ignmsg << "  - HTTP Port: " << httpPort << std::endl;
    ignmsg << "  - Drones: ";
    for (const auto &name : droneNames) {
        ignmsg << name << " ";
    }
    ignmsg << std::endl;

    // Find drone entities in the world
    for (const auto &droneName : droneNames) {
        ecm.Each<components::Model, components::Name>(
            [&](const Entity &entity,
                const components::Model *,
                const components::Name *name) -> bool {
                if (name->Data() == droneName) {
                    droneEntities[droneName] = entity;
                    ignmsg << "Found drone entity: " << droneName
                          << " (Entity: " << entity << ")" << std::endl;
                    return false;  // Stop searching once found
                }
                return true;  // Continue searching
            });
    }

    // Verify all drones were found (will retry in PreUpdate if not found)
    for (const auto &droneName : droneNames) {
        if (droneEntities.find(droneName) == droneEntities.end()) {
            ignmsg << "Note: Drone '" << droneName << "' not found yet, will search again during simulation." << std::endl;
        }
    }

    // Start HTTP server
    httpServer = std::make_unique<HttpServer>(httpPort, this);
    httpServer->Start();

    ignmsg << "RestBridgePlugin initialized successfully on port " << httpPort << std::endl;
}

void RestBridgePlugin::PreUpdate(const UpdateInfo &info,
                                  EntityComponentManager &ecm) {
    std::lock_guard<std::mutex> lock(commandMutex);

    // Lazy search for drones that weren't found during Configure
    if (droneEntities.size() < droneNames.size()) {
        for (const auto &droneName : droneNames) {
            if (droneEntities.find(droneName) == droneEntities.end()) {
                // Try to find this drone
                ecm.Each<components::Model, components::Name>(
                    [&](const Entity &entity,
                        const components::Model *,
                        const components::Name *name) -> bool {
                        if (name->Data() == droneName) {
                            droneEntities[droneName] = entity;
                            ignmsg << "Found drone entity (lazy): " << droneName
                                  << " (Entity: " << entity << ")" << std::endl;

                            // Initialize droneStates with default values immediately
                            {
                                std::lock_guard<std::mutex> stateLock(stateMutex);
                                ignition::math::Pose3d defaultPose;
                                ignition::math::Vector3d defaultVel;
                                droneStates[droneName] = std::make_pair(defaultPose, defaultVel);
                                ignmsg << "Initialized state for " << droneName << std::endl;
                            }

                            return false;  // Stop searching once found
                        }
                        return true;  // Continue searching
                    });
            }
        }
    }

    // Get time delta
    double dt = std::chrono::duration<double>(info.dt).count();
    if (dt <= 0) dt = 0.01;  // Default to 10ms if no time info

    static int preUpdateCount = 0;
    preUpdateCount++;
    bool shouldLog = (preUpdateCount % 100 == 0);

    if (shouldLog && !droneCommands.empty()) {
        ignmsg << "[PreUpdate] Applying " << droneCommands.size() << " commands" << std::endl;
    }

    // Apply commands to drones
    for (const auto &[droneId, targetPos] : droneCommands) {
        auto it = droneEntities.find(droneId);
        if (it == droneEntities.end()) {
            if (shouldLog) {
                ignwarn << "[PreUpdate] Drone " << droneId << " not found in droneEntities!" << std::endl;
            }
            continue;
        }

        Entity droneEntity = it->second;
        Model model(droneEntity);

        // Get current pose
        auto poseComp = ecm.Component<components::Pose>(droneEntity);
        if (!poseComp) {
            if (shouldLog) {
                ignwarn << "[PreUpdate] No Pose component for " << droneId << std::endl;
            }
            continue;
        }

        ignition::math::Pose3d currentPose = poseComp->Data();
        ignition::math::Vector3d currentPos = currentPose.Pos();

        // Calculate movement towards target (simple interpolation)
        ignition::math::Vector3d error = targetPos - currentPos;
        double distance = error.Length();

        if (shouldLog) {
            ignmsg << "[PreUpdate] " << droneId << ": pos=(" << currentPos.X() << ", " << currentPos.Y() << ", " << currentPos.Z()
                   << "), target=(" << targetPos.X() << ", " << targetPos.Y() << ", " << targetPos.Z()
                   << "), distance=" << distance << std::endl;
        }

        if (distance > 0.01) {  // Only move if not at target
            // Maximum speed: 5 m/s
            double maxSpeed = 5.0;
            double moveDistance = std::min(maxSpeed * dt, distance);

            ignition::math::Vector3d newPos = currentPos + error.Normalized() * moveDistance;

            if (shouldLog) {
                ignmsg << "[PreUpdate] Moving " << droneId << " by " << moveDistance << " meters" << std::endl;
            }

            // Create new pose with updated position (keep same rotation)
            ignition::math::Pose3d newPose(newPos, currentPose.Rot());

            // Set the new pose using ECM to ensure GUI updates
            ecm.SetComponentData<components::Pose>(droneEntity, newPose);
        }
    }
}

void RestBridgePlugin::PostUpdate(const UpdateInfo &/*info*/,
                                   const EntityComponentManager &ecm) {
    std::lock_guard<std::mutex> lock(stateMutex);

    static int postUpdateCount = 0;
    postUpdateCount++;
    bool shouldLogPost = (postUpdateCount % 100 == 0);

    if (shouldLogPost) {
        ignmsg << "[PostUpdate] Update #" << postUpdateCount << ", updating states for "
               << droneEntities.size() << " drones" << std::endl;
    }

    // Always update drone states for HTTP GET requests
    for (const auto &[droneId, droneEntity] : droneEntities) {
        auto poseComp = ecm.Component<components::Pose>(droneEntity);

        if (!poseComp) {
            if (shouldLogPost) {
                ignwarn << "[PostUpdate] Drone " << droneId << " has no Pose component, skipping" << std::endl;
            }
            continue;
        }

        ignition::math::Pose3d pose = poseComp->Data();

        // LinearVelocity is optional - use zero velocity if not available
        ignition::math::Vector3d velocity(0, 0, 0);
        auto velComp = ecm.Component<components::LinearVelocity>(droneEntity);
        if (velComp) {
            velocity = velComp->Data();
        } else if (shouldLogPost && postUpdateCount == 100) {
            // Log once that velocity component is missing
            ignmsg << "[PostUpdate] Note: Drone " << droneId << " has no LinearVelocity component, using zero velocity" << std::endl;
        }

        // Store states for HTTP GET /drones/states endpoint
        droneStates[droneId] = std::make_pair(pose, velocity);

        if (shouldLogPost) {
            ignmsg << "[PostUpdate] Updated " << droneId << ": pos=("
                   << pose.Pos().X() << ", " << pose.Pos().Y() << ", " << pose.Pos().Z()
                   << "), vel=(" << velocity.X() << ", " << velocity.Y() << ", " << velocity.Z()
                   << ")" << std::endl;
        }

        // Optionally send to Rust API if sync is enabled (push model)
        if (syncEnabled) {
            SendDroneStateToRust(droneId, pose, velocity);
        }
    }
}

void RestBridgePlugin::SetDroneCommand(const std::string &drone_id,
                                       const ignition::math::Vector3d &target_position) {
    std::lock_guard<std::mutex> lock(commandMutex);
    droneCommands[drone_id] = target_position;
    ignmsg << "Received command for " << drone_id << ": ("
          << target_position.X() << ", "
          << target_position.Y() << ", "
          << target_position.Z() << ")" << std::endl;
}

std::map<std::string, std::pair<ignition::math::Pose3d, ignition::math::Vector3d>>
RestBridgePlugin::GetDroneStates() const {
    std::lock_guard<std::mutex> lock(stateMutex);
    return droneStates;
}

void RestBridgePlugin::SendDroneStateToRust(const std::string &drone_id,
                                            const ignition::math::Pose3d &pose,
                                            const ignition::math::Vector3d &velocity) {
    // Build JSON payload
    std::ostringstream json;
    json << std::fixed << std::setprecision(6);
    json << "{"
         << "\"position\": {"
         << "\"x\": " << pose.Pos().X() << ","
         << "\"y\": " << pose.Pos().Y() << ","
         << "\"z\": " << pose.Pos().Z()
         << "},"
         << "\"velocity\": {"
         << "\"vx\": " << velocity.X() << ","
         << "\"vy\": " << velocity.Y() << ","
         << "\"vz\": " << velocity.Z()
         << "}"
         << "}";

    // Send HTTP PUT request to Rust API (async)
    std::string url = rustApiUrl + "/api/drones/" + drone_id + "/state";
    std::string body = json.str();

    // Use thread pool from HttpServer to avoid blocking
    httpServer->SendToRust(url, body);
}

// Register the plugin
IGNITION_ADD_PLUGIN(
    RestBridgePlugin,
    ignition::gazebo::System,
    RestBridgePlugin::ISystemConfigure,
    RestBridgePlugin::ISystemPreUpdate,
    RestBridgePlugin::ISystemPostUpdate)
