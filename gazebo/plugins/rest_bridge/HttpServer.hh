#ifndef HTTP_SERVER_HH
#define HTTP_SERVER_HH

#include <httplib.h>
#include <thread>
#include <memory>
#include <atomic>
#include <queue>
#include <mutex>
#include <condition_variable>
#include <string>

namespace gazebo_plugins {

// Forward declaration
class RestBridgePlugin;

struct HttpRequest {
    std::string url;
    std::string body;
};

class HttpServer {
public:
    HttpServer(int port, RestBridgePlugin *plugin);
    ~HttpServer();

    void Start();
    void Stop();

    // Async send to Rust API (queued in thread pool)
    void SendToRust(const std::string &url, const std::string &body);

private:
    void RunServer();
    void RunWorker();

    int port_;
    RestBridgePlugin *plugin_;
    std::unique_ptr<httplib::Server> server_;
    std::thread serverThread_;
    std::atomic<bool> running_;

    // Thread pool for async HTTP requests to Rust
    static const int NUM_WORKERS = 4;
    std::vector<std::thread> workers_;
    std::queue<HttpRequest> requestQueue_;
    std::mutex queueMutex_;
    std::condition_variable queueCv_;
};

}  // namespace gazebo_plugins

#endif  // HTTP_SERVER_HH
