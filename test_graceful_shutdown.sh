#!/bin/bash

echo "Starting 30 requests with delays to simulate in-flight requests..."
echo "Press Ctrl+C on the server in 3 seconds"

# Send requests with small delays to keep them in-flight
for i in {1..30}; do
    # Use timeout to simulate slow endpoints (if you have a slow endpoint, use that instead)
    curl -s --max-time 5 "http://localhost:3000/api/products" > /dev/null 2>&1 &
    echo "Request $i sent"
    sleep 0.2
done

echo "All requests sent. Server should now be shutting down gracefully..."
wait
echo "All requests completed"
