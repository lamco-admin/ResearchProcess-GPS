# Sync and Operation Modes Specification

## Overview

ResearchProcess-GPS supports a full spectrum of operation modes from completely offline to real-time collaborative, with sophisticated sync capabilities and granular privacy controls. The system is designed for researchers who work in archives without internet, those who demand complete privacy, and those who want real-time collaboration.

## Operation Modes

### 1. Offline-First Architecture
```yaml
OfflineMode:
  core_principle: "Everything works offline"
  
  local_storage:
    database: "SQLite/IndexedDB"
    file_storage: "Local filesystem"
    encryption: "At-rest encryption mandatory"
    
  capabilities:
    - Full CRUD operations
    - Complete research workflow
    - Evidence analysis
    - Report generation
    - Export/Import
    
  limitations:
    - No external API calls
    - No collaboration features
    - No cloud backup
    - Manual sync required
```

### 2. Standalone Private Mode
```yaml
StandaloneMode:
  description: "Never connects to internet"
  
  features:
    - Air-gapped operation
    - No telemetry
    - No auto-updates
    - No external dependencies
    
  data_exchange:
    - Encrypted USB export
    - QR code transfer
    - Printed reports only
    - Sneakernet protocols
    
  use_cases:
    - Professional researchers
    - Sensitive family research
    - Legal/estate work
    - Privacy-conscious users
```

### 3. Selective Sync Mode
```yaml
SelectiveSync:
  sync_control:
    - Manual sync triggers
    - Selective data sets
    - One-way sync options
    - Conflict resolution
    
  privacy_settings:
    what_syncs:
      - Nothing (default)
      - Research logs only
      - Conclusions only
      - Everything except living
      - Custom rules
      
    when_syncs:
      - Manual only
      - On network detection
      - Scheduled
      - On explicit save
      
  sync_targets:
    - Private cloud
    - Institutional server
    - Peer-to-peer
    - Selected collaborators
```

### 4. Real-Time Collaborative Mode
```yaml
CollaborativeMode:
  real_time_features:
    - Simultaneous editing
    - Live cursor tracking
    - Voice/video chat
    - Screen sharing
    - Shared workspace
    
  collaboration_types:
    - Document co-editing
    - Evidence review sessions
    - Theory brainstorming
    - Peer review meetings
    
  presence_awareness:
    - Who's online
    - What they're viewing
    - Active editing indicators
    - Communication status
```

## Device Support

### 1. Mobile Ingestion
```yaml
MobileCapture:
  phone_capabilities:
    - Camera for documents
    - Voice recording
    - GPS tagging
    - Quick notes
    - Offline operation
    
  tablet_features:
    - Full research interface
    - Handwriting support
    - Document annotation
    - Split-screen research
    
  sync_to_desktop:
    - Automatic on wifi
    - Manual transfer
    - Cloud bridge
    - Direct connection
```

### 2. Archive Laptop Mode
```yaml
ArchiveLaptop:
  optimizations:
    - Low power mode
    - Aggressive caching
    - Batch operations
    - Queue for later sync
    
  archive_specific:
    - Repository templates
    - Call number tracking
    - Session management
    - Photo batch processing
    
  return_sync:
    - Conflict detection
    - Merge assistance
    - Change review
    - Selective import
```

## Data Model Architecture

### 1. Distributed Data Model
```yaml
DistributedModel:
  core_principles:
    - Every entity has UUID
    - Immutable event log
    - Vector clocks for ordering
    - Merkle trees for integrity
    
  entity_structure:
    Entity:
      id: UUID
      version: vector_clock
      created: timestamp
      created_by: researcher_id
      created_device: device_id
      
    Change:
      change_id: UUID
      entity_id: UUID
      operation: "create|update|delete"
      data: JSON
      timestamp: precise_timestamp
      device_id: UUID
      sync_status: "local|syncing|synced"
```

### 2. Conflict Resolution
```yaml
ConflictResolution:
  detection:
    - Same entity modified
    - Conflicting relationships
    - Incompatible changes
    
  resolution_strategies:
    automatic:
      - Last write wins
      - Most evidence wins
      - Higher confidence wins
      - Designated primary wins
      
    manual:
      - Show both versions
      - Merge interface
      - Keep both as variants
      - Escalate to team
      
  preservation:
    - Never lose data
    - Archive conflicts
    - Audit trail complete
    - Rollback capability
```

### 3. Sync Protocol
```yaml
SyncProtocol:
  phases:
    discovery:
      - Exchange version vectors
      - Identify differences
      - Calculate sync plan
      
    transfer:
      - Chunk large data
      - Compress transfers
      - Resume capability
      - Progress tracking
      
    resolution:
      - Apply changes
      - Resolve conflicts
      - Update vectors
      - Confirm completion
      
  optimization:
    - Delta sync only
    - Binary diff for files
    - Deduplication
    - Bandwidth limiting
```

## Research Certification & Attribution

### 1. Research Packages
```yaml
ResearchPackage:
  immutable_record:
    package_id: UUID
    created_date: timestamp
    researcher_id: UUID
    
  contents:
    - Research question
    - Evidence collected
    - Analysis performed
    - Conclusions reached
    - Supporting documents
    
  certification:
    digital_signature: signature
    hash: SHA-256
    timestamp_authority: URL
    blockchain_option: true
    
  versioning:
    - Original (immutable)
    - Amendments (linked)
    - Retractions (explicit)
    - Superseded_by (reference)
```

### 2. Attribution System
```yaml
Attribution:
  citation_format:
    - Researcher name
    - Package ID
    - Creation date
    - Version referenced
    - Access date
    - Verification hash
    
  usage_rights:
    - Read only
    - Quote with attribution
    - Build upon (if allowed)
    - Commercial use (if allowed)
    
  modification_rights:
    original_researcher:
      - Update allowed
      - Must preserve original
      - Link to amendments
      - Notify subscribers
      
    other_researchers:
      - Fork if permitted
      - Must attribute
      - Cannot modify original
      - Can dispute/comment
```

### 3. Negative Research Publication
```yaml
NegativeResearch:
  disproven_theory:
    theory_id: UUID
    disproof_type: "impossible|unlikely|superseded"
    
  evidence_package:
    - Original theory
    - Contradicting evidence
    - Analysis/reasoning
    - Confidence level
    
  publication:
    - Permanent record
    - Searchable database
    - Link to original
    - Peer review option
    
  impact:
    - Warn future researchers
    - Prevent duplicate work
    - Update related theories
    - Academic credit
```

## GEDCOM 7 Integration

### 1. Embedded Research Metadata
```yaml
GEDCOM7Research:
  research_extension:
    0 @I1@ INDI
    1 NAME John /Smith/
    1 _RESEARCH @R1@
    2 _PACKAGE_ID 550e8400-e29b-41d4-a716-446655440000
    2 _RESEARCHER "Jane Doe <jane@example.com>"
    2 _CERTIFIED 2025-07-29T10:30:00Z
    2 _SIGNATURE "base64-encoded-signature"
    2 _RIGHTS "read-only, attribution required"
    2 _VERSION 1.0
    3 _AMENDMENT @R2@
    
  negative_finding:
    0 @I99@ _DISPROVEN_PERSON
    1 _THEORY "Son of William Smith"
    1 _DISPROOF @D1@
    2 _TYPE "impossible"
    2 _REASON "Born 10 years after William died"
    2 _EVIDENCE @E1@ @E2@ @E3@
    2 _RESEARCHER "Jane Doe"
    2 _DATE 2025-07-29
```

### 2. Sync Metadata
```yaml
SyncMetadata:
  0 HEAD
  1 _SYNC
  2 _MODE "selective"
  2 _LAST_SYNC 2025-07-29T10:30:00Z
  2 _DEVICE_ID "laptop-archive-001"
  2 _SYNC_VECTOR "A:5,B:3,C:7"
  
  per_entity:
    1 _SYNC_STATUS "local|synced|conflict"
    1 _LAST_MODIFIED 2025-07-29T10:30:00Z
    1 _MODIFIED_BY "researcher-id"
    1 _DEVICE "device-id"
```

## Privacy & Security

### 1. Granular Privacy Controls
```yaml
PrivacyControls:
  entity_level:
    - Private (never sync)
    - Team (sync to team only)
    - Public (available to all)
    - Embargo (public after date)
    
  field_level:
    - Redact living data
    - Hide locations
    - Anonymize names
    - Remove dates
    
  export_controls:
    - Watermarking
    - Encryption required
    - Track recipients
    - Expiring access
```

### 2. Security Features
```yaml
Security:
  encryption:
    - At rest: AES-256
    - In transit: TLS 1.3
    - End-to-end: Optional
    - Key management: Local
    
  authentication:
    - Multi-factor required
    - Biometric option
    - Device registration
    - Session management
    
  audit:
    - All access logged
    - Change tracking
    - Export monitoring
    - Compliance reports
```

## Implementation Considerations

### 1. Offline Performance
```yaml
OfflineOptimization:
  database:
    - Efficient indexes
    - Query optimization
    - Lazy loading
    - Background sync
    
  storage:
    - Compression
    - Deduplication
    - Archival strategy
    - Cleanup routines
```

### 2. Sync Efficiency
```yaml
SyncEfficiency:
  strategies:
    - Incremental sync
    - Parallel transfers
    - Intelligent ordering
    - Failure recovery
    
  monitoring:
    - Sync status UI
    - Conflict alerts
    - Performance metrics
    - Error reporting
```

### 3. Collaboration Performance
```yaml
CollaborationPerformance:
  real_time:
    - WebRTC for peer-to-peer
    - Operational transformation
    - Conflict-free replicated data types
    - Presence protocols
    
  scalability:
    - Room size limits
    - Resource management
    - Degradation strategies
    - Load balancing
```

This comprehensive sync and operation modes specification ensures ResearchProcess-GPS can support every type of researcher, from the most privacy-conscious to the most collaborative, while maintaining data integrity and enabling powerful new workflows.