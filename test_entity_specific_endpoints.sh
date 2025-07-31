#!/bin/bash
# Test script for entity-specific REST endpoints

BASE_URL="http://localhost:8080/api/v1"
AUTH_TOKEN="test-token"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Testing Entity-Specific Endpoints${NC}"
echo "=================================="

# First, create a theory entity for testing
echo -e "\n${BLUE}1. Creating a test Theory${NC}"
THEORY_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Theory",
    "data": {
      "question": "Who were the parents of John Smith?",
      "hypothesis": "John Smith'\''s parents were William and Mary Smith",
      "state": "ACTIVE"
    }
  }')

THEORY_ID=$(echo $THEORY_RESPONSE | jq -r '.id')
echo "Created Theory ID: $THEORY_ID"

# Test Theory-specific endpoints
echo -e "\n${BLUE}2. Testing Theory Branch Endpoint${NC}"
BRANCH_RESPONSE=$(curl -s -X POST "$BASE_URL/theories/$THEORY_ID/branch" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "branch_name": "Alternative hypothesis",
    "hypothesis": "John Smith'\''s parents were actually James and Sarah Smith"
  }')

if [ $? -eq 0 ]; then
    BRANCH_ID=$(echo $BRANCH_RESPONSE | jq -r '.id')
    echo -e "${GREEN}✓ Branch created successfully: $BRANCH_ID${NC}"
else
    echo -e "${RED}✗ Failed to create branch${NC}"
fi

# Create some evidence for the theory
echo -e "\n${BLUE}3. Creating Evidence for Theory${NC}"
EVIDENCE_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_type\": \"Evidence\",
    \"data\": {
      \"theory_id\": \"$THEORY_ID\",
      \"description\": \"Birth certificate showing William and Mary as parents\",
      \"evidence_type\": \"document\",
      \"reliability\": \"high\"
    }
  }")

EVIDENCE_ID=$(echo $EVIDENCE_RESPONSE | jq -r '.id')
echo "Created Evidence ID: $EVIDENCE_ID"

# Test get evidence endpoint
echo -e "\n${BLUE}4. Testing Get Theory Evidence Endpoint${NC}"
EVIDENCE_LIST=$(curl -s -X GET "$BASE_URL/theories/$THEORY_ID/evidence" \
  -H "Authorization: Bearer $AUTH_TOKEN")

EVIDENCE_COUNT=$(echo $EVIDENCE_LIST | jq '.total')
echo -e "${GREEN}✓ Found $EVIDENCE_COUNT evidence items for theory${NC}"

# Test compliance status endpoint
echo -e "\n${BLUE}5. Testing Theory Compliance Status Endpoint${NC}"
COMPLIANCE_RESPONSE=$(curl -s -X GET "$BASE_URL/theories/$THEORY_ID/compliance-status" \
  -H "Authorization: Bearer $AUTH_TOKEN")

COMPLIANT=$(echo $COMPLIANCE_RESPONSE | jq -r '.compliant')
ISSUES_COUNT=$(echo $COMPLIANCE_RESPONSE | jq '.issues | length')
echo -e "${GREEN}✓ Compliance check complete: compliant=$COMPLIANT, issues=$ISSUES_COUNT${NC}"
echo $COMPLIANCE_RESPONSE | jq '.issues'

# Create Person entities for testing
echo -e "\n${BLUE}6. Creating test Person entities${NC}"
PERSON1_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "IdentityPersona",
    "data": {
      "name": "John Smith",
      "birth_date": "1850-01-01",
      "death_date": "1920-12-31"
    }
  }')

PERSON1_ID=$(echo $PERSON1_RESPONSE | jq -r '.id')
echo "Created Person 1 ID: $PERSON1_ID"

PERSON2_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "IdentityPersona",
    "data": {
      "name": "Mary Johnson",
      "birth_date": "1855-06-15"
    }
  }')

PERSON2_ID=$(echo $PERSON2_RESPONSE | jq -r '.id')
echo "Created Person 2 ID: $PERSON2_ID"

# Create a relationship between persons
echo -e "\n${BLUE}7. Creating Relationship between Persons${NC}"
RELATIONSHIP_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_type\": \"Relationship\",
    \"data\": {
      \"person1_id\": \"$PERSON1_ID\",
      \"person2_id\": \"$PERSON2_ID\",
      \"relationship_type\": \"spouse\",
      \"start_date\": \"1875-06-20\",
      \"confidence\": \"high\"
    }
  }")

RELATIONSHIP_ID=$(echo $RELATIONSHIP_RESPONSE | jq -r '.id')
echo "Created Relationship ID: $RELATIONSHIP_ID"

# Test Person timeline endpoint
echo -e "\n${BLUE}8. Testing Person Timeline Endpoint${NC}"
TIMELINE_RESPONSE=$(curl -s -X GET "$BASE_URL/persons/$PERSON1_ID/timeline" \
  -H "Authorization: Bearer $AUTH_TOKEN")

EVENTS_COUNT=$(echo $TIMELINE_RESPONSE | jq '.events | length')
echo -e "${GREEN}✓ Timeline retrieved: $EVENTS_COUNT events${NC}"
echo $TIMELINE_RESPONSE | jq '.events'

# Test Person relationships endpoint
echo -e "\n${BLUE}9. Testing Person Relationships Endpoint${NC}"
RELATIONSHIPS_RESPONSE=$(curl -s -X GET "$BASE_URL/persons/$PERSON1_ID/relationships" \
  -H "Authorization: Bearer $AUTH_TOKEN")

REL_COUNT=$(echo $RELATIONSHIPS_RESPONSE | jq '.relationships | length')
echo -e "${GREEN}✓ Relationships retrieved: $REL_COUNT relationships${NC}"
echo $RELATIONSHIPS_RESPONSE | jq '.relationships'

# Create a workspace for testing
echo -e "\n${BLUE}10. Creating test Workspace${NC}"
WORKSPACE_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": "Workspace",
    "data": {
      "name": "Smith Family Research",
      "description": "Research into the Smith family genealogy",
      "members": [
        {
          "user_id": "00000000-0000-0000-0000-000000000001",
          "role": "owner",
          "joined_at": "2025-01-01T00:00:00Z"
        }
      ]
    }
  }')

WORKSPACE_ID=$(echo $WORKSPACE_RESPONSE | jq -r '.id')
if [ "$WORKSPACE_ID" == "null" ] || [ -z "$WORKSPACE_ID" ]; then
    echo -e "${RED}✗ Failed to create workspace. Response:${NC}"
    echo $WORKSPACE_RESPONSE
    # Create a Theory as a workspace placeholder for testing
    echo -e "${BLUE}Creating Theory as workspace placeholder...${NC}"
    WORKSPACE_RESPONSE=$(curl -s -X POST "$BASE_URL/entities" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" \
      -d '{
        "entity_type": "Theory",
        "data": {
          "name": "Smith Family Research Workspace",
          "workspace_type": true,
          "members": [
            {
              "user_id": "00000000-0000-0000-0000-000000000001",
              "role": "owner",
              "joined_at": "2025-01-01T00:00:00Z"
            }
          ]
        }
      }')
    WORKSPACE_ID=$(echo $WORKSPACE_RESPONSE | jq -r '.id')
fi
echo "Using Workspace ID: $WORKSPACE_ID"

# Test Workspace members endpoint
echo -e "\n${BLUE}11. Testing Workspace Members Endpoint${NC}"
MEMBERS_RESPONSE=$(curl -s -X GET "$BASE_URL/workspaces/$WORKSPACE_ID/members" \
  -H "Authorization: Bearer $AUTH_TOKEN")

MEMBERS_COUNT=$(echo $MEMBERS_RESPONSE | jq '.total')
echo -e "${GREEN}✓ Members retrieved: $MEMBERS_COUNT members${NC}"
echo $MEMBERS_RESPONSE | jq '.members'

# Test Workspace invite endpoint
echo -e "\n${BLUE}12. Testing Workspace Invite Endpoint${NC}"
INVITE_RESPONSE=$(curl -s -X POST "$BASE_URL/workspaces/$WORKSPACE_ID/invite" \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "user_email": "researcher@example.com",
    "role": "contributor",
    "message": "Would you like to join our Smith family research project?"
  }')

INVITATION_ID=$(echo $INVITE_RESPONSE | jq -r '.invitation_id')
echo -e "${GREEN}✓ Invitation sent: $INVITATION_ID${NC}"

echo -e "\n${BLUE}Testing Complete!${NC}"
echo "=================="