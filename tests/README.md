# UAV Swarm System - Test Suite Documentation

This directory contains **two distinct test suites** organized in separate subdirectories:

1. **Software Tests** (`tests/software/`) - Traditional unit and integration tests
2. **MBSE Traceability Tests** (`tests/mbse/`) - Tests validating MBSE ↔ Software alignment

## 🎯 Quick Start

```bash
# Run ALL tests (Software + MBSE)
cargo test --tests

# Run ONLY Software tests
cargo test --test software

# Run ONLY MBSE traceability tests
cargo test --test mbse
```

---

## 📦 Suite 1: Software Tests (Pure Software Validation)

### Purpose
Traditional software testing focused on **code correctness, edge cases, and internal logic** WITHOUT reference to MBSE specifications.

### Test Files

| Test File | Purpose | Test Count |
|-----------|---------|------------|
| `software/unit_tests.rs` | Unit tests for individual components | 34 tests |
| `software/integration_tests.rs` | Integration tests for component interactions | 13 tests |

**Directory Structure:**
```
tests/
├── software.rs              # Entry point for software tests
└── software/
    ├── unit_tests.rs        # Unit tests
    └── integration_tests.rs # Integration tests
```

### Test Categories

#### Unit Tests (`software/unit_tests.rs`)
- ✅ **Position** - Distance, add, subtract, normalize, scale operations
- ✅ **Velocity** - Magnitude calculations, zero velocity
- ✅ **Drone** - Creation, movement, state transitions
- ✅ **FormationManager** - Formation types, separation distance
- ✅ **Edge Cases** - Large coordinates, negative values, zero speeds
- ✅ **Precision** - Floating-point accuracy, calculation consistency

#### Integration Tests (`software/integration_tests.rs`)
- ✅ **Multi-Drone** - Independent drone movements
- ✅ **Formation Transitions** - Triangle → Line → V-Formation
- ✅ **Dynamic Reconfiguration** - Real-time formation changes
- ✅ **Moving Leader** - Formation following moving leader
- ✅ **Stress Tests** - Many updates, large swarms, rapid changes
- ✅ **Realistic Scenarios** - Assembly, altitude maintenance, collision avoidance

### Running Software Tests

```bash
# All software tests
cargo test --test software

# Only unit tests
cargo test --test software unit_tests::

# Only integration tests
cargo test --test software integration_tests::

# With verbose output
cargo test --test software -- --nocapture

# Run specific test
cargo test --test software unit_tests::test_position_distance_calculation
```

### Expected Results
```
running 40 tests (unit tests)
test result: ok. 40 passed; 0 failed

running 15 tests (integration tests)
test result: ok. 15 passed; 0 failed
```

---

## 🔗 Suite 2: MBSE Traceability Tests (MBSE ↔ Software Validation)

### Purpose
Validates the **complete traceability** between MBSE model and software implementation, ensuring all requirements, components, and constraints are properly implemented.

### Test Files

| Test File | Purpose | Test Count | Coverage |
|-----------|---------|------------|----------|
| `mbse/component_mapping_tests.rs` | Validates MBSE components → Software modules mapping | 10 tests | Components & Data Types |
| `mbse/requirements_validation_tests.rs` | Validates implementation of system requirements | 12 tests | All Requirements |
| `mbse/safety_constraints_tests.rs` | Validates safety-critical constraints | 11 tests | Safety Requirements |
| `mbse/traceability_matrix_tests.rs` | Complete traceability matrices and reports | 9 tests | Traceability & Coverage |

**Directory Structure:**
```
tests/
├── mbse.rs                           # Entry point for MBSE tests
└── mbse/
    ├── component_mapping_tests.rs    # Component mapping tests
    ├── requirements_validation_tests.rs  # Requirements tests
    ├── safety_constraints_tests.rs    # Safety tests
    └── traceability_matrix_tests.rs  # Traceability tests
```

**Total: 42 tests** providing comprehensive MBSE → Software validation

### Test Categories

#### 1. Component Mapping Tests
Validates MBSE SysML v2 components map to Rust modules:
- `UAVSwarmManagementSystem` → `main.rs` + `swarm.rs`
- `DroneSwarmController` → `swarm.rs::DroneSwarm`
- `FormationManagementSubsystem` → `formation.rs::FormationManager`
- `MissionExecutionSubsystem` → `mission.rs::MissionExecutor`
- `UAV` → `drone.rs::Drone`

#### 2. Requirements Validation Tests
- **Navigation (SYS_NAV_*)**: Autonomous navigation, speed constraints, arrival detection
- **Formation (SYS_FORM_*)**: Triangle/Line/V geometries, stability, separation
- **State Management (SYS_STATE_*)**: State machine, transitions
- **Performance (SYS_PERF_*)**: 10 Hz update rate, delta-time accuracy

#### 3. Safety Constraints Tests (CRITICAL)
- **SYS_SAFE_001**: Minimum altitude ≥ 0m (ground collision prevention)
- **SYS_SAFE_002**: Maximum altitude ≤ 100m (airspace compliance)
- **SYS_SAFE_003**: Formation spacing ≥ 5m (collision avoidance)
- **SYS_NAV_002**: Max speed ≤ 5.0 m/s (control authority)

#### 4. Traceability Matrix Tests
- Requirements → Components mapping
- Use Cases → Requirements traceability
- State Machines → Requirements traceability
- Requirements coverage analysis (100% coverage)
- MBSE ↔ Software architecture consistency

### Running MBSE Tests

```bash
# All MBSE traceability tests
cargo test --test mbse

# Individual test suites
cargo test --test mbse component_mapping_tests::
cargo test --test mbse requirements_validation_tests::
cargo test --test mbse safety_constraints_tests::
cargo test --test mbse traceability_matrix_tests::

# Generate complete traceability report
cargo test --test mbse traceability_matrix_tests::test_complete_traceability_report -- --nocapture

# Requirements coverage analysis
cargo test --test mbse traceability_matrix_tests::test_requirements_coverage_analysis -- --nocapture
```

### Expected Results
```
running 10 tests (component mapping)
test result: ok. 10 passed; 0 failed

running 12 tests (requirements validation)
test result: ok. 12 passed; 0 failed

running 11 tests (safety constraints)
test result: ok. 11 passed; 0 failed

running 9 tests (traceability)
test result: ok. 9 passed; 0 failed
```

---

## 📊 Complete Test Summary

| Test Suite | Test Files | Test Count | Purpose |
|------------|-----------|------------|---------|
| **Software Tests** | 2 files | ~55 tests | Code correctness & behavior |
| **MBSE Tests** | 4 files | 42 tests | MBSE ↔ Software traceability |
| **TOTAL** | **6 files** | **~97 tests** | Complete validation |

---

## 🚀 Common Commands

### Run Everything
```bash
# All tests (Software + MBSE)
cargo test --tests

# With verbose output
cargo test --tests -- --nocapture
```

### Run by Type
```bash
# Only Software tests (unit + integration)
cargo test --test software

# Only MBSE tests (all 4 suites)
cargo test --test mbse
```

### Run Specific Tests
```bash
# Software: specific test by name
cargo test --test software unit_tests::test_position_distance_calculation

# MBSE: specific requirement test
cargo test --test mbse requirements_validation_tests::test_sys_nav_001

# MBSE: specific safety test
cargo test --test mbse safety_constraints_tests::test_sys_safe_001
```

### Reports
```bash
# Full traceability report
cargo test test_complete_traceability_report -- --nocapture

# Requirements coverage
cargo test test_requirements_coverage_analysis -- --nocapture

# Safety constraints summary
cargo test test_safety_constraints_documentation -- --nocapture
```

---

## 🎯 When to Use Which Suite?

### Use **Software Tests** when:
- ✅ Developing new features
- ✅ Fixing bugs
- ✅ Refactoring code
- ✅ Testing edge cases
- ✅ Verifying internal logic
- ✅ Performance testing
- ✅ Standard software development workflow

### Use **MBSE Tests** when:
- ✅ Validating MBSE model implementation
- ✅ Verifying requirements compliance
- ✅ Checking safety constraints
- ✅ Generating traceability reports
- ✅ Preparing for reviews/audits
- ✅ After MBSE model changes
- ✅ Before releases (V&V phase)

---

## 📋 Requirements Coverage

### Software Tests
- **Coverage**: Internal implementation details
- **Focus**: Code correctness, edge cases, integration
- **Type**: Traditional software testing

### MBSE Tests
- **Coverage**: 23 system requirements (100%)
- **Focus**: MBSE ↔ Software alignment
- **Type**: Model-based verification

| Requirement Category | Total | Covered | Status |
|---------------------|-------|---------|--------|
| Navigation (SYS_NAV_*) | 3 | 3 | ✅ 100% |
| Formation (SYS_FORM_*) | 5 | 5 | ✅ 100% |
| State Management (SYS_STATE_*) | 3 | 3 | ✅ 100% |
| Performance (SYS_PERF_*) | 2 | 2 | ✅ 100% |
| Safety (SYS_SAFE_*) | 3 | 3 | ✅ 100% |
| Interface (SYS_IF_*) | 2 | 2 | ✅ 100% |
| Stakeholder (SR_*) | 5 | 5 | ✅ 100% |
| **TOTAL** | **23** | **23** | **✅ 100%** |

---

## 🔍 Key Differences

| Aspect | Software Tests | MBSE Tests |
|--------|---------------|------------|
| **Purpose** | Code correctness | MBSE compliance |
| **Reference** | Internal implementation | MBSE documentation |
| **Naming** | `software_*` | `mbse_*` |
| **Documentation** | Code comments | MBSE references |
| **Frequency** | Every commit | Before releases, after MBSE changes |
| **Audience** | Developers | Systems engineers, auditors |
| **Coverage** | Implementation details | Requirements & constraints |

---

## 📚 Key References

### For Software Tests
- `src/drone.rs` - Drone implementation
- `src/formation.rs` - Formation logic
- `src/mission.rs` - Mission execution
- `src/swarm.rs` - Swarm coordination

### For MBSE Tests
- `doc/mbse/MBSE_ARCHITECTURE.md` - Complete MBSE documentation
- `doc/mbse/system_definition.sysml` - System structure (SysML v2)
- `doc/mbse/requirements.sysml` - Requirements specification
- `doc/software/ARCHITECTURE.md` - Software architecture

---

## 🔧 Continuous Integration

### Recommended CI Pipeline

```yaml
# Run on every commit
- Software unit tests (fast feedback)
- Software integration tests

# Run on pull requests
- All software tests
- MBSE component mapping tests

# Run before release
- All tests (Software + MBSE)
- Generate traceability report
- Verify 100% requirements coverage
```

### CI Commands
```bash
# Fast feedback (unit tests only)
cargo test --test software unit_tests::

# Full validation (all tests)
cargo test --tests --release

# Generate reports for documentation
cargo test --test mbse traceability_matrix_tests::test_complete_traceability_report -- --nocapture > traceability_report.txt
```

---

## ✨ Success Criteria

### Software Tests ✅
- All unit tests pass
- All integration tests pass
- Edge cases covered
- Performance acceptable

### MBSE Tests ✅
- 42/42 tests passing
- 100% requirements coverage
- All safety constraints verified
- Traceability matrices complete
- No MBSE → Software gaps

---

## 🛠️ Extending the Test Suites

### Adding Software Tests
1. Add test to `tests/software/unit_tests.rs` (for unit tests)
2. Add test to `tests/software/integration_tests.rs` (for integration tests)
3. Follow existing test patterns
4. Run tests to verify: `cargo test --test software`

### Adding MBSE Tests
1. **First** update MBSE model (`doc/mbse/*.sysml`)
2. Add requirement to MBSE documentation
3. Add test to appropriate file in `tests/mbse/`
4. Update traceability matrices
5. Verify 100% coverage maintained: `cargo test --test mbse`

---

## 📞 Support

- **Software test issues**: Check test output and implementation
- **MBSE test failures**: Review MBSE documentation and requirements
- **Traceability gaps**: See `doc/mbse/MBSE_ARCHITECTURE.md` Section 8
- **New requirements**: Update MBSE model first, then add tests

---

**Last Updated**: 2026-01-06
**Software Tests**: ~55 tests ✅
**MBSE Tests**: 42 tests ✅
**Total Test Coverage**: ~97 tests ✅
**Requirements Coverage**: 100% ✅
