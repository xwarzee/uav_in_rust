#include "HttpServer.hh"
#include "RestBridgePlugin.hh"

#include <ignition/common/Console.hh>
#include <ignition/math/Vector3.hh>
#include <sstream>

using namespace gazebo_plugins;

HttpServer::HttpServer(int port, RestBridgePlugin *plugin)
    : port_(port), plugin_(plugin), running_(false) {
}

HttpServer::~HttpServer() {
    Stop();
}

void HttpServer::Start() {
    running_ = true;

    // Start worker threads for async requests to Rust
    for (int i = 0; i < NUM_WORKERS; ++i) {
        workers_.emplace_back(&HttpServer::RunWorker, this);
    }

    // Start HTTP server thread
    serverThread_ = std::thread(&HttpServer::RunServer, this);

    ignmsg << "HTTP server started on port " << port_ << std::endl;
}

void HttpServer::Stop() {
    if (!running_) {
        return;
    }

    running_ = false;

    // Stop HTTP server
    if (server_) {
        server_->stop();
    }

    // Wake up and join worker threads
    queueCv_.notify_all();
    for (auto &worker : workers_) {
        if (worker.joinable()) {
            worker.join();
        }
    }

    // Join server thread
    if (serverThread_.joinable()) {
        serverThread_.join();
    }

    ignmsg << "HTTP server stopped" << std::endl;
}

void HttpServer::RunServer() {
    server_ = std::make_unique<httplib::Server>();

    // Health check endpoint
    server_->Get("/health", [this](const httplib::Request &, httplib::Response &res) {
        std::ostringstream json;
        json << "{"
             << "\"status\": \"ok\","
             << "\"sync_enabled\": " << (plugin_->IsSyncEnabled() ? "true" : "false") << ","
             << "\"drones\": [";

        auto droneNames = plugin_->GetDroneNames();
        for (size_t i = 0; i < droneNames.size(); ++i) {
            json << "\"" << droneNames[i] << "\"";
            if (i < droneNames.size() - 1) {
                json << ",";
            }
        }
        json << "]}";

        res.set_content(json.str(), "application/json");
    });

    // Start sync endpoint
    server_->Post("/start", [this](const httplib::Request &, httplib::Response &res) {
        plugin_->SetSyncEnabled(true);
        ignmsg << "Sync enabled (Gazebo → Rust)" << std::endl;
        res.set_content("{\"message\": \"Sync started\"}", "application/json");
    });

    // Stop sync endpoint
    server_->Post("/stop", [this](const httplib::Request &, httplib::Response &res) {
        plugin_->SetSyncEnabled(false);
        ignmsg << "Sync disabled" << std::endl;
        res.set_content("{\"message\": \"Sync stopped\"}", "application/json");
    });

    // Get drone states endpoint
    server_->Get("/drones/states", [this](const httplib::Request &, httplib::Response &res) {
        auto states = plugin_->GetDroneStates();

        std::ostringstream json;
        json << "{";
        size_t count = 0;
        for (const auto &[droneId, state] : states) {
            if (count > 0) json << ",";
            const auto &pose = state.first;
            const auto &vel = state.second;

            json << "\"" << droneId << "\": {"
                 << "\"position\": {"
                 << "\"x\": " << pose.Pos().X() << ","
                 << "\"y\": " << pose.Pos().Y() << ","
                 << "\"z\": " << pose.Pos().Z()
                 << "},"
                 << "\"velocity\": {"
                 << "\"vx\": " << vel.X() << ","
                 << "\"vy\": " << vel.Y() << ","
                 << "\"vz\": " << vel.Z()
                 << "}}";
            ++count;
        }
        json << "}";

        res.set_content(json.str(), "application/json");
    });

    // Receive command from Rust (POST /drones/{id}/command)
    server_->Post("/drones/:id/command", [this](const httplib::Request &req, httplib::Response &res) {
        std::string droneId = req.path_params.at("id");

        // Parse JSON body for target position
        // Expected: {"target_position": {"x": 1.0, "y": 2.0, "z": 3.0}}
        std::string body = req.body;

        // Simple JSON parsing (for production, use a proper JSON library)
        double x = 0, y = 0, z = 0;
        size_t xPos = body.find("\"x\":");
        size_t yPos = body.find("\"y\":");
        size_t zPos = body.find("\"z\":");

        if (xPos != std::string::npos && yPos != std::string::npos && zPos != std::string::npos) {
            try {
                x = std::stod(body.substr(xPos + 4));
                y = std::stod(body.substr(yPos + 4));
                z = std::stod(body.substr(zPos + 4));

                ignition::math::Vector3d targetPos(x, y, z);
                plugin_->SetDroneCommand(droneId, targetPos);

                res.set_content("{\"message\": \"Command received\"}", "application/json");
            } catch (const std::exception &e) {
                res.status = 400;
                res.set_content("{\"error\": \"Invalid JSON format\"}", "application/json");
            }
        } else {
            res.status = 400;
            res.set_content("{\"error\": \"Missing x, y, or z coordinates\"}", "application/json");
        }
    });

    // Listen on all interfaces
    ignmsg << "Starting HTTP server on 0.0.0.0:" << port_ << std::endl;
    if (!server_->listen("0.0.0.0", port_)) {
        ignerr << "Failed to start HTTP server on port " << port_ << std::endl;
    }
}

void HttpServer::RunWorker() {
    httplib::Client client("localhost", 8080);  // Connection to Rust API
    client.set_read_timeout(5, 0);  // 5 second timeout

    while (running_) {
        HttpRequest request;

        {
            std::unique_lock<std::mutex> lock(queueMutex_);
            queueCv_.wait(lock, [this] { return !requestQueue_.empty() || !running_; });

            if (!running_) {
                break;
            }

            if (requestQueue_.empty()) {
                continue;
            }

            request = requestQueue_.front();
            requestQueue_.pop();
        }

        // Extract path from full URL
        std::string path = request.url;
        size_t pathStart = path.find("/api/");
        if (pathStart != std::string::npos) {
            path = path.substr(pathStart);
        }

        // Send HTTP PUT request
        auto res = client.Put(path.c_str(), request.body, "application/json");

        if (!res) {
            ignwarn << "Failed to send state update to Rust API: " << path << std::endl;
        } else if (res->status != 200) {
            ignwarn << "Rust API returned status " << res->status << " for " << path << std::endl;
        }
    }
}

void HttpServer::SendToRust(const std::string &url, const std::string &body) {
    std::lock_guard<std::mutex> lock(queueMutex_);
    requestQueue_.push({url, body});
    queueCv_.notify_one();
}
