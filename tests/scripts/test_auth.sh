#!/bin/bash
# Test API authentication

BASE_URL="http://localhost:8080/api/v1"

echo "Testing API Authentication"
echo "========================="

# Test 1: No auth header
echo -e "\n1. Testing request without auth header..."
response=$(curl -s -w "\n%{http_code}" http://localhost:8080/api/v1/entities)
http_code=$(echo "$response" | tail -n1)
echo "HTTP Status: $http_code"
if [ "$http_code" = "401" ]; then
    echo "✓ Correctly rejected request without auth"
else
    echo "✗ Expected 401, got $http_code"
fi

# Test 2: Invalid token
echo -e "\n2. Testing request with invalid token..."
response=$(curl -s -w "\n%{http_code}" -H "Authorization: Bearer invalid-token" http://localhost:8080/api/v1/entities)
http_code=$(echo "$response" | tail -n1)
echo "HTTP Status: $http_code"
if [ "$http_code" = "401" ]; then
    echo "✓ Correctly rejected invalid token"
else
    echo "✗ Expected 401, got $http_code"
fi

# Test 3: Valid test token
echo -e "\n3. Testing request with valid test token..."
response=$(curl -s -w "\n%{http_code}" -H "Authorization: Bearer test-token" http://localhost:8080/api/v1/entities)
http_code=$(echo "$response" | tail -n1)
echo "HTTP Status: $http_code"
if [ "$http_code" = "200" ]; then
    echo "✓ Successfully authenticated with test token"
else
    echo "✗ Expected 200, got $http_code"
fi

# Test 4: Full access dev key
echo -e "\n4. Testing request with full access dev key..."
response=$(curl -s -w "\n%{http_code}" -H "Authorization: Bearer dev_key_full_access" http://localhost:8080/api/v1/entities)
http_code=$(echo "$response" | tail -n1)
echo "HTTP Status: $http_code"
if [ "$http_code" = "200" ]; then
    echo "✓ Successfully authenticated with dev key"
else
    echo "✗ Expected 200, got $http_code"
fi

# Test 5: Read-only dev key
echo -e "\n5. Testing read-only key with GET request..."
response=$(curl -s -w "\n%{http_code}" -H "Authorization: Bearer dev_key_readonly" http://localhost:8080/api/v1/entities)
http_code=$(echo "$response" | tail -n1)
echo "HTTP Status: $http_code"
if [ "$http_code" = "200" ]; then
    echo "✓ Read-only key can read"
else
    echo "✗ Expected 200, got $http_code"
fi

# Test 6: Health endpoint (no auth required)
echo -e "\n6. Testing health endpoint without auth..."
response=$(curl -s -w "\n%{http_code}" http://localhost:8080/health)
http_code=$(echo "$response" | tail -n1)
echo "HTTP Status: $http_code"
if [ "$http_code" = "200" ]; then
    echo "✓ Health endpoint accessible without auth"
else
    echo "✗ Expected 200, got $http_code"
fi

echo -e "\nAuthentication tests complete!"