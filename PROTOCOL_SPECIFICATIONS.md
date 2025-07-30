# Protocol Specifications for ResearchProcess-GPS

## Core Communication Protocols

### 1. Entity Exchange Protocol (EXP)

```yaml
EntityExchangeProtocol:
  version: "1.0"
  
  # Base message format (JSON/MessagePack/Protobuf)
  message_structure:
    header:
      version: string  # Protocol version
      message_id: UUID
      timestamp: ISO8601
      sender: actor_id
      signature: string  # Optional digital signature
      
    meta:
      entity_type: string
      operation: enum ["create", "update", "delete", "query", "sync"]
      theory_context: theory_id
      privacy_level: enum
      
    payload:
      entity: object  # The actual data
      patches: [json_patch]  # For updates
      
    audit:
      justification: string
      evidence_refs: [UUID]
      compliance: gps_compliance
      
  # Example messages
  examples:
    create_identity:
      header: {version: "1.0", message_id: "..."}
      meta: {entity_type: "Identity", operation: "create"}
      payload: {
        entity: {
          names: [{given: "John", surname: "Henderson"}],
          birth: {date: "1820", place: "Scotland"}
        }
      }
```

### 2. Theory Synchronization Protocol (TSP)

```yaml
TheorySyncProtocol:
  # For synchronizing theory branches between systems
  
  operations:
    theory_announce:
      # Announce available theories
      message: {
        theories: [
          {id: UUID, name: string, base_theory: UUID, 
           last_modified: timestamp, entity_count: int}
        ]
      }
      
    theory_request:
      # Request specific theory state
      message: {
        theory_id: UUID,
        since_version: version_id,  # For incremental sync
        include_evidence: boolean
      }
      
    theory_delta:
      # Send theory changes
      message: {
        theory_id: UUID,
        base_version: version_id,
        changes: [
          {entity_id: UUID, operation: string, data: object}
        ],
        conflicts: [conflict]
      }
      
    theory_merge:
      # Propose theory merge
      message: {
        source_theory: UUID,
        target_theory: UUID,
        merge_strategy: enum,
        conflict_resolutions: [resolution]
      }
```

### 3. Evidence Chain Protocol (ECP)

```yaml
EvidenceChainProtocol:
  # For maintaining evidence integrity and chains
  
  structures:
    evidence_package:
      evidence_id: UUID
      content_hash: SHA256
      metadata:
        source_type: string
        extraction_date: date
        extractor: actor_id
        quality_assessment: object
        
      chain:
        previous_version: UUID
        derived_from: [UUID]
        supports_claims: [claim_id]
        
      signatures:
        extractor_signature: string
        witness_signatures: [signature]
        timestamp_proof: string  # RFC3161
        
  operations:
    submit_evidence:
      # Add new evidence
      validates:
        - "Content hash matches"
        - "Signature valid"
        - "Timestamp valid"
        
    query_evidence_chain:
      # Trace evidence provenance
      returns:
        - "Full chain of custody"
        - "All transformations"
        - "All claims supported"
```

### 4. Privacy Control Protocol (PCP)

```yaml
PrivacyControlProtocol:
  # Enforces privacy across all operations
  
  request_filtering:
    # Every request includes
    requester:
      identity: authenticated_user
      roles: [role]
      purpose: string
      consent_tokens: [token]
      
    # System checks
    access_decision:
      - "Check entity privacy level"
      - "Verify requester permissions"
      - "Apply redaction rules"
      - "Log access attempt"
      
  redaction_rules:
    living_person:
      public: {show: ["name"], hide: ["all_else"]}
      member: {show: ["name", "relationships"], hide: ["details"]}
      authorized: {show: ["all"]}
      
    sensitive_record:
      default: {hide: ["all"]}
      with_consent: {show: ["all"]}
      
  consent_protocol:
    consent_token:
      subject: person_id
      granted_by: person_id
      scope: [allowed_operations]
      expiry: timestamp
      signature: digital_signature
```

### 5. Service Integration Protocol (SIP)

```yaml
ServiceIntegrationProtocol:
  # For integrating with external genealogy services
  
  service_adapters:
    familysearch:
      auth: "OAuth2"
      operations:
        - search_persons
        - get_person_details
        - attach_source
        - watch_changes
        
      mapping:
        fs_person → rgps_identity
        fs_source → rgps_evidence
        fs_relationship → rgps_relationship
        
    ancestry:
      auth: "API Key"
      operations:
        - sync_tree
        - get_dna_matches
        - fetch_records
        
    gramps:
      auth: "Local/Remote"
      operations:
        - import_xml
        - export_xml
        - live_sync
        
  webhook_protocol:
    # For receiving updates
    endpoint: "/webhooks/{service}"
    verification: "HMAC-SHA256"
    retry_policy: "exponential backoff"
    
  event_mapping:
    # Normalize events from different services
    normalized_events:
      - person.created
      - person.updated
      - evidence.discovered
      - match.found
      - hint.available
```

### 6. Apache AGE Graph Protocol (AGP)

```yaml
GraphProtocol:
  # For graph database operations
  
  graph_operations:
    create_node:
      cypher: |
        CREATE (n:Person {
          id: $id,
          names: $names,
          birth: $birth
        })
        
    create_relationship:
      cypher: |
        MATCH (a:Person {id: $parent_id})
        MATCH (b:Person {id: $child_id})
        CREATE (a)-[r:PARENT_OF {
          confidence: $confidence,
          evidence: $evidence_ids
        }]->(b)
        
    find_relationships:
      cypher: |
        MATCH path = (a:Person {id: $person1})
          -[*..10]-(b:Person {id: $person2})
        RETURN path
        
    dna_network:
      cypher: |
        MATCH (p:Person)-[m:DNA_MATCH]-(match:Person)
        WHERE m.cM >= $threshold
        RETURN p, m, match
        
  graph_sync:
    # Keep graph in sync with primary storage
    strategy: "event_driven"
    operations:
      - "Entity change triggers graph update"
      - "Batch updates for performance"
      - "Consistency checks"
```

### 7. Plugin Communication Protocol (PCP)

```yaml
PluginProtocol:
  # How plugins interact with core
  
  plugin_interface:
    lifecycle:
      - initialize(config)
      - activate()
      - deactivate()
      - cleanup()
      
    hooks:
      # Plugins can hook into
      - before_entity_save
      - after_entity_save
      - before_theory_merge
      - after_evidence_added
      - custom_menu_items
      - custom_reports
      
    capabilities:
      # Plugins declare what they can do
      manifest:
        name: string
        version: string
        requires_rgps_version: string
        provides: [capability]
        requires: [capability]
        permissions_needed: [permission]
        
  sandboxing:
    # Plugins run with restrictions
    allowed:
      - "Read entities through API"
      - "Create new entity types"
      - "Add UI components"
      
    denied:
      - "Direct database access"
      - "Modify core entities"
      - "Access other plugins"
      
  communication:
    # Plugins communicate via
    - "Event bus"
    - "Service registry"
    - "Message queue"
```

## Protocol Implementation Layers

```yaml
ImplementationStack:
  transport_layer:
    options:
      - "HTTP/REST for simple operations"
      - "WebSocket for real-time"
      - "gRPC for high performance"
      - "GraphQL for flexible queries"
      
  serialization:
    formats:
      - "JSON for compatibility"
      - "MessagePack for efficiency"
      - "Protobuf for typed APIs"
      
  security_layer:
    - "TLS 1.3 minimum"
    - "Mutual TLS for service-to-service"
    - "JWT for authentication"
    - "HMAC for message integrity"
    
  reliability_layer:
    - "Message acknowledgment"
    - "Retry with backoff"
    - "Circuit breakers"
    - "Request deduplication"
```

## Benefits of These Protocols

1. **Extensible**: New operations without breaking changes
2. **Secure**: Privacy and integrity built in
3. **Traceable**: Full audit trail of all operations
4. **Performant**: Multiple serialization options
5. **Interoperable**: Clear mapping to external services

These protocols ensure that ResearchProcess-GPS can communicate reliably and securely while maintaining the flexibility needed for genealogical research.