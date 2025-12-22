# UAV Swarm System - MBSE Documentation

This directory contains **Model-Based Systems Engineering (MBSE)** documentation for the UAV Swarm Management System using **SysML v2** textual notation.

## What is MBSE?

Model-Based Systems Engineering (MBSE) is a formalized methodology that uses visual and textual modeling to support system requirements, design, analysis, and validation activities throughout the development lifecycle. MBSE provides:

- **Precise System Specifications**: Unambiguous definitions of system structure and behavior
- **Traceability**: Clear links between requirements, design, and implementation
- **Analysis Support**: Enable formal verification and simulation
- **Communication**: Shared language between stakeholders, engineers, and developers

## What is SysML v2?

**SysML v2** (Systems Modeling Language version 2) is the next-generation systems modeling language standardized by the Object Management Group (OMG). Key features:

- **Textual Notation**: Human-readable and version-control friendly syntax
- **First-Class Language**: Complete textual representation (not just a serialization)
- **Improved Semantics**: More precise execution and analysis capabilities
- **Enhanced Expressiveness**: Better support for complex systems and behaviors

SysML v2 extends and improves upon SysML v1.x with modern language features, better tool integration, and clearer semantics.

## Documentation Structure

This MBSE documentation is organized into five main models:

### 1. System Definition (`system_definition.sysml`)

**Purpose**: Defines the complete system architecture, components, interfaces, and structural relationships.

**Contains**:
- System context and boundaries
- Subsystem definitions (DroneSwarm, FormationManager, MissionExecutor)
- UAV component specifications
- Interface definitions (ports and protocols)
- Data type definitions (Position, Velocity, etc.)
- System allocations

**When to use**: Understanding overall system structure, component responsibilities, and how subsystems connect.

### 2. Requirements Model (`requirements.sysml`)

**Purpose**: Captures all system requirements in a hierarchical, traceable structure.

**Contains**:
- **Stakeholder Requirements**: High-level needs (SR_001, SR_002, etc.)
- **System Requirements**: Detailed technical requirements organized by category:
  - Navigation requirements (SYS_NAV_*)
  - Formation requirements (SYS_FORM_*)
  - Mission requirements (SYS_MISS_*)
  - State management requirements (SYS_STATE_*)
  - Performance requirements (SYS_PERF_*)
  - Safety requirements (SYS_SAFE_*)
  - Interface requirements (SYS_IF_*)
- **Traceability Links**: Requirements satisfied by specific components

**When to use**: Verifying completeness, understanding requirements rationale, tracing requirements to implementation.

### 3. Use Case Model (`use_cases.sysml`)

**Purpose**: Describes how operators interact with the system to accomplish goals.

**Contains**:
- Primary use cases:
  - Start Simulation
  - Change Formation
  - Execute Mission (MoveTo, Patrol, Search)
  - Monitor Swarm Status
- Secondary/internal use cases:
  - Continuous Formation Maintenance
  - Navigate to Position
- Error handling scenarios
- Concrete scenarios with initial/final states

**When to use**: Understanding user workflows, system behaviors from operator perspective, scenario planning.

### 4. State Machine Model (`state_machines.sysml`)

**Purpose**: Defines state-based behavior for key system components.

**Contains**:
- **Drone State Machine**: Complete lifecycle (Idle, Navigating, InFormation, ExecutingMission, Error)
- **Mission State Machine**: Mission lifecycle (NotStarted, InProgress, Completed, Failed)
- **Formation State Machine**: Formation management states
- **Simulation State Machine**: Overall simulation lifecycle
- State transitions with guards and actions

**When to use**: Understanding dynamic behavior, state transitions, error handling, temporal logic.

### 5. Activity Model (`activities.sysml`)

**Purpose**: Describes operational flows, algorithms, and concurrent activities.

**Contains**:
- Simulation Loop Activity (main control loop)
- Mission Execution Activity (waypoint navigation)
- Formation Change Activity (geometric calculations)
- Search Pattern Generation Activity (circular pattern)
- Drone Position Update Activity (physics update)
- Command Processing Activity (CLI routing)
- Status Reporting Activity (monitoring)

**When to use**: Understanding algorithms, control flow, parallelism, decision logic, operational sequences.

## How Models Relate

The models are interconnected and provide different views of the same system:

```
Requirements ─────> satisfied by ──────> System Definition
     │                                          │
     │                                          │
     └──> traces to ──> Use Cases ──> realizes ──┘
                           │
                           └──> detailed by ──> State Machines
                                                      │
                                                      └──> implemented by ──> Activities
```

- **Requirements** drive the system design
- **System Definition** shows structural solution
- **Use Cases** describe operational scenarios
- **State Machines** define component behavior
- **Activities** detail operational algorithms

## Viewing and Working with SysML v2 Models

### Text Editors

SysML v2 uses a textual notation, so any text editor works:

```bash
# View with any editor
vim doc/mbse/system_definition.sysml
code doc/mbse/requirements.sysml
```

### SysML v2 Tools

Several tools support SysML v2:

#### 1. **SysML v2 Pilot Implementation** (Open Source)
```bash
# Clone the reference implementation
git clone https://github.com/Systems-Modeling/SysML-v2-Release.git

# Run the API server
cd SysML-v2-Release
./gradlew bootRun

# Load models through REST API or web interface
```

#### 2. **Eclipse with SysML v2 Plugin**
- Install Eclipse Modeling Tools
- Add SysML v2 plugin from update site
- Import `.sysml` files as projects

#### 3. **Jupyter Notebooks (SysML v2 Kernel)**
```bash
# Install SysML v2 Jupyter kernel
pip install sysml2py

# Launch notebook
jupyter notebook

# Create notebook with SysML v2 kernel
# Load and execute .sysml files
```

### Validation and Analysis

The models can be validated using SysML v2 tools:

```bash
# Example: Validate requirements satisfaction
# (Requires SysML v2 tooling)

# Check that all requirements are satisfied
# Verify constraint satisfaction
# Analyze state machine reachability
# Simulate activity flows
```

## Reading the Models

### Understanding SysML v2 Syntax

Key syntax elements:

```sysml
// Packages organize models
package PackageName { ... }

// Parts represent components
part def ComponentName { ... }

// Attributes define properties
attribute name : Type;

// Actions define behaviors
action def ActionName { ... }

// States define state machines
state def StateMachineName { ... }

// Requirements capture needs
requirement def RequirementName { ... }

// Use cases describe scenarios
use case UseCaseName { ... }

// Connections link components
connect portA to portB;

// Constraints specify rules
assert constraint { expression }

// Traceability links
satisfy RequirementX by ComponentY;
```

### Navigation Tips

1. **Start with System Definition**: Get high-level structure
2. **Review Requirements**: Understand what system must do
3. **Explore Use Cases**: See how system is used
4. **Study State Machines**: Understand component behavior
5. **Analyze Activities**: See detailed algorithms

### Cross-References

Models reference each other using imports:

```sysml
import UAVSwarmSystem::*;           // Import all from system definition
import UAVSwarmRequirements::*;     // Import requirements
```

Follow these imports to understand dependencies.

## Relationship to Implementation

These MBSE models are **specifications** for the Rust implementation in `src/`:

| Model Element | Implementation Location |
|---------------|------------------------|
| `UAV` part | `src/drone.rs::Drone` struct |
| `FormationManagementSubsystem` | `src/formation.rs::FormationManager` |
| `MissionExecutionSubsystem` | `src/mission.rs::MissionExecutor` |
| `DroneSwarmController` | `src/swarm.rs::DroneSwarm` |
| `DroneStateMachine` | `src/drone.rs::DroneStatus` enum |
| `SimulationLoopActivity` | `src/swarm.rs::start_simulation()` |

The models provide:
- **What** the system does (requirements)
- **How** it's structured (system definition)
- **Why** design choices were made (use cases)
- **When** things happen (state machines)
- **In what order** (activities)

The Rust code provides:
- **Executable implementation** of the models
- **Performance optimizations**
- **Concrete data structures**

## Model Maintenance

When updating the system:

1. **Requirements change** → Update `requirements.sysml` first
2. **Architecture change** → Update `system_definition.sysml`
3. **Behavior change** → Update `state_machines.sysml` and `activities.sysml`
4. **User interaction change** → Update `use_cases.sysml`
5. **Implementation** → Update Rust code in `src/`
6. **Verify** → Check traceability links and constraints

Keep models synchronized with implementation!

## Benefits of MBSE for UAV Swarm

For this project, MBSE provides:

1. **Precise Requirements**: Clear specifications for autonomous behaviors
2. **Safety Analysis**: Formal verification of state transitions and constraints
3. **Complexity Management**: Structured view of multi-drone coordination
4. **Concurrency Analysis**: Clear representation of parallel activities
5. **Traceability**: Requirements linked to implementation
6. **Communication**: Shared understanding between team members
7. **Documentation**: Always-current system specifications

## Additional Resources

### SysML v2 Resources
- [SysML v2 Specification](https://www.omgsysml.org/SysML-2.htm)
- [SysML v2 Submission](https://github.com/Systems-Modeling/SysML-v2-Release)
- [SysML v2 API & Services](https://github.com/Systems-Modeling/SysML-v2-API-Services)

### MBSE Resources
- [INCOSE MBSE Initiative](https://www.incose.org/products-and-publications/incose-mbse-initiative)
- [NASA Systems Engineering Handbook](https://www.nasa.gov/seh/)

### UAV Swarm References
- Project architecture: `../software/ARCHITECTURE.md`
- Software implementation: `../../src/`
- Build instructions: `../../README.md`

## Questions?

For questions about:
- **SysML v2 syntax**: See SysML v2 specification
- **Model content**: Review model documentation blocks
- **Implementation**: See `../software/ARCHITECTURE.md` and source code
- **Requirements**: See `requirements.sysml` with traceability links

---

**Document Version**: 1.0
**Last Updated**: 2025-12-22
**Model Language**: SysML v2 (OMG Standard)
**Implementation Language**: Rust
**Maintainer**: Development Team
