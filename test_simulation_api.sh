#!/bin/bash
# Script de test pour les endpoints de simulation API

BASE_URL="http://localhost:8080"
GAZEBO_SERVER="${GAZEBO_SERVER_URL:-http://localhost:8092}"  # URL serveur Gazebo (défaut: local)
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=========================================="
echo "Test des Endpoints de Simulation API"
echo "=========================================="
echo ""

# Fonction de test
test_endpoint() {
    local method=$1
    local path=$2
    local data=$3
    local description=$4

    echo -e "${YELLOW}Test:${NC} $description"
    echo "  → $method $path"

    if [ -z "$data" ]; then
        response=$(curl -s -w "\n%{http_code}" -X $method "$BASE_URL$path")
    else
        response=$(curl -s -w "\n%{http_code}" -X $method "$BASE_URL$path" \
            -H "Content-Type: application/json" \
            -d "$data")
    fi

    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')

    if [ "$http_code" = "200" ] || [ "$http_code" = "201" ]; then
        echo -e "  ${GREEN}✓${NC} Status: $http_code"
        echo "  Response: $body" | head -c 100
        echo "..."
    else
        echo -e "  ${RED}✗${NC} Status: $http_code"
        echo "  Error: $body"
    fi
    echo ""
}

# Vérifier que le serveur est accessible
echo "Vérification du serveur..."
if ! curl -s "$BASE_URL/health" > /dev/null 2>&1; then
    echo -e "${RED}✗ Erreur: Le serveur n'est pas accessible sur $BASE_URL${NC}"
    echo "Démarrez le serveur avec: cargo run -- serve"
    exit 1
fi
echo -e "${GREEN}✓ Serveur accessible${NC}"
echo ""

# Vérifier le serveur Gazebo (si configuré)
echo "Vérification du serveur Gazebo distant..."
echo "  URL: $GAZEBO_SERVER"
if curl -s "$GAZEBO_SERVER/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Serveur Gazebo accessible${NC}"
    GAZEBO_AVAILABLE=true
else
    echo -e "${YELLOW}⚠ Serveur Gazebo non accessible (mode gazebo désactivé pour les tests)${NC}"
    GAZEBO_AVAILABLE=false
fi
echo ""

# Tests des endpoints
echo "=========================================="
echo "1. Tests du Mode de Simulation"
echo "=========================================="
echo ""

test_endpoint "GET" "/api/simulation/mode" "" \
    "Obtenir le mode actuel"

test_endpoint "POST" "/api/simulation/mode" '{"mode": "internal"}' \
    "Changer vers mode internal"

test_endpoint "POST" "/api/simulation/mode" '{"mode": "gazebo"}' \
    "Changer vers mode gazebo (devrait fallback si bridge indisponible)"

test_endpoint "POST" "/api/simulation/mode" '{"mode": "invalid"}' \
    "Test mode invalide (devrait échouer)"

echo "=========================================="
echo "2. Tests du Statut de Simulation"
echo "=========================================="
echo ""

test_endpoint "GET" "/api/simulation/status" "" \
    "Obtenir le statut complet"

echo "=========================================="
echo "3. Tests de Contrôle de Simulation"
echo "=========================================="
echo ""

test_endpoint "POST" "/api/simulation/start" "" \
    "Démarrer la simulation"

sleep 1

test_endpoint "POST" "/api/simulation/stop" "" \
    "Arrêter la simulation"

echo "=========================================="
echo "4. Tests de Mise à Jour de Drone (depuis Gazebo)"
echo "=========================================="
echo ""

test_endpoint "PUT" "/api/drones/drone_1/state" \
    '{"position": {"x": 1.5, "y": 2.0, "z": 10.5}, "velocity": {"vx": 0.5, "vy": 0.0, "vz": 0.1}}' \
    "Mettre à jour état du drone_1"

test_endpoint "GET" "/api/drones/drone_1" "" \
    "Vérifier mise à jour du drone_1"

test_endpoint "PUT" "/api/drones/nonexistent/state" \
    '{"position": {"x": 0, "y": 0, "z": 0}, "velocity": {"vx": 0, "vy": 0, "vz": 0}}' \
    "Test avec drone inexistant (devrait échouer)"

echo "=========================================="
echo "5. Tests d'Intégration"
echo "=========================================="
echo ""

# Scénario complet
echo "Scénario: Mode internal → Gazebo → Internal"
test_endpoint "POST" "/api/simulation/mode" '{"mode": "internal"}' "1. Mode internal"
test_endpoint "GET" "/api/simulation/status" "" "2. Vérifier statut"
test_endpoint "POST" "/api/simulation/mode" '{"mode": "gazebo"}' "3. Tenter gazebo"
test_endpoint "GET" "/api/simulation/mode" "" "4. Vérifier mode final"

echo ""
echo "=========================================="
echo "Tests Terminés!"
echo "=========================================="
