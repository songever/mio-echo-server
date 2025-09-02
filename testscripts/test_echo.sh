echo "Running TCPstream echo test..."

for i in $(seq 1 3); do
    echo "Hello, Mio!" | nc -q 1 127.0.0.1 8080 &
done

echo "TCPstream echo test completed"