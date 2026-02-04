#ifndef GAZEBO_REST_BRIDGE_PLUGIN_HH
#define GAZEBO_REST_BRIDGE_PLUGIN_HH

#include <ignition/gazebo/System.hh>
#include <ignition/gazebo/Model.hh>
#include <ignition/math/Vector3.hh>
#include <ignition/math/Pose3.hh>
#include <memory>
#include <map>
#include <string>
#include <mutex>

namespace gazebo_plugins {

// Forward declaration
class HttpServer;

class RestBridgePlugin : public ignition::gazebo::System,
                         public ignition::gazebo::ISystemConfigure,
                         public ignition::gazebo::ISystemPreUpdate,
                         public ignition::gazebo::ISystemPostUpdate {
public:
    RestBridgePlugin();
    ~RestBridgePlugin() override;

    // Configure the plugin
    void Configure(const ignition::gazebo::Entity &entity,
                   const std::shared_ptr<const sdf::Element> &sdf,
                   ignition::gazebo::EntityComponentManager &ecm,
                   ignition::gazebo::EventManager &eventMgr) override;

    // Pre-update: Apply commands to drones
    void PreUpdate(const ignition::gazebo::UpdateInfo &info,
                   ignition::gazebo::EntityComponentManager &ecm) override;

    // Post-update: Read drone states and sync to Rust API
    void PostUpdate(const ignition::gazebo::UpdateInfo &info,
                    const ignition::gazebo::EntityComponentManager &ecm) override;

    // Called by HTTP server when command is received
    void SetDroneCommand(const std::string &drone_id,
                        const ignition::math::Vector3d &target_position);

    // Get current drone states (called by HTTP server)
    std::map<std::string, std::pair<ignition::math::Pose3d, ignition::math::Vector3d>>
    GetDroneStates() const;

    // Enable/disable sync to Rust API
    void SetSyncEnabled(bool enabled) { syncEnabled = enabled; }
    bool IsSyncEnabled() const { return syncEnabled; }

    // Get drone names
    std::vector<std::string> GetDroneNames() const { return droneNames; }

private:
    // HTTP server for receiving commands from Rust
    std::unique_ptr<HttpServer> httpServer;

    // Configuration
    std::string rustApiUrl;
    int httpPort;
    std::vector<std::string> droneNames;

    // State
    bool syncEnabled;
    std::map<std::string, ignition::gazebo::Entity> droneEntities;
    std::map<std::string, ignition::math::Vector3d> droneCommands;

    // Thread safety
    mutable std::mutex commandMutex;
    mutable std::mutex stateMutex;

    // Helper to send HTTP request to Rust API
    void SendDroneStateToRust(const std::string &drone_id,
                              const ignition::math::Pose3d &pose,
                              const ignition::math::Vector3d &velocity);
};

}  // namespace gazebo_plugins

#endif  // GAZEBO_REST_BRIDGE_PLUGIN_HH
