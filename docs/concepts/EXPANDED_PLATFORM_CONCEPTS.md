# Expanded Platform Concepts for ResearchProcess-GPS

## Core Entity Model

### 1. Enhanced Person Entity
```yaml
Person:
  # Standard GEDCOM-like fields
  - person_id
  - names[]
  - events[]
  - attributes[]
  
  # Research extensions
  - confidence_level
  - identity_links[]
  - hypothesis_state
  - research_status
  - privacy_level
```

### 2. Identity System (Mystery People)
```yaml
Identity:
  - identity_id
  - known_facts[]
  - evidence_items[]
  - linked_identities[]
  - possible_persons[]
  - confidence_scores{}
  - source_mentions[]
  - clustering_data
  
  # Identity Resolution
  IdentityCluster:
    - cluster_id
    - member_identities[]
    - merge_confidence
    - conflict_points[]
    - resolution_history[]
```

### 3. Flexible Link System (Beyond GEDCOM)
```yaml
FlexibleLink:
  - link_id
  - link_type (custom definable)
  - participants[]
    - entity_type (person/identity/source/place/etc)
    - entity_id
    - role_in_link
    - confidence
  - temporal_scope
  - spatial_scope
  - evidence_basis[]
  - link_strength
  - bidirectional: boolean
  
  # Example Link Types
  LinkTypes:
    - "possibly_same_person"
    - "mentioned_together_in"
    - "shared_household_member"
    - "dna_match_unplaced"
    - "conflicting_parent_claim"
    - "business_partner_of"
    - "godparent_to"
    - "witnessed_document_for"
    - "traveled_with"
    - [user_defined_types]
```

## Research Documentation System

### 1. Research Log Architecture
```yaml
ResearchLog:
  # Core Fields (BCG Standards)
  - log_id
  - date_time (precise timestamp)
  - researcher_id
  - research_goal
  - repository_type (online/offline/personal)
  - repository_name
  - repository_location (URL/address)
  - call_numbers[]
  - search_parameters
    - search_terms[]
    - date_ranges[]
    - geographic_limits[]
    - record_types[]
  - results_summary
  - documents_found[]
  - negative_result: boolean
  - follow_up_needed[]
  - time_spent
  
  # Auto-capture fields
  - browser_session_id
  - screenshots[]
  - scraped_data{}
  - page_urls_visited[]
  - search_queries_used[]
  - interaction_timeline[]
```

### 2. Evidence & Analysis
```yaml
Evidence:
  - evidence_id
  - source_reference
  - information_items[]
  - extraction_method (manual/OCR/AI/scraped)
  - extraction_confidence
  - negative_evidence_flag
  - contradicts[]
  - supports[]
  - quality_assessment
    - source_type (original/derivative/authored)
    - information_type (primary/secondary)
    - evidence_type (direct/indirect/negative)
  
NegativeEvidence:
  - search_performed
  - expected_finding
  - actual_result: "not found"
  - significance
  - thoroughness_score
```

### 3. Theory Versioning System
```yaml
TheoryVersion:
  - version_id
  - theory_type (hypothesis/relationship/identity)
  - version_number
  - timestamp
  - author_id
  - change_summary
  - previous_version_id
  - evidence_added[]
  - evidence_removed[]
  - confidence_change
  - peer_reviews[]
  
TheoryBranch:
  - branch_id
  - base_theory_id
  - divergence_point
  - alternative_interpretation
  - supporting_researchers[]
```

## Browser Integration & Auto-Capture

### 1. Research Browser Extension
```yaml
BrowserExtension:
  AutoCapture:
    - URL tracking
    - Time on page
    - Scroll depth
    - Click patterns
    - Form submissions
    - Search queries
    
  SmartScraping:
    - Record detection (birth/death/marriage)
    - Name extraction
    - Date parsing
    - Location normalization
    - Relationship inference
    
  SessionManagement:
    - Research goal association
    - Automatic categorization
    - Screenshot triggers
    - Data preservation
```

### 2. AI Integration Layer
```yaml
AIServices:
  DocumentAnalysis:
    - OCR with confidence scores
    - Handwriting recognition
    - Language detection/translation
    - Entity extraction
    
  PatternRecognition:
    - Record type identification
    - Duplicate detection
    - Anomaly flagging
    - Relationship inference
    
  ResearchAssistant:
    - Next step suggestions
    - Source recommendations
    - Conflict detection
    - Hypothesis generation
```

## DNA Model System

### 1. Multi-Type DNA Support
```yaml
DNAModel:
  TestTypes:
    Autosomal:
      - match_list[]
      - shared_cm
      - shared_segments[]
      - triangulation_groups[]
      - ethnicity_estimates{}
      
    YDNA:
      - haplogroup
      - str_markers{}
      - snp_results[]
      - genetic_distance_matches[]
      
    MtDNA:
      - haplogroup
      - hvr1_mutations[]
      - hvr2_mutations[]
      - full_sequence_matches[]
      
    XDNA:
      - match_segments[]
      - inheritance_path_constraints
```

### 2. DNA Analysis Tools
```yaml
DNAAnalysis:
  ClusteringTools:
    - Leeds_method
    - Auto_clustering
    - Shared_match_analysis
    
  TriangulationTools:
    - Segment_triangulation
    - Group_identification
    - MRCA_estimation
    
  InheritanceMapping:
    - Visual_chromosome_browser
    - Segment_tracking
    - Ancestor_assignment
```

## Compliance & Standards

### 1. GPS Compliance Engine
```yaml
GPSCompliance:
  Criteria:
    - Reasonably_exhaustive_search
      - repositories_checked[]
      - search_completeness_score
      - peer_review_validation
      
    - Complete_accurate_citations
      - citation_completeness_check
      - format_validation
      - source_quality_assessment
      
    - Analysis_correlation
      - evidence_items_analyzed[]
      - correlation_matrix
      - conflict_resolution[]
      
    - Conflict_resolution
      - conflicts_identified[]
      - resolution_reasoning[]
      - remaining_uncertainties[]
      
    - Sound_written_conclusion
      - proof_argument_quality
      - logic_flow_analysis
      - conclusion_support_score
```

### 2. Privacy Compliance
```yaml
PrivacyCompliance:
  LivingPersonProtection:
    - Auto_detection_rules
    - Redaction_levels
    - Access_controls
    - Export_restrictions
    
  GDPR_Support:
    - Consent_tracking
    - Data_portability
    - Right_to_deletion
    - Audit_trail
    
  Ethical_Guidelines:
    - Adoptee_protection
    - DNA_surprise_handling
    - Sensitive_information_flags
```

## Composition & Publishing Module

### 1. Document Types
```yaml
CompositionModule:
  OutputFormats:
    ResearchLog:
      - BCG_standard_format
      - Custom_templates
      - Export_formats[]
      
    ProofArgument:
      - Narrative_style
      - Academic_format
      - Legal_brief_style
      
    ResearchReport:
      - Client_report_template
      - Family_history_narrative
      - Academic_paper_format
      
    ScholaryArticle:
      - Journal_templates[]
      - Citation_styles[]
      - Peer_review_ready
```

### 2. Advanced Composition Features
```yaml
PublishingTools:
  AnnotationSystem:
    - Footnotes/Endnotes
    - Marginalia
    - Hyperlinked_sources
    - Image_placement
    
  VisualizationEngine:
    - Relationship_charts
    - Timeline_graphics
    - Map_integration
    - DNA_diagrams
    
  CollaborativeEditing:
    - Multi_author_support
    - Version_control
    - Comment_threads
    - Approval_workflow
```

## Integration Architecture

### 1. Module System
```yaml
CoreModules:
  - Identity_Resolution
  - Link_Management
  - Research_Logger
  - Evidence_Analyzer
  - Theory_Versioning
  - DNA_Analytics
  - GPS_Compliance
  - Privacy_Manager
  - Composition_Engine
  
ExtensionPoints:
  - Custom_link_types
  - Analysis_algorithms
  - Export_formats
  - Visualization_types
  - AI_models
```

### 2. API Architecture
```yaml
APIs:
  ResearchAPI:
    - /identities
    - /links
    - /theories/versions
    - /research/logs
    - /evidence/analyze
    
  BrowserAPI:
    - /capture/start
    - /capture/screenshot
    - /scrape/analyze
    
  AIAPI:
    - /extract/entities
    - /analyze/document
    - /suggest/next
    
  CompositionAPI:
    - /compose/report
    - /export/format
    - /publish/validate
```

## Data Model Principles

### 1. Everything is Versioned
- All entities support full version history
- Branching for alternative theories
- Merge capabilities with conflict resolution

### 2. Everything is Linked
- No isolated data points
- Rich relationship model
- Confidence scores on all connections

### 3. Everything is Evidenced
- Source required for all claims
- Evidence quality tracked
- Negative evidence explicitly recorded

### 4. Everything is Collaborative
- Multi-user theory development
- Peer review at every level
- Attribution and credit tracking

This expanded concept creates a research platform that goes far beyond traditional genealogy software, supporting the full complexity of professional genealogical research while maintaining the flexibility to handle uncertain, theoretical, and collaborative work.