# Identity & Persona System Specification

## Overview

The Identity/Persona system manages uncertain, fragmentary, and evolving information about people during the research process. Unlike traditional person records that assume consolidated conclusions, this system supports the messy reality of genealogical research where the same person may appear under different names, in different contexts, with conflicting information.

## Core Concepts

### 1. Identity (Mystery Person)
```yaml
Identity:
  # Core Identification
  identity_id: UUID
  created_date: timestamp
  created_by: researcher_id
  last_modified: timestamp
  research_status: enum
    # "active", "pending_merge", "merged", "abandoned", "resolved"
    
  # What We Know
  known_facts:
    names:
      - name_text: string
        name_type: string  # "given", "maiden", "married", "alias", "nickname"
        source_reference: source_id
        confidence: percent
        context: text  # "listed as", "signed as", "called"
        
    temporal_markers:
      - event_type: string
        date: fuzzy_date
        place: place_reference
        age_given: string  # "28", "adult", "infant"
        calculated_birth_range: date_range
        source: source_id
        
    attributes:
      - attribute_type: string  # "occupation", "religion", "race"
        value: string
        date_applicable: fuzzy_date
        source: source_id
        confidence: percent
        
  # Where Found
  source_appearances:
    - source_id: UUID
      appearance_type: string  # "primary_subject", "witness", "mentioned"
      extracted_text: text
      image_coordinates: bbox  # For image sources
      transcription_confidence: percent
      context_persons: identity_id[]  # Others mentioned with them
      
  # Relationships (Uncertain)
  claimed_relationships:
    - relationship_type: string
      related_identity: identity_id
      related_person: person_id  # If known
      source: source_id
      confidence: percent
      notes: text
```

### 2. Persona (Evidence-Based Fragment)
```yaml
Persona:
  # Identification
  persona_id: UUID
  parent_identity: identity_id
  extraction_date: timestamp
  extracted_by: researcher_id
  
  # Single Source Extraction
  source:
    source_id: UUID
    source_type: string
    record_type: string  # "birth", "census", "will", etc.
    record_date: fuzzy_date
    
  # Extracted Information
  extracted_data:
    name_as_recorded: string
    name_variations: string[]  # Spelling variations in same source
    
    demographic_snapshot:
      age: string
      birthplace: string
      residence: place_reference
      occupation: string
      marital_status: string
      
    relationships_stated:
      - relationship: string
        to_name: string
        to_persona: persona_id  # If identified
        confidence: percent
        
  # Quality Assessment
  extraction_quality:
    legibility: percent
    completeness: percent
    internal_consistency: boolean
    extraction_method: string  # "manual", "OCR", "AI"
    verification_status: string
```

### 3. Identity Cluster
```yaml
IdentityCluster:
  cluster_id: UUID
  cluster_status: enum
    # "analyzing", "probable_match", "confirmed", "split"
    
  # Members
  member_identities:
    - identity_id: UUID
      inclusion_confidence: percent
      added_by: researcher_id
      added_date: timestamp
      
  member_personas:
    - persona_id: UUID
      assigned_identity: identity_id
      assignment_confidence: percent
      
  # Analysis
  correlation_analysis:
    name_similarity_matrix: JSON
    temporal_consistency: percent
    geographic_clustering: float
    relationship_network_overlap: percent
    
  merge_analysis:
    merge_confidence: percent
    supporting_factors: text[]
    conflicting_factors: text[]
    requires_resolution: conflict_id[]
    
  # Target Person
  resolved_to_person: person_id
  resolution_confidence: percent
  resolution_date: timestamp
  resolution_proof: proof_id
```

## Identity Resolution Process

### 1. Correlation Engine
```yaml
CorrelationEngine:
  # Name Matching
  name_analysis:
    algorithms:
      - exact_match: weight
      - soundex: weight
      - levenshtein: weight
      - nickname_variants: weight
      - cultural_variants: weight  # Johann/John, etc.
      
    contextual_factors:
      - name_frequency: float  # Common vs rare names
      - cultural_naming_patterns: string
      - time_period_variants: boolean
      
  # Temporal Analysis
  temporal_analysis:
    birth_year_estimation:
      - from_age_statements: year_range[]
      - from_children_births: year_range[]
      - from_marriage_dates: year_range[]
      
    lifespan_consistency:
      - max_reasonable_age: integer
      - typical_lifespan: integer
      - mortality_patterns: JSON
      
  # Geographic Analysis
  geographic_analysis:
    location_clustering:
      - distance_threshold: miles
      - migration_patterns: JSON
      - transportation_available: string[]
      
  # Relationship Network
  network_analysis:
    shared_associates: person_id[]
    family_reconstruction: JSON
    social_network_overlap: percent
```

### 2. Conflict Detection
```yaml
ConflictDetection:
  conflict_types:
    - temporal_impossibility:
        description: "Cannot be in two places at once"
        detection_rule: JSON
        
    - biological_impossibility:
        description: "Children born too close together"
        detection_rule: JSON
        
    - name_incompatibility:
        description: "Completely different names, same timeframe"
        detection_rule: JSON
        
  conflict_record:
    conflict_id: UUID
    identities_involved: identity_id[]
    conflict_type: string
    evidence_items: evidence_id[]
    
  resolution_strategies:
    - split_identity:
        create_new_identity: boolean
        reassign_personas: persona_id[]
        
    - prefer_evidence:
        preferred_source_hierarchy: string[]
        quality_threshold: percent
        
    - document_uncertainty:
        add_research_note: text
        flag_for_review: boolean
```

### 3. Progressive Merging
```yaml
MergeProcess:
  merge_stages:
    - tentative_link:
        confidence_required: 50%
        creates_watch_list: boolean
        
    - probable_same:
        confidence_required: 70%
        combines_research: boolean
        maintains_separate: boolean
        
    - confirmed_same:
        confidence_required: 85%
        full_merge: boolean
        creates_person: boolean
        
  merge_record:
    merge_id: UUID
    source_identities: identity_id[]
    target_identity: identity_id
    merge_date: timestamp
    merge_rationale: text
    evidence_basis: evidence_id[]
    
  rollback_capability:
    can_unmerge: boolean
    preservation_period: days
    archived_state: JSON
```

## Research Integration

### 1. Research Tracking
```yaml
IdentityResearch:
  research_priority: enum
    # "critical", "high", "medium", "low", "resolved"
    
  research_questions:
    - question: text
      priority: integer
      assigned_to: researcher_id
      target_date: date
      
  research_log_entries: research_log_id[]
  
  breakthrough_potential:
    - record_type_needed: string
      likely_location: place_id
      probability_exists: percent
```

### 2. Hypothesis Management
```yaml
IdentityHypothesis:
  hypothesis_types:
    - same_person_as:
        other_identity: identity_id
        probability: percent
        test_plan: text
        
    - child_of:
        parent_identities: identity_id[]
        supporting_evidence: evidence_id[]
        
    - relocated_from:
        origin_place: place_id
        destination_place: place_id
        migration_date: fuzzy_date
        
  hypothesis_testing:
    - test_description: text
      expected_result: text
      actual_result: text
      conclusion: enum  # "supports", "refutes", "inconclusive"
```

### 3. Collaborative Features
```yaml
CollaborativeIdentity:
  visibility: enum
    # "private", "team", "public"
    
  collaboration_notes:
    - note_id: UUID
      author: researcher_id
      date: timestamp
      content: text
      tags: string[]
      
  proposed_merges:
    - proposal_id: UUID
      proposer: researcher_id
      target_identities: identity_id[]
      rationale: text
      votes:
        - voter: researcher_id
          vote: enum  # "agree", "disagree", "needs_work"
          comment: text
```

## Advanced Features

### 1. AI-Assisted Correlation
```yaml
AICorrelation:
  feature_extraction:
    - facial_recognition:  # For photos
        confidence: percent
        facial_features: vector
        
    - handwriting_analysis:  # For signatures
        similarity_score: percent
        characteristics: JSON
        
    - context_analysis:
        mentioned_entities: entity[]
        event_patterns: pattern[]
        linguistic_markers: string[]
        
  suggestion_engine:
    - possible_matches: identity_id[]
    - confidence_scores: percent[]
    - reasoning: text[]
```

### 2. Smart Clustering
```yaml
SmartClustering:
  clustering_methods:
    - hierarchical:
        distance_metric: string
        linkage_method: string
        
    - density_based:
        min_points: integer
        epsilon: float
        
    - graph_based:
        edge_weights: string  # Property to use
        community_detection: string
        
  auto_cluster_triggers:
    - new_persona_added: boolean
    - confidence_threshold_met: boolean
    - time_based: days
```

### 3. Evidence Visualization
```yaml
IdentityVisualization:
  views:
    - timeline_view:
        show_all_appearances: boolean
        highlight_conflicts: boolean
        
    - geographic_view:
        plot_all_locations: boolean
        show_migration_paths: boolean
        
    - relationship_network:
        include_uncertain: boolean
        edge_confidence_display: boolean
        
    - source_matrix:
        sources_as_columns: boolean
        facts_as_rows: boolean
        confidence_heat_map: boolean
```

## Quality Assurance

### 1. Identity Metrics
```yaml
QualityMetrics:
  completeness_score:
    - has_birth_info: weight
    - has_death_info: weight
    - has_parents: weight
    - has_spouse: weight
    - source_diversity: weight
    
  confidence_metrics:
    - overall_confidence: percent
    - name_confidence: percent
    - date_confidence: percent
    - relationship_confidence: percent
    
  research_coverage:
    - vital_records_checked: boolean
    - census_coverage: percent
    - local_records_checked: boolean
```

### 2. Warning Systems
```yaml
WarningSystem:
  automatic_flags:
    - impossible_dates: severity
    - missing_expected_records: severity
    - unusual_patterns: severity
    - merge_conflicts: severity
    
  research_alerts:
    - new_records_available: boolean
    - related_identity_updated: boolean
    - conflict_detected: boolean
```

## Export/Import

### 1. Identity Exchange Format
```yaml
ExchangeFormat:
  identity_package:
    - identities: identity[]
    - personas: persona[]
    - correlations: correlation[]
    - evidence: evidence[]
    - research_notes: note[]
    
  format_options:
    - JSON-LD: boolean
    - XML: boolean
    - GEDCOM_X: boolean
```

### 2. Person Record Generation
```yaml
PersonGeneration:
  synthesis_rules:
    - confidence_threshold: 85%
    - conflict_resolution: "document_all"
    - uncertain_facts: "include_with_note"
    
  generated_person:
    - consolidated_name: string
    - birth: event
    - death: event
    - relationships: relationship[]
    - identity_history: identity_id[]
    - synthesis_notes: text
```

This Identity & Persona System provides a robust framework for handling the uncertainty and complexity inherent in genealogical research, supporting the journey from fragmentary evidence to confident conclusions while maintaining the full research trail.