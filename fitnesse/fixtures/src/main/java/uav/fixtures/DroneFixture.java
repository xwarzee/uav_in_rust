package uav.fixtures;

import com.google.gson.JsonObject;

/**
 * FitNesse SLIM fixture for Drone API endpoints
 */
public class DroneFixture extends RestApiFixture {

    private String currentDroneId;

    /**
     * Public constructor required by SLIM
     */
    public DroneFixture() {
        super();
    }

    /**
     * List all drones
     */
    public String listDrones() {
        try {
            return executeGet("/api/drones");
        } catch (Exception e) {
            throw new RuntimeException("Failed to list drones: " + e.getMessage(), e);
        }
    }

    /**
     * Get details for a specific drone
     */
    public String getDroneDetail(String droneId) {
        try {
            this.currentDroneId = droneId;
            return executeGet("/api/drones/" + droneId);
        } catch (Exception e) {
            throw new RuntimeException("Failed to get drone detail: " + e.getMessage(), e);
        }
    }

    /**
     * Get status for a specific drone
     */
    public String getDroneStatus(String droneId) {
        try {
            this.currentDroneId = droneId;
            return executeGet("/api/drones/" + droneId + "/status");
        } catch (Exception e) {
            throw new RuntimeException("Failed to get drone status: " + e.getMessage(), e);
        }
    }

    /**
     * Update drone target position
     */
    public String updateDroneTarget(String droneId, double x, double y, double z) {
        try {
            this.currentDroneId = droneId;

            JsonObject target = new JsonObject();
            target.addProperty("x", x);
            target.addProperty("y", y);
            target.addProperty("z", z);

            return executePut("/api/drones/" + droneId + "/target", gson.toJson(target));
        } catch (Exception e) {
            throw new RuntimeException("Failed to update drone target: " + e.getMessage(), e);
        }
    }

    /**
     * Get drone ID from response
     */
    public String droneId() {
        return responseField("id");
    }

    /**
     * Get drone status
     */
    public String droneStatus() {
        return responseField("status");
    }

    /**
     * Get position X
     */
    public double positionX() {
        if (lastJsonResponse != null && lastJsonResponse.has("position")) {
            return lastJsonResponse.getAsJsonObject("position").get("x").getAsDouble();
        }
        return 0.0;
    }

    /**
     * Get position Y
     */
    public double positionY() {
        if (lastJsonResponse != null && lastJsonResponse.has("position")) {
            return lastJsonResponse.getAsJsonObject("position").get("y").getAsDouble();
        }
        return 0.0;
    }

    /**
     * Get position Z
     */
    public double positionZ() {
        if (lastJsonResponse != null && lastJsonResponse.has("position")) {
            return lastJsonResponse.getAsJsonObject("position").get("z").getAsDouble();
        }
        return 0.0;
    }

    /**
     * Get max speed
     */
    public String maxSpeed() {
        if (lastJsonResponse != null && lastJsonResponse.has("max_speed")) {
            return String.valueOf(lastJsonResponse.get("max_speed").getAsDouble());
        }
        return "0.0";
    }

    /**
     * Get number of drones in list response
     */
    public String numberOfDrones() {
        if (lastJsonResponse != null && lastJsonResponse.has("drones")) {
            return String.valueOf(lastJsonResponse.getAsJsonArray("drones").size());
        }
        return "0";
    }
}
