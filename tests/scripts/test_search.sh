#!/bin/bash
# Test search functionality

BASE_URL="http://localhost:8080/api/v1"
AUTH_TOKEN="test-token"

echo "Testing Search Functionality"
echo "==========================="

# First create some test data
echo -e "\n1. Creating test entities..."

# Create theories
curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "What is the meaning of life?",
      "hypothesis": "The meaning of life is 42",
      "state": "ACTIVE"
    }
  }' > /dev/null

curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "Is there life on Mars?",
      "hypothesis": "Microbial life may exist on Mars",
      "state": "TESTING"
    }
  }' > /dev/null

# Create person
curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "IdentityPersona",
    "data": {
      "name": "Douglas Adams",
      "description": "Author of Hitchhikers Guide",
      "life_dates": "1952-2001"
    }
  }' > /dev/null

echo "Test data created."

# Test basic search
echo -e "\n2. Testing basic search for 'life'..."
curl -s -X GET "$BASE_URL/search?q=life" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq '{total: .total, results: .results | length}'

# Test search with entity type filter
echo -e "\n3. Testing search with entity_type filter..."
curl -s -X GET "$BASE_URL/search?q=life&entity_type=Theory" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq '{total: .total, entity_types: [.results[].entity.entity_type] | unique}'

# Test search with pagination
echo -e "\n4. Testing search with pagination..."
curl -s -X GET "$BASE_URL/search?q=life&limit=1&offset=0" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq '{total: .total, returned: .results | length, has_more: .has_more}'

# Test fuzzy search
echo -e "\n5. Testing fuzzy search..."
curl -s -X GET "$BASE_URL/search?q=martian&fuzzy=true" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq '{total: .total, found: [.results[].entity.data.question]}'

# Test search with highlights
echo -e "\n6. Testing search with highlights..."
curl -s -X GET "$BASE_URL/search?q=life&highlight=true&limit=1" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq '.results[0].highlights'

# Test faceted search results
echo -e "\n7. Testing facets in search results..."
curl -s -X GET "$BASE_URL/search?q=life" \
  -H "Authorization: Bearer $AUTH_TOKEN" | jq '.facets'

echo -e "\nSearch tests complete!"