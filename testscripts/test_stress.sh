#!/bin/bash
# filepath: testscripts/test_stress.sh

echo "Running TCP stress test..."

for i in $(seq 1 1000); do
    nc -z 127.0.0.1 8080 &
done

echo "TCP stress test completed"