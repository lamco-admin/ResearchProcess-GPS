#!/bin/bash
# Quick test of entity-specific endpoints

BASE_URL="http://localhost:8080/api/v1"
AUTH_TOKEN="test-token"

# Create a theory
echo "Creating theory..."
THEORY_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "Test question?",
      "hypothesis": "Test hypothesis"
    }
  }')

THEORY_ID=$(echo $THEORY_RESPONSE | jq -r '.id')
echo "Theory ID: $THEORY_ID"

# Test branch endpoint
echo -e "\nTesting branch endpoint..."
curl -X POST "$BASE_URL/theories/$THEORY_ID/branch" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "branch_name": "Alt branch",
    "hypothesis": "Alternative hypothesis"
  }' | jq .

# Test compliance endpoint
echo -e "\nTesting compliance endpoint..."
curl -X GET "$BASE_URL/theories/$THEORY_ID/compliance-status" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq .