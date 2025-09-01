#!/bin/bash
# filepath: test_basic_connect.sh

# 简单的netcat连接测试
echo "Testing basic connection to server..."
nc -zv 127.0.0.1 8080
if [ $? -eq 0 ]; then
    echo "Connection successful!"
else 
    echo "Connection failed!"
fi

