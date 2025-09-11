#!/bin/sh

PORT=${1:-8080}

echo "Starting web server on port $PORT"
echo "Access: http://localhost:$PORT"
echo ""

python3 -m http.server "$PORT" -d www
