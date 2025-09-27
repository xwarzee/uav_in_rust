#!/bin/bash

# Initialize and demonstrate capabilities
cargo run -- start &

# Change to V-formation
cargo run -- formation v_formation &

# Execute coordinate mission
cargo run -- mission 200.0 100.0 30.0
