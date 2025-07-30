# Flexible Link System Specification

## Overview

The Flexible Link System enables researchers to define and track any type of relationship or connection between entities in genealogical research. This goes far beyond GEDCOM's limited relationship model, supporting uncertain, theoretical, and complex real-world connections.

## Core Concepts

### 1. Link Definition
```yaml
FlexibleLink:
  # Identification
  link_id: UUID
  link_type: string  # User-definable
  link_subtype: string  # Optional refinement
  
  # Versioning
  version: integer
  created_date: timestamp
  created_by: researcher_id
  modified_date: timestamp
  modified_by: researcher_id
  
  # Core Properties
  directionality: enum
    # "unidirectional" - A→B only
    # "bidirectional" - A↔B
    # "asymmetric" - A→B different from B→A
    
  temporal_scope:
    start_date: date_object  # Fuzzy dates supported
    end_date: date_object
    date_precision: string
    recurring: boolean
    frequency: string  # "daily", "weekly", "occasional"
    
  spatial_scope:
    places: place_id[]
    geographic_range: polygon
    jurisdiction: string
    
  confidence:
    overall_confidence: percent
    confidence_factors:
      - factor: string
        score: percent
        explanation: text
```

### 2. Link Participants
```yaml
LinkParticipant:
  entity_type: enum
    # "person", "identity", "place", "source", 
    # "event", "organization", "artifact", "dna_match"
    
  entity_id: UUID
  
  role_in_link: string  # Flexible, definable per link type
  
  role_properties:
    primary: boolean
    confidence: percent
    evidence_basis: evidence_id[]
    notes: text
    
  participation_period:
    start_date: date_object
    end_date: date_object
    
  participation_certainty: enum
    # "confirmed", "probable", "possible", "speculative"
```

## Predefined Link Types

### 1. Traditional Relationships (Enhanced)
```yaml
TraditionalLinks:
  Biological:
    - parent_child:
        allows_uncertainty: true
        supports_adoption: true
        supports_step: true
        dna_evidence_integration: true
        
    - sibling:
        subtypes: ["full", "half", "step", "adopted", "foster"]
        maternal_certainty: percent
        paternal_certainty: percent
        
  Legal:
    - marriage:
        ceremony_types: ["civil", "religious", "common_law"]
        validity_status: ["valid", "annulled", "disputed"]
        
    - divorce:
        decree_reference: source_id
        custody_arrangements: text
        
  Social:
    - godparent:
        ceremony_reference: event_id
        denomination: string
        
    - guardian:
        legal_status: boolean
        court_reference: source_id
```

### 2. Uncertain/Theoretical Relationships
```yaml
UncertainLinks:
  Identity:
    - possibly_same_person:
        similarity_score: percent
        matching_facts: fact_id[]
        conflicting_facts: fact_id[]
        
    - probably_not_same:
        distinguishing_factors: text[]
        confidence: percent
        
  Theoretical_Family:
    - hypothetical_parent:
        hypothesis_id: UUID
        supporting_evidence: evidence_id[]
        probability: percent
        
    - potential_sibling:
        shared_characteristics: text[]
        dna_prediction: cm_range
        
  Research_Connections:
    - mentioned_together:
        source_reference: source_id
        context: text
        significance: text
        
    - traveled_together:
        journey_reference: event_id
        departure_place: place_id
        arrival_place: place_id
```

### 3. Complex Social Networks
```yaml
SocialNetworkLinks:
  Economic:
    - business_partner:
        business_name: string
        partnership_type: string
        financial_records: source_id[]
        
    - employer_employee:
        position: string
        salary: monetary_amount
        duration: period
        
    - creditor_debtor:
        amount: monetary_amount
        terms: text
        resolution: text
        
  Military:
    - commanding_officer:
        unit: string
        rank_differential: string
        campaign: string
        
    - messmate:
        ship: string
        voyage: event_id
        
  Religious:
    - congregation_member:
        church: organization_id
        role: string
        sacraments: event_id[]
        
    - spiritual_advisor:
        denomination: string
        period: date_range
        
  Legal:
    - witness_to:
        document_type: string
        document_reference: source_id
        witness_order: integer
        
    - sued_by:
        case_reference: source_id
        court: string
        outcome: text
        
    - executor_of_estate:
        deceased: person_id
        probate_reference: source_id
        co_executors: person_id[]
```

### 4. Geographic/Migration Links
```yaml
GeographicLinks:
  Residence:
    - neighbor:
        property_adjacency: boolean
        distance: measurement
        boundary_disputes: text
        
    - landlord_tenant:
        property: place_id
        rent: monetary_amount
        lease_terms: text
        
  Migration:
    - traveled_with:
        vessel: string
        departure: place_id
        arrival: place_id
        manifest_position: integer
        
    - migration_chain:
        previous_migrant: person_id
        connection_type: string
        time_lag: duration
```

### 5. DNA-Specific Links
```yaml
DNALinks:
  Genetic:
    - dna_match:
        shared_cm: float
        shared_segments: integer
        relationship_predictions: relationship[]
        triangulation_group: group_id
        
    - chromosome_segment_share:
        chromosome: integer
        start_pos: integer
        end_pos: integer
        pile_up_region: boolean
        
  Analysis:
    - cluster_member:
        cluster_id: UUID
        cluster_method: string
        central_figure: boolean
        
    - triangulated_with:
        triangulation_group: group_id
        confirmed_ancestor: person_id
```

## Link Properties & Metadata

### 1. Evidence Basis
```yaml
LinkEvidence:
  evidence_items:
    - evidence_id: UUID
      supports_aspect: string  # Which part of link
      evidence_quality: string
      
  source_citations:
    - source_id: UUID
      relevant_portion: text
      
  reasoning:
    explanation: text
    logic_type: string  # "direct", "inferred", "circumstantial"
    assumptions: text[]
```

### 2. Confidence Scoring
```yaml
ConfidenceModel:
  calculation_method: string
  
  factors:
    - source_reliability: weight
    - information_directness: weight
    - corroboration_level: weight
    - temporal_proximity: weight
    - conflict_absence: weight
    
  threshold_definitions:
    - certain: 95-100%
    - highly_probable: 85-94%
    - probable: 70-84%
    - possible: 50-69%
    - speculative: 30-49%
    - doubtful: 0-29%
```

### 3. Link Lifecycle
```yaml
LinkLifecycle:
  states:
    - proposed:
        by_researcher: researcher_id
        rationale: text
        
    - under_review:
        reviewers: researcher_id[]
        comments: comment[]
        
    - accepted:
        approval_date: date
        approvers: researcher_id[]
        
    - disputed:
        disputants: researcher_id[]
        dispute_reasons: text[]
        
    - rejected:
        rejection_reasons: text[]
        can_repropose: boolean
        
    - superseded:
        replaced_by: link_id
        reason: text
```

## Advanced Features

### 1. Link Templates
```yaml
LinkTemplate:
  template_id: UUID
  template_name: string
  link_type: string
  
  required_participants:
    - entity_type: string
      role: string
      cardinality: string  # "1", "1+", "2", etc.
      
  required_properties:
    - property_name: string
      data_type: string
      validation_rules: JSON
      
  evidence_requirements:
    - evidence_type: string
      minimum_quality: string
```

### 2. Link Inference Engine
```yaml
InferenceRules:
  rule_sets:
    - name: "Family Reconstruction"
      rules:
        - if: "parent_child(A,B) AND parent_child(A,C)"
          then: "sibling(B,C, confidence=0.9)"
          
        - if: "marriage(A,B) AND parent_child(B,C)"
          then: "step_parent_child(A,C, confidence=0.8)"
          
    - name: "Network Analysis"
      rules:
        - if: "witness_to(A,doc1) AND witness_to(B,doc1) 
               AND distance(A.residence, B.residence) < 5mi"
          then: "neighbor(A,B, confidence=0.7)"
```

### 3. Visualization Support
```yaml
LinkVisualization:
  display_properties:
    - line_style: string  # "solid", "dashed", "dotted"
    - line_weight: float  # Based on confidence
    - color_coding: string  # By type, confidence, time
    - directional_arrows: boolean
    - label_placement: string
    
  layout_algorithms:
    - force_directed: boolean
    - hierarchical: boolean
    - temporal: boolean
    - geographic: boolean
    
  filtering:
    - by_type: string[]
    - by_confidence: range
    - by_date: date_range
    - by_participant: entity_id[]
```

## Query Capabilities

### 1. Link Queries
```yaml
QueryExamples:
  # Find all business relationships in 1850s New York
  - type: "business_partner"
    spatial_scope.includes: "New York"
    temporal_scope.overlaps: "1850-1859"
    
  # Find all uncertain parent-child relationships
  - type: "hypothetical_parent"
    confidence.less_than: 70
    
  # Find all links involving a specific person
  - participants.contains:
      entity_id: "person_123"
      
  # Find witness relationships that might indicate family
  - type: "witness_to"
    participants.count: 2
    participants.all.type: "person"
```

### 2. Network Analysis Queries
```yaml
NetworkQueries:
  # Find connection paths between two people
  - shortest_path:
      from: person_id
      to: person_id
      max_hops: integer
      allowed_link_types: string[]
      
  # Find clusters of connected people
  - community_detection:
      algorithm: string
      min_cluster_size: integer
      link_weight_property: string
      
  # Find central figures in networks
  - centrality_analysis:
      measure: string  # "betweenness", "degree", "closeness"
      network_subset: criteria
```

## Integration Points

### 1. GEDCOM Export
```yaml
GEDCOMMapping:
  # Map to standard GEDCOM where possible
  - parent_child → "1 FAMC"
  - marriage → "1 FAMS"
  
  # Use extensions for complex links
  - business_partner → "1 _LINK @L1@"
    "2 _TYPE business_partner"
    "2 _PARTICIPANT @I1@"
    "3 _ROLE partner"
    "2 _PARTICIPANT @I2@"
    "3 _ROLE partner"
```

### 2. Research Integration
```yaml
ResearchIntegration:
  - Link evidence to research logs
  - Track link evolution through theories
  - Connect links to proof arguments
  - GPS compliance for link assertions
```

This Flexible Link System provides the foundation for representing the full complexity of human relationships and connections in genealogical research, supporting everything from traditional family relationships to complex social networks and uncertain theoretical connections.