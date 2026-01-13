package uav.fixtures;

import com.google.gson.JsonObject;

/**
 * FitNesse SLIM fixture for Formation API endpoints
 */
public class FormationFixture extends RestApiFixture {

    /**
     * Public constructor required by SLIM
     */
    public FormationFixture() {
        super();
    }

    /**
     * List available formations
     */
    public String listFormations() {
        try {
            return executeGet("/api/formations");
        } catch (Exception e) {
            throw new RuntimeException("Failed to list formations: " + e.getMessage(), e);
        }
    }

    /**
     * Get current formation
     */
    public String getCurrentFormation() {
        try {
            return executeGet("/api/formations/current");
        } catch (Exception e) {
            throw new RuntimeException("Failed to get current formation: " + e.getMessage(), e);
        }
    }

    /**
     * Set formation type
     */
    public String setFormation(String formationType) {
        try {
            JsonObject request = new JsonObject();
            request.addProperty("formation_type", formationType);

            return executePost("/api/formations/current", gson.toJson(request));
        } catch (Exception e) {
            throw new RuntimeException("Failed to set formation: " + e.getMessage(), e);
        }
    }

    /**
     * Update separation distance
     */
    public String updateSeparationDistance(double distance) {
        try {
            JsonObject request = new JsonObject();
            request.addProperty("separation_distance", distance);

            return executePut("/api/formations/separation", gson.toJson(request));
        } catch (Exception e) {
            throw new RuntimeException("Failed to update separation distance: " + e.getMessage(), e);
        }
    }

    /**
     * Get formation type from response
     */
    public String formationType() {
        return responseField("formation_type");
    }

    /**
     * Get separation distance
     */
    public double separationDistance() {
        if (lastJsonResponse != null && lastJsonResponse.has("separation_distance")) {
            return lastJsonResponse.get("separation_distance").getAsDouble();
        }
        return 0.0;
    }

    /**
     * Check if formation is stable
     */
    public String isStable() {
        return String.valueOf(responseFieldAsBoolean("is_stable"));
    }

    /**
     * Get number of available formations
     */
    public String numberOfAvailableFormations() {
        if (lastJsonResponse != null && lastJsonResponse.has("available_formations")) {
            return String.valueOf(lastJsonResponse.getAsJsonArray("available_formations").size());
        }
        return "0";
    }

    /**
     * Check if specific formation is available
     */
    public String formationAvailable(String formationType) {
        if (lastJsonResponse != null && lastJsonResponse.has("available_formations")) {
            var formations = lastJsonResponse.getAsJsonArray("available_formations");
            for (var element : formations) {
                if (element.getAsString().equals(formationType)) {
                    return "true";
                }
            }
        }
        return "false";
    }
}
