# Linkage Utility Design Specification

## Overview

The Linkage Utility provides both manual and automated tools for creating, managing, and maintaining links between ResearchProcess-GPS and external genealogy systems. It includes quality checks, change detection, and intelligent re-matching capabilities.

## Core Components

### 1. Link Discovery Engine
```yaml
DiscoveryEngine:
  # Search Strategies
  search_methods:
    exact_match:
      - Name (with variants)
      - Dates (with fuzzy matching)
      - Places (with hierarchical matching)
      - IDs (if known)
      
    fuzzy_match:
      - Soundex/Metaphone
      - Levenshtein distance
      - Date proximity
      - Geographic proximity
      
    contextual_match:
      - Family member names
      - Associated events
      - Source citations
      - DNA matches
      
  # Scoring Algorithm
  scoring:
    weights:
      name_match: 0.3
      birth_date_match: 0.2
      death_date_match: 0.15
      location_match: 0.15
      family_match: 0.2
      
    thresholds:
      certain_match: 0.95
      probable_match: 0.85
      possible_match: 0.70
      review_needed: 0.50
```

### 2. Manual Linking Interface
```yaml
ManualLinkingUI:
  # Split Screen Comparison
  comparison_view:
    left_panel:
      - ResearchProcess-GPS entity
      - All known facts
      - Evidence summary
      - Confidence indicators
      
    right_panel:
      - External system browser
      - Search interface
      - Filter controls
      - Preview pane
      
    center_panel:
      - Match indicators
      - Difference highlighting
      - Action buttons
      - Notes field
      
  # Linking Workflow
  workflow:
    search:
      - Multi-field search form
      - Advanced query builder
      - Previous search history
      - Saved search templates
      
    review:
      - Side-by-side comparison
      - Fact-by-fact analysis
      - Source comparison
      - Family tree preview
      
    confirm:
      - Confidence selection
      - Sync preferences
      - Notes/rationale
      - Create link button
```

### 3. Automated Matching
```yaml
AutomatedMatching:
  # Batch Processing
  batch_operations:
    scope_selection:
      - All unlinked persons
      - Specific families
      - Date ranges
      - Geographic regions
      
    platform_targeting:
      - Single platform
      - Multiple platforms
      - Priority order
      - Parallel search
      
  # Match Queue
  queue_management:
    queue_entry:
      entity_id: UUID
      platforms_to_check: list
      priority: integer
      retry_count: integer
      
    processing:
      - Rate limiting
      - Error handling
      - Progress tracking
      - Result caching
      
  # Review Interface
  review_ui:
    match_list:
      - Confidence scores
      - Quick preview
      - Bulk actions
      - Filter/sort options
      
    actions:
      - Accept match
      - Reject match
      - Request human review
      - Modify and accept
```

## Quality Assurance System

### 1. Pre-Link Validation
```yaml
PreLinkValidation:
  # Data Compatibility Checks
  compatibility:
    temporal_validation:
      - Birth before death
      - Parent age feasibility
      - Event sequence logic
      - Generation spacing
      
    geographic_validation:
      - Location existence
      - Distance feasibility
      - Migration patterns
      - Historical boundaries
      
    relationship_validation:
      - No circular relationships
      - Biological possibility
      - Cultural appropriateness
      - Legal validity
      
  # Warning System
  warnings:
    levels:
      - Info (proceed)
      - Warning (review)
      - Error (block)
      
    examples:
      - "Birth dates differ by 2 years"
      - "Different death locations"
      - "Spouse name mismatch"
      - "Missing children in family"
```

### 2. Post-Link Monitoring
```yaml
LinkMonitoring:
  # Change Detection
  change_detection:
    monitoring_schedule:
      - Real-time (webhooks)
      - Hourly (API polls)
      - Daily (batch checks)
      - Weekly (deep scan)
      
    change_types:
      - Field updates
      - New relationships
      - Deleted data
      - Merge operations
      
  # Impact Analysis
  impact_assessment:
    severity_levels:
      minor:
        - Name spelling
        - Note additions
        - Source additions
        
      moderate:
        - Date changes
        - Place changes
        - New family members
        
      major:
        - Identity merge/split
        - Parent changes
        - Deletion threats
        
  # Alert Management
  alerts:
    delivery:
      - In-app notifications
      - Email summaries
      - SMS for critical
      - API webhooks
      
    actions:
      - Review change
      - Accept update
      - Reject change
      - Break link
```

### 3. Re-Linking Logic
```yaml
ReLinkingSystem:
  # Trigger Conditions
  triggers:
    automatic:
      - Broken link detected
      - Major change detected
      - Confidence drop
      - Platform migration
      
    manual:
      - User initiated
      - Bulk re-evaluation
      - Platform update
      - Schema change
      
  # Re-Link Process
  process:
    evaluation:
      - Current link validity
      - Changed field impact
      - New match search
      - Alternative candidates
      
    decision_tree:
      - If minor change → Update metadata
      - If moderate → Request review
      - If major → Full re-match
      - If broken → Archive and search
      
    preservation:
      - Archive old link
      - Document reason
      - Maintain history
      - Enable rollback
```

## Platform-Specific Handlers

### 1. API-Based Platforms
```yaml
APIHandlers:
  FamilySearch:
    capabilities:
      - Real-time webhooks
      - Change log API
      - Batch operations
      - Rate limiting
      
    special_handling:
      - Living person privacy
      - Ordinance data
      - Collaborative changes
      - Memory attachments
      
  WikiTree:
    capabilities:
      - Open API
      - Watchlist integration
      - Biography access
      - Relationship queries
      
    special_handling:
      - Privacy levels
      - Trusted lists
      - Project spaces
      - DNA confirmations
```

### 2. File-Based Systems
```yaml
FileHandlers:
  GEDCOM_Files:
    monitoring:
      - File system watch
      - Hash comparison
      - Lock file check
      - Backup creation
      
    parsing:
      - Version detection
      - Extension support
      - Custom tag handling
      - Media path resolution
      
  Database_Files:
    monitoring:
      - Transaction log
      - Schema version
      - Integrity check
      - Connection pool
      
    safety:
      - Read-only mode
      - Transaction wrapper
      - Rollback capability
      - Backup trigger
```

### 3. Web Scraping Fallback
```yaml
WebScrapingHandler:
  when_used:
    - No API available
    - API limitations
    - Additional data needed
    - Verification purposes
    
  implementation:
    - Headless browser
    - Session management
    - Rate limiting
    - Error recovery
    
  extraction:
    - CSS selectors
    - XPath queries
    - Pattern matching
    - AI-assisted parsing
    
  compliance:
    - Robots.txt respect
    - Rate limiting
    - User agent honesty
    - Terms compliance
```

## User Experience Design

### 1. Link Management Dashboard
```yaml
Dashboard:
  overview_widgets:
    - Total links by status
    - Recent activity
    - Health score
    - Action needed count
    
  platform_cards:
    - Platform name/logo
    - Link count
    - Last sync time
    - Status indicator
    - Quick actions
    
  activity_feed:
    - Recent changes
    - New matches found
    - Errors/warnings
    - User actions
```

### 2. Linking Wizard
```yaml
LinkingWizard:
  intelligent_defaults:
    - Pre-select likely platforms
    - Auto-fill search fields
    - Suggest match criteria
    - Remember preferences
    
  guided_process:
    - Clear step indicators
    - Help at each step
    - Preview before commit
    - Undo capability
    
  bulk_operations:
    - CSV import/export
    - Template system
    - Batch review
    - Progress tracking
```

### 3. Quality Review Interface
```yaml
QualityReview:
  change_visualization:
    - Red/green diff view
    - Timeline comparison
    - Map view for places
    - Family tree overlay
    
  decision_support:
    - Impact preview
    - Confidence scoring
    - Similar case history
    - Expert system advice
    
  workflow_integration:
    - Task assignment
    - Review queues
    - Approval chains
    - Audit trail
```

## Performance Optimization

### 1. Caching Strategy
```yaml
Caching:
  cache_layers:
    - Memory cache (hot data)
    - Redis cache (session data)
    - Database cache (results)
    - CDN cache (platform data)
    
  cache_keys:
    - Platform + Entity ID
    - Search parameters
    - Match results
    - Validation results
    
  invalidation:
    - Time-based expiry
    - Event-based clear
    - Manual refresh
    - Cascade updates
```

### 2. Batch Processing
```yaml
BatchProcessing:
  queue_optimization:
    - Platform batching
    - Geographic grouping
    - Priority scheduling
    - Resource pooling
    
  parallel_execution:
    - Multi-threaded search
    - Distributed workers
    - Rate limit sharing
    - Result aggregation
```

## Integration with ResearchProcess-GPS

### 1. Entity Enhancement
```yaml
EntityEnhancement:
  link_summary:
    - Platform badges
    - Sync status icons
    - Last update time
    - Quick link menu
    
  evidence_integration:
    - External sources
    - Platform citations
    - Cross-references
    - Verification status
```

### 2. Research Workflow
```yaml
ResearchIntegration:
  research_aids:
    - Check external platforms
    - Find additional records
    - Verify conclusions
    - Monitor changes
    
  collaboration:
    - Share links
    - Coordinate updates
    - Resolve conflicts
    - Maintain consensus
```

This Linkage Utility Design provides a comprehensive system for managing connections between ResearchProcess-GPS and external platforms, ensuring data quality while supporting both manual and automated workflows.