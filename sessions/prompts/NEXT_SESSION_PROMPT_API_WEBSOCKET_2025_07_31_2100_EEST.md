# ResearchProcess-GPS: Continue API Implementation
## Phase 3 - WebSocket & Real-time Features
### Generated: 2025-07-31 21:00:00 EEST

---

## 🚨 CRITICAL: Read These Documents First (IN THIS EXACT ORDER)

**DO NOT SKIP THIS STEP - Your success depends on understanding the full context**

### 1. Comprehensive Handover Document
```bash
cd ~/ResearchProcess-GPS
bat COMPREHENSIVE_HANDOVER_API_COMPLETE_2025_07_31_2057_EEST.md
```
This contains:
- Current system state with REST API COMPLETE
- Known issues (EVENT PUBLISHING SQL ERROR - HIGH PRIORITY)
- All reference document locations
- Server configuration (running on port 8080)
- Next implementation tasks

### 2. Today's Implementation Summary
```bash
bat API_SERVER_IMPLEMENTATION_SUMMARY_2025_07_31_2056_EEST.md
```
Review what was accomplished with the REST API implementation.

### 3. Master Architecture Plan
```bash
bat ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
```
Focus on:
- Line 293-344: Networking Layer (WebSocket implementation)
- Line 420-436: Phase 3 requirements

### 4. Protocol Specification
```bash
bat docs/PROTOCOL_SPECIFICATION_v1.md
```
Study the WebSocket protocol section for real-time implementation.

---

## 🎯 Your Mission: Fix Event Publishing & Implement WebSocket

### Current Status
- ✅ REST API fully implemented and tested
- ✅ Server running on port 8080
- ⚠️ Event publishing has SQL error (MUST FIX FIRST)
- 🚧 WebSocket endpoint exists but not implemented
- 🚧 No real-time features yet

### High Priority Task #1: Fix Event Publishing
```
Error: "FOR UPDATE is not allowed with aggregate functions"
Location: EventSourcedTransaction::publish_event
```

The issue is in the event store SQL queries. You need to:
1. Find the problematic SQL query in `rp-events` or `rp-storage-postgres`
2. Remove or modify the FOR UPDATE clause
3. Test that events are properly published

### High Priority Task #2: Implement WebSocket Protocol
After fixing events, implement the WebSocket handler:
```rust
// In crates/rp-server/src/handlers/websocket.rs
- Handle authentication message
- Implement subscription management
- Stream events to connected clients
- Handle client disconnection
```

### Task #3: PostgreSQL LISTEN/NOTIFY
Set up real-time notifications:
1. Create NOTIFY trigger on events table
2. Add listener in server
3. Forward events to WebSocket subscribers

---

## ⚠️ Critical Requirements

### NO_FALLBACK_POLICY (MANDATORY)
- **ZERO** warnings allowed
- **ALL** errors must be handled explicitly
- **NO** silent failures or degraded operation
- Read: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`

### Server is Currently Running!
```bash
# Check if server is still running
ps aux | grep rp-server

# If not running, restart it:
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server &

# Test health endpoint
curl http://localhost:8080/health | jq .
```

### Database Connection
```bash
# PostgreSQL on LAN server
Server: 192.168.10.90:5432
Database: researchprocess_gps
User/Pass: researchprocess_gps

# ALWAYS use env override:
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps [command]
```

---

## 📋 Implementation Checklist

### 1. Fix Event Publishing (CRITICAL)
- [ ] Locate the SQL query with FOR UPDATE + aggregate
- [ ] Fix the query to work without FOR UPDATE
- [ ] Test event publishing works
- [ ] Verify events appear in events table

### 2. WebSocket Implementation
- [ ] Parse incoming WebSocket messages
- [ ] Handle authentication
- [ ] Implement subscription types:
  - Subscribe to specific entity
  - Subscribe to entity type
  - Subscribe to workspace
  - Subscribe to query
- [ ] Stream events to subscribers
- [ ] Handle disconnection cleanup

### 3. LISTEN/NOTIFY Integration
- [ ] Create PostgreSQL notification function
- [ ] Add trigger to events table
- [ ] Implement listener in server
- [ ] Connect to WebSocket broadcaster

### 4. Testing
- [ ] Create WebSocket test client
- [ ] Test real-time updates
- [ ] Verify multiple subscribers work
- [ ] Test reconnection handling

---

## 🚀 Quick Start Commands

```bash
# 1. Navigate to project
cd ~/ResearchProcess-GPS

# 2. Check server status
ps aux | grep rp-server
curl http://localhost:8080/health | jq .

# 3. View server logs
tail -f server.log

# 4. Run API tests
./test_api.sh

# 5. Check for event publishing errors
tail -50 server.log | grep -i event

# 6. When ready to test WebSocket
wscat -c ws://localhost:8080/api/v1/ws
```

---

## 💡 Implementation Tips

### For Event Publishing Fix
Look in these files:
- `crates/rp-storage-postgres/src/event_sourced_wrapper.rs`
- `crates/rp-events/src/postgres_store.rs`
- Search for queries with both "FOR UPDATE" and aggregate functions

### For WebSocket
Use the existing protocol types:
- `ClientMessage` and `ServerMessage` from `rp-protocol`
- Event types from `rp-events`
- Consider using `tokio::sync::broadcast` for pub/sub

### For LISTEN/NOTIFY
```sql
-- Example trigger
CREATE OR REPLACE FUNCTION notify_event_change()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify('entity_events', 
    json_build_object(
      'entity_id', NEW.aggregate_id,
      'event_type', NEW.event_type
    )::text
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

---

## 🎯 Success Criteria

1. **Event Publishing Fixed**
   - No more SQL errors in logs
   - Events appear in database after CRUD operations
   - Can query events table and see results

2. **WebSocket Working**
   - Can connect with WebSocket client
   - Can authenticate
   - Can subscribe to entities
   - Receive real-time updates when entities change

3. **Full Integration**
   - Create entity via REST → Event published → WebSocket clients notified
   - Multiple clients receive same updates
   - System remains stable under load

---

**Remember**: 
- NO_FALLBACK_POLICY - fix ALL issues properly
- Event publishing MUST be fixed before WebSocket work
- Server is running on port 8080, not 3000
- The foundation is solid - focus on real-time features

*Generated: 2025-07-31 21:00:00 EEST*
*Purpose: Enable seamless continuation of ResearchProcess-GPS real-time features*
*Goal: Complete Phase 3 with working WebSocket and event streaming*