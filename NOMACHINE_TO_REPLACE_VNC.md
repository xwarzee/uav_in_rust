# Installation NoMachine pour Remplacer VNC

## Pourquoi NoMachine au lieu de VNC ?

NoMachine offre plusieurs avantages par rapport à VNC :

- ✅ **Meilleure performance** - Compression H.264 matérielle
- ✅ **Latence plus faible** - Optimisé pour les connexions Internet
- ✅ **Meilleure qualité visuelle** - Rendu fluide même à distance
- ✅ **Audio supporté** - Transmission du son (utile pour certaines apps)
- ✅ **Transfert de fichiers** - Glisser-déposer entre Mac et serveur
- ✅ **Plus simple** - Configuration automatique
- ✅ **Gratuit** - Pour usage personnel et commercial de base
- ✅ **Multi-plateforme** - Windows, macOS, Linux, mobile

**Performance comparative :**
- VNC : ~15-30 fps, latence 100-500ms
- NoMachine : ~30-60 fps, latence 30-100ms

---

## Partie 1: Installation sur le Serveur Linux

### Étape 1: Connexion SSH

```bash
# Depuis votre Mac
ssh ubuntu@VOTRE_SERVEUR_IP
```

### Étape 2: Désactiver VNC (Optionnel)

Si vous aviez installé VNC précédemment :

```bash
# Arrêter VNC server
vncserver -kill :1 2>/dev/null || true

# Désactiver le service VNC au démarrage (si configuré)
sudo systemctl disable vncserver@:1.service 2>/dev/null || true

# Fermer le port VNC dans le firewall (optionnel)
sudo ufw delete allow 5901/tcp 2>/dev/null || true
```

### Étape 3: Mettre à Jour le Système

```bash
sudo apt-get update
sudo apt-get upgrade -y
```

### Étape 4: Installer un Environnement de Bureau

NoMachine nécessite un environnement de bureau. XFCE4 est recommandé (léger et performant) :

```bash
# Installer XFCE4
sudo apt-get install -y xfce4 xfce4-goodies dbus-x11

# Installer les outils essentiels
sudo apt-get install -y firefox curl wget git nano
```

**Alternatives (choisir UNE seule) :**

```bash
# Option 1: XFCE4 (Recommandé - léger et rapide)
sudo apt-get install -y xfce4 xfce4-goodies

# Option 2: GNOME (Plus lourd mais plus joli)
sudo apt-get install -y ubuntu-desktop

# Option 3: LXDE (Très léger)
sudo apt-get install -y lxde
```

### Étape 5: Télécharger NoMachine

```bash
# Créer un répertoire temporaire
cd /tmp

# Télécharger la dernière version (9.3.7 au moment de l'écriture)
wget https://download.nomachine.com/download/9.3/Linux/nomachine_9.3.7_1_amd64.deb
```

**Vérifier la dernière version :** https://www.nomachine.com/download/linux&id=1

**Pour serveurs ARM64 :**

```bash
wget https://download.nomachine.com/download/9.3/Linux/nomachine_9.3.7_1_arm64.deb
```

### Étape 6: Installer NoMachine

```bash
# Installer le package
sudo dpkg -i nomachine_9.3.7_1_amd64.deb

# Résoudre les dépendances si nécessaire
sudo apt-get install -f -y

# Vérifier l'installation
/usr/NX/bin/nxserver --status
```

**Résultat attendu :**
```
NX> 110 NX Server is running.
NX> 162 Service started.
```

### Étape 7: Configurer le Firewall

NoMachine utilise le port **4000** par défaut :

```bash
# Ouvrir le port NoMachine
sudo ufw allow 4000/tcp

# Ouvrir aussi le port Gazebo (pour plus tard)
sudo ufw allow 8092/tcp

# Activer le firewall si pas déjà fait
sudo ufw enable

# Vérifier les règles
sudo ufw status numbered
```

### Étape 8: Vérifier l'Installation

```bash
# Vérifier que NoMachine tourne
/usr/NX/bin/nxserver --status

# Vérifier que le port 4000 écoute
sudo netstat -tlnp | grep 4000
# ou
sudo ss -tlnp | grep 4000
```

**Résultat attendu :**
```
tcp        0      0 0.0.0.0:4000            0.0.0.0:*               LISTEN      1234/nxd
```

✅ **Installation serveur terminée !**

---

## Partie 2: Installation sur macOS (Client)

### Option 1: Via Homebrew (Recommandé)

```bash
# Installer NoMachine
brew install --cask nomachine

# Lancer NoMachine
open -a NoMachine
```

### Option 2: Téléchargement Direct

1. Aller sur https://www.nomachine.com/download/macos
2. Télécharger le .dmg
3. Ouvrir le fichier et glisser NoMachine vers Applications
4. Lancer depuis Applications

✅ **Installation client terminée !**

---

## Partie 3: Première Connexion

### Étape 1: Lancer NoMachine sur macOS

```bash
open -a NoMachine
```

### Étape 2: Ajouter une Nouvelle Connexion

1. Cliquer sur **"New"** ou le bouton **"+"**
2. Choisir **"NX"** protocol
3. Cliquer **"Continue"**

### Étape 3: Configuration du Serveur

```
Host: VOTRE_SERVEUR_IP
Port: 4000
```

Exemple :
```
Host: 51.210.100.200
Port: 4000
```

Cliquer **"Continue"**

### Étape 4: Authentification

- **Method:** Password
- Laisser les autres options par défaut

Cliquer **"Continue"**

### Étape 5: Proxy

- Sélectionner **"Don't use a proxy"**

Cliquer **"Continue"**

### Étape 6: Nom de la Connexion

```
Name: Gazebo Server
```

Cliquer **"Done"**

### Étape 7: Se Connecter

1. **Double-cliquer** sur la connexion "Gazebo Server"
2. Entrer **nom d'utilisateur** : `ubuntu`
3. Entrer le **mot de passe** de votre serveur
4. Cliquer **"OK"**

### Étape 8: Créer un Nouveau Bureau

NoMachine propose de créer un nouveau bureau virtuel :

- Sélectionner **"Create a new virtual desktop"**
- Choisir **"XFCE"** (si demandé)
- Cliquer **"OK"**

**🎉 Vous êtes connecté !**

Vous devriez voir le bureau XFCE4 du serveur.

---

## Partie 4: Lancer Gazebo dans NoMachine

### Étape 1: Ouvrir un Terminal

Dans la session NoMachine :

1. Clic droit sur le bureau → **Applications** → **Terminal Emulator**

OU :

- Cliquer sur le menu **Applications** (en haut à gauche)
- **Système** → **Terminal**

### Étape 2: Naviguer vers le Projet

```bash
# Aller dans le répertoire du projet Gazebo
cd /chemin/vers/uav_in_rust/gazebo/launch
```

### Étape 3: Lancer Gazebo

```bash
# Lancer avec interface graphique
./start_simulation.sh
```

**Gazebo devrait s'ouvrir dans la fenêtre NoMachine !** 🚁

Vous verrez :
- ✅ Le monde 3D avec terrain
- ✅ Les 3 drones (drone_1, drone_2, drone_3)
- ✅ L'interface Gazebo complète

### Étape 4: Tester le Plugin

Ouvrir un **second terminal** dans NoMachine :

```bash
# Vérifier que le plugin fonctionne
curl http://localhost:8092/health

# Activer la synchronisation
curl -X POST http://localhost:8092/start

# Envoyer une commande test
curl -X POST http://localhost:8092/drones/drone_1/command \
  -H "Content-Type: application/json" \
  -d '{"target_position": {"x": 10, "y": 5, "z": 3}}'
```

**Le drone_1 devrait se déplacer vers la position (10, 5, 3) !** 🎯

---

## Partie 5: Configuration de l'Application Rust

### Scénario: Rust sur Mac, Gazebo sur Serveur

**Architecture :**

```
┌─────────────────────┐         ┌──────────────────────┐
│   macOS             │         │   Serveur Linux      │
│                     │         │                      │
│  Rust App :8080     │◄────────┤  Gazebo + Plugin     │
│  (Développement)    │  HTTP   │  :8092               │
│                     │  :8092  │                      │
│  NoMachine Client   │◄────────┤  NoMachine Server    │
│  (Visualisation)    │  :4000  │  :4000               │
└─────────────────────┘         └──────────────────────┘
```

### Modifier la Configuration Rust

Sur **votre Mac**, éditer `config/simulation.toml` :

```toml
[simulation]
mode = "gazebo"           # Mode Gazebo par défaut
update_rate_hz = 10.0

[gazebo]
bridge_url = "http://VOTRE_SERVEUR_IP:8092"  # ← Remplacer par l'IP réelle
enabled = true
auto_start = false
timeout_ms = 10000        # 10 secondes de timeout (connexion Internet)
```

Exemple avec IP réelle :

```toml
bridge_url = "http://51.210.100.200:8092"
```

### Lancer l'Application Rust

```bash
# Sur votre Mac
cd /Users/scrumconseil/dev/claudecode/uav_in_rust

# Lancer en mode Gazebo
cargo run -- --mode gazebo serve
```

### Tester l'Intégration

**Terminal 1 (Mac) :**
```bash
cargo run -- --mode gazebo serve
```

**Terminal 2 (Mac) :**
```bash
# Vérifier le statut
curl http://localhost:8080/api/simulation/status

# Envoyer une commande à drone_2
curl -X PUT http://localhost:8080/api/drones/drone_2/target \
  -H "Content-Type: application/json" \
  -d '{"target": {"x": 15, "y": 10, "z": 5}}'
```

**Dans NoMachine, vous verrez drone_2 bouger !** 🎉

---

## Partie 6: Optimisation et Paramètres

### Régler la Qualité d'Affichage

Dans NoMachine, cliquer sur l'**icône de menu** (en haut) :

1. **Display** → **Change settings**
2. Ajuster :
   - **Quality:** Adaptive (ou High si bonne connexion)
   - **Resolution:** 1920x1080 (ajuster selon votre écran)
   - **Frame rate:** 30-60 fps

### Raccourcis Clavier Utiles

| Raccourci | Action |
|-----------|--------|
| **Cmd+Ctrl+0** | Afficher/masquer menu |
| **Cmd+Ctrl+F** | Plein écran |
| **Cmd+Ctrl+E** | Ouvrir presse-papier |
| **Cmd+Ctrl+Alt+R** | Redimensionner écran |
| **Cmd+Ctrl+Alt+M** | Minimiser |

### Transfert de Fichiers

**Méthode 1 : Glisser-Déposer**
- Glissez un fichier depuis votre Mac directement dans la fenêtre NoMachine

**Méthode 2 : Interface de Transfert**
1. Menu NoMachine → **Transfer files**
2. Naviguer dans les deux systèmes
3. Copier/coller entre Mac et serveur

---

## Partie 7: Gestion des Sessions

### Suspendre une Session (Applications Continuent)

1. Menu NoMachine → **Disconnect**
2. Choisir **"Suspend the session"**

**Les applications (comme Gazebo) continuent de tourner !**

### Reconnecter à une Session Suspendue

1. Relancer NoMachine
2. Se connecter au serveur
3. NoMachine détecte la session suspendue
4. Choisir **"Attach to the session"**

**Vous retrouvez Gazebo exactement où vous l'aviez laissé !**

### Terminer une Session Complètement

1. Dans NoMachine, fermer toutes les applications
2. Menu → **Disconnect** → **"Terminate the session"**

---

## Partie 8: Troubleshooting

### Problème 1: "Unable to connect to server"

**Causes possibles :**

1. **NoMachine n'est pas démarré sur le serveur**

```bash
# Sur le serveur
sudo /usr/NX/bin/nxserver --status

# Si arrêté, démarrer
sudo /usr/NX/bin/nxserver --start
```

2. **Firewall bloque le port 4000**

```bash
# Sur le serveur
sudo ufw status | grep 4000

# Si absent, ajouter
sudo ufw allow 4000/tcp
sudo ufw reload
```

3. **Firewall OVH (si applicable)**

- Vérifier dans le panel OVH que le port 4000 TCP est ouvert

### Problème 2: Écran Noir après Connexion

**Cause :** Environnement de bureau non installé

**Solution :**

```bash
# Sur le serveur
sudo apt-get install -y xfce4 xfce4-goodies dbus-x11
sudo /usr/NX/bin/nxserver --restart
```

### Problème 3: Performance Lente / Lag

**Solutions :**

1. **Réduire la qualité dans NoMachine :**
   - Display → Change settings → Quality: Low/Medium

2. **Réduire la résolution :**
   - Display → Resolution: 1280x720

3. **Fermer les applications inutiles sur le serveur**

4. **Vérifier la bande passante :**
```bash
# Test de vitesse (sur le serveur)
curl -s https://raw.githubusercontent.com/sivel/speedtest-cli/master/speedtest.py | python3 -
```

### Problème 4: Gazebo ne Démarre pas

**Erreur :** "cannot open display"

**Solution :**

```bash
# Dans le terminal NoMachine
export DISPLAY=:0

# Relancer Gazebo
cd /path/to/gazebo/launch
./start_simulation.sh
```

### Problème 5: Connexion Rust → Gazebo Échoue

**Symptôme :** L'application Rust ne peut pas joindre le plugin Gazebo

**Vérifier :**

```bash
# Sur le serveur, dans NoMachine
curl http://localhost:8092/health
```

**Si erreur "Connection refused" :**
- Gazebo n'est pas lancé
- Plugin non chargé

**Si erreur "timeout" depuis Mac :**
- Firewall bloque le port 8092
- Vérifier `sudo ufw allow 8092/tcp`

---

## Partie 9: Scripts d'Automatisation

### Script de Démarrage Gazebo (Serveur)

Créer sur le serveur :

```bash
cat > ~/start-gazebo.sh << 'EOF'
#!/bin/bash
# Script pour démarrer Gazebo automatiquement

export DISPLAY=:0
cd /path/to/uav_in_rust/gazebo/launch

echo "Démarrage de Gazebo..."
./start_simulation.sh

EOF

chmod +x ~/start-gazebo.sh
```

### Script de Connexion (macOS)

Créer sur votre Mac :

```bash
cat > ~/connect-gazebo.sh << 'EOF'
#!/bin/bash
# Script pour se connecter rapidement au serveur Gazebo

SERVER_IP="VOTRE_SERVEUR_IP"

echo "Connexion à NoMachine..."
open -a NoMachine

echo ""
echo "Connectez-vous à: $SERVER_IP"
echo ""
echo "Une fois connecté, lancez Gazebo avec:"
echo "  ~/start-gazebo.sh"

EOF

chmod +x ~/connect-gazebo.sh
```

---

## Partie 10: Sécurité

### Recommandations

1. **Mot de passe fort** pour le compte Linux

2. **Limiter les connexions simultanées :**

```bash
# Sur le serveur, éditer /usr/NX/etc/server.cfg
sudo nano /usr/NX/etc/server.cfg

# Ajouter/modifier :
SessionLimit 2
```

3. **Restreindre par IP (firewall) :**

```bash
# Autoriser NoMachine uniquement depuis votre IP
sudo ufw delete allow 4000/tcp
sudo ufw allow from VOTRE_IP_MAC to any port 4000
```

4. **Utiliser un tunnel SSH (très sécurisé) :**

```bash
# Sur votre Mac
ssh -L 4000:localhost:4000 ubuntu@SERVEUR_IP

# Puis dans NoMachine, se connecter à localhost:4000
```

### Surveiller les Connexions

```bash
# Voir les sessions actives
/usr/NX/bin/nxserver --list

# Logs NoMachine
tail -f /usr/NX/var/log/nxserver.log
```

---

## Comparaison VNC vs NoMachine

| Critère | VNC | NoMachine |
|---------|-----|-----------|
| **Performance** | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Latence** | 100-500ms | 30-100ms |
| **FPS** | 15-30 | 30-60 |
| **Qualité** | Moyenne | Haute (H.264) |
| **Audio** | ❌ | ✅ |
| **Transfert fichiers** | ❌ | ✅ Drag&drop |
| **Configuration** | Complexe | Simple |
| **Connexion lente** | Difficile | Optimisé |

**Verdict :** NoMachine est **largement supérieur** pour Gazebo.

---

## Résumé : Commandes Essentielles

### Installation Serveur (Ubuntu)

```bash
# 1. Installer environnement de bureau
sudo apt-get install -y xfce4 xfce4-goodies dbus-x11

# 2. Télécharger NoMachine
wget https://download.nomachine.com/download/9.3/Linux/nomachine_9.3.7_1_amd64.deb

# 3. Installer
sudo dpkg -i nomachine_9.3.7_1_amd64.deb
sudo apt-get install -f -y

# 4. Configurer firewall
sudo ufw allow 4000/tcp
sudo ufw allow 8092/tcp
sudo ufw enable

# 5. Vérifier
/usr/NX/bin/nxserver --status
```

### Installation Client (macOS)

```bash
brew install --cask nomachine
```

### Utilisation Quotidienne

```bash
# 1. Sur Mac : Lancer NoMachine et se connecter
open -a NoMachine
# Connecter à SERVEUR_IP:4000

# 2. Dans NoMachine : Ouvrir terminal et lancer Gazebo
cd /path/to/uav_in_rust/gazebo/launch
./start_simulation.sh

# 3. Sur Mac : Lancer l'app Rust
cargo run -- --mode gazebo serve

# 4. Tester
curl http://localhost:8080/api/simulation/status
```

---

## Checklist Complète

### Installation

- [ ] XFCE4 installé sur le serveur
- [ ] NoMachine serveur installé et démarré
- [ ] Firewall configuré (ports 4000, 8092)
- [ ] NoMachine client installé sur Mac
- [ ] Connexion créée et testée

### Configuration Gazebo

- [ ] Gazebo installé sur le serveur
- [ ] Plugin RestBridge compilé
- [ ] Gazebo lancé dans NoMachine
- [ ] Plugin répond sur :8092

### Intégration Rust

- [ ] `config/simulation.toml` configuré avec IP serveur
- [ ] Application Rust lancée en mode Gazebo
- [ ] Commandes envoyées depuis Rust
- [ ] Drones bougent dans Gazebo visualisé via NoMachine

---

## Support

- **Documentation NoMachine :** https://www.nomachine.com/documentation
- **Forum :** https://forums.nomachine.com/
- **Guide Gazebo :** Voir `gazebo/README.md` dans ce projet

---

## Conclusion

NoMachine offre une **excellente expérience** pour visualiser vos simulations Gazebo à distance :

✅ Installation en 10 minutes
✅ Performance fluide (30-60 fps)
✅ Qualité visuelle haute
✅ Transfert de fichiers facile
✅ Sessions persistantes
✅ Gratuit

**Vous êtes maintenant prêt à développer avec Gazebo sur serveur distant depuis votre Mac !** 🚁✨
