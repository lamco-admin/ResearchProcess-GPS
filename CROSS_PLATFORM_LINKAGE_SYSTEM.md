# Cross-Platform Linkage System Specification

## Overview

The Cross-Platform Linkage System enables bidirectional connections between ResearchProcess-GPS and external genealogy systems, ranging from GEDCOM 7-compliant platforms to proprietary online services. This creates a unified research environment where data remains connected across platforms while respecting each system's capabilities and constraints.

## Linkage Architecture

### 1. Universal Link Model
```yaml
UniversalLink:
  # Core Link Structure
  link_id: UUID
  link_type: enum
    # "gedcom7_compliant", "database_reference", "web_service",
    # "file_reference", "api_connection", "manual_reference"
    
  # Source (ResearchProcess-GPS)
  source:
    entity_type: string  # "person", "identity", "evidence", etc.
    entity_id: UUID
    entity_version: string
    last_verified: timestamp
    
  # Target (External System)
  target:
    system_type: string
    system_identifier:
      name: string  # "FamilySearch", "Ancestry", "MyFamilyTree.ged"
      version: string
      instance: string  # For multiple databases/trees
      
    entity_reference:
      type: string  # "person", "source", "family", etc.
      id: string  # System-specific ID
      url: URL  # If web-accessible
      path: string  # If file-based
      
    access_method:
      type: enum  # "api", "url", "file", "database", "manual"
      credentials_ref: encrypted_reference  # Not the actual credentials
      
  # Link Metadata
  metadata:
    created_date: timestamp
    created_by: researcher_id
    confidence: percent
    verification_status: enum
      # "verified", "pending", "broken", "outdated", "conflict"
    notes: text
```

### 2. System-Specific Connectors
```yaml
SystemConnectors:
  # GEDCOM 7 Compliant Systems
  gedcom7_connector:
    capabilities:
      - Full bidirectional sync
      - Extension support
      - Change tracking
      - Conflict resolution
      
    data_mapping:
      - Direct field mapping
      - Extension preservation
      - Custom field handling
      - Media reference sync
      
  # Database Systems
  database_connector:
    supported_types:
      - SQLite
      - MySQL/MariaDB
      - PostgreSQL
      - Access
      - Custom schemas
      
    connection_info:
      path_or_connection: string
      table_mappings: JSON
      query_templates: SQL[]
      
  # Web Services
  web_service_connector:
    platforms:
      FamilySearch:
        api_version: string
        endpoints: map
        rate_limits: limits
        
      Ancestry:
        tree_id: string
        api_available: boolean
        scraping_rules: JSON
        
      WikiTree:
        profile_format: string
        api_endpoints: map
        space_restrictions: rules
        
      MyHeritage:
        api_version: string
        sync_capabilities: list
        
      FindMyPast:
        tree_reference: string
        export_formats: list
        
  # File-Based Systems
  file_connector:
    formats:
      - GEDCOM (5.5, 5.5.1, 7.0)
      - Family Tree Maker
      - RootsMagic
      - Legacy Family Tree
      - GRAMPS XML
      
    monitoring:
      - File change detection
      - Lock file handling
      - Backup before sync
```

### 3. Link Types and Capabilities
```yaml
LinkTypes:
  # Strong Links (Bidirectional)
  strong_link:
    characteristics:
      - Two-way sync capable
      - Change propagation
      - Conflict detection
      - Version tracking
      
    systems:
      - GEDCOM 7 compliant
      - API-enabled platforms
      - Direct database access
      
  # Reference Links (Unidirectional)
  reference_link:
    characteristics:
      - One-way reference
      - Manual verification
      - Periodic checking
      - No auto-sync
      
    systems:
      - Read-only databases
      - Web URLs
      - Archived files
      - Paper references
      
  # Hybrid Links (Selective Sync)
  hybrid_link:
    characteristics:
      - Selective field sync
      - Read some, write some
      - Transformation rules
      - Approval workflows
      
    systems:
      - Limited API access
      - Partial permissions
      - Privacy-restricted
```

## Linkage Management

### 1. Link Creation and Discovery
```yaml
LinkCreation:
  # Manual Linking
  manual_process:
    steps:
      - Select source entity
      - Choose target system
      - Search/browse for match
      - Confirm correspondence
      - Set sync preferences
      
    validation:
      - Name similarity check
      - Date compatibility
      - Location correlation
      - Relationship consistency
      
  # Automated Discovery
  auto_discovery:
    methods:
      - API search
      - Database query
      - Pattern matching
      - ML-based matching
      
    scoring:
      - Name match score
      - Date proximity score
      - Location match score
      - Network similarity score
      - Combined confidence
      
  # Bulk Linking
  bulk_operations:
    - Import from GEDCOM
    - Match existing records
    - Review suggestions
    - Approve/reject matches
    - Create link batch
```

### 2. Quality Assurance
```yaml
QualityChecks:
  # Validation Rules
  validation:
    consistency_checks:
      - Same person basics match
      - No impossible dates
      - Compatible relationships
      - Source agreement
      
    completeness_checks:
      - Required fields present
      - Media references valid
      - Links accessible
      - Permissions verified
      
  # Monitoring
  link_monitoring:
    scheduled_checks:
      - Daily quick verify
      - Weekly deep check
      - Monthly full audit
      
    checks_performed:
      - Link accessibility
      - Data consistency
      - Change detection
      - Conflict identification
      
  # Repair Actions
  repair_options:
    - Update reference
    - Re-match entity
    - Mark as broken
    - Remove link
    - Manual intervention
```

### 3. Sync Management
```yaml
SyncManagement:
  # Sync Strategies
  strategies:
    push_only:
      - Send updates out
      - Never pull changes
      - Preserve local authority
      
    pull_only:
      - Monitor external changes
      - Import updates
      - Local read-only
      
    bidirectional:
      - Full sync both ways
      - Conflict resolution
      - Version management
      
    selective:
      - Sync specific fields
      - Ignore others
      - Custom rules
      
  # Conflict Resolution
  conflict_handling:
    detection:
      - Compare timestamps
      - Check version vectors
      - Identify differences
      
    resolution:
      automatic:
        - Newest wins
        - Source priority
        - Merge compatible
        
      manual:
        - Show differences
        - Choose version
        - Merge manually
        - Keep both
        
  # Change Propagation
  propagation:
    rules:
      - Which changes sync
      - Which direction
      - Transformation needed
      - Approval required
      
    scheduling:
      - Real-time
      - Batch hourly
      - Daily sync
      - Manual only
```

## Platform-Specific Implementations

### 1. FamilySearch Integration
```yaml
FamilySearchIntegration:
  # API Integration
  api_features:
    - Person read/write
    - Source attachment
    - Memory upload
    - Watch/notify
    
  # Link Structure
  link_format:
    person_link:
      system: "FamilySearch"
      type: "person"
      id: "KWQR-BYF"
      url: "https://familysearch.org/tree/person/KWQR-BYF"
      api_endpoint: "/platform/tree/persons/KWQR-BYF"
      
  # Sync Capabilities
  sync:
    - Pull person updates
    - Push source attachments
    - Memory synchronization
    - Change log tracking
```

### 2. Ancestry Integration
```yaml
AncestryIntegration:
  # Limited API
  capabilities:
    - Tree export/import
    - DNA match lists
    - Record hints
    - No direct write
    
  # Link Structure
  link_format:
    person_link:
      system: "Ancestry"
      tree_id: "12345678"
      person_id: "987654321"
      url: "https://www.ancestry.com/family-tree/person/tree/12345678/person/987654321"
      
  # Workarounds
  sync_methods:
    - GEDCOM export/import
    - Browser automation
    - Manual verification
    - Screenshot proof
```

### 3. WikiTree Integration
```yaml
WikiTreeIntegration:
  # Open API
  capabilities:
    - Profile read/write
    - Relationship access
    - Biography editing
    - Source management
    
  # Link Structure
  link_format:
    person_link:
      system: "WikiTree"
      wiki_id: "Smith-12345"
      url: "https://www.wikitree.com/wiki/Smith-12345"
      api_endpoint: "/api.php?action=getPerson&key=Smith-12345"
      
  # Collaboration
  features:
    - Trusted list sync
    - Biography updates
    - Source sharing
    - Privacy compliance
```

### 4. Local Database Integration
```yaml
LocalDatabaseIntegration:
  # Direct SQL Access
  connection:
    type: "SQLite"
    path: "/Users/john/Genealogy/family.db"
    schema_version: "3.2"
    
  # Mapping Configuration
  mappings:
    person_table:
      table: "tbl_people"
      fields:
        id: "person_id"
        name: "full_name"
        birth_date: "birth_date"
        death_date: "death_date"
        
    link_query: |
      SELECT * FROM tbl_people 
      WHERE person_id = ?
      
  # Sync Rules
  sync:
    - Monitor file changes
    - Backup before write
    - Transaction safety
    - Rollback capability
```

## GEDCOM 7 Extension for Links
```yaml
GEDCOM7LinkExtension:
  # External Link Structure
  0 @I1@ INDI
  1 NAME John /Smith/
  1 _LINK @L1@
  2 _SYSTEM FamilySearch
  2 _TYPE person
  2 _ID KWQR-BYF
  2 _URL https://familysearch.org/tree/person/KWQR-BYF
  2 _VERIFIED 2025-07-29T10:30:00Z
  2 _SYNC bidirectional
  1 _LINK @L2@
  2 _SYSTEM Ancestry
  2 _TREE_ID 12345678
  2 _PERSON_ID 987654321
  2 _URL https://www.ancestry.com/family-tree/person/tree/12345678/person/987654321
  2 _SYNC pull_only
  1 _LINK @L3@
  2 _SYSTEM LocalDatabase
  2 _TYPE SQLite
  2 _PATH /Users/john/Genealogy/family.db
  2 _TABLE tbl_people
  2 _ID 5467
  2 _SYNC bidirectional
  
  # Link Metadata
  0 @L1@ _LINK_META
  1 _CREATED 2025-07-20T08:00:00Z
  1 _CREATOR John Smith
  1 _CONFIDENCE 95
  1 _LAST_VERIFIED 2025-07-29T10:30:00Z
  1 _STATUS verified
```

## User Interface Components

### 1. Link Manager
```yaml
LinkManagerUI:
  # Dashboard View
  dashboard:
    - Total links by platform
    - Health status overview
    - Recent changes
    - Broken links alert
    - Sync queue status
    
  # Link Browser
  browser:
    - Filter by platform
    - Sort by status
    - Search capabilities
    - Bulk operations
    - Quick actions
    
  # Link Editor
  editor:
    - Connection details
    - Mapping configuration
    - Sync preferences
    - Test connection
    - Manual sync trigger
```

### 2. Linking Wizard
```yaml
LinkingWizard:
  steps:
    1_select_source:
      - Choose entity type
      - Select specific entity
      - Review entity data
      
    2_choose_platform:
      - Available platforms
      - Connection requirements
      - Feature comparison
      
    3_find_match:
      - Search interface
      - Suggested matches
      - Confidence scores
      - Manual selection
      
    4_configure_sync:
      - Sync direction
      - Field mapping
      - Conflict rules
      - Schedule options
      
    5_confirm_create:
      - Review summary
      - Test connection
      - Create link
      - Initial sync
```

### 3. Monitoring Interface
```yaml
MonitoringUI:
  # Status Dashboard
  status:
    - Real-time sync status
    - Queue depth
    - Error alerts
    - Performance metrics
    
  # Change Log
  changes:
    - Recent syncs
    - What changed
    - Conflict history
    - Rollback options
    
  # Health Checks
  health:
    - Platform availability
    - Link verification
    - Data consistency
    - Repair suggestions
```

## Benefits

### 1. Unified Research Environment
- Work across platforms seamlessly
- No data silos
- Comprehensive view of all research
- Leverage each platform's strengths

### 2. Data Integrity
- Prevent duplicate work
- Maintain consistency
- Track all changes
- Preserve attribution

### 3. Collaboration Enhancement
- Share across platforms
- Coordinate with other researchers
- Maintain authoritative source
- Enable peer review

This Cross-Platform Linkage System transforms ResearchProcess-GPS into a hub that connects all genealogical data sources while respecting platform differences and maintaining data integrity.