#!/bin/bash

# Start the Rust API server in the background
echo "Starting Rust API server..."
cargo run --bin api_server &
RUST_PID=$!

# Wait for the API server to start
sleep 2

# Navigate to the web UI directory
cd web_ui

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Please install Node.js to run the web UI server."
    kill $RUST_PID
    exit 1
fi

# Check if npm packages are installed
if [ ! -d "node_modules" ]; then
    echo "Installing npm packages..."
    npm install
fi

# Start the web UI server
echo "Starting web UI server..."
node server.js &
NODE_PID=$!

# Function to handle script termination
function cleanup {
    echo "Shutting down servers..."
    kill $RUST_PID
    kill $NODE_PID
    exit 0
}

# Register the cleanup function for when the script is terminated
trap cleanup SIGINT SIGTERM

echo "Both servers are running!"
echo "API server: http://localhost:3000"
echo "Web UI: http://localhost:8080"
echo "Press Ctrl+C to stop both servers"

# Keep the script running
wait 