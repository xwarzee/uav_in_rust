package uav.fixtures;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import org.apache.hc.client5.http.classic.methods.*;
import org.apache.hc.client5.http.impl.classic.CloseableHttpClient;
import org.apache.hc.client5.http.impl.classic.HttpClients;
import org.apache.hc.core5.http.io.entity.EntityUtils;
import org.apache.hc.core5.http.io.entity.StringEntity;

import java.io.IOException;

/**
 * Base fixture for REST API testing with FitNesse SLIM
 */
public class RestApiFixture {
    protected static final String BASE_URL = System.getProperty("api.base.url", "http://localhost:8080");
    protected final Gson gson = new Gson();
    protected CloseableHttpClient httpClient;

    protected String lastResponse;
    protected int lastStatusCode;
    protected JsonObject lastJsonResponse;

    /**
     * Public constructor required by SLIM
     */
    public RestApiFixture() {
        // Initialize HTTP client lazily to avoid issues in constructor
    }

    /**
     * Get or create HTTP client
     */
    protected CloseableHttpClient getHttpClient() {
        if (httpClient == null) {
            httpClient = HttpClients.createDefault();
        }
        return httpClient;
    }

    /**
     * Execute GET request
     */
    protected String executeGet(String endpoint) throws IOException {
        HttpGet request = new HttpGet(BASE_URL + endpoint);
        request.setHeader("Accept", "application/json");

        getHttpClient().execute(request, response -> {
            lastStatusCode = response.getCode();
            lastResponse = EntityUtils.toString(response.getEntity());

            if (lastResponse != null && !lastResponse.isEmpty()) {
                try {
                    lastJsonResponse = gson.fromJson(lastResponse, JsonObject.class);
                } catch (Exception e) {
                    lastJsonResponse = null;
                }
            }
            return null;
        });
        return "OK";
    }

    /**
     * Execute POST request
     */
    protected String executePost(String endpoint, String jsonBody) throws IOException {
        HttpPost request = new HttpPost(BASE_URL + endpoint);
        request.setHeader("Content-Type", "application/json");
        request.setHeader("Accept", "application/json");

        if (jsonBody != null && !jsonBody.isEmpty()) {
            request.setEntity(new StringEntity(jsonBody));
        }

        getHttpClient().execute(request, response -> {
            lastStatusCode = response.getCode();
            lastResponse = EntityUtils.toString(response.getEntity());

            if (lastResponse != null && !lastResponse.isEmpty()) {
                try {
                    lastJsonResponse = gson.fromJson(lastResponse, JsonObject.class);
                } catch (Exception e) {
                    lastJsonResponse = null;
                }
            }
            return null;
        });
        return "OK";
    }

    /**
     * Execute PUT request
     */
    protected String executePut(String endpoint, String jsonBody) throws IOException {
        HttpPut request = new HttpPut(BASE_URL + endpoint);
        request.setHeader("Content-Type", "application/json");
        request.setHeader("Accept", "application/json");

        if (jsonBody != null && !jsonBody.isEmpty()) {
            request.setEntity(new StringEntity(jsonBody));
        }

        getHttpClient().execute(request, response -> {
            lastStatusCode = response.getCode();
            lastResponse = EntityUtils.toString(response.getEntity());

            if (lastResponse != null && !lastResponse.isEmpty()) {
                try {
                    lastJsonResponse = gson.fromJson(lastResponse, JsonObject.class);
                } catch (Exception e) {
                    lastJsonResponse = null;
                }
            }
            return null;
        });
        return "OK";
    }

    /**
     * Execute DELETE request
     */
    protected String executeDelete(String endpoint) throws IOException {
        HttpDelete request = new HttpDelete(BASE_URL + endpoint);
        request.setHeader("Accept", "application/json");

        getHttpClient().execute(request, response -> {
            lastStatusCode = response.getCode();
            lastResponse = EntityUtils.toString(response.getEntity());

            if (lastResponse != null && !lastResponse.isEmpty()) {
                try {
                    lastJsonResponse = gson.fromJson(lastResponse, JsonObject.class);
                } catch (Exception e) {
                    lastJsonResponse = null;
                }
            }
            return null;
        });
        return "OK";
    }

    /**
     * Get HTTP status code from last request
     */
    public String statusCode() {
        return String.valueOf(lastStatusCode);
    }

    /**
     * Get response body from last request
     */
    public String responseBody() {
        return lastResponse;
    }

    /**
     * Check if response contains a field
     */
    public String responseContains(String fieldName) {
        boolean contains = lastJsonResponse != null && lastJsonResponse.has(fieldName);
        return String.valueOf(contains);
    }

    /**
     * Get value from response JSON
     */
    public String responseField(String fieldName) {
        if (lastJsonResponse == null || !lastJsonResponse.has(fieldName)) {
            return null;
        }

        JsonElement element = lastJsonResponse.get(fieldName);
        if (element.isJsonPrimitive()) {
            return element.getAsString();
        }
        return element.toString();
    }

    /**
     * Get integer value from response JSON
     */
    public int responseFieldAsInt(String fieldName) {
        if (lastJsonResponse == null || !lastJsonResponse.has(fieldName)) {
            return -1;
        }
        return lastJsonResponse.get(fieldName).getAsInt();
    }

    /**
     * Get boolean value from response JSON
     */
    public boolean responseFieldAsBoolean(String fieldName) {
        if (lastJsonResponse == null || !lastJsonResponse.has(fieldName)) {
            return false;
        }
        return lastJsonResponse.get(fieldName).getAsBoolean();
    }

    /**
     * Get message from response (common method for all fixtures)
     */
    public String message() {
        return responseField("message");
    }

    /**
     * Clean up resources
     */
    public void close() {
        try {
            if (httpClient != null) {
                httpClient.close();
            }
        } catch (IOException e) {
            // Ignore
        }
    }
}
