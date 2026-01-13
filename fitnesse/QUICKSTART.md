# FitNesse Quick Start Guide

## 3-Minute Setup

### Step 1: Start the API Server
```bash
cd /path/to/uav_in_rust
cargo run -- serve --port 8080
```
Keep this terminal open.

### Step 2: Build Fixtures (First Time Only)
```bash
cd fitnesse/fixtures
mvn clean package
```

### Step 3: Run FitNesse
**macOS/Linux:**
```bash
cd ..
./run-fitnesse.sh
```

**Windows:**
```cmd
cd ..
run-fitnesse.bat
```

### Step 4: Run Tests
1. Open browser: http://localhost:8000/UavSwarmApi
2. Click "Suite" button to run all tests
3. Or click individual test suites (SwarmTests, DroneTests, etc.)

## What Gets Tested?

| Test Suite | Tests | Coverage |
|------------|-------|----------|
| SwarmTests | 5 | Swarm status, start/stop simulation |
| DroneTests | 7 | List drones, get details, update targets |
| FormationTests | 10 | List/change formations, update separation |
| MissionTests | 10 | Create missions (MoveTo/Patrol/Search), status, cancel |

**Total: 35+ tests** covering all REST API endpoints

## Interpreting Results

- ✅ **Green** - Test passed
- ❌ **Red** - Test failed
- ⚠️ **Yellow** - Test ignored/skipped

## Common Issues

### "Connection refused"
→ API server not running. Start with: `cargo run -- serve --port 8080`

### "Fixtures not found"
→ Build them: `cd fitnesse/fixtures && mvn clean package`

### "Port 8000 already in use"
→ Change port: `FITNESSE_PORT=9000 ./run-fitnesse.sh`

## File Locations

```
fitnesse/
├── run-fitnesse.sh         # Launch script
├── FitNesseRoot/
│   └── UavSwarmApi/        # Test pages (click to edit)
└── fixtures/
    ├── pom.xml             # Maven config
    └── src/main/java/      # Fixture code
```

## Next Steps

1. **View Tests**: Browse http://localhost:8000/UavSwarmApi
2. **Edit Tests**: Click "Edit" on any test page
3. **Add Tests**: Click "Add Child" to create new test pages
4. **Modify Fixtures**: Edit Java files in `fixtures/src/main/java/`

## Tips

- **Test Syntax**: FitNesse uses tables. See existing tests for examples.
- **Debugging**: Check browser console and FitNesse logs
- **Auto-refresh**: Tests can be re-run without restarting FitNesse
- **Documentation**: Full docs in [README.md](README.md)

## Example Test Table

```
|script|swarm fixture         |
|get swarm status             |
|check|status code      |200  |
|check|drone count      |3    |
```

This:
1. Creates a SwarmFixture instance
2. Calls `getSwarmStatus()` method
3. Checks status code is 200
4. Checks drone count is 3

Easy! 🎉
