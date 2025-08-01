#!/bin/bash

# Test script for ResearchProcess-GPS API endpoints

BASE_URL="http://localhost:8080/api/v1"

echo "Testing ResearchProcess-GPS API endpoints..."
echo

# 1. Health check
echo "1. Testing health endpoint..."
curl -s "$BASE_URL/health" | jq .
echo

# 2. Create a Theory entity
echo "2. Creating a Theory entity..."
THEORY_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "Who were the parents of John Smith?",
      "hypothesis": "John Smith'"'"'s parents were William and Mary Smith",
      "status": "EXPLORING"
    }
  }')
echo "$THEORY_RESPONSE" | jq .
THEORY_ID=$(echo "$THEORY_RESPONSE" | jq -r '.id')
echo "Theory ID: $THEORY_ID"
echo

# 3. Get the Theory entity
if [ "$THEORY_ID" != "null" ]; then
  echo "3. Getting Theory entity..."
  curl -s "$BASE_URL/entities/$THEORY_ID" | jq .
  echo

  # 4. Update the Theory entity
  echo "4. Updating Theory entity..."
  UPDATE_RESPONSE=$(curl -s -X PUT "$BASE_URL/entities/$THEORY_ID" \
    -H "Content-Type: application/json" \
    -d '{
      "entity_type": "Theory",
      "data": {
        "question": "Who were the parents of John Smith?",
        "hypothesis": "John Smith'"'"'s parents were William and Mary Smith",
        "status": "TESTING",
        "conclusion": "Evidence suggests William and Mary Smith were indeed the parents"
      }
    }')
  echo "$UPDATE_RESPONSE" | jq .
  echo

  # 5. List entities
  echo "5. Listing entities..."
  curl -s "$BASE_URL/entities?limit=10" | jq .
  echo

  # 6. Delete the Theory entity
  echo "6. Deleting Theory entity..."
  curl -s -X DELETE "$BASE_URL/entities/$THEORY_ID" -w "Status: %{http_code}\n"
  echo
fi

echo "API testing complete!"