#!/bin/bash

echo "Running concurrent TCP echo test..."

for i in $(seq 1 5); do
    (
        echo "Client $i connecting..."
        echo "Hello from client $i" | nc 127.0.0.1 8080 &
        sleep 0.1
    ) &
done

wait
echo "Concurrent TCP test completed"