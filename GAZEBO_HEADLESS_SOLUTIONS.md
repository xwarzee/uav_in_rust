# Solutions pour Visualiser Gazebo sur Serveur Headless

## Problème

Votre serveur Linux distant n'a pas d'écran physique attaché, mais vous souhaitez visualiser la simulation 3D Gazebo depuis votre Mac.

## Solutions Possibles

### Solution 1: Mode Headless (Pas de Visualisation) ⭐ **Recommandé pour Production**

**Principe:** Gazebo tourne en mode serveur uniquement (pas de GUI), la visualisation se fait via les données API.

**Avantages:**
- ✅ Pas besoin de GPU sur le serveur
- ✅ Très léger en ressources
- ✅ Idéal pour production/CI
- ✅ Aucune configuration graphique

**Inconvénients:**
- ❌ Pas de visualisation 3D directe
- ❌ Debugging plus difficile

**Configuration:**

```bash
# Lancer Gazebo en mode headless (sans GUI)
ign gazebo -s uav_swarm.sdf
```

Modification du fichier `gazebo/launch/start_simulation.sh`:

```bash
# Ajouter l'option -s (server only)
ign gazebo -s "$WORLD_FILE" --verbose 2
```

**Alternative - Visualisation via Web Dashboard:**
Créer une interface web simple qui affiche les positions des drones en 2D/3D via Three.js ou Plotly:

```javascript
// Exemple: Visualisation temps réel via WebSocket
const ws = new WebSocket('ws://localhost:8080/ws/drones');
ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  // Afficher les drones dans une scène Three.js
  updateDrone3DPosition(update.drone_id, update.position);
};
```

---

### Solution 2: X11 Forwarding via SSH 🚀 **Plus Simple pour Développement**

**Principe:** Transférer l'affichage graphique X11 du serveur vers votre Mac via SSH.

**Avantages:**
- ✅ Simple à configurer
- ✅ Pas de logiciel supplémentaire sur serveur
- ✅ Interface Gazebo complète

**Inconvénients:**
- ❌ Nécessite XQuartz sur Mac
- ❌ Latence réseau importante
- ❌ Consommation bande passante élevée
- ❌ Nécessite GPU sur serveur

**Installation sur macOS:**

```bash
# Installer XQuartz
brew install --cask xquartz

# Redémarrer votre session macOS après installation
```

**Configuration:**

```bash
# Se connecter au serveur avec X11 forwarding
ssh -X user@serveur_ip

# Ou pour de meilleures performances
ssh -Y user@serveur_ip  # Trusted X11 forwarding

# Sur le serveur, lancer Gazebo normalement
cd /path/to/gazebo/launch
./start_simulation.sh
```

**Optimisation des performances:**

```bash
# Compression pour réduire latence
ssh -X -C user@serveur_ip

# Tunnel avec compression aggressive
ssh -X -C -o CompressionLevel=9 user@serveur_ip
```

**Troubleshooting:**

```bash
# Vérifier que DISPLAY est défini
echo $DISPLAY  # Doit afficher quelque chose comme "localhost:10.0"

# Tester X11 forwarding
xeyes  # Doit afficher une fenêtre

# Si erreur "cannot open display"
export DISPLAY=localhost:10.0
```

---

### Solution 3: VNC Server (Bureau Virtuel) 🖥️ **Meilleure Performance**

**Principe:** Créer un bureau virtuel sur le serveur, accessible via VNC depuis votre Mac.

**Avantages:**
- ✅ Meilleure performance que X11
- ✅ Bureau complet Linux disponible
- ✅ Persistant (survit à la déconnexion SSH)
- ✅ Plusieurs utilisateurs possibles

**Inconvénients:**
- ❌ Configuration plus complexe
- ❌ Port supplémentaire à ouvrir (5900+)
- ❌ Nécessite GPU virtuel (ou logiciel)

**Installation sur le serveur:**

```bash
# Installer TigerVNC server et desktop environment
sudo apt-get update
sudo apt-get install tigervnc-standalone-server tigervnc-common
sudo apt-get install xfce4 xfce4-goodies  # Desktop léger

# Configurer VNC password
vncpasswd
# Entrer un mot de passe (8 caractères max)
```

**Configuration VNC:**

Créer `~/.vnc/xstartup`:

```bash
#!/bin/bash
unset SESSION_MANAGER
unset DBUS_SESSION_BUS_ADDRESS
exec startxfce4
```

Rendre exécutable:

```bash
chmod +x ~/.vnc/xstartup
```

**Démarrer le serveur VNC:**

```bash
# Démarrer sur display :1 (port 5901)
vncserver :1 -geometry 1920x1080 -depth 24

# Ou avec systemd pour auto-start
sudo systemctl enable vncserver@:1.service
sudo systemctl start vncserver@:1.service
```

**Configurer le firewall:**

```bash
# Ouvrir port VNC (5901 pour display :1)
sudo ufw allow 5901/tcp
```

**Se connecter depuis macOS:**

```bash
# Option 1: Utiliser Screen Sharing intégré
# Finder → Go → Connect to Server
# vnc://serveur_ip:5901

# Option 2: Utiliser RealVNC Viewer (plus performant)
brew install --cask vnc-viewer
# Puis connecter à serveur_ip:5901
```

**Optimisation pour Gazebo:**

Créer `~/.vnc/config`:

```
geometry=1920x1080
depth=24
dpi=96
```

**Lancer Gazebo dans VNC:**

```bash
# Via SSH
ssh user@serveur_ip

# Définir DISPLAY pour la session VNC
export DISPLAY=:1

# Lancer Gazebo
cd /path/to/gazebo/launch
./start_simulation.sh
```

---

### Solution 4: Ignition Gazebo Client-Serveur ⚡ **Meilleure Qualité**

**Principe:** Gazebo Garden (pas Fortress) supporte nativement la séparation serveur de simulation et client de visualisation.

**Avantages:**
- ✅ Architecture native Ignition
- ✅ Excellente performance
- ✅ Simulation sur serveur, GUI sur Mac
- ✅ Pas de GPU requis sur serveur

**Inconvénients:**
- ❌ Nécessite Gazebo Garden (pas Fortress)
- ❌ Configuration plus complexe
- ❌ Pas encore stable sur tous les plugins

**Sur le serveur (Simulation):**

```bash
# Installer Gazebo Garden au lieu de Fortress
# https://gazebosim.org/docs/garden/install

# Lancer en mode serveur uniquement
ign gazebo -s -v 4 uav_swarm.sdf
```

**Sur votre Mac (Visualisation):**

```bash
# Installer Gazebo Garden
brew install gz-garden

# Configurer l'adresse du serveur
export IGN_IP=SERVEUR_IP

# Lancer le client GUI seulement
ign gazebo -g
```

**Note:** Cette solution nécessite de migrer de Fortress vers Garden, ce qui peut nécessiter des modifications des fichiers SDF.

---

### Solution 5: noVNC (VNC via Navigateur Web) 🌐 **Sans Client VNC**

**Principe:** Accéder au bureau virtuel via un navigateur web (HTML5).

**Avantages:**
- ✅ Pas de client VNC à installer
- ✅ Accessible depuis n'importe quel navigateur
- ✅ Multiplateforme
- ✅ Tunnel HTTPS possible

**Inconvénients:**
- ❌ Performance légèrement inférieure à VNC natif
- ❌ Configuration web serveur nécessaire

**Installation:**

```bash
# Sur le serveur
sudo apt-get install novnc python3-websockify

# Configurer noVNC
cd /usr/share/novnc
./utils/novnc_proxy --vnc localhost:5901 --listen 6080
```

**Accès:**

Ouvrir dans votre navigateur:
```
http://SERVEUR_IP:6080/vnc.html
```

**Sécurisation avec nginx (recommandé):**

```nginx
# /etc/nginx/sites-available/novnc
server {
    listen 443 ssl;
    server_name gazebo.example.com;

    ssl_certificate /etc/letsencrypt/live/gazebo.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/gazebo.example.com/privkey.pem;

    location / {
        proxy_pass http://localhost:6080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

### Solution 6: VirtualGL + TurboVNC (Pour GPU Intensif) 🎮

**Principe:** Utiliser le GPU du serveur pour le rendu, puis transférer via VNC compressé.

**Avantages:**
- ✅ Utilise le GPU du serveur
- ✅ Meilleure qualité graphique
- ✅ Compression optimisée pour 3D

**Inconvénients:**
- ❌ Nécessite GPU sur serveur (NVIDIA/AMD)
- ❌ Configuration très complexe
- ❌ Overkill pour 3 drones

**Quand l'utiliser:**
- Serveur avec GPU NVIDIA/AMD dédié
- Simulations avec beaucoup d'objets visuels
- Besoin de haute qualité graphique

**Installation:** (Complexe - documentation complète disponible sur demande)

---

## Comparaison des Solutions

| Solution | Difficulté | Performance | Coût Serveur | Coût Réseau | GPU Requis |
|----------|------------|-------------|--------------|-------------|------------|
| Headless | ⭐ Facile | ⭐⭐⭐⭐⭐ | Très faible | Très faible | ❌ Non |
| X11 Forward | ⭐⭐ Moyen | ⭐⭐ Faible | Moyen | Élevé | ✅ Oui |
| VNC | ⭐⭐⭐ Moyen | ⭐⭐⭐⭐ Bon | Moyen | Moyen | ⚠️ Conseillé |
| noVNC | ⭐⭐⭐ Moyen | ⭐⭐⭐ Moyen | Moyen | Moyen | ⚠️ Conseillé |
| Gazebo C/S | ⭐⭐⭐⭐ Difficile | ⭐⭐⭐⭐⭐ Excellent | Faible | Moyen | ❌ Non |
| VirtualGL | ⭐⭐⭐⭐⭐ Très difficile | ⭐⭐⭐⭐⭐ Excellent | Élevé | Moyen | ✅ Oui (dédié) |

---

## Recommandations par Cas d'Usage

### Cas 1: Production / CI/CD ➜ **Headless Mode**

```bash
# Pas besoin de visualisation, juste la physique
ign gazebo -s uav_swarm.sdf
```

Créer un dashboard web simple avec Three.js pour visualiser les positions.

### Cas 2: Développement Occasionnel ➜ **X11 Forwarding**

```bash
# Sur Mac: installer XQuartz
brew install --cask xquartz

# Se connecter et lancer
ssh -Y user@serveur
cd gazebo/launch && ./start_simulation.sh
```

### Cas 3: Développement Fréquent ➜ **VNC Server** ⭐ **Recommandé**

- Installation: TigerVNC + XFCE4
- Connexion: VNC Viewer ou Screen Sharing macOS
- Port: 5901 (display :1)
- Persistant et performant

### Cas 4: Équipe Multiple ➜ **noVNC**

- Accessible via navigateur
- Pas de client à installer
- Partage facile avec l'équipe

---

## Guide de Mise en Place Rapide: VNC Server (Recommandé)

### Sur le Serveur Linux:

```bash
# 1. Installer VNC + Desktop
sudo apt-get update
sudo apt-get install -y tigervnc-standalone-server xfce4 xfce4-goodies

# 2. Configurer password
vncpasswd

# 3. Créer xstartup
cat > ~/.vnc/xstartup << 'EOF'
#!/bin/bash
unset SESSION_MANAGER
unset DBUS_SESSION_BUS_ADDRESS
exec startxfce4
EOF

chmod +x ~/.vnc/xstartup

# 4. Démarrer VNC server
vncserver :1 -geometry 1920x1080 -depth 24

# 5. Ouvrir firewall
sudo ufw allow 5901/tcp

# 6. Tester
echo "VNC server démarré sur port 5901"
echo "Connectez-vous avec VNC client à: $(hostname -I | awk '{print $1}'):5901"
```

### Sur votre Mac:

```bash
# Option 1: Screen Sharing intégré
# Finder → Aller → Se connecter au serveur...
# vnc://SERVEUR_IP:5901

# Option 2: Installer VNC Viewer (plus performant)
brew install --cask vnc-viewer
# Puis connecter à: SERVEUR_IP:5901
```

### Lancer Gazebo dans VNC:

```bash
# SSH vers le serveur
ssh user@serveur_ip

# Définir DISPLAY pour VNC session
export DISPLAY=:1

# Naviguer vers gazebo
cd /path/to/uav_in_rust/gazebo/launch

# Lancer
./start_simulation.sh
```

Vous verrez alors Gazebo s'afficher dans votre client VNC sur Mac ! 🎉

---

## Alternative: Visualisation Web 3D Custom

Si vous ne voulez vraiment pas de VNC/X11, je peux créer une **interface web 3D** qui se connecte via WebSocket à votre API Rust et affiche les drones en temps réel avec Three.js.

**Avantages:**
- Léger, pas de VNC
- Accessible depuis navigateur
- Personnalisable
- Pas de GPU serveur requis

**Voulez-vous que je crée cette interface web ?**

---

## Quelle Solution Choisir ?

**Ma recommandation: VNC Server (Solution 3)** pour les raisons suivantes:

1. ✅ Bon compromis performance/simplicité
2. ✅ Bureau complet pour debugging
3. ✅ Persistant (ne se ferme pas si SSH déconnecté)
4. ✅ Peut utiliser GPU si disponible (mais pas obligatoire)
5. ✅ Compatible avec tous les outils graphiques Linux
6. ✅ Latence acceptable sur LAN/VPN

**Si vous voulez juste voir les drones bouger** sans la GUI Gazebo complète, je peux aussi créer un **dashboard web 3D léger** qui visualise les positions via votre API WebSocket existante.

Quelle approche préférez-vous ?
