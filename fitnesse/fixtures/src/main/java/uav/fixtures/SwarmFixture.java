package uav.fixtures;

/**
 * FitNesse SLIM fixture for Swarm API endpoints
 */
public class SwarmFixture extends RestApiFixture {

    /**
     * Public constructor required by SLIM
     */
    public SwarmFixture() {
        super();
    }

    /**
     * Get swarm status
     */
    public String getSwarmStatus() {
        try {
            return executeGet("/api/swarm");
        } catch (Exception e) {
            throw new RuntimeException("Failed to get swarm status: " + e.getMessage(), e);
        }
    }

    /**
     * Start simulation
     */
    public String startSimulation() {
        try {
            return executePost("/api/swarm/start", "{}");
        } catch (Exception e) {
            throw new RuntimeException("Failed to start simulation: " + e.getMessage(), e);
        }
    }

    /**
     * Stop simulation
     */
    public String stopSimulation() {
        try {
            return executePost("/api/swarm/stop", "{}");
        } catch (Exception e) {
            throw new RuntimeException("Failed to stop simulation: " + e.getMessage(), e);
        }
    }

    /**
     * Get drone count from swarm status
     */
    public String droneCount() {
        int value = responseFieldAsInt("drone_count");
        return value >= 0 ? String.valueOf(value) : "0";
    }

    /**
     * Check if simulation is running
     */
    public String simulationRunning() {
        return String.valueOf(responseFieldAsBoolean("simulation_running"));
    }

    /**
     * Check if formation is stable
     */
    public String formationStable() {
        return String.valueOf(responseFieldAsBoolean("formation_stable"));
    }

    /**
     * Get message from response
     */
    public String message() {
        return responseField("message");
    }
}
