# Unleashed Core Data Model for ResearchProcess-GPS

## Design Philosophy

We're taking familiar genealogical concepts (persons, relationships, sources, events, etc.) and **unleashing them** from traditional constraints. Every entity is:
- **Versionable** - can exist in multiple theoretical states
- **Collaborative** - supports multi-researcher work with attribution
- **Evidential** - everything has provenance and confidence
- **Flexible** - no artificial constraints on relationships or structures
- **Standards-compliant** - GPS/BCG methodology built into the core

## Core Entity Architecture

### 1. Identity (The Unleashed Person)

```yaml
Identity:
  # Core Identification
  id: UUID
  type: "Identity"
  
  # Versioning & Theory Support
  existence_status:
    - status: enum ["hypothetical", "evidenced", "confirmed", "disproven", "merged"]
    - theory_contexts: [theory_id]  # Can exist differently in different theories
    - confidence_level: confidence_container
  
  # Multi-dimensional Naming
  names:
    - name_id: UUID
    - name_components:
        given: [string]  # Multiple given names
        surnames: [string]  # Multiple surnames
        titles: [string]
        suffixes: [string]
        nicknames: [string]
    - cultural_context: string  # "English", "Chinese", "Spanish"
    - name_type: string  # "birth", "married", "professional", "religious"
    - usage_period: temporal_range
    - source_attestations: [evidence_link]
    - confidence: confidence_container
    - theories_applicable: [theory_id]
  
  # Temporal Existence
  existence_periods:
    - period_type: string  # "biological", "legal", "social"
    - start_event: event_link  # Birth, creation, first evidence
    - end_event: event_link  # Death, dissolution, last evidence
    - certainty: confidence_container
    - applicable_theories: [theory_id]
  
  # Biological/Genetic Information
  biological_profile:
    sex:
      - biological_sex: string  # "male", "female", "intersex", "unknown"
      - chromosomal: string  # If DNA tested
      - phenotypical: string  # As observed/recorded
      - certainty: confidence_container
    genetic_markers:
      - marker_type: string  # Y-DNA, mtDNA, autosomal
      - values: json
      - test_source: evidence_link
      - matches: [identity_link]
  
  # Social Identity
  social_identities:
    - identity_type: string  # "legal", "professional", "religious", "cultural"
    - attributes: json  # Flexible attributes per identity type
    - period: temporal_range
    - geographic_scope: location_link
    - source_evidence: [evidence_link]
```

### 2. Relationship (Unleashed from Family Constraints)

```yaml
Relationship:
  # Core
  id: UUID
  type: "Relationship"
  
  # Multi-party Relationships (not just two people)
  participants:
    - identity: identity_link
    - role: string  # "parent", "child", "spouse", "partner", "guardian", etc.
    - role_period: temporal_range
    - certainty: confidence_container
    - theories: [theory_id]
  
  # Relationship Classification
  relationship_type:
    primary_type: string  # "biological", "legal", "social", "spiritual"
    subtype: string  # "marriage", "adoption", "fostering", "apprenticeship"
    cultural_context: string
    legal_framework: string  # Jurisdiction/time period
  
  # Temporal Dynamics
  relationship_events:
    - event: event_link  # Marriage, divorce, adoption decree
    - effect: string  # "initiated", "modified", "terminated"
  
  # Evidence and Confidence
  supporting_evidence: [evidence_link]
  conflicting_evidence: [evidence_link]
  researcher_notes: [note]
  peer_reviews: [review]
  confidence: confidence_container
  
  # Theory Support
  theories_containing: [theory_id]
  alternative_interpretations: [relationship_link]
```

### 3. Event (Unleashed from Person/Family Ownership)

```yaml
Event:
  # Core
  id: UUID
  type: "Event"
  
  # Event Classification
  event_classification:
    category: string  # "vital", "civic", "religious", "military", "educational"
    specific_type: string  # "birth", "baptism", "census", "graduation"
    cultural_significance: string
    
  # Temporal Complexity
  temporal_data:
    primary_date: temporal_point  # Best estimated date/time
    date_as_recorded: [string]  # Original text
    calendar_system: string  # Gregorian, Julian, Hebrew, etc.
    precision_level: string  # "exact", "day", "month", "year", "decade"
    sequence_markers: [string]  # "before", "after", "during"
    duration: temporal_duration  # For extended events
    
  # Spatial Complexity  
  spatial_data:
    primary_location: location_link
    location_as_recorded: [string]
    location_type: string  # "occurrence", "registration", "reporting"
    geographic_precision: string  # "exact", "town", "county", "region"
    jurisdiction_at_time: jurisdiction_link
    
  # Participation (Unlimited, Role-based)
  participants:
    - identity: identity_link
    - role: string  # "principal", "witness", "officiant", "informant"
    - role_certainty: confidence_container
    - age_at_event: string  # As recorded
    - presence_type: string  # "physical", "legal", "proxy"
    - theories: [theory_id]
    
  # Rich Evidence Links
  evidence_trail:
    primary_sources: [evidence_link]
    derivative_sources: [evidence_link]
    negative_evidence: [evidence_link]  # Searches that found nothing
    
  # Research Metadata
  extraction_history:
    - researcher: researcher_id
    - extraction_date: timestamp
    - interpretation_notes: text
    - confidence_assessment: confidence_container
```

### 4. Evidence (First-Class Research Object)

```yaml
Evidence:
  # Core
  id: UUID
  type: "Evidence"
  
  # Source Classification
  source_classification:
    source_type: string  # "original", "derivative", "authored"
    record_type: string  # "birth_certificate", "census", "newspaper"
    information_type: string  # "primary", "secondary", "indeterminate"
    evidence_type: string  # "direct", "indirect", "negative"
    
  # Provenance Chain
  provenance:
    original_creator: string  # Person/organization who created
    creation_context: string  # Why/how created
    chain_of_custody: [custodian]
    current_repository: repository_link
    access_restrictions: [string]
    
  # Content Representation
  content:
    transcription: text  # Full transcription
    abstract: text  # Summary of relevant info
    translations: [{language: string, text: text}]
    images: [media_link]
    structured_data: json  # Extracted structured information
    
  # Extraction Points (What was found)
  extracted_facts:
    - fact_id: UUID
    - fact_type: string  # "name", "date", "place", "relationship"
    - extracted_value: string
    - normalized_value: string
    - applies_to: identity_link
    - confidence: confidence_container
    - extraction_notes: text
    
  # Quality Assessment
  quality_metrics:
    legibility: percent
    completeness: percent
    reliability_score: percent
    researcher_assessment: text
    peer_reviews: [review]
    
  # Research Process
  research_trail:
    found_date: date
    found_by: researcher_id
    search_context: text  # What they were looking for
    repository_visit: visit_id
    negative_result: boolean  # Important: recording what wasn't found
```

### 5. Location (Unleashed Spatial-Temporal Entity)

```yaml
Location:
  # Core
  id: UUID
  type: "Location"
  
  # Spatial Representation
  spatial_definitions:
    - definition_type: string  # "point", "boundary", "region"
    - coordinates: geometry  # PostGIS geometry
    - coordinate_system: string
    - precision: string
    - valid_period: temporal_range
    - source: evidence_link
    
  # Place Names (Historical)
  place_names:
    - name: string
    - language: string
    - name_type: string  # "official", "colloquial", "historical"
    - usage_period: temporal_range
    - cultural_group: string
    - source_attestation: evidence_link
    
  # Jurisdictional History
  jurisdictions:
    - jurisdiction_type: string  # "civil", "religious", "military"
    - parent_jurisdiction: location_link
    - governing_body: string
    - legal_framework: string
    - period: temporal_range
    - boundary_changes: [boundary_change]
    
  # Cultural Geography
  cultural_associations:
    - culture_group: string
    - association_type: string  # "homeland", "diaspora", "colonial"
    - period: temporal_range
    - demographic_notes: text
    - sources: [evidence_link]
```

### 6. Theory (The Innovation Container)

```yaml
Theory:
  # Core
  id: UUID
  type: "Theory"
  
  # Theory Definition
  hypothesis:
    statement: text  # "What if John Smith died in 1853?"
    assumptions: [text]
    testable_predictions: [text]
    
  # Theory Scope
  scope:
    identities_affected: [identity_link]
    relationships_affected: [relationship_link]
    events_reinterpreted: [event_link]
    time_period: temporal_range
    geographic_scope: [location_link]
    
  # Version Control
  versioning:
    parent_theory: theory_link  # Branched from
    version_number: string
    created_date: timestamp
    created_by: researcher_id
    change_description: text
    
  # Evidence Assessment
  evidence_evaluation:
    supporting_evidence: 
      - evidence: evidence_link
        relevance_score: percent
        interpretation: text
    contradicting_evidence:
      - evidence: evidence_link
      - conflict_description: text
      - proposed_resolution: text
    evidence_gaps: [text]  # What's still needed
    
  # Theory State
  status:
    current_state: enum ["draft", "testing", "supported", "refuted", "merged"]
    confidence_level: percent
    peer_reviews: [review]
    test_results: [test_result]
    
  # Comparison Metrics
  comparison_data:
    gps_compliance_score: percent
    evidence_coverage: percent
    conflict_count: integer
    researcher_consensus: percent
```

### 7. Confidence Container (Not Just a Number)

```yaml
ConfidenceContainer:
  # Overall Assessment
  summary_confidence: percent  # 0-100
  
  # Detailed Breakdown
  evidence_quality:
    source_reliability: percent
    information_credibility: percent
    evidence_directness: percent
    
  # Research Completeness  
  research_coverage:
    geographic_coverage: [location: percent]
    temporal_coverage: [period: percent]
    repository_coverage: [repository: boolean]
    record_type_coverage: [type: boolean]
    
  # Specific Checks (The Innovation)
  audit_checklist:
    - check_item: "Checked Smith County birth records 1850-1860"
    - completed: boolean
    - completed_by: researcher_id
    - completed_date: date
    - result: text
    - next_steps: text
    
  # Peer Assessment
  peer_confidence:
    - reviewer: researcher_id
    - confidence_given: percent
    - review_notes: text
    - suggestions: [text]
    
  # GPS Compliance
  gps_elements:
    reasonably_exhaustive_research: 
      status: boolean
      evidence: text
    complete_accurate_citations:
      status: boolean
      issues: [text]
    thorough_analysis_correlation:
      status: boolean
      description: text
    conflict_resolution:
      conflicts_identified: [text]
      resolutions: [text]
    sound_written_conclusion:
      status: boolean
      location: text
```

## Collaboration & Security Layer

```yaml
CollaborationFramework:
  # Attribution (Everything tracked)
  attribution_model:
    - action_type: string  # "created", "edited", "reviewed", "merged"
    - entity_type: string
    - entity_id: UUID
    - actor: researcher_id
    - timestamp: timestamp
    - change_description: text
    - theory_context: theory_id
    
  # Access Control (Granular)
  access_control:
    - entity_type: string
    - entity_id: UUID
    - permission_grants:
        - grantee: researcher_id or team_id
        - permission_level: enum ["view", "comment", "edit", "admin"]
        - grant_scope: string  # "all_theories", "specific_theory", "published_only"
        - expiration: timestamp
        
  # Conflict Resolution
  merge_conflicts:
    - conflict_type: string
    - conflicting_versions: [version_id]
    - proposed_resolutions: [resolution]
    - consensus_method: string
    - final_resolution: resolution_id
    
  # Publication States
  publication_model:
    - entity: entity_link
    - publication_state: enum ["private", "team", "peer_review", "public"]
    - embargo_until: date
    - citation_requirements: text
    - license: string
```

## The Power of This Model

1. **No Artificial Constraints**: 
   - Events aren't owned by persons/families
   - Relationships can have multiple participants
   - Locations have temporal dimensions
   - Everything can exist in multiple theories

2. **Research Process Native**:
   - Evidence is a first-class object
   - Confidence includes audit trails
   - Negative evidence is recorded
   - Theory versioning is built-in

3. **True Collaboration**:
   - Every change is attributed
   - Peer review is systematic
   - Conflicts are resolved transparently
   - Knowledge builds cumulatively

4. **Standards Compliant by Design**:
   - GPS elements are checkable
   - BCG standards are measurable
   - Research logs are automatic
   - Citations are complete

This model takes everything genealogists need and removes all the artificial limitations that make research difficult in traditional systems.