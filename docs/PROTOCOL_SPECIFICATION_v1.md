# ResearchProcess-GPS Protocol Specification v1.0
## API Layer Implementation - Phase 3

### Overview

This document defines the protocol for the ResearchProcess-GPS API layer, covering REST, WebSocket, and future GraphQL endpoints. The protocol supports both synchronous request/response patterns and asynchronous real-time updates.

### Protocol Principles

1. **JSON-first**: Initial implementation uses JSON for ease of debugging
2. **Versioned**: All endpoints include version in URL path
3. **Event-driven**: All mutations publish events for real-time subscribers
4. **Type-safe**: Strong typing via Rust enums and TypeScript interfaces
5. **Flexible**: Supports both generic and entity-specific operations

### Base URL Structure

```
https://api.researchprocess.localhost/api/v1/
wss://api.researchprocess.localhost/ws/v1/
```

## REST API Specification

### Generic Entity Endpoints

#### Create Entity
```http
POST /api/v1/entities
Content-Type: application/json
Authorization: Bearer {token}

{
  "entity_type": "Theory",
  "data": {
    "question": "Who were the parents of John Smith?",
    "hypothesis": "John Smith's parents were William and Mary Smith",
    "workspace_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}

Response: 201 Created
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "entity_type": "Theory",
  "version": 1,
  "created_at": "2025-01-31T10:00:00Z",
  "created_by": "550e8400-e29b-41d4-a716-446655440001",
  "data": { ... }
}
```

#### Get Entity
```http
GET /api/v1/entities/{id}
Authorization: Bearer {token}

Response: 200 OK
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "entity_type": "Theory",
  "version": 3,
  "data": { ... },
  "metadata": {
    "created_at": "2025-01-31T10:00:00Z",
    "updated_at": "2025-01-31T11:30:00Z",
    "created_by": "550e8400-e29b-41d4-a716-446655440001"
  }
}
```

#### Update Entity
```http
PUT /api/v1/entities/{id}
Content-Type: application/json
Authorization: Bearer {token}
If-Match: {version}

{
  "entity_type": "Theory",
  "data": {
    "state": "TESTING",
    "conclusion": "Hypothesis partially confirmed"
  }
}

Response: 200 OK
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "entity_type": "Theory",
  "version": 4,
  "data": { ... }
}
```

#### List Entities
```http
GET /api/v1/entities?type=Theory&state=ACTIVE&limit=20&offset=0
Authorization: Bearer {token}

Response: 200 OK
{
  "items": [ ... ],
  "total": 150,
  "limit": 20,
  "offset": 0,
  "has_more": true
}
```

#### Search Entities
```http
POST /api/v1/entities/search
Content-Type: application/json
Authorization: Bearer {token}

{
  "filters": {
    "entity_types": ["Theory", "Person"],
    "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
    "created_after": "2025-01-01T00:00:00Z",
    "text_search": "John Smith"
  },
  "sort": {
    "field": "created_at",
    "order": "desc"
  },
  "pagination": {
    "limit": 50,
    "offset": 0
  }
}

Response: 200 OK
{
  "results": [ ... ],
  "facets": {
    "entity_types": {
      "Theory": 45,
      "Person": 23
    }
  },
  "total": 68
}
```

#### Bulk Operations
```http
POST /api/v1/entities/bulk
Content-Type: application/json
Authorization: Bearer {token}

{
  "operations": [
    {
      "action": "create",
      "entity_type": "Person",
      "data": { "name": "John Smith" }
    },
    {
      "action": "update",
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "data": { "state": "CONCLUDED" }
    }
  ]
}

Response: 200 OK
{
  "results": [
    { "index": 0, "success": true, "id": "new-id-1", "version": 1 },
    { "index": 1, "success": true, "id": "123e4567...", "version": 5 }
  ]
}
```

### Entity-Specific Endpoints

#### Theory Operations
```http
POST /api/v1/theories/{id}/branch
Content-Type: application/json

{
  "branch_name": "Alternative hypothesis",
  "hypothesis": "John Smith's parents were actually James and Sarah Smith"
}

GET /api/v1/theories/{id}/evidence
GET /api/v1/theories/{id}/compliance-status
```

#### Person Operations
```http
GET /api/v1/persons/{id}/timeline
GET /api/v1/persons/{id}/relationships
POST /api/v1/persons/{id}/merge
```

#### Workspace Operations
```http
GET /api/v1/workspaces/{id}/members
POST /api/v1/workspaces/{id}/invite
DELETE /api/v1/workspaces/{id}/members/{member_id}
```

## WebSocket Protocol

### Connection
```javascript
const ws = new WebSocket('wss://api.researchprocess.localhost/ws/v1/');

// Initial authentication
ws.send(JSON.stringify({
  type: 'auth',
  token: 'Bearer {token}'
}));
```

### Message Format

#### Client to Server
```typescript
interface ClientMessage {
  id: string;           // Client-generated request ID
  type: 'subscribe' | 'unsubscribe' | 'request';
  payload: SubscribeRequest | UnsubscribeRequest | ApiRequest;
}

interface SubscribeRequest {
  subscription_type: 'entity' | 'entity_type' | 'workspace' | 'query';
  params: {
    entity_id?: string;
    entity_type?: EntityType;
    workspace_id?: string;
    query?: SearchQuery;
  };
}
```

#### Server to Client
```typescript
interface ServerMessage {
  id?: string;          // Request ID if responding to request
  type: 'event' | 'response' | 'error' | 'ping';
  payload: EventPayload | ResponsePayload | ErrorPayload;
}

interface EventPayload {
  event_type: 'created' | 'updated' | 'deleted' | 'state_changed';
  entity_type: EntityType;
  entity_id: string;
  version: number;
  data?: any;
  metadata: {
    occurred_at: string;
    actor_id: string;
    workspace_id?: string;
  };
}
```

### Subscription Examples

```javascript
// Subscribe to specific entity
ws.send(JSON.stringify({
  id: 'req-1',
  type: 'subscribe',
  payload: {
    subscription_type: 'entity',
    params: {
      entity_id: '123e4567-e89b-12d3-a456-426614174000'
    }
  }
}));

// Subscribe to all theories in workspace
ws.send(JSON.stringify({
  id: 'req-2',
  type: 'subscribe',
  payload: {
    subscription_type: 'query',
    params: {
      query: {
        entity_types: ['Theory'],
        workspace_id: '550e8400-e29b-41d4-a716-446655440000'
      }
    }
  }
}));
```

## Error Handling

### Error Response Format
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid entity data",
    "details": {
      "field": "birth_date",
      "reason": "Date cannot be in the future"
    },
    "request_id": "req-123"
  }
}
```

### Error Codes
- `AUTHENTICATION_ERROR` - Invalid or missing auth token
- `AUTHORIZATION_ERROR` - Insufficient permissions
- `VALIDATION_ERROR` - Invalid request data
- `NOT_FOUND` - Entity not found
- `CONFLICT` - Version conflict or duplicate
- `RATE_LIMIT` - Too many requests
- `INTERNAL_ERROR` - Server error

## Authentication

### Initial Implementation (API Key)
```http
Authorization: Bearer {api_key}
```

### Future Implementations
- JWT tokens with refresh flow
- OAuth2 integration
- Session-based auth for web UI

## Rate Limiting

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## Pagination

All list endpoints support:
- `limit` - Number of results (max 100)
- `offset` - Skip N results
- `cursor` - Cursor-based pagination (future)

## Filtering & Sorting

### Common Filters
- `workspace_id` - Filter by workspace
- `created_after` / `created_before` - Date ranges
- `state` - Entity state
- `tags` - Tag-based filtering

### Sorting
- `sort_by` - Field to sort by
- `sort_order` - 'asc' or 'desc'

## Change Streaming via PostgreSQL NOTIFY

Events are published to PostgreSQL channels:
- `entity_changes` - All entity changes
- `entity_changes:{type}` - Type-specific changes
- `workspace_changes:{id}` - Workspace-specific changes

## Protocol Versioning

- URL versioning: `/api/v1/`, `/api/v2/`
- Version negotiation via headers (future)
- Backward compatibility for 2 major versions

## Security Considerations

1. All connections require TLS
2. Authentication required for all endpoints
3. Rate limiting per API key
4. Input validation on all endpoints
5. SQL injection prevention via parameterized queries
6. XSS prevention in JSON responses

## Performance Targets

- REST response time: < 100ms p95
- WebSocket latency: < 50ms p95
- Throughput: 10,000 req/sec
- WebSocket connections: 100,000 concurrent

## Future Enhancements

1. Binary protocol (MessagePack/Protobuf)
2. GraphQL endpoint
3. gRPC for inter-service communication
4. Server-sent events as WebSocket alternative
5. Request batching
6. Response compression