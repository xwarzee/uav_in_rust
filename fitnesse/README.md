# UAV Swarm API - FitNesse Test Suite

This directory contains FitNesse acceptance tests for the UAV Swarm REST API.

## Structure

```
fitnesse/
├── FitNesseRoot/          # FitNesse wiki pages
│   └── UavSwarmApi/       # Test suite root
│       ├── SwarmTests/    # Swarm management tests
│       ├── DroneTests/    # Drone operation tests
│       ├── FormationTests/# Formation control tests
│       └── MissionTests/  # Mission execution tests
├── fixtures/              # Java fixtures for API testing
│   ├── pom.xml           # Maven project file
│   └── src/main/java/    # Java source code
├── run-fitnesse.sh       # Launch script (Unix/Mac)
└── run-fitnesse.bat      # Launch script (Windows)
```

## 📋 Prerequisites

1. **Java 11 or later** - Required for Maven and FitNesse
2. **Maven** - For building Java fixtures
3. **Rust API Server** - Must be running on http://localhost:8080

## 🚀 Quick Start

### 1. Build the Rust API Server

```bash
cd /Users/scrumconseil/dev/claudecode/uav_in_rust
cargo build --release
```

### 2. Start the API Server

```bash
cargo run -- serve --port 8080
```

Leave this running in a terminal window.

### 3. Build Java Fixtures

```bash
cd fitnesse/fixtures
mvn clean package
```

This will download dependencies and build the fixture JAR.

### 4. Run FitNesse

**On Unix/Mac:**
```bash
cd fitnesse
./run-fitnesse.sh
```

**On Windows:**
```cmd
cd fitnesse
run-fitnesse.bat
```

FitNesse will start on http://localhost:8000

### 5. Run Tests

Open your browser to: **http://localhost:8000/UavSwarmApi**

Click on any test suite to run:
- **SwarmTests** - Tests swarm management
- **DroneTests** - Tests individual drone operations
- **FormationTests** - Tests formation control
- **MissionTests** - Tests mission execution

Or run all tests with the "Suite" button on the main page.

## Test Coverage

The FitNesse tests cover the same scenarios as the curl examples:

### Swarm Tests (5 tests)
- Get swarm status
- Start simulation
- Stop simulation
- Verify swarm status fields
- Complete simulation lifecycle

### Drone Tests (7 tests)
- List all drones
- Get specific drone details
- Get drone status
- Update drone target position
- Verify all drones exist
- Invalid drone ID returns 404
- Multiple target updates

### Formation Tests (10 tests)
- List available formations
- Get current formation
- Change to triangle formation
- Change to line formation
- Change to V-formation
- Update separation distance
- Invalid formation type
- Invalid separation distance (too small)
- Invalid separation distance (too large)
- Formation lifecycle

### Mission Tests (10 tests)
- List missions
- Create MoveTo mission
- Get mission details
- Get mission status
- Create Search mission
- Cancel mission
- Multiple missions
- Invalid mission
- Get non-existent mission
- Mission lifecycle

## Running the Tests

### 1. Start the API Server

```bash
cd /path/to/uav_in_rust
cargo run -- serve --port 8080
```

### 2. Build the Fixtures (First Time Only)

```bash
cd fitnesse/fixtures
mvn clean package
```

### 3. Run FitNesse

**On macOS/Linux:**
```bash
cd fitnesse
./run-fitnesse.sh
```

**On Windows:**
```bash
cd fitnesse
run-fitnesse.bat
```

### 4. Access the Tests

Open your browser and navigate to:
```
http://localhost:8000/UavSwarmApi
```

### 5. Run Tests

- Click on any test suite (SwarmTests, DroneTests, etc.)
- Click the "Test" button to run individual tests
- Click "Suite" button on the main page to run all tests

## Project Structure

```
uav_in_rust/
├── src/                          # Rust API implementation
├── fitnesse/                     # FitNesse test suite
│   ├── FitNesseRoot/
│   │   └── UavSwarmApi/         # Test pages
│   │       ├── SwarmTests/      # Swarm management tests
│   │       ├── DroneTests/      # Drone operation tests
│   │       ├── FormationTests/  # Formation control tests
│   │       └── MissionTests/    # Mission execution tests
│   ├── fixtures/                # Java fixtures
│   │   ├── pom.xml             # Maven configuration
│   │   └── src/main/java/
│   │       └── uav/fixtures/   # Fixture implementations
│   │           ├── RestApiFixture.java
│   │           ├── SwarmFixture.java
│   │           ├── DroneFixture.java
│   │           ├── FormationFixture.java
│   │           └── MissionFixture.java
│   ├── run-fitnesse.sh         # Launch script (Unix)
│   ├── run-fitnesse.bat        # Launch script (Windows)
│   └── README.md               # This file
└── Cargo.toml
```

## Fixtures Overview

### RestApiFixture (Base Class)
- HTTP GET, POST, PUT, DELETE operations
- JSON response parsing
- Common assertion methods

### SwarmFixture
- Get swarm status
- Start/stop simulation
- Check drone count, simulation state, formation stability

### DroneFixture
- List all drones
- Get drone details and status
- Update drone target positions
- Validate drone count

### FormationFixture
- List available formations
- Get/set current formation
- Update separation distance
- Validate formation types

### MissionFixture
- Create missions (MoveTo, Patrol, Search)
- Get mission details and status
- Cancel missions
- Wait for mission completion

## Configuration

### API Base URL
By default, tests connect to `http://localhost:8080`.

To override, set the system property:
```bash
java -Dapi.base.url=http://your-server:port -jar fitnesse-standalone.jar
```

### FitNesse Port
By default, FitNesse runs on port 8000.

To override:
```bash
export FITNESSE_PORT=9000  # Unix
set FITNESSE_PORT=9000     # Windows
```

## Troubleshooting

### Tests Failing with Connection Errors
- Ensure the Rust API server is running on port 8080
- Check with: `curl http://localhost:8080/health`

### Fixtures Not Found
- Build the fixtures: `cd fitnesse/fixtures && mvn clean package`
- Verify JAR exists: `fixtures/target/fitnesse-fixtures-1.0.0-jar-with-dependencies.jar`

### FitNesse Won't Start
- Ensure Java 11+ is installed: `java -version`
- Check if port 8000 is available
- Download FitNesse JAR if missing (script will prompt)

## Development

### Adding New Tests
1. Create a new test page in `FitNesseRoot/UavSwarmApi/`
2. Use existing fixtures or create new ones in `fixtures/src/main/java/`
3. Rebuild fixtures if modified: `mvn clean package`
4. Refresh FitNesse page

### Modifying Fixtures
1. Edit Java files in `fixtures/src/main/java/uav/fixtures/`
2. Rebuild: `cd fixtures && mvn clean package`
3. Restart FitNesse to pick up changes

## Test Coverage

The FitNesse test suite provides comprehensive coverage equivalent to the following curl commands:

- **35+ test cases** across 4 test suites
- All REST endpoints covered
- Success and error scenarios
- Complete lifecycle tests

This provides an automated, maintainable alternative to manual curl testing with:
- Easy-to-read test documentation
- Automated regression testing
- Team collaboration via wiki pages
- Clear pass/fail reporting
