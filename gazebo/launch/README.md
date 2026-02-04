# Gazebo Launch Scripts

## start_simulation.sh

Script principal pour lancer la simulation Gazebo avec le monde UAV swarm.

### Usage

#### Mode GUI (avec visualisation 3D)

```bash
./start_simulation.sh
```

Utiliser ce mode si :
- Vous êtes sur macOS/Linux avec écran
- Vous voulez voir la simulation 3D
- Vous avez un GPU disponible

#### Mode Headless (sans GUI - pour serveurs)

```bash
./start_simulation.sh --headless
```

Utiliser ce mode si :
- Le serveur n'a pas d'écran (headless)
- Vous voulez économiser des ressources
- Pas besoin de visualisation 3D directe

### Options de Visualisation pour Serveurs Headless

Si votre serveur n'a pas d'écran mais que vous voulez visualiser la simulation, consultez :

**[GAZEBO_HEADLESS_SOLUTIONS.md](../../GAZEBO_HEADLESS_SOLUTIONS.md)**

Solutions disponibles :
- X11 Forwarding (simple)
- VNC Server (recommandé)
- noVNC (via navigateur)
- Dashboard web custom

### Ce que fait le script

1. Vérifie que Ignition Gazebo est installé
2. Configure les variables d'environnement (paths modèles et plugins)
3. Compile le plugin RestBridgePlugin si nécessaire
4. Vérifie que le monde SDF existe
5. Vérifie si l'API Rust est accessible (optionnel)
6. Lance Gazebo avec le monde `uav_swarm.sdf`

### Prérequis

- Ignition Gazebo Fortress installé
- CMake et compilateur C++ (pour compiler le plugin)
- Monde SDF dans `gazebo/worlds/uav_swarm.sdf`

### Installation Gazebo

**macOS:**
```bash
brew install ignition-fortress
```

**Ubuntu/Debian:**
```bash
sudo apt-get install ignition-fortress
```

Voir [REMOTE_GAZEBO_SETUP.md](../../REMOTE_GAZEBO_SETUP.md) pour installation complète sur serveur.

### Endpoints du Plugin

Une fois lancé, le plugin RestBridgePlugin expose les endpoints suivants sur le port **8092** :

| Méthode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/health` | Status et liste des drones |
| POST | `/start` | Activer sync Gazebo → Rust |
| POST | `/stop` | Désactiver sync |
| GET | `/drones/states` | États actuels de tous les drones |
| POST | `/drones/{id}/command` | Envoyer commande à un drone |

### Exemples

```bash
# Vérifier que le plugin fonctionne
curl http://localhost:8092/health

# Activer la synchronisation vers l'API Rust
curl -X POST http://localhost:8092/start

# Envoyer une commande à drone_1
curl -X POST http://localhost:8092/drones/drone_1/command \
  -H "Content-Type: application/json" \
  -d '{"target_position": {"x": 10, "y": 5, "z": 3}}'
```

### Troubleshooting

#### "Error: Ignition Gazebo not found"

Installez Ignition Gazebo :
- macOS: `brew install ignition-fortress`
- Linux: Voir REMOTE_GAZEBO_SETUP.md

#### "Plugin not found"

Le plugin sera compilé automatiquement au premier lancement. Si erreur :

```bash
cd gazebo/plugins/rest_bridge
./build.sh
```

#### "World file not found"

Vérifiez que le fichier existe :
```bash
ls ../../gazebo/worlds/uav_swarm.sdf
```

#### Mode headless ne fonctionne pas

Vérifiez que vous utilisez bien le flag :
```bash
./start_simulation.sh --headless
# ou
./start_simulation.sh -s
```

#### Gazebo ne démarre pas en headless

Certaines versions nécessitent des variables d'environnement supplémentaires :

```bash
export LIBGL_ALWAYS_SOFTWARE=1  # Force software rendering
export DISPLAY=:99  # Fake display
./start_simulation.sh --headless
```

### Arrêter la simulation

Appuyer sur `Ctrl+C` dans le terminal où Gazebo tourne.

### Relancer après modifications

Si vous modifiez :
- **Le plugin C++** : Recompiler avec `cd gazebo/plugins/rest_bridge && ./build.sh`
- **Le monde SDF** : Juste relancer le script
- **Le modèle drone** : Juste relancer le script

### Logs

Les logs Gazebo apparaissent dans la sortie standard du script.

Pour plus de détails :
```bash
# Niveau de verbosité maximum
ign gazebo -s uav_swarm.sdf --verbose 4
```

### Architecture

```
start_simulation.sh
    ↓
Vérifie Gazebo installé
    ↓
Configure env vars (IGN_GAZEBO_RESOURCE_PATH, IGN_GAZEBO_SYSTEM_PLUGIN_PATH)
    ↓
Compile plugin si nécessaire
    ↓
Lance: ign gazebo [--headless] uav_swarm.sdf
    ↓
Gazebo charge RestBridgePlugin
    ↓
Plugin démarre HTTP server sur :8092
    ↓
Prêt pour communication avec Rust API
```

## Pour Aller Plus Loin

- **Configuration serveur distant** : [REMOTE_GAZEBO_SETUP.md](../../REMOTE_GAZEBO_SETUP.md)
- **Solutions headless** : [GAZEBO_HEADLESS_SOLUTIONS.md](../../GAZEBO_HEADLESS_SOLUTIONS.md)
- **Documentation Gazebo** : [gazebo/README.md](../README.md)
- **Guide implémentation** : [IMPLEMENTATION_SUMMARY.md](../../IMPLEMENTATION_SUMMARY.md)
