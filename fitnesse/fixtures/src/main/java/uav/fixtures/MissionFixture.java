package uav.fixtures;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

/**
 * FitNesse SLIM fixture for Mission API endpoints
 */
public class MissionFixture extends RestApiFixture {

    private String currentMissionId;

    /**
     * Public constructor required by SLIM
     */
    public MissionFixture() {
        super();
    }

    /**
     * List all missions
     */
    public String listMissions() {
        try {
            return executeGet("/api/missions");
        } catch (Exception e) {
            throw new RuntimeException("Failed to list missions: " + e.getMessage(), e);
        }
    }

    /**
     * Create MoveTo mission
     */
    public String createMoveToMission(double x, double y, double z) {
        try {
            JsonObject target = new JsonObject();
            target.addProperty("x", x);
            target.addProperty("y", y);
            target.addProperty("z", z);

            JsonObject params = new JsonObject();
            params.add("target", target);

            JsonObject mission = new JsonObject();
            mission.addProperty("type", "MoveTo");
            mission.add("params", params);

            String result = executePost("/api/missions", gson.toJson(mission));

            // Store mission ID from response
            if (lastJsonResponse != null && lastJsonResponse.has("id")) {
                currentMissionId = lastJsonResponse.get("id").getAsString();
            }
            return result;
        } catch (Exception e) {
            throw new RuntimeException("Failed to create MoveTo mission: " + e.getMessage(), e);
        }
    }

    /**
     * Create Search mission
     */
    public String createSearchMission(double centerX, double centerY, double centerZ, double radius) {
        try {
            JsonObject center = new JsonObject();
            center.addProperty("x", centerX);
            center.addProperty("y", centerY);
            center.addProperty("z", centerZ);

            JsonObject params = new JsonObject();
            params.add("center", center);
            params.addProperty("radius", radius);

            JsonObject mission = new JsonObject();
            mission.addProperty("type", "Search");
            mission.add("params", params);

            String result = executePost("/api/missions", gson.toJson(mission));

            if (lastJsonResponse != null && lastJsonResponse.has("id")) {
                currentMissionId = lastJsonResponse.get("id").getAsString();
            }
            return result;
        } catch (Exception e) {
            throw new RuntimeException("Failed to create Search mission: " + e.getMessage(), e);
        }
    }

    /**
     * Get mission details
     */
    public String getMissionDetail(String missionId) {
        try {
            this.currentMissionId = missionId;
            return executeGet("/api/missions/" + missionId);
        } catch (Exception e) {
            throw new RuntimeException("Failed to get mission detail: " + e.getMessage(), e);
        }
    }

    /**
     * Get mission status
     */
    public String getMissionStatus(String missionId) {
        try {
            this.currentMissionId = missionId;
            return executeGet("/api/missions/" + missionId + "/status");
        } catch (Exception e) {
            throw new RuntimeException("Failed to get mission status: " + e.getMessage(), e);
        }
    }

    /**
     * Cancel mission
     */
    public String cancelMission(String missionId) {
        try {
            this.currentMissionId = missionId;
            return executeDelete("/api/missions/" + missionId);
        } catch (Exception e) {
            throw new RuntimeException("Failed to cancel mission: " + e.getMessage(), e);
        }
    }

    /**
     * Get current mission ID
     */
    public String missionId() {
        return currentMissionId != null ? currentMissionId : responseField("id");
    }

    /**
     * Get mission type
     */
    public String missionType() {
        return responseField("mission_type");
    }

    /**
     * Get mission status
     */
    public String missionStatus() {
        return responseField("status");
    }

    /**
     * Get number of assigned drones
     */
    public String numberOfAssignedDrones() {
        if (lastJsonResponse != null && lastJsonResponse.has("assigned_drones")) {
            return String.valueOf(lastJsonResponse.getAsJsonArray("assigned_drones").size());
        }
        return "0";
    }

    /**
     * Get current waypoint
     */
    public String currentWaypoint() {
        int value = responseFieldAsInt("current_waypoint");
        return value >= 0 ? String.valueOf(value) : "0";
    }

    /**
     * Get total waypoints
     */
    public String totalWaypoints() {
        int value = responseFieldAsInt("total_waypoints");
        return value >= 0 ? String.valueOf(value) : "0";
    }

    /**
     * Get number of missions
     */
    public String numberOfMissions() {
        if (lastJsonResponse != null && lastJsonResponse.has("missions")) {
            return String.valueOf(lastJsonResponse.getAsJsonArray("missions").size());
        }
        return "0";
    }

    /**
     * Check if number of missions is at least the given minimum
     */
    public String numberOfMissionsAtLeast(int minimum) {
        if (lastJsonResponse != null && lastJsonResponse.has("missions")) {
            int count = lastJsonResponse.getAsJsonArray("missions").size();
            return String.valueOf(count >= minimum);
        }
        return "false";
    }
}
