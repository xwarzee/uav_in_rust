# Guide d'Installation Ignition Gazebo pour UAV Swarm

## Installation sur macOS

### Option 1: Via Homebrew (Recommandé)

**Prérequis:**
- macOS (testé sur votre version)
- Homebrew installé ✅
- Xcode Command Line Tools

### Étapes d'Installation

#### 1. Ajouter le tap OSRF (Fait ✅)
```bash
brew tap osrf/simulation
```

#### 2. Installer Ignition Gazebo Fortress (En cours...)
```bash
brew install ignition-fortress
```

**Durée estimée:** 10-20 minutes
**Taille:** ~1-2 GB avec dépendances

#### 3. Vérifier l'installation
```bash
ign gazebo --version
```

Devrait afficher: `Ignition Gazebo, version 6.x.x` ou similaire

#### 4. Tester avec un monde simple
```bash
ign gazebo shapes.sdf
```

### Dépendances de Compilation (pour le plugin C++)

Une fois Gazebo installé, installer les headers de développement:

```bash
# CMake (si pas déjà installé)
brew install cmake

# Vérifier Xcode Command Line Tools
xcode-select --install  # Si pas déjà installé
```

### Résolution de Problèmes

**Problème:** `ign: command not found`
**Solution:** Ajouter à votre PATH:
```bash
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Problème:** Erreurs de compilation du plugin
**Solution:** Vérifier que `ignition-gazebo7-dev` est installé:
```bash
brew list | grep ignition
```

**Problème:** Gazebo ne lance pas
**Solution:** Vérifier les logs:
```bash
ign gazebo -v 4 shapes.sdf  # Mode verbose
```

### Architecture Ignition Gazebo

Ignition Gazebo (maintenant appelé "Gazebo") est la nouvelle génération:
- **Gazebo Classic** (gazebo11): Ancienne version, dépend de ROS
- **Ignition Gazebo** (Fortress): Nouvelle version, indépendante de ROS ✅

Nous utilisons **Ignition Fortress** (Gazebo 6.x) pour:
- Meilleur support macOS
- Pas de dépendance ROS2
- API C++ moderne
- Communication via Ignition Transport (ZeroMQ)

### Prochaines Étapes

Une fois Gazebo installé:
1. ✅ Tester un monde simple
2. 📝 Créer le plugin C++ avec serveur HTTP
3. 🌍 Créer les mondes SDF personnalisés
4. 🚁 Intégrer les modèles de drones

## Commandes Utiles

```bash
# Lister les mondes disponibles
ign gazebo --list

# Lancer Gazebo sans GUI (headless)
ign gazebo -s shapes.sdf

# Afficher les topics actifs
ign topic -l

# Publier sur un topic
ign topic -t /topic_name -m ignition.msgs.StringMsg -p 'data:"test"'
```

## Références

- [Ignition Gazebo Documentation](https://gazebosim.org/docs/fortress)
- [Ignition Transport](https://gazebosim.org/libs/transport)
- [Plugin Tutorial](https://gazebosim.org/api/gazebo/6/createsystemplugins.html)
