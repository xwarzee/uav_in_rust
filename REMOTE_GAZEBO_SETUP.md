# Configuration Serveur Distant Gazebo

Ce guide explique comment configurer un serveur Linux distant pour exécuter les simulations Gazebo, accessible depuis votre machine de développement macOS.

## Architecture

```
┌─────────────────────────────────┐         ┌──────────────────────────────────┐
│   macOS - Développement         │         │   Serveur Linux Distant          │
│                                 │         │   (Debian/Ubuntu)                │
│  Rust UAV Application :8080     │         │                                  │
│    ├─ Mode Internal (local)     │         │  Ignition Gazebo Fortress        │
│    └─ Mode Gazebo (remote)      │         │    └─ RestBridgePlugin           │
│         │                       │         │         └─ HTTP Server :8092     │
│         │                       │         │                                  │
│         └─────── HTTP ──────────┼─────────┤                                  │
│          (via Internet/VPN)     │         │  Port 8092 exposé                │
└─────────────────────────────────┘         └──────────────────────────────────┘
```

## Prérequis Serveur

- **OS**: Debian 11/12 ou Ubuntu 20.04/22.04
- **RAM**: Minimum 4 GB (recommandé 8 GB)
- **CPU**: 2+ cœurs
- **Réseau**: IP publique ou VPN, port 8092 accessible
- **Accès**: SSH avec droits sudo

## ⚠️ Serveur Sans Écran (Headless) ?

**Si votre serveur n'a pas d'écran physique**, vous avez plusieurs options pour visualiser la simulation 3D :

1. **Mode Headless** (recommandé pour production) - Pas de GUI, visualisation via API/WebSocket
2. **X11 Forwarding** - Affichage via SSH (simple mais lent)
3. **VNC Server** - Bureau virtuel (recommandé pour développement)
4. **noVNC** - Accès via navigateur web

**📖 Voir le guide complet:** [GAZEBO_HEADLESS_SOLUTIONS.md](./GAZEBO_HEADLESS_SOLUTIONS.md)

**Pour ce guide, nous utiliserons le mode headless** (option `-s` pour server-only). Si vous voulez visualiser Gazebo, consultez le guide des solutions headless après l'installation.

---

## Partie 1: Installation sur le Serveur Distant

### Étape 1: Connexion SSH

```bash
# Depuis votre Mac
ssh user@SERVEUR_IP
```

### Étape 2: Installation d'Ignition Gazebo Fortress

```bash
# Mettre à jour le système
sudo apt-get update && sudo apt-get upgrade -y

# Installer les dépendances
sudo apt-get install -y wget lsb-release gnupg curl

# Ajouter le repository OSRF
sudo wget https://packages.osrfoundation.org/gazebo.gpg -O /usr/share/keyrings/pkgs-osrf-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/pkgs-osrf-archive-keyring.gpg] http://packages.osrfoundation.org/gazebo/ubuntu-stable $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/gazebo-stable.list

# Installer Ignition Gazebo Fortress
sudo apt-get update
sudo apt-get install -y ignition-fortress

# Vérifier l'installation
ign gazebo --version
```

**Résultat attendu:** `Ignition Gazebo, version 6.x.x`

### Étape 3: Installation des Outils de Compilation

```bash
# Installer CMake et compilateurs
sudo apt-get install -y \
    cmake \
    g++ \
    make \
    libignition-gazebo7-dev \
    libignition-transport12-dev \
    libignition-math7-dev \
    git

# Vérifier CMake
cmake --version  # Devrait afficher 3.10+
```

### Étape 4: Créer Répertoire de Travail

```bash
# Créer structure pour le projet
mkdir -p ~/uav_gazebo_server
cd ~/uav_gazebo_server

# Créer sous-répertoires
mkdir -p plugins/rest_bridge
mkdir -p worlds
mkdir -p models/x3_uav
```

---

## Partie 2: Création du Plugin C++ REST Bridge

### Fichier 1: `plugins/rest_bridge/CMakeLists.txt`

```cmake
cmake_minimum_required(VERSION 3.10)
project(gazebo_rest_bridge)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Ignition packages
find_package(ignition-cmake2 REQUIRED)
find_package(ignition-gazebo7 REQUIRED)
find_package(ignition-transport12 REQUIRED)
find_package(Threads REQUIRED)

# cpp-httplib (header-only HTTP server library)
include(FetchContent)
FetchContent_Declare(
    httplib
    URL https://github.com/yhirose/cpp-httplib/archive/v0.14.3.tar.gz
)
FetchContent_MakeAvailable(httplib)

# Create plugin library
add_library(RestBridgePlugin SHARED
    RestBridgePlugin.cc
)

target_link_libraries(RestBridgePlugin
    PRIVATE
    ignition-gazebo7::core
    ignition-transport12::core
    httplib::httplib
    Threads::Threads
)

# Install plugin
install(TARGETS RestBridgePlugin
    LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/lib
)
```

### Fichier 2: `plugins/rest_bridge/RestBridgePlugin.cc`

```cpp
#include <ignition/gazebo/System.hh>
#include <ignition/gazebo/Model.hh>
#include <ignition/gazebo/components/Pose.hh>
#include <ignition/gazebo/components/LinearVelocity.hh>
#include <ignition/gazebo/components/Name.hh>
#include <ignition/plugin/Register.hh>
#include <ignition/math/Pose3.hh>
#include <ignition/math/Vector3.hh>

#define CPPHTTPLIB_OPENSSL_SUPPORT
#include "httplib.h"

#include <thread>
#include <memory>
#include <map>
#include <mutex>

namespace gazebo_plugins
{
  class RestBridgePlugin :
    public ignition::gazebo::System,
    public ignition::gazebo::ISystemConfigure,
    public ignition::gazebo::ISystemPostUpdate
  {
    public:
      RestBridgePlugin() : syncEnabled(false), serverRunning(false) {}

      ~RestBridgePlugin() override {
        serverRunning = false;
        if (serverThread.joinable()) {
          serverThread.join();
        }
      }

      void Configure(
        const ignition::gazebo::Entity &_entity,
        const std::shared_ptr<const sdf::Element> &_sdf,
        ignition::gazebo::EntityComponentManager &_ecm,
        ignition::gazebo::EventManager &/*_eventMgr*/) override
      {
        // Configuration
        rustApiUrl = _sdf->Get<std::string>("rust_api_url", "http://localhost:8080");
        httpPort = _sdf->Get<int>("http_port", 8092);

        // Parse drone names
        if (_sdf->HasElement("drone")) {
          auto droneElem = _sdf->GetElement("drone");
          while (droneElem) {
            std::string droneName = droneElem->Get<std::string>();
            droneNames.push_back(droneName);
            droneElem = droneElem->GetNextElement("drone");
          }
        }

        ignmsg << "RestBridgePlugin configured with " << droneNames.size() << " drones" << std::endl;
        ignmsg << "Rust API URL: " << rustApiUrl << std::endl;
        ignmsg << "HTTP Server port: " << httpPort << std::endl;

        // Start HTTP server in separate thread
        serverRunning = true;
        serverThread = std::thread(&RestBridgePlugin::RunHttpServer, this);
      }

      void PostUpdate(
        const ignition::gazebo::UpdateInfo &_info,
        const ignition::gazebo::EntityComponentManager &_ecm) override
      {
        if (!syncEnabled) return;

        std::lock_guard<std::mutex> lock(dataMutex);

        // Update drone states cache
        for (const auto &droneName : droneNames) {
          // Find drone entity
          ignition::gazebo::Entity droneEntity = ignition::gazebo::kNullEntity;
          _ecm.Each<ignition::gazebo::components::Name,
                    ignition::gazebo::components::Pose>(
            [&](const ignition::gazebo::Entity &_entity,
                const ignition::gazebo::components::Name *_name,
                const ignition::gazebo::components::Pose *) -> bool
            {
              if (_name->Data() == droneName) {
                droneEntity = _entity;
                return false;
              }
              return true;
            });

          if (droneEntity == ignition::gazebo::kNullEntity) continue;

          auto poseComp = _ecm.Component<ignition::gazebo::components::Pose>(droneEntity);
          auto velComp = _ecm.Component<ignition::gazebo::components::LinearVelocity>(droneEntity);

          if (poseComp && velComp) {
            auto pose = poseComp->Data();
            auto vel = velComp->Data();

            DroneState state;
            state.position_x = pose.Pos().X();
            state.position_y = pose.Pos().Y();
            state.position_z = pose.Pos().Z();
            state.velocity_x = vel.X();
            state.velocity_y = vel.Y();
            state.velocity_z = vel.Z();

            droneStates[droneName] = state;
          }
        }
      }

    private:
      struct DroneState {
        double position_x, position_y, position_z;
        double velocity_x, velocity_y, velocity_z;
      };

      void RunHttpServer() {
        httplib::Server server;

        // Health check endpoint
        server.Get("/health", [this](const httplib::Request &, httplib::Response &res) {
          std::lock_guard<std::mutex> lock(dataMutex);
          std::string dronesJson = "[";
          for (size_t i = 0; i < droneNames.size(); ++i) {
            dronesJson += "\"" + droneNames[i] + "\"";
            if (i < droneNames.size() - 1) dronesJson += ",";
          }
          dronesJson += "]";

          res.set_content(
            "{\"status\":\"ok\",\"drones\":" + dronesJson + ",\"sync_enabled\":" +
            (syncEnabled ? "true" : "false") + "}",
            "application/json"
          );
        });

        // Start sync endpoint
        server.Post("/start", [this](const httplib::Request &, httplib::Response &res) {
          syncEnabled = true;
          ignmsg << "Sync enabled" << std::endl;
          res.set_content("{\"message\":\"Sync started\"}", "application/json");
        });

        // Stop sync endpoint
        server.Post("/stop", [this](const httplib::Request &, httplib::Response &res) {
          syncEnabled = false;
          ignmsg << "Sync disabled" << std::endl;
          res.set_content("{\"message\":\"Sync stopped\"}", "application/json");
        });

        // Get all drone states
        server.Get("/drones/states", [this](const httplib::Request &, httplib::Response &res) {
          std::lock_guard<std::mutex> lock(dataMutex);
          std::string json = "{";
          bool first = true;
          for (const auto &[name, state] : droneStates) {
            if (!first) json += ",";
            first = false;
            json += "\"" + name + "\":{";
            json += "\"position\":{";
            json += "\"x\":" + std::to_string(state.position_x) + ",";
            json += "\"y\":" + std::to_string(state.position_y) + ",";
            json += "\"z\":" + std::to_string(state.position_z);
            json += "},\"velocity\":{";
            json += "\"vx\":" + std::to_string(state.velocity_x) + ",";
            json += "\"vy\":" + std::to_string(state.velocity_y) + ",";
            json += "\"vz\":" + std::to_string(state.velocity_z);
            json += "}}";
          }
          json += "}";
          res.set_content(json, "application/json");
        });

        // Command endpoint (placeholder)
        server.Post("/drones/(.*)/command", [](const httplib::Request &req, httplib::Response &res) {
          res.set_content("{\"message\":\"Command received\"}", "application/json");
        });

        ignmsg << "Starting HTTP server on 0.0.0.0:" << httpPort << std::endl;
        server.listen("0.0.0.0", httpPort);
      }

      std::string rustApiUrl;
      int httpPort;
      std::vector<std::string> droneNames;
      std::map<std::string, DroneState> droneStates;
      bool syncEnabled;
      bool serverRunning;
      std::thread serverThread;
      std::mutex dataMutex;
  };
}

IGNITION_ADD_PLUGIN(
  gazebo_plugins::RestBridgePlugin,
  ignition::gazebo::System,
  gazebo_plugins::RestBridgePlugin::ISystemConfigure,
  gazebo_plugins::RestBridgePlugin::ISystemPostUpdate)
```

### Compilation du Plugin

```bash
cd ~/uav_gazebo_server/plugins/rest_bridge

# Créer répertoire de build
mkdir -p build && cd build

# Configurer avec CMake
cmake ..

# Compiler
make -j$(nproc)

# Vérifier que le plugin est créé
ls -lh libRestBridgePlugin.so
```

**Résultat attendu:** Fichier `libRestBridgePlugin.so` créé

---

## Partie 3: Création du Monde Gazebo

### Fichier: `~/uav_gazebo_server/worlds/uav_swarm.sdf`

```xml
<?xml version="1.0"?>
<sdf version="1.8">
  <world name="uav_swarm_world">

    <!-- Physics -->
    <physics name="1ms" type="ode">
      <max_step_size>0.001</max_step_size>
      <real_time_factor>1.0</real_time_factor>
    </physics>

    <!-- System Plugins -->
    <plugin filename="ignition-gazebo-physics-system"
            name="ignition::gazebo::systems::Physics">
    </plugin>

    <plugin filename="ignition-gazebo-scene-broadcaster-system"
            name="ignition::gazebo::systems::SceneBroadcaster">
    </plugin>

    <!-- REST Bridge Plugin -->
    <plugin filename="libRestBridgePlugin.so"
            name="gazebo_plugins::RestBridgePlugin">
      <rust_api_url>http://localhost:8080</rust_api_url>
      <http_port>8092</http_port>
      <drone>drone_1</drone>
      <drone>drone_2</drone>
      <drone>drone_3</drone>
    </plugin>

    <!-- Lighting -->
    <light type="directional" name="sun">
      <cast_shadows>true</cast_shadows>
      <pose>0 0 10 0 0 0</pose>
      <diffuse>0.8 0.8 0.8 1</diffuse>
    </light>

    <!-- Ground -->
    <model name="ground_plane">
      <static>true</static>
      <link name="link">
        <collision name="collision">
          <geometry>
            <plane>
              <normal>0 0 1</normal>
              <size>100 100</size>
            </plane>
          </geometry>
        </collision>
        <visual name="visual">
          <geometry>
            <plane>
              <normal>0 0 1</normal>
              <size>100 100</size>
            </plane>
          </geometry>
          <material>
            <ambient>0.8 0.8 0.8 1</ambient>
            <diffuse>0.8 0.8 0.8 1</diffuse>
          </material>
        </visual>
      </link>
    </model>

    <!-- Simple Drone Models -->
    <model name="drone_1">
      <pose>0 0 1 0 0 0</pose>
      <link name="body">
        <inertial>
          <mass>1.5</mass>
          <inertia>
            <ixx>0.03</ixx><iyy>0.03</iyy><izz>0.04</izz>
          </inertia>
        </inertial>
        <collision name="collision">
          <geometry>
            <box><size>0.3 0.3 0.1</size></box>
          </geometry>
        </collision>
        <visual name="visual">
          <geometry>
            <box><size>0.3 0.3 0.1</size></box>
          </geometry>
          <material>
            <ambient>1 0 0 1</ambient>
            <diffuse>1 0 0 1</diffuse>
          </material>
        </visual>
      </link>
    </model>

    <model name="drone_2">
      <pose>5 0 1 0 0 0</pose>
      <link name="body">
        <inertial>
          <mass>1.5</mass>
          <inertia>
            <ixx>0.03</ixx><iyy>0.03</iyy><izz>0.04</izz>
          </inertia>
        </inertial>
        <collision name="collision">
          <geometry>
            <box><size>0.3 0.3 0.1</size></box>
          </geometry>
        </collision>
        <visual name="visual">
          <geometry>
            <box><size>0.3 0.3 0.1</size></box>
          </geometry>
          <material>
            <ambient>0 1 0 1</ambient>
            <diffuse>0 1 0 1</diffuse>
          </material>
        </visual>
      </link>
    </model>

    <model name="drone_3">
      <pose>-5 0 1 0 0 0</pose>
      <link name="body">
        <inertial>
          <mass>1.5</mass>
          <inertia>
            <ixx>0.03</ixx><iyy>0.03</iyy><izz>0.04</izz>
          </inertia>
        </inertial>
        <collision name="collision">
          <geometry>
            <box><size>0.3 0.3 0.1</size></box>
          </geometry>
        </collision>
        <visual name="visual">
          <geometry>
            <box><size>0.3 0.3 0.1</size></box>
          </geometry>
          <material>
            <ambient>0 0 1 1</ambient>
            <diffuse>0 0 1 1</diffuse>
          </material>
        </visual>
      </link>
    </model>

  </world>
</sdf>
```

---

## Partie 4: Configuration Réseau

### Configuration Firewall (UFW)

```bash
# Installer UFW si nécessaire
sudo apt-get install -y ufw

# Autoriser SSH (IMPORTANT avant d'activer UFW!)
sudo ufw allow 22/tcp

# Autoriser le port Gazebo Bridge
sudo ufw allow 8092/tcp

# Activer le firewall
sudo ufw enable

# Vérifier le statut
sudo ufw status
```

**Résultat attendu:**
```
Status: active

To                         Action      From
--                         ------      ----
22/tcp                     ALLOW       Anywhere
8092/tcp                   ALLOW       Anywhere
```

### Script de Démarrage

Créer `~/uav_gazebo_server/start_gazebo.sh`:

```bash
#!/bin/bash
set -e

echo "=========================================="
echo "UAV Gazebo Server - Starting..."
echo "=========================================="

# Set plugin path
export IGN_GAZEBO_SYSTEM_PLUGIN_PATH=$HOME/uav_gazebo_server/plugins/rest_bridge/build:$IGN_GAZEBO_SYSTEM_PLUGIN_PATH

# Change to worlds directory
cd $HOME/uav_gazebo_server/worlds

# Start Gazebo (headless mode for server)
echo "Starting Ignition Gazebo (headless)..."
ign gazebo -s -r uav_swarm.sdf --verbose 2

echo "Gazebo started. Bridge listening on port 8092"
echo "Press Ctrl+C to stop"
```

Rendre exécutable:
```bash
chmod +x ~/uav_gazebo_server/start_gazebo.sh
```

---

## Partie 5: Test sur le Serveur

### Test 1: Démarrer Gazebo

```bash
cd ~/uav_gazebo_server
./start_gazebo.sh
```

**Messages attendus:**
```
Starting Ignition Gazebo (headless)...
RestBridgePlugin configured with 3 drones
Rust API URL: http://localhost:8080
HTTP Server port: 8092
Starting HTTP server on 0.0.0.0:8092
```

### Test 2: Vérifier le Plugin (depuis le serveur)

```bash
# Dans un autre terminal SSH
curl http://localhost:8092/health
```

**Résultat attendu:**
```json
{
  "status":"ok",
  "drones":["drone_1","drone_2","drone_3"],
  "sync_enabled":false
}
```

### Test 3: Tester depuis l'Extérieur

```bash
# Depuis votre Mac
curl http://SERVEUR_IP:8092/health
```

Si ça fonctionne, le serveur est prêt! ✅

---

## Partie 6: Configuration sur macOS

### Mettre à Jour la Configuration Locale

Éditer `config/simulation.toml`:

```toml
[gazebo]
bridge_url = "http://SERVEUR_IP:8092"  # Remplacer SERVEUR_IP
enabled = true
auto_start = false
timeout_ms = 15000  # Timeout plus élevé pour réseau
```

Ou utiliser variable d'environnement:

```bash
export UAV_GAZEBO_BRIDGE_URL="http://SERVEUR_IP:8092"
```

### Tester la Connexion

```bash
# Démarrer l'application Rust en mode Gazebo
cargo run -- --mode gazebo serve
```

**Messages attendus:**
```
Simulation mode set to: gazebo
Using Gazebo simulation engine
Gazebo bridge URL: http://SERVEUR_IP:8092
Swarm initialized with 3 drones
Current simulation mode: gazebo
Starting REST API server on 127.0.0.1:8080...
```

### Test Complet

```bash
# Terminal 1: Application Rust
cargo run -- --mode gazebo serve

# Terminal 2: Tests API
curl http://localhost:8080/api/simulation/status
curl -X POST http://localhost:8080/api/simulation/mode -H "Content-Type: application/json" -d '{"mode": "gazebo"}'
curl http://SERVEUR_IP:8092/health
```

---

## Sécurité et Bonnes Pratiques

### 1. Authentification (Recommandé)

Ajouter une clé API au plugin pour sécuriser l'accès:

```cpp
// Dans RestBridgePlugin.cc, ajouter vérification header
server.Post("/start", [](const httplib::Request &req, httplib::Response &res) {
  auto auth = req.get_header_value("X-API-Key");
  if (auth != "VOTRE_CLE_SECRETE") {
    res.status = 401;
    res.set_content("{\"error\":\"Unauthorized\"}", "application/json");
    return;
  }
  // ... reste du code
});
```

### 2. HTTPS/TLS (Production)

Utiliser un reverse proxy (nginx) pour HTTPS:

```nginx
server {
    listen 443 ssl;
    server_name gazebo.example.com;

    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;

    location / {
        proxy_pass http://localhost:8092;
        proxy_http_version 1.1;
    }
}
```

### 3. VPN (Recommandé)

Utiliser WireGuard ou OpenVPN pour sécuriser la connexion.

### 4. Monitoring

Installer monitoring basique:

```bash
# Installer htop pour surveiller ressources
sudo apt-get install -y htop

# Logs Gazebo
tail -f ~/.ignition/gazebo/server.log
```

---

## Dépannage

### Problème: Port 8092 non accessible

```bash
# Vérifier que Gazebo écoute
sudo netstat -tlnp | grep 8092

# Vérifier firewall
sudo ufw status

# Tester localement
curl http://localhost:8092/health
```

### Problème: Plugin non chargé

```bash
# Vérifier path
echo $IGN_GAZEBO_SYSTEM_PLUGIN_PATH

# Vérifier que le .so existe
ls -lh ~/uav_gazebo_server/plugins/rest_bridge/build/libRestBridgePlugin.so

# Lancer avec verbose
ign gazebo -v 4 -s uav_swarm.sdf
```

### Problème: Timeouts réseau

Augmenter le timeout dans `config/simulation.toml`:

```toml
timeout_ms = 30000  # 30 secondes
```

---

## Service Systemd (Optionnel)

Pour démarrer Gazebo automatiquement au boot:

`/etc/systemd/system/uav-gazebo.service`:

```ini
[Unit]
Description=UAV Gazebo Simulation Server
After=network.target

[Service]
Type=simple
User=YOUR_USER
WorkingDirectory=/home/YOUR_USER/uav_gazebo_server
Environment="IGN_GAZEBO_SYSTEM_PLUGIN_PATH=/home/YOUR_USER/uav_gazebo_server/plugins/rest_bridge/build"
ExecStart=/home/YOUR_USER/uav_gazebo_server/start_gazebo.sh
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Activer:
```bash
sudo systemctl enable uav-gazebo
sudo systemctl start uav-gazebo
sudo systemctl status uav-gazebo
```

---

## Résumé des Ports

| Service | Port | Description |
|---------|------|-------------|
| Rust UAV API | 8080 | API REST locale (macOS) |
| Gazebo Bridge | 8092 | HTTP server du plugin (serveur distant) |

---

## Checklist Finale

### Sur le Serveur:
- [ ] Ignition Gazebo installé
- [ ] Plugin C++ compilé
- [ ] Monde SDF créé
- [ ] Port 8092 ouvert dans firewall
- [ ] Gazebo démarre sans erreur
- [ ] `/health` répond depuis localhost
- [ ] `/health` répond depuis l'extérieur

### Sur macOS:
- [ ] `config/simulation.toml` mis à jour avec IP serveur
- [ ] Application Rust compile
- [ ] Mode gazebo se connecte au serveur
- [ ] API `/api/simulation/status` fonctionne
- [ ] Drones visibles dans Gazebo (GUI si activée)

---

## Support

En cas de problème:
1. Vérifier les logs Gazebo: `~/.ignition/gazebo/server.log`
2. Vérifier connectivité réseau: `ping SERVEUR_IP`
3. Vérifier port ouvert: `telnet SERVEUR_IP 8092`
4. Consulter documentation Ignition: https://gazebosim.org/docs/fortress
