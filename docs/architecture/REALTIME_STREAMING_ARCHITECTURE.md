# Real-Time Streaming Architecture for Genealogy Services

## Overview

Real-time collaboration that works regardless of storage backend, integrating with genealogy services and enabling live research sessions.

## Streaming Layer Architecture

```yaml
StreamingArchitecture:
  # Core streaming engine (storage agnostic)
  streaming_core:
    protocol: "WebSocket + WebRTC"
    fallback: "Server-Sent Events + Long Polling"
    message_format: "JSON-RPC 2.0"
    
  # Event types
  event_streams:
    research_events:
      - entity.created
      - entity.updated
      - entity.deleted
      - evidence.linked
      - theory.branched
      - conflict.detected
      
    collaboration_events:
      - user.joined
      - user.left
      - cursor.moved
      - selection.changed
      - comment.added
      - review.requested
      
    service_events:
      - search.completed
      - match.found
      - hint.available
      - record.discovered
      - dna.updated
```

## Multi-Service Integration Hub

```yaml
ServiceIntegrationHub:
  # Real-time connections to genealogy services
  service_connectors:
    
    FamilySearch:
      connection_type: "WebSocket API"
      capabilities:
        - real_time_hints
        - collaborative_indexing
        - live_record_attach
        - person_watch_notifications
      
      streams:
        hints_stream:
          subscribe: "/ws/hints/{person_id}"
          events: ["hint.available", "record.attached", "merge.suggested"]
          
        collaboration_stream:
          subscribe: "/ws/temple/{session_id}"
          events: ["user.action", "ordinance.reserved", "person.modified"]
          
    Ancestry:
      connection_type: "Webhook + Polling"
      capabilities:
        - dna_match_updates
        - tree_change_notifications
        - hint_notifications
        
      streams:
        dna_stream:
          webhook: "https://api.ancestry.com/webhooks/dna"
          events: ["match.new", "match.updated", "ethnicity.refined"]
          
        tree_stream:
          poll_interval: "30s"
          events: ["person.added", "source.attached", "photo.uploaded"]
          
    MyHeritage:
      connection_type: "EventSource API"
      capabilities:
        - smart_match_alerts
        - record_match_notifications
        - photo_enhancement_complete
        
    FindMyPast:
      connection_type: "MQTT"
      capabilities:
        - record_set_updates
        - transcript_corrections
        - community_contributions
```

## Unified Streaming Protocol

```yaml
UnifiedProtocol:
  # All services speak this protocol internally
  
  message_structure:
    header:
      id: UUID
      timestamp: ISO8601
      source: "rgps|familysearch|ancestry|etc"
      type: "event|command|query|response"
      correlation_id: UUID  # For request/response
      
    payload:
      event_type: string
      entity_type: string
      entity_id: UUID
      data: object
      metadata:
        author: user_id
        theory: theory_id
        confidence: number
        
    routing:
      topics: [string]  # Pub/sub topics
      recipients: [user_id]  # Direct messages
      broadcast: boolean
      
  # Example messages
  examples:
    person_update:
      header:
        type: "event"
        source: "rgps"
      payload:
        event_type: "entity.updated"
        entity_type: "Identity"
        entity_id: "person_001"
        data:
          names: [{given: "John", surname: "Smith"}]
          
    familysearch_hint:
      header:
        type: "event"
        source: "familysearch"
      payload:
        event_type: "hint.available"
        entity_type: "Record"
        data:
          record_type: "census"
          confidence: 0.92
          url: "https://familysearch.org/ark:/..."
```

## Real-Time Collaboration Features

```yaml
CollaborationFeatures:
  # Live research sessions
  research_session:
    session_id: UUID
    participants: [user]
    active_theory: theory_id
    
    features:
      screen_sharing:
        protocol: "WebRTC"
        quality: "adaptive"
        annotations: true
        
      voice_chat:
        protocol: "WebRTC Audio"
        push_to_talk: optional
        transcription: available
        
      live_cursors:
        update_rate: "60fps"
        smoothing: "cubic-bezier"
        labels: "user names"
        
      synchronized_scrolling:
        master: "presenter"
        followers: "auto-sync"
        
  # Collaborative evidence analysis
  evidence_review:
    shared_viewport:
      image: "high-res scan"
      zoom_sync: true
      annotation_layers:
        - transcription
        - highlighting
        - comments
        
    real_time_transcription:
      character_by_character: true
      conflict_resolution: "operational_transform"
      suggestion_mode: available
      
  # Theory comparison
  theory_workshop:
    side_by_side_view: true
    difference_highlighting: "real-time"
    participant_theories: "live-update"
    consensus_building: "voting + discussion"
```

## Offline-First Streaming

```yaml
OfflineCapabilities:
  # Queue events when offline
  event_queue:
    storage: "IndexedDB"
    max_size: "100MB"
    compression: "gzip"
    
  # Sync strategies
  reconnection:
    strategy: "smart_replay"
    steps:
      1. "Send queued commands"
      2. "Receive missed events"
      3. "Resolve conflicts"
      4. "Update UI state"
      
  # Conflict resolution
  offline_conflicts:
    detection: "vector_clocks"
    resolution:
      automated: "last-write-wins for metadata"
      manual: "important data changes"
      
  # P2P capabilities
  peer_to_peer:
    enabled: "when possible"
    protocol: "WebRTC DataChannel"
    use_cases:
      - "Local network collaboration"
      - "Family reunion research"
      - "Offline sync between devices"
```

## Service-Specific Streams

```yaml
ServiceStreams:
  # DNA Updates
  dna_streaming:
    sources:
      - ancestry_dna
      - myheritage_dna
      - familytree_dna
      - gedmatch
      
    unified_stream:
      new_matches:
        entity_type: "DNAMatch"
        data: {person_id, cM_shared, segments}
        
      ethnicity_updates:
        entity_type: "EthnicityEstimate"
        data: {regions, percentages, confidence}
        
  # Record Discovery
  record_streaming:
    sources:
      - familysearch_historical
      - ancestry_records
      - findmypast_archives
      
    unified_stream:
      new_collection:
        entity_type: "RecordCollection"
        data: {location, time_period, record_types}
        
      matching_record:
        entity_type: "Record"
        data: {person_matches, confidence, preview}
        
  # Community Contributions
  community_streaming:
    sources:
      - wikitree_updates
      - geni_merges
      - familysearch_changes
      
    unified_stream:
      profile_updated:
        entity_type: "CommunityProfile"
        data: {platform, profile_id, changes}
```

## Performance & Scalability

```yaml
PerformanceArchitecture:
  # Message routing
  message_broker:
    options:
      - "Redis Pub/Sub" # Simple, fast
      - "Apache Kafka" # High volume
      - "RabbitMQ" # Reliable delivery
      - "NATS" # Lightweight
      
  # Scaling strategies
  horizontal_scaling:
    websocket_servers: "Load balanced"
    message_brokers: "Clustered"
    storage_adapters: "Sharded"
    
  # Caching layers
  caching:
    edge_cache: "CloudFlare Workers"
    session_cache: "Redis"
    query_cache: "Storage adapter specific"
    
  # Rate limiting
  rate_limits:
    per_user: "1000 events/minute"
    per_session: "100 participants"
    per_theory: "10000 entities"
```

## Implementation Example

```yaml
ResearchSessionExample:
  # Three researchers working together
  
  scenario: "Investigating John Smith's origins"
  
  participant_1:
    action: "Searches FamilySearch"
    triggers:
      - event: "search.initiated"
      - broadcast: "all participants"
      
  familysearch_response:
    event: "records.found"
    data: "5 matching census records"
    delivered_to: "all participants"
    
  participant_2:
    action: "Clicks on 1850 census"
    triggers:
      - event: "record.selected"
      - stream: "document image to all"
      
  participant_3:
    action: "Starts transcribing"
    triggers:
      - event: "transcription.started"
      - stream: "character updates"
      
  collaborative_result:
    - "All see same document"
    - "Transcription appears live"
    - "Comments thread in sidebar"
    - "Evidence links to theory in real-time"
    - "Changes saved to chosen storage"
```

## Benefits

1. **True Real-Time Collaboration** - See changes as they happen
2. **Service Integration** - One stream from all genealogy services  
3. **Offline Capable** - Work continues without internet
4. **Storage Agnostic** - Streaming works with any backend
5. **Scalable** - From two people to thousands

This architecture enables the kind of real-time collaboration genealogists need while integrating with all major genealogy services and working with any storage backend.