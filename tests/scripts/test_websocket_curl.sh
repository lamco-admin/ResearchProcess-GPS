#!/bin/bash
# Test WebSocket real-time events using curl and manual REST calls

echo "Testing WebSocket Real-time Events"
echo "================================="
echo ""

# Create a unique entity ID for testing
TEST_ID=$(uuidgen)
echo "Test entity ID: $TEST_ID"

# Start WebSocket connection in background and log output
echo ""
echo "1. Starting WebSocket listener..."
(
    # Use curl to establish WebSocket connection
    # Note: This is a simplified test - real WebSocket needs proper client
    echo "Attempting WebSocket connection..."
    timeout 30 curl -i -N \
        -H "Connection: Upgrade" \
        -H "Upgrade: websocket" \
        -H "Sec-WebSocket-Version: 13" \
        -H "Sec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==" \
        -H "Sec-WebSocket-Protocol: chat" \
        http://localhost:8080/api/v1/ws 2>&1 | tee websocket.log
) &
WS_PID=$!

# Wait for connection
sleep 2

echo ""
echo "2. Creating entity via REST API..."
RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/entities \
    -H "Content-Type: application/json" \
    -d "{
        \"entity_type\": \"Theory\",
        \"data\": {
            \"question\": \"Testing real-time WebSocket events\",
            \"hypothesis\": \"Events will be broadcast to WebSocket clients\",
            \"status\": \"EXPLORING\"
        }
    }")

ENTITY_ID=$(echo "$RESPONSE" | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "   Created entity: $ENTITY_ID"
echo "   Response: $RESPONSE" | jq . 2>/dev/null || echo "$RESPONSE"

# Wait for event propagation
sleep 2

echo ""
echo "3. Checking server logs for event notifications..."
echo "Recent event activity:"
tail -20 server.log | grep -E "event|WebSocket|notification" | tail -10

echo ""
echo "4. Checking database for events..."
PGPASSWORD=researchprocess_gps psql -h 192.168.10.90 -U researchprocess_gps -d researchprocess_gps \
    -c "SELECT event_id, aggregate_id, event_type, aggregate_version FROM events WHERE aggregate_id = '$ENTITY_ID';" 2>/dev/null

echo ""
echo "5. Updating entity..."
UPDATE_RESPONSE=$(curl -s -X PUT "http://localhost:8080/api/v1/entities/$ENTITY_ID" \
    -H "Content-Type: application/json" \
    -d "{
        \"entity_type\": \"Theory\",
        \"data\": {
            \"status\": \"TESTING\",
            \"conclusion\": \"WebSocket events are working!\"
        }
    }")
echo "   Update response: $UPDATE_RESPONSE" | jq . 2>/dev/null || echo "$UPDATE_RESPONSE"

# Wait for event propagation
sleep 2

echo ""
echo "6. Final server log check..."
tail -30 server.log | grep -E "event|WebSocket|notification" | tail -15

# Cleanup
kill $WS_PID 2>/dev/null

echo ""
echo "Test completed!"
echo ""
echo "Summary:"
echo "- Entity created: $ENTITY_ID"
echo "- Check websocket.log for WebSocket output"
echo "- Check server.log for event notifications"