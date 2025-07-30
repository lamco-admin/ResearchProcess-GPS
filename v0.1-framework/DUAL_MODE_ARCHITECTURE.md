# Dual-Mode Architecture: Mutable Working vs Immutable Published

## Core Architectural Pattern

Every entity in ResearchProcess-GPS operates in two modes:
1. **Working Mode** - Mutable, collaborative, exploratory
2. **Published Mode** - Immutable, citable, versioned

## Universal Entity Wrapper

```yaml
UniversalEntity:
  # Core Identity
  entity_id: UUID
  entity_type: string  # Extensible type system
  schema_version: string
  
  # Dual Mode State
  mode_state:
    current_mode: enum ["working", "published", "hybrid"]
    
    working_state:
      content: json  # Flexible, extensible content
      last_modified: timestamp
      active_editors: [researcher_id]
      change_tracking: event_stream
      
    published_states: [  # Multiple published versions possible
      {
        publication_id: UUID
        version: semantic_version
        published_date: timestamp
        published_by: researcher_id
        content_hash: sha256
        content: json  # Frozen snapshot
        doi: string  # Optional
        citation_text: string
        signature: digital_signature
        retracted: boolean
        superseded_by: publication_id
      }
    ]
    
  # Attribution Layer (Immutable)
  attribution:
    created_by: researcher_id
    created_date: timestamp
    
    contributors: [
      {
        contributor_id: researcher_id
        contribution_type: string  # Extensible
        contribution_percentage: float
        contribution_details: json
        period: temporal_range
        verified: boolean
      }
    ]
    
    intellectual_property:
      rights_holder: actor_id
      license: string
      restrictions: [string]
      citation_requirements: text
      
  # Extension System
  extensions: {
    # Any additional fields can be added here
    # Without breaking core functionality
  }
```

## Citation as Dual-Mode Entity

```yaml
Citation:
  inherits: UniversalEntity
  entity_type: "Citation"
  
  # Working Mode Structure
  working_content:
    citation_elements:
      author: 
        value: string
        locked: boolean  # Can lock individual elements
        last_modified_by: researcher_id
        
      title:
        value: string
        locked: boolean
        variant_forms: [string]
        
      publication_info:
        publisher: string
        place: string
        date: string
        locked_as_unit: boolean
        
      locators:  # Page, URL, etc.
        - type: string
          value: string
          verified: boolean
          
      # Extensible elements
      custom_elements: {
        "archive_series": string
        "digital_ark": string
        # Any field can be added
      }
      
    formatting_rules:
      style_guide: string  # "Chicago", "MLA", etc.
      custom_template: string
      
    quality_checks:
      completeness: percent
      standard_compliance: percent
      broken_links: [string]
      
  # Published Mode Transformations
  publication_rules:
    auto_generate_citation_text: boolean
    freeze_formatting: boolean
    include_access_date: boolean
    
  # Special Citation Features
  citation_features:
    source_linking:
      source_entity: entity_id
      source_published_version: publication_id
      
    citation_chaining:  # Citations citing citations
      cites: [citation_id]
      cited_by: [citation_id]
      
    collaborative_editing:
      edit_proposals: [proposal]
      consensus_required: boolean
      approval_threshold: percent
```

## Making Everything Dual-Mode

```yaml
# Example: Identity as Dual-Mode
Identity:
  inherits: UniversalEntity
  entity_type: "Identity"
  
  working_content:
    # All the flexible research fields
    names: [...]
    events: [...]
    relationships: [...]
    theories: [...]
    
  # When published becomes
  published_snapshot:
    persona_sheet:
      title: "John Smith of Ohio (c.1810-1878)"
      abstract: text
      documented_facts: [fact]
      evidence_summary: text
      research_notes: text
      
    citable_elements:
      preferred_citation: text
      persistent_identifier: string
      included_evidence: [evidence_id]
      
# Example: Theory as Dual-Mode      
Theory:
  inherits: UniversalEntity
  entity_type: "Theory"
  
  working_content:
    hypothesis: text
    evidence_evaluation: json
    test_results: [result]
    
  published_snapshot:
    proof_argument:
      title: string
      argument_text: markdown
      evidence_appendix: [evidence]
      peer_reviews: [review]
      
# Example: Event as Dual-Mode
Event:
  inherits: UniversalEntity
  entity_type: "Event"
  
  working_content:
    event_details: json
    participant_list: [participant]
    evidence_links: [evidence]
    
  published_snapshot:
    event_record:
      canonical_description: text
      verified_participants: [identity]
      supporting_documentation: [doc]
```

## Extension System Architecture

```yaml
ExtensionFramework:
  # Core Extension Registry
  extension_registry:
    registered_extensions: [
      {
        extension_id: "custom_field_type"
        schema: json_schema
        validators: [validator]
        ui_components: [component]
        version: string
      }
    ]
    
  # Field-Level Extensions
  field_extensions:
    # Any entity can have custom fields
    add_field:
      entity_type: string
      field_name: string
      field_schema: json_schema
      field_behavior: behavior_rules
      
    # Example: Add "burial_plot" to Event
    - entity_type: "Event"
      field_name: "burial_plot"
      field_schema: {
        "type": "object",
        "properties": {
          "cemetery": "string",
          "section": "string",
          "plot": "string"
        }
      }
      
  # Behavior Extensions
  behavior_extensions:
    # Add new capabilities to existing entities
    - extension_name: "dna_matching"
      applies_to: ["Identity"]
      adds_methods: ["calculate_match", "predict_relationship"]
      adds_fields: ["dna_kit_numbers", "match_list"]
      
  # Workflow Extensions
  workflow_extensions:
    # Add new workflows
    - workflow_name: "peer_review_process"
      applies_to: ["Theory", "Citation", "Identity"]
      steps: [step_definition]
      permissions: [permission_rule]
```

## Publication & Versioning System

```yaml
PublicationSystem:
  # Publication Process
  publication_workflow:
    initiate_publication:
      entity_id: UUID
      publication_type: enum ["snapshot", "release", "milestone"]
      
    pre_publication_checks:
      - attribution_complete: boolean
      - citations_valid: boolean
      - peer_review_status: string
      - standards_compliance: percent
      
    publication_package:
      content_snapshot: json
      metadata:
        version: semantic_version
        changelog: text
        contributors: [contributor]
        citations_required: text
        
      immutability_proof:
        content_hash: sha256
        merkle_root: string
        blockchain_anchor: string  # Optional
        timestamp_authority: string
        
  # Version Relationships
  version_management:
    version_tree:
      root_version: version_id
      branches: [branch]
      merges: [merge_record]
      
    version_comparison:
      diff_algorithm: string
      change_visualization: format
      impact_analysis: [impact]
      
  # Citation Generation
  citation_generator:
    formats: {
      "academic": template,
      "genealogical": template,
      "legal": template,
      "custom": template
    }
    
    persistent_identifiers:
      doi_minting: boolean
      handle_system: boolean
      internal_permalink: string
```

## Extensibility Patterns

```yaml
ExtensibilityPatterns:
  # Plugin Architecture
  plugin_system:
    plugin_types:
      - entity_extensions  # New fields/entities
      - validator_plugins  # Custom validation
      - transformer_plugins  # Import/export
      - ui_plugins  # Custom interfaces
      - workflow_plugins  # New processes
      
  # Schema Evolution
  schema_evolution:
    backward_compatibility: required
    migration_scripts: automatic
    deprecation_warnings: boolean
    
  # Custom Entity Types
  custom_entities:
    define_new_entity:
      entity_type: "MilitaryService"
      extends: "Event"
      custom_fields: [field_definition]
      custom_behaviors: [behavior]
      
  # Integration Points
  integration_apis:
    rest_api: openapi_spec
    graphql_api: schema
    plugin_sdk: typescript
    webhook_system: event_based
```

## Benefits of This Architecture

1. **Everything is Citable** - Any entity can be published and cited
2. **Collaborative AND Authoritative** - Work together, publish definitively  
3. **Infinitely Extensible** - Add any field, any entity, any workflow
4. **Version Control Native** - Every change tracked, every version preserved
5. **Standards Compliant** - Attribution, versioning, citation built into core

This dual-mode architecture ensures that ResearchProcess-GPS can handle both the messy collaborative research process AND the formal requirements of academic/professional publication.