#!/bin/bash
# Test PostgreSQL NOTIFY mechanism

echo "Testing PostgreSQL NOTIFY for events..."
echo ""

# Start listening for notifications in background
echo "1. Starting LISTEN on 'events' channel..."
PGPASSWORD=researchprocess_gps psql -h 192.168.10.90 -U researchprocess_gps -d researchprocess_gps -c "LISTEN events;" &
LISTEN_PID=$!

# Give it time to connect
sleep 1

# Create an entity via API
echo ""
echo "2. Creating entity via REST API..."
RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "Testing NOTIFY",
      "hypothesis": "NOTIFY will fire when entity is created",
      "status": "EXPLORING"
    }
  }')

ENTITY_ID=$(echo "$RESPONSE" | grep -o '"id":"[^"]*' | cut -d'"' -f4)
echo "   Created entity: $ENTITY_ID"

# Check if event was stored
echo ""
echo "3. Checking events table..."
PGPASSWORD=researchprocess_gps psql -h 192.168.10.90 -U researchprocess_gps -d researchprocess_gps -c "SELECT event_id, aggregate_id, event_type FROM events WHERE aggregate_id = '$ENTITY_ID';"

# Manually trigger a notification to test
echo ""
echo "4. Manually triggering notification..."
PGPASSWORD=researchprocess_gps psql -h 192.168.10.90 -U researchprocess_gps -d researchprocess_gps -c "NOTIFY events, '{\"test\": \"manual notification\"}';"

# Clean up
kill $LISTEN_PID 2>/dev/null

echo ""
echo "Test completed!"