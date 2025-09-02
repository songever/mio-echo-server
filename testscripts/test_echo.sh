echo "Running TCPstream echo test..."

for i in $(seq 1 5); do
    echo "Hello, Mio!" | nc 127.0.0.1 8080 &
done

wait
echo "TCPstream echo test completed"