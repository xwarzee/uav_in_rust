#!/bin/bash

# Script to run FitNesse with UAV Swarm API tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FITNESSE_VERSION="20251025"
FITNESSE_JAR="fitnesse-${FITNESSE_VERSION}-standalone.jar"
FITNESSE_PORT="${FITNESSE_PORT:-8000}"

echo "========================================="
echo "UAV Swarm API - FitNesse Test Runner"
echo "========================================="
echo ""

# Check if FitNesse JAR exists
if [ ! -f "$SCRIPT_DIR/$FITNESSE_JAR" ]; then
    echo "FitNesse JAR not found. Downloading..."
    curl -L "https://github.com/unclebob/fitnesse/releases/download/v${FITNESSE_VERSION}/fitnesse-${FITNESSE_VERSION}-standalone.jar" \
        -o "$SCRIPT_DIR/$FITNESSE_JAR"
    echo "Download complete."
    echo ""
fi

# Check if fixtures are built
if [ ! -f "$SCRIPT_DIR/fixtures/target/fitnesse-fixtures-1.0.0-jar-with-dependencies.jar" ]; then
    echo "Fixtures not built. Building..."
    cd "$SCRIPT_DIR/fixtures"
    mvn clean package
    cd "$SCRIPT_DIR"
    echo "Build complete."
    echo ""
fi

# Check if API server is running
echo "Checking if API server is running on http://localhost:8080..."
if ! curl -s -f -o /dev/null http://localhost:8080/health; then
    echo ""
    echo "⚠️  WARNING: API server does not appear to be running!"
    echo "Please start it with: cargo run -- serve --port 8080"
    echo ""
    read -p "Press Enter to continue anyway, or Ctrl+C to exit..."
else
    echo "✓ API server is running"
fi

echo ""
echo "Starting FitNesse on port $FITNESSE_PORT..."
echo "Access the wiki at: http://localhost:$FITNESSE_PORT/UavSwarmApi"
echo ""
echo "Press Ctrl+C to stop FitNesse"
echo ""

java -jar "$FITNESSE_JAR" -p $FITNESSE_PORT -d "$SCRIPT_DIR"
