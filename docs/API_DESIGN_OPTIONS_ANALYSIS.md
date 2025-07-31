# API Design Options Analysis for ResearchProcess-GPS
## REST API URL Structure Comparison

### Option 1: Traditional RESTful Routes (Resource-Centric)

```
/api/v1/theories
/api/v1/theories/{id}
/api/v1/theories/{id}/evidence
/api/v1/theories/{id}/relationships

/api/v1/persons
/api/v1/persons/{id}
/api/v1/persons/{id}/relationships
/api/v1/persons/{id}/evidence

/api/v1/evidence
/api/v1/evidence/{id}
/api/v1/evidence/{id}/citations

/api/v1/workspaces
/api/v1/workspaces/{id}
/api/v1/workspaces/{id}/members
```

**Advantages:**
- Industry standard, immediately familiar to developers
- Clear resource hierarchy
- Natural for REST clients and OpenAPI/Swagger docs
- Each entity type has its own dedicated endpoint
- Easy to implement entity-specific business logic
- Natural sub-resource relationships (e.g., theory's evidence)
- Simple to add entity-specific query parameters

**Disadvantages:**
- Requires 23+ different route handlers
- More boilerplate code for similar CRUD operations
- Client needs to know all entity endpoints
- Harder to implement generic operations across entity types

**Example Usage:**
```bash
# Create a new theory
POST /api/v1/theories
{
  "question": "Who were the parents of John Smith?",
  "hypothesis": "John Smith's parents were William and Mary Smith"
}

# Get all evidence for a theory
GET /api/v1/theories/123e4567-e89b-12d3-a456-426614174000/evidence

# Update a person
PUT /api/v1/persons/987fcdeb-51a2-43f1-b123-426614174000
{
  "name": "John Smith",
  "birth_date": "1850-01-01"
}
```

### Option 2: Generic Entity Endpoint (Type-Discriminated)

```
/api/v1/entities
/api/v1/entities/{id}
/api/v1/entities?type=Theory
/api/v1/entities?type=Person
/api/v1/entities/{id}/relationships
/api/v1/entities/{id}/children?type=Evidence
```

**Advantages:**
- Single endpoint to maintain
- Easier to implement generic operations
- Consistent handling across all entity types
- Simpler server-side routing
- Natural fit with your existing discriminated union approach
- Easy to add new entity types without new routes

**Disadvantages:**
- Less RESTful, may confuse developers
- Entity type must be specified in query or body
- Harder to document entity-specific features
- Less natural for OpenAPI/Swagger
- May need complex validation logic

**Example Usage:**
```bash
# Create a new theory
POST /api/v1/entities
{
  "entity_type": "Theory",
  "data": {
    "question": "Who were the parents of John Smith?",
    "hypothesis": "John Smith's parents were William and Mary Smith"
  }
}

# Get all theories
GET /api/v1/entities?type=Theory

# Update a person
PUT /api/v1/entities/987fcdeb-51a2-43f1-b123-426614174000
{
  "entity_type": "Person",
  "data": {
    "name": "John Smith",
    "birth_date": "1850-01-01"
  }
}
```

### Option 3: Hybrid Approach (Best of Both)

```
# Generic endpoints for common operations
/api/v1/entities                    # List/search across all types
/api/v1/entities/{id}               # Get any entity by ID
/api/v1/entities/bulk               # Bulk operations

# Type-specific endpoints for specialized operations
/api/v1/theories                    # Theory-specific operations
/api/v1/theories/{id}/evidence      # Theory's evidence
/api/v1/theories/{id}/branch        # Create hypothesis branch

/api/v1/persons                     # Person-specific operations  
/api/v1/persons/{id}/timeline       # Person's life timeline
/api/v1/persons/{id}/relationships  # Family relationships

/api/v1/workspaces                  # Workspace operations
/api/v1/workspaces/{id}/members     # Workspace members
```

**Advantages:**
- Flexibility to use generic or specific endpoints
- Can start generic and add specific routes as needed
- Best developer experience
- Natural entity-specific operations
- Efficient bulk operations

**Disadvantages:**
- More complex routing table
- Potential confusion about which endpoint to use
- More code to maintain

### Option 4: Action-Based API (RPC-style)

```
/api/v1/actions/create-theory
/api/v1/actions/update-person
/api/v1/actions/add-evidence
/api/v1/actions/query-entities
/api/v1/actions/analyze-relationships
/api/v1/actions/generate-report
```

**Advantages:**
- Clear action semantics
- Easy to implement complex operations
- Natural for workflow-based operations
- Good match for command/query separation

**Disadvantages:**
- Not RESTful at all
- Harder to cache
- Less standardized

## Recommendation

I recommend **Option 3: Hybrid Approach** for the following reasons:

1. **Aligns with your architecture**: Your existing code uses a discriminated union pattern for entities, making the generic endpoint natural to implement.

2. **Progressive enhancement**: Start with generic endpoints for MVP, add entity-specific routes as needed.

3. **Best developer experience**: Developers can use whichever pattern fits their use case.

4. **Matches your protocol design**: The protocol enums in ULTRATHINK already show both generic operations (CreateEntity, UpdateEntity) and specific ones (CreateWorkspace).

5. **Future-proof**: Easy to add GraphQL later, which naturally handles both generic and specific queries.

## Implementation Plan

```rust
// Start with these generic routes
router
    .route("/api/v1/entities", post(create_entity).get(list_entities))
    .route("/api/v1/entities/:id", get(get_entity).put(update_entity).delete(delete_entity))
    .route("/api/v1/entities/bulk", post(bulk_operations))
    .route("/api/v1/entities/search", post(search_entities))

// Then add type-specific routes as needed
router
    .route("/api/v1/theories/:id/branch", post(branch_theory))
    .route("/api/v1/persons/:id/timeline", get(get_person_timeline))
    .route("/api/v1/workspaces/:id/members", get(list_members).post(add_member))
```

This approach gives you the flexibility to start simple and evolve based on actual usage patterns.