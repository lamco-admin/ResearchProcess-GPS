#!/bin/bash
# Full real-time test using wscat

echo "Full Real-time WebSocket Test"
echo "============================="
echo ""

# Check if wscat is available
if ! command -v wscat &> /dev/null; then
    echo "wscat not found. Installing with npm..."
    npm install -g wscat
fi

# Create a test script for wscat
cat > /tmp/ws_test.txt << 'EOF'
{"id":"auth-1","type":"auth","token":"test-token"}
{"id":"sub-1","type":"subscribe","subscription_type":"entity","params":{"entity_id":"019861b7-8646-7f33-891a-d5706a5679ff"}}
EOF

echo "1. Starting WebSocket client in background..."
(
    wscat -c ws://localhost:8080/api/v1/ws < /tmp/ws_test.txt 2>&1 | while IFS= read -r line; do
        echo "   WS: $line"
    done
) &
WS_PID=$!

# Wait for connection
sleep 2

echo ""
echo "2. Creating entity via REST API..."
RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "Real-time test",
      "hypothesis": "WebSocket will receive notifications",
      "status": "EXPLORING"
    }
  }')

ENTITY_ID=$(echo "$RESPONSE" | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "   Created entity: $ENTITY_ID"

# Wait for events
sleep 2

echo ""
echo "3. Updating entity..."
curl -s -X PUT "http://localhost:8080/api/v1/entities/$ENTITY_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "status": "TESTING",
      "conclusion": "It works!"
    }
  }' > /dev/null
echo "   Updated entity"

# Wait for events
sleep 2

echo ""
echo "4. Checking server logs for event activity..."
tail -20 server.log | grep -i "event\|websocket" | tail -10

# Cleanup
kill $WS_PID 2>/dev/null
rm -f /tmp/ws_test.txt

echo ""
echo "Test completed!"