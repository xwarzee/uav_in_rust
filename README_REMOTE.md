# UAV Swarm - Configuration Serveur Distant Gazebo

## Vue d'Ensemble

L'application UAV Swarm peut fonctionner en deux modes:
- **Mode Internal**: Simulation physique simple en Rust (local)
- **Mode Gazebo**: Simulation réaliste via serveur distant (réseau)

## Architecture avec Serveur Distant

```
┌───────────────────────┐         Internet/VPN        ┌─────────────────────────┐
│  macOS Dev (Local)    │◄──────────────────────────►│  Serveur Linux Distant  │
│                       │                             │                         │
│  Rust App :8080       │         HTTP :8092          │  Gazebo + Plugin :8092  │
│  - Mode Internal      │                             │  - Physique réaliste    │
│  - Mode Gazebo ───────┼─────────────────────────────┤  - 3 drones simulés     │
│                       │                             │  - REST Bridge          │
└───────────────────────┘                             └─────────────────────────┘
```

## Démarrage Rapide

### 1. Configuration Locale (macOS)

Éditer `config/simulation.toml`:

```toml
[gazebo]
bridge_url = "http://VOTRE_SERVEUR_IP:8092"  # Remplacer par l'IP réelle
enabled = true
timeout_ms = 15000
```

Ou utiliser variable d'environnement:

```bash
export UAV_GAZEBO_BRIDGE_URL="http://VOTRE_SERVEUR_IP:8092"
```

### 2. Démarrer l'Application

**Mode Internal (local, pas besoin du serveur):**
```bash
cargo run -- serve
# ou explicitement:
cargo run -- --mode internal serve
```

**Mode Gazebo (serveur distant):**
```bash
cargo run -- --mode gazebo serve
```

L'application tentera de se connecter au serveur Gazebo. Si la connexion échoue, elle basculera automatiquement en mode internal.

### 3. Tester

```bash
# Vérifier le mode actuel
curl http://localhost:8080/api/simulation/mode

# Vérifier le statut
curl http://localhost:8080/api/simulation/status

# Changer de mode
curl -X POST http://localhost:8080/api/simulation/mode \
  -H "Content-Type: application/json" \
  -d '{"mode": "gazebo"}'
```

## Configuration du Serveur Distant

Voir le guide complet: **[REMOTE_GAZEBO_SETUP.md](./REMOTE_GAZEBO_SETUP.md)**

**Résumé rapide:**
1. Installer Ignition Gazebo Fortress sur serveur Linux
2. Compiler le plugin C++ RestBridgePlugin
3. Créer le monde SDF avec 3 drones
4. Ouvrir le port 8092 dans le firewall
5. Démarrer Gazebo avec le plugin

## Endpoints API

### Gestion de Simulation

```bash
# Obtenir mode actuel
GET /api/simulation/mode

# Changer de mode
POST /api/simulation/mode
Body: {"mode": "internal" | "gazebo"}

# Statut détaillé
GET /api/simulation/status

# Démarrer/arrêter
POST /api/simulation/start
POST /api/simulation/stop
```

### Gestion des Drones

```bash
# Lister les drones
GET /api/drones

# Détails d'un drone
GET /api/drones/{id}

# Mettre à jour cible
PUT /api/drones/{id}/target
Body: {"target": {"x": 10, "y": 5, "z": 15}}

# Mettre à jour état (depuis Gazebo)
PUT /api/drones/{id}/state
Body: {"position": {...}, "velocity": {...}}
```

## Test de Connectivité

### Tester le Serveur Gazebo

```bash
# Depuis votre Mac, tester l'accès au serveur
curl http://VOTRE_SERVEUR_IP:8092/health
```

**Réponse attendue:**
```json
{
  "status": "ok",
  "drones": ["drone_1", "drone_2", "drone_3"],
  "sync_enabled": false
}
```

### Script de Test Complet

```bash
# Définir l'URL du serveur Gazebo
export GAZEBO_SERVER_URL="http://VOTRE_SERVEUR_IP:8092"

# Lancer les tests
./test_simulation_api.sh
```

## Flux de Données

### Gazebo → Rust (Synchronisation des États)

```
1. Gazebo simule physique (gravité, collisions, etc.)
2. Plugin RestBridge lit positions/velocités
3. Plugin HTTP PUT → http://localhost:8080/api/drones/{id}/state
4. Rust met à jour état interne
5. WebSocket broadcast aux clients connectés
```

### Rust → Gazebo (Envoi de Commandes)

```
1. Client envoie commande → POST /api/drones/{id}/target
2. Rust stocke target_position
3. Rust HTTP POST → http://SERVEUR:8092/drones/{id}/command
4. Plugin Gazebo applique force au modèle
5. Gazebo simule mouvement
```

## Dépannage

### Erreur: "Bridge connection failed"

```bash
# Vérifier que le serveur est accessible
ping VOTRE_SERVEUR_IP

# Vérifier que le port est ouvert
telnet VOTRE_SERVEUR_IP 8092

# Tester le endpoint health
curl http://VOTRE_SERVEUR_IP:8092/health
```

**Solutions:**
- Vérifier firewall sur le serveur: `sudo ufw status`
- Vérifier que Gazebo tourne: `ps aux | grep ign`
- Vérifier logs Gazebo: `tail -f ~/.ignition/gazebo/server.log`

### L'application bascule toujours en mode internal

**Cause:** Le serveur Gazebo n'est pas accessible

**Vérifications:**
```bash
# 1. URL configurée correctement?
cat config/simulation.toml | grep bridge_url

# 2. Serveur accessible?
curl http://VOTRE_SERVEUR_IP:8092/health

# 3. Timeout suffisant?
# Augmenter timeout_ms dans config/simulation.toml
```

### Timeout lors du changement de mode

**Solution:** Augmenter le timeout dans `config/simulation.toml`:

```toml
[gazebo]
timeout_ms = 30000  # 30 secondes pour réseau lent
```

## Sécurité

### Production

Pour un environnement de production:

1. **VPN**: Utiliser WireGuard ou OpenVPN
2. **Firewall**: Restreindre l'accès au port 8092
3. **HTTPS**: Utiliser nginx comme reverse proxy
4. **Authentification**: Ajouter API key au plugin

Voir [REMOTE_GAZEBO_SETUP.md](./REMOTE_GAZEBO_SETUP.md) section "Sécurité".

### Développement

Pour le développement, HTTP simple suffit si:
- Serveur sur réseau local privé
- Ou serveur dans cloud avec firewall restreint à votre IP

## Variables d'Environnement

```bash
# URL du serveur Gazebo
export UAV_GAZEBO_BRIDGE_URL="http://SERVEUR_IP:8092"

# Mode de simulation par défaut
export UAV_SIMULATION_MODE="gazebo"

# Timeout (millisecondes)
export UAV_GAZEBO_TIMEOUT_MS="15000"

# Pour les tests
export GAZEBO_SERVER_URL="http://SERVEUR_IP:8092"
```

## Commandes Utiles

```bash
# Démarrer en mode Gazebo avec config custom
cargo run -- --mode gazebo --config config/production.toml serve

# Vérifier le statut complet
curl -s http://localhost:8080/api/simulation/status | jq .

# Activer la synchro Gazebo→Rust
curl -X POST http://SERVEUR_IP:8092/start

# Désactiver la synchro
curl -X POST http://SERVEUR_IP:8092/stop

# Obtenir états depuis Gazebo
curl -s http://SERVEUR_IP:8092/drones/states | jq .
```

## Performances

### Latence Réseau

| Réseau | Latence Typique |
|--------|----------------|
| Localhost | ~1ms |
| Réseau local (LAN) | 1-5ms |
| Même datacenter | 1-10ms |
| Internet (même pays) | 10-50ms |
| Internet (international) | 100-300ms |

**Recommandation:** Utiliser un serveur dans le même datacenter ou pays pour latence < 50ms.

### Fréquence de Mise à Jour

- **Gazebo**: 1000 Hz (1ms physics tick)
- **Plugin HTTP**: ~10-100 Hz (configurable)
- **Rust API**: ~10 Hz (configurable dans simulation.toml)

## Monitoring

### Sur le Serveur

```bash
# Ressources système
htop

# Logs Gazebo
tail -f ~/.ignition/gazebo/server.log

# Vérifier port
sudo netstat -tlnp | grep 8092
```

### Métriques API

```bash
# Statut simulation
watch -n 1 'curl -s http://localhost:8080/api/simulation/status | jq .'

# Positions drones
watch -n 1 'curl -s http://localhost:8080/api/drones | jq .'
```

## Prochaines Étapes

1. **Configuration serveur**: Suivre [REMOTE_GAZEBO_SETUP.md](./REMOTE_GAZEBO_SETUP.md)
2. **Tests**: Lancer `./test_simulation_api.sh`
3. **WebSocket**: Se connecter à `ws://localhost:8080/ws/drones` pour updates temps réel
4. **Formations**: Tester formations de drones en mode Gazebo
5. **Missions**: Exécuter missions MoveTo/Patrol/Search

## Support

- **Guide Installation Serveur**: [REMOTE_GAZEBO_SETUP.md](./REMOTE_GAZEBO_SETUP.md)
- **Documentation Ignition**: https://gazebosim.org/docs/fortress
- **API Swagger**: http://localhost:8080/swagger-ui/
