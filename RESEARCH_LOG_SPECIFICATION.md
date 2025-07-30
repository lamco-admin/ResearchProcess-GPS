# Research Log Specification for ResearchProcess-GPS

## Overview

The Research Log is the cornerstone of professional genealogical research. This specification defines a comprehensive, standards-based research log system that supports both manual and automated data capture.

## Core Fields (Based on BCG Standards)

### 1. Session Identification
```yaml
SessionInfo:
  log_entry_id: UUID
  session_id: UUID  # Groups related searches
  date: ISO-8601 date
  time_start: HH:MM:SS timezone
  time_end: HH:MM:SS timezone
  duration_minutes: calculated
  researcher_id: UUID
  researcher_name: string
  research_project_id: UUID
```

### 2. Research Goal & Context
```yaml
ResearchContext:
  research_goal: text
    # Examples:
    # - "Find birth record for John Smith"
    # - "Identify parents of Mary Jones"
    # - "Verify death date from obituary"
  
  research_question_id: UUID
  hypothesis_testing_id: UUID  # If testing specific theory
  
  subject_persons: 
    - person_id: UUID
      person_name: string
      role: string  # "primary", "spouse", "parent", etc.
  
  subject_identities:  # For unresolved people
    - identity_id: UUID
      known_as: string
  
  geographic_focus: 
    - place_name: string
      place_id: UUID
      jurisdiction_level: string
  
  temporal_focus:
    - date_range_start: date
    - date_range_end: date
    - date_precision: string  # "exact", "circa", "before", "after"
```

### 3. Repository Information
```yaml
RepositoryData:
  repository_type: enum
    # "online_database", "physical_archive", "library", 
    # "courthouse", "church", "cemetery", "personal_collection",
    # "government_website", "subscription_site", "free_website"
  
  repository_name: string
  repository_id: UUID  # For known repositories
  
  physical_location:
    address: string
    city: string
    state_province: string
    country: string
    postal_code: string
    gps_coordinates: lat/long
  
  online_location:
    base_url: URL
    database_name: string
    collection_name: string
    access_requirements: string  # "subscription", "free", "onsite_only"
    access_credentials_used: boolean  # Not the actual credentials
  
  contact_information:
    phone: string
    email: string
    hours: string
    access_notes: text
```

### 4. Search Details
```yaml
SearchParameters:
  call_numbers_consulted:
    - call_number: string
      description: string
      format: string  # "microfilm", "book", "digital", etc.
  
  search_strategy:
    search_type: enum
      # "name_search", "browse", "catalog_search", 
      # "full_text_search", "image_browse", "index_search"
    
    search_terms_used:
      - term: string
        field: string  # "surname", "given_name", "any_field", etc.
        variations_tried: string[]  # Wildcards, soundex, etc.
    
    filters_applied:
      - filter_type: string
        filter_value: string
    
    date_ranges_searched:
      - start_date: date
        end_date: date
        date_fields: string[]  # Which date fields searched
    
    geographic_limits:
      - place_name: string
        include_subdivisions: boolean
    
    record_types_targeted:
      - record_type: string
        sub_type: string
```

### 5. Results Summary
```yaml
SearchResults:
  result_summary: enum
    # "found_relevant", "found_partial", "found_unrelated", 
    # "no_results", "access_denied", "technical_error"
  
  documents_found:
    - document_id: UUID
      title: string
      relevance: enum  # "direct", "indirect", "peripheral"
      source_citation: text  # Full citation
      persistent_url: URL
      local_copy_path: path
      
  relevant_entries_found: integer
  total_entries_reviewed: integer
  
  negative_result_details:
    confirmed_absence: boolean
    search_thoroughness: percent
    limitations_encountered: text
    alternative_spellings_tried: string[]
    expanded_date_ranges_tried: string[]
```

### 6. Analysis & Next Steps
```yaml
AnalysisNotes:
  findings_summary: text
  
  evidence_quality_assessment:
    source_type: enum  # "original", "derivative", "authored"
    information_type: enum  # "primary", "secondary", "indeterminable"
    evidence_type: enum  # "direct", "indirect", "negative"
  
  conflicts_identified:
    - conflicting_information: text
      conflicts_with: reference
      possible_explanations: text[]
  
  new_leads_discovered:
    - lead_description: text
      priority: enum  # "high", "medium", "low"
      assigned_to: researcher_id
  
  follow_up_required:
    - task_description: text
      reason: text
      deadline: date
      assigned_to: researcher_id
```

### 7. Auto-Capture Fields
```yaml
AutoCaptureData:
  browser_session:
    session_id: UUID
    browser: string
    start_url: URL
    
  pages_visited:
    - url: URL
      title: string
      visit_time: timestamp
      time_on_page: seconds
      scrolled_percentage: percent
      screenshots_taken: path[]
      
  interactions_captured:
    - timestamp: timestamp
      action_type: string  # "search", "click", "form_submit"
      element_interacted: string
      data_submitted: JSON  # Sanitized
  
  scraped_content:
    - page_url: URL
      extraction_timestamp: timestamp
      extracted_data: JSON
      extraction_confidence: percent
      ai_interpretation: text
```

### 8. Research Log Metadata
```yaml
LogMetadata:
  created_timestamp: timestamp
  last_modified: timestamp
  modified_by: researcher_id
  
  review_status:
    reviewed: boolean
    reviewer_id: researcher_id
    review_date: date
    review_notes: text
  
  quality_indicators:
    completeness_score: percent
    citation_quality: percent
    gps_compliance: percent
  
  sharing_permissions:
    visibility: enum  # "private", "team", "public"
    shared_with: researcher_id[]
    embargo_until: date
  
  tags: string[]
  custom_fields: JSON
```

## Advanced Features

### 1. Bulk Operations Support
```yaml
BulkSearchLog:
  bulk_search_id: UUID
  individual_searches:
    - Modified search parameters for each
    - Aggregated results
    - Pattern detection across searches
```

### 2. Collaborative Research Logging
```yaml
CollaborativeFeatures:
  team_session:
    participants: researcher_id[]
    division_of_labor: text
    combined_findings: boolean
  
  peer_additions:
    - added_by: researcher_id
      addition_type: string
      content: text
      timestamp: timestamp
```

### 3. AI Enhancement Fields
```yaml
AIEnhancements:
  suggested_search_terms: string[]
  identified_patterns: text[]
  anomaly_flags: text[]
  next_step_recommendations: text[]
  
  auto_extracted_entities:
    persons: []
    places: []
    dates: []
    relationships: []
```

### 4. Integration Points
```yaml
SystemIntegration:
  linked_evidence_items: evidence_id[]
  linked_theories: theory_id[]
  linked_sources: source_id[]
  linked_tasks: task_id[]
  
  export_history:
    - export_date: timestamp
      export_format: string
      included_in_report: report_id
```

## Browser Extension Specification

### Auto-Capture Rules
```yaml
CaptureRules:
  triggers:
    - Start capture on research session begin
    - Screenshot on significant finds
    - Full capture on form submission
    - Pause on idle (configurable)
    
  privacy_rules:
    - No password capture
    - Sanitize personal data
    - Respect robots.txt
    - Honor privacy mode
    
  storage_rules:
    - Local encryption required
    - Cloud sync optional
    - Retention policies apply
    - User owns all data
```

## Best Practices Implementation

### 1. Entry Completeness
- Required fields clearly marked
- Validation on save
- Completeness scoring
- Prompts for missing data

### 2. Standardization
- Controlled vocabularies where appropriate
- Auto-complete from previous entries
- Template support for common searches
- Import/export standards compliance

### 3. Efficiency Features
- Quick entry modes
- Voice-to-text support
- Batch operations
- Keyboard shortcuts

### 4. Quality Assurance
- Automatic GPS compliance checking
- Citation format validation
- Duplicate search detection
- Research coverage analysis

This specification provides a comprehensive foundation for a professional research log system that meets BCG standards while leveraging modern technology for automation and efficiency.